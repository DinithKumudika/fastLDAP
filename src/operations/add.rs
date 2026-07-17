use crate::utils::error::Result;
use crate::protocol::modify::{AddRequest, AddResponse};
use crate::protocol::result::{LdapResult, ResultCode};
use crate::store::backend::Backend;
use crate::store::entry::DirectoryEntry;

pub async fn handle_add<B: Backend>(req: &AddRequest, backend: &B) -> Result<AddResponse> {
    let mut entry = DirectoryEntry::new(req.entry.clone());
    
    for attr in &req.attributes {
        let key = attr.type_.to_lowercase();
        let vals: Vec<String> = attr.vals.iter().map(|v| String::from_utf8_lossy(v).into_owned()).collect();
        
        if key == "objectclass" {
            entry.object_classes.extend(vals);
        } else {
            entry.attributes.insert(key, vals);
        }
    }
    
    match backend.add_entry(entry).await {
        Ok(_) => Ok(AddResponse {
            result: LdapResult::success(),
        }),
        Err(e) => Ok(AddResponse {
            result: LdapResult {
                code: ResultCode::EntryAlreadyExists,
                matched_dn: req.entry.clone(),
                diagnostic_message: e.to_string(),
            },
        }),
    }
}
