use super::bind::*;
use super::search::*;

#[derive(Debug, Clone)]
pub struct LdapMessage {
    pub message_id: i32,
    pub protocol_op: ProtocolOp,
}

#[derive(Debug, Clone)]
pub enum ProtocolOp {
    BindRequest(BindRequest),
    BindResponse(BindResponse),
    SearchRequest(SearchRequest),
    SearchResultEntry(SearchResultEntry),
    SearchResultDone(SearchResultDone),
    UnbindRequest,
}
