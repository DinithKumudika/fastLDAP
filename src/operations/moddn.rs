use crate::utils::error::Result;
use crate::protocol::modify::{ModDNRequest, ModDNResponse};
use crate::protocol::result::{LdapResult, ResultCode};
use crate::store::backend::Backend;

pub async fn handle_moddn<B: Backend>(req: &ModDNRequest, backend: &B) -> Result<ModDNResponse> {
    match backend.moddn_entry(&req.entry, &req.new_rdn, req.delete_old_rdn, req.new_superior.as_deref()).await {
        Ok(_) => Ok(ModDNResponse {
            result: LdapResult::success(),
        }),
        Err(e) => {
            let code = if e.to_string().contains("already exists") {
                ResultCode::EntryAlreadyExists
            } else if e.to_string().contains("non-leaf") {
                ResultCode::NotAllowedOnNonLeaf
            } else {
                ResultCode::NoSuchObject
            };
            
            Ok(ModDNResponse {
                result: LdapResult {
                    code,
                    matched_dn: req.entry.clone(),
                    diagnostic_message: e.to_string(),
                },
            })
        }
    }
}
