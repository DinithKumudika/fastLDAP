use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;
use crate::error::{Result, LdapError};
use super::entry::DirectoryEntry;
use super::backend::Backend;

pub struct MemoryBackend {
    entries: DashMap<String, Arc<RwLock<DirectoryEntry>>>,
}

impl MemoryBackend {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }
}

impl Backend for MemoryBackend {
    async fn get_entry(&self, dn: &str) -> Result<Option<Arc<RwLock<DirectoryEntry>>>> {
        let dn_lower = dn.to_lowercase();
        if let Some(entry_ref) = self.entries.get(&dn_lower) {
            Ok(Some(entry_ref.clone()))
        } else {
            Ok(None)
        }
    }
    
    async fn add_entry(&self, entry: DirectoryEntry) -> Result<()> {
        let dn_lower = entry.dn.to_lowercase();
        if self.entries.contains_key(&dn_lower) {
            return Err(LdapError::Operation("Entry already exists".into()));
        }
        self.entries.insert(dn_lower, Arc::new(RwLock::new(entry)));
        Ok(())
    }
    
    async fn get_all_entries(&self) -> Result<Vec<Arc<RwLock<DirectoryEntry>>>> {
        let mut all = Vec::new();
        for kv in self.entries.iter() {
            all.push(kv.value().clone());
        }
        Ok(all)
    }
}
