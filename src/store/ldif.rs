use crate::error::Result;
use super::entry::DirectoryEntry;
use super::backend::Backend;

// Minimal LDIF parser to load initial data
pub async fn load_ldif<B: Backend>(backend: &B, content: &str) -> Result<()> {
    let mut current_entry: Option<DirectoryEntry> = None;
    
    for line in content.lines() {
        if line.trim().is_empty() {
            if let Some(entry) = current_entry.take() {
                backend.add_entry(entry).await?;
            }
            continue;
        }
        
        // Handle line continuations or comments if needed, but for simplicity we ignore them here
        if line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_lowercase();
            let value = parts[1].trim().to_string();
            
            if key == "dn" {
                if let Some(entry) = current_entry.take() {
                    backend.add_entry(entry).await?;
                }
                current_entry = Some(DirectoryEntry::new(value));
            } else if let Some(ref mut entry) = current_entry {
                if key == "objectclass" {
                    entry.object_classes.push(value.clone());
                }
                entry.attributes.entry(key).or_insert_with(Vec::new).push(value);
            }
        }
    }
    
    if let Some(entry) = current_entry {
        backend.add_entry(entry).await?;
    }
    
    Ok(())
}
