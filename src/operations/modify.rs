use crate::utils::error::Result;
use crate::protocol::modify::{ModifyRequest, ModifyResponse};
use crate::protocol::result::{LdapResult, ResultCode};
use crate::store::backend::Backend;

pub async fn handle_modify<B: Backend>(req: &ModifyRequest, backend: &B) -> Result<ModifyResponse> {
    match backend.modify_entry(&req.object, req.changes.clone()).await {
        Ok(_) => Ok(ModifyResponse {
            result: LdapResult::success(),
        }),
        Err(e) => Ok(ModifyResponse {
            result: LdapResult {
                code: ResultCode::NoSuchObject,
                matched_dn: req.object.clone(),
                diagnostic_message: e.to_string(),
            },
        }),
    }
}
