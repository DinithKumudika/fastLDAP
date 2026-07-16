use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub dn: String,
    // Note: in a real implementation we would preserve case for the key but do case-insensitive lookups.
    // For simplicity, we assume lowercased keys here.
    pub attributes: HashMap<String, Vec<String>>,
    pub object_classes: Vec<String>,
}

impl DirectoryEntry {
    pub fn new(dn: String) -> Self {
        Self {
            dn,
            attributes: HashMap::new(),
            object_classes: Vec::new(),
        }
    }
}
