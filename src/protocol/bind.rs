#[derive(Debug, Clone)]
pub struct BindRequest {
    pub version: i32,
    pub name: String,
    pub authentication: AuthenticationChoice,
}

#[derive(Debug, Clone)]
pub enum AuthenticationChoice {
    Simple(String),
}

#[derive(Debug, Clone)]
pub struct BindResponse {
    pub result: crate::protocol::result::LdapResult,
}
