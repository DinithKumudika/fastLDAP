use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;
use crate::utils::error::{Result, LdapError};
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

    async fn delete_entry(&self, dn: &str) -> Result<()> {
        let dn_lower = dn.to_lowercase();
        let suffix = format!(",{}", dn_lower);
        if self.entries.iter().any(|kv| kv.key().ends_with(&suffix)) {
            return Err(LdapError::Operation("Not allowed on non-leaf".into()));
        }
        
        if self.entries.remove(&dn_lower).is_none() {
            return Err(LdapError::Operation("No such object".into()));
        }
        Ok(())
    }

    async fn modify_entry(&self, dn: &str, changes: Vec<crate::protocol::modify::ModifyChange>) -> Result<()> {
        let dn_lower = dn.to_lowercase();
        let entry_arc = self.entries.get(&dn_lower).ok_or_else(|| LdapError::Operation("No such object".into()))?.clone();
        
        let mut entry = entry_arc.write();
        
        for change in changes {
            let attr_name = change.modification.type_.to_lowercase();
            let vals = change.modification.vals.iter().map(|v| String::from_utf8_lossy(v).into_owned()).collect::<Vec<_>>();
            
            match change.operation {
                crate::protocol::modify::ModifyOperation::Add => {
                    let entry_vals = entry.attributes.entry(attr_name).or_insert_with(Vec::new);
                    for val in vals {
                        if !entry_vals.contains(&val) {
                            entry_vals.push(val);
                        }
                    }
                }
                crate::protocol::modify::ModifyOperation::Delete => {
                    if vals.is_empty() {
                        entry.attributes.remove(&attr_name);
                    } else {
                        if let Some(entry_vals) = entry.attributes.get_mut(&attr_name) {
                            entry_vals.retain(|v| !vals.contains(v));
                        }
                    }
                }
                crate::protocol::modify::ModifyOperation::Replace => {
                    if vals.is_empty() {
                        entry.attributes.remove(&attr_name);
                    } else {
                        entry.attributes.insert(attr_name, vals);
                    }
                }
            }
        }
        
        Ok(())
    }

    async fn moddn_entry(&self, dn: &str, new_rdn: &str, delete_old_rdn: bool, new_superior: Option<&str>) -> Result<()> {
        let dn_lower = dn.to_lowercase();
        let new_rdn_lower = new_rdn.to_lowercase();
        
        let old_superior = if let Some(idx) = dn_lower.find(',') {
            dn_lower[idx+1..].to_string()
        } else {
            "".to_string()
        };
        
        let superior = if let Some(sup) = new_superior {
            sup.to_lowercase()
        } else {
            old_superior
        };
        
        let new_dn = if superior.is_empty() {
            new_rdn_lower.clone()
        } else {
            format!("{},{}", new_rdn_lower, superior)
        };
        
        if self.entries.contains_key(&new_dn) {
            return Err(LdapError::Operation("Entry already exists".into()));
        }
        
        let suffix = format!(",{}", dn_lower);
        if self.entries.iter().any(|kv| kv.key().ends_with(&suffix)) {
            return Err(LdapError::Operation("Not allowed on non-leaf for ModDN".into()));
        }
        
        let (_key, entry_arc) = self.entries.remove(&dn_lower).ok_or_else(|| LdapError::Operation("No such object".into()))?;
        
        {
            let mut entry = entry_arc.write();
            
            let original_superior = if let Some(idx) = dn.find(',') {
                &dn[idx+1..]
            } else {
                ""
            };
            
            let final_superior = if let Some(sup) = new_superior {
                sup
            } else {
                original_superior
            };
            
            entry.dn = if final_superior.is_empty() {
                new_rdn.to_string()
            } else {
                format!("{},{}", new_rdn, final_superior)
            };
            
            let parts: Vec<&str> = new_rdn.splitn(2, '=').collect();
            if parts.len() == 2 {
                let attr = parts[0].to_lowercase();
                let val = parts[1].to_string();
                
                let attr_vals = entry.attributes.entry(attr).or_insert_with(Vec::new);
                if !attr_vals.contains(&val) {
                    attr_vals.push(val);
                }
            }
            
            if delete_old_rdn {
                let old_rdn_parts: Vec<&str> = dn.splitn(2, ',').collect();
                if !old_rdn_parts.is_empty() {
                    let old_rdn = old_rdn_parts[0];
                    let rdn_kv: Vec<&str> = old_rdn.splitn(2, '=').collect();
                    if rdn_kv.len() == 2 {
                        let attr = rdn_kv[0].to_lowercase();
                        let val = rdn_kv[1].to_string();
                        
                        if let Some(vals) = entry.attributes.get_mut(&attr) {
                            vals.retain(|v| !v.eq_ignore_ascii_case(&val));
                        }
                    }
                }
            }
        }
        
        self.entries.insert(new_dn, entry_arc);
        
        Ok(())
    }
}
