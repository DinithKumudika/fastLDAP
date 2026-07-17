use super::bind::*;
use super::search::*;
use super::modify::*;

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
    AddRequest(AddRequest),
    AddResponse(AddResponse),
    DeleteRequest(DeleteRequest),
    DeleteResponse(DeleteResponse),
    ModifyRequest(ModifyRequest),
    ModifyResponse(ModifyResponse),
    ModDNRequest(ModDNRequest),
    ModDNResponse(ModDNResponse),
}
