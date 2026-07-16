use std::sync::Arc;
use parking_lot::RwLock;
use crate::error::Result;
use super::entry::DirectoryEntry;

pub trait Backend: Send + Sync {
    fn get_entry(&self, dn: &str) -> impl std::future::Future<Output = Result<Option<Arc<RwLock<DirectoryEntry>>>>> + Send;
    fn add_entry(&self, entry: DirectoryEntry) -> impl std::future::Future<Output = Result<()>> + Send;
    fn get_all_entries(&self) -> impl std::future::Future<Output = Result<Vec<Arc<RwLock<DirectoryEntry>>>>> + Send;
}
