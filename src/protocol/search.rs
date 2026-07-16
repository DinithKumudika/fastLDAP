use crate::filter::types::Filter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    BaseObject = 0,
    SingleLevel = 1,
    WholeSubtree = 2,
}

impl TryFrom<i32> for Scope {
    type Error = String;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Scope::BaseObject),
            1 => Ok(Scope::SingleLevel),
            2 => Ok(Scope::WholeSubtree),
            _ => Err(format!("Invalid scope: {}", v)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerefAliases {
    Never = 0,
    InSearching = 1,
    FindingBaseObj = 2,
    Always = 3,
}

impl TryFrom<i32> for DerefAliases {
    type Error = String;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(DerefAliases::Never),
            1 => Ok(DerefAliases::InSearching),
            2 => Ok(DerefAliases::FindingBaseObj),
            3 => Ok(DerefAliases::Always),
            _ => Err(format!("Invalid deref_aliases: {}", v)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchRequest {
    pub base_object: String,
    pub scope: Scope,
    pub deref_aliases: DerefAliases,
    pub size_limit: i32,
    pub time_limit: i32,
    pub types_only: bool,
    pub filter: Filter,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResultEntry {
    pub object_name: String,
    pub attributes: Vec<PartialAttribute>,
}

#[derive(Debug, Clone)]
pub struct PartialAttribute {
    pub type_: String,
    pub vals: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct SearchResultDone {
    pub result: crate::protocol::result::LdapResult,
}
