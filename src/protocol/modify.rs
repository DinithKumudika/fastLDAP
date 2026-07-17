use super::result::LdapResult;
use crate::protocol::search::PartialAttribute;

#[derive(Debug, Clone)]
pub struct AddRequest {
    pub entry: String,
    pub attributes: Vec<PartialAttribute>,
}

#[derive(Debug, Clone)]
pub struct AddResponse {
    pub result: LdapResult,
}

#[derive(Debug, Clone)]
pub struct DeleteRequest {
    pub entry: String,
}

#[derive(Debug, Clone)]
pub struct DeleteResponse {
    pub result: LdapResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifyOperation {
    Add = 0,
    Delete = 1,
    Replace = 2,
}

#[derive(Debug, Clone)]
pub struct ModifyChange {
    pub operation: ModifyOperation,
    pub modification: PartialAttribute,
}

#[derive(Debug, Clone)]
pub struct ModifyRequest {
    pub object: String,
    pub changes: Vec<ModifyChange>,
}

#[derive(Debug, Clone)]
pub struct ModifyResponse {
    pub result: LdapResult,
}

#[derive(Debug, Clone)]
pub struct ModDNRequest {
    pub entry: String,
    pub new_rdn: String,
    pub delete_old_rdn: bool,
    pub new_superior: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModDNResponse {
    pub result: LdapResult,
}
