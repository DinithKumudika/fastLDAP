use crate::utils::error::Result;
use crate::protocol::modify::{DeleteRequest, DeleteResponse};
use crate::protocol::result::{LdapResult, ResultCode};
use crate::store::backend::Backend;

pub async fn handle_delete<B: Backend>(req: &DeleteRequest, backend: &B) -> Result<DeleteResponse> {
    match backend.delete_entry(&req.entry).await {
        Ok(_) => Ok(DeleteResponse {
            result: LdapResult::success(),
        }),
        Err(e) => {
            let code = if e.to_string().contains("non-leaf") {
                ResultCode::NotAllowedOnNonLeaf
            } else {
                ResultCode::NoSuchObject
            };
            
            Ok(DeleteResponse {
                result: LdapResult {
                    code,
                    matched_dn: req.entry.clone(),
                    diagnostic_message: e.to_string(),
                },
            })
        }
    }
}
