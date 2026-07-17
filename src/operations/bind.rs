use crate::utils::error::Result;
use crate::protocol::bind::{BindRequest, AuthenticationChoice, BindResponse};
use crate::protocol::result::{LdapResult, ResultCode};
use crate::store::backend::Backend;

pub async fn handle_bind<B: Backend>(req: &BindRequest, backend: &B) -> Result<BindResponse> {
    if let AuthenticationChoice::Simple(password) = &req.authentication {
        if let Some(entry_arc) = backend.get_entry(&req.name).await? {
            let entry = entry_arc.read();
            if let Some(user_password_vals) = entry.attributes.get("userpassword") {
                if user_password_vals.iter().any(|v| v == password) {
                    return Ok(BindResponse {
                        result: LdapResult::success(),
                    });
                }
            }
        }
        return Ok(BindResponse {
            result: LdapResult {
                code: ResultCode::InvalidCredentials,
                matched_dn: String::new(),
                diagnostic_message: "Invalid credentials".into(),
            }
        });
    }
    
    Ok(BindResponse {
        result: LdapResult {
            code: ResultCode::AuthMethodNotSupported,
            matched_dn: String::new(),
            diagnostic_message: "Only simple auth supported".into(),
        }
    })
}
