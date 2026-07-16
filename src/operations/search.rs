use crate::error::Result;
use crate::protocol::search::{SearchRequest, SearchResultEntry, PartialAttribute, Scope};
use crate::store::backend::Backend;
use crate::store::entry::DirectoryEntry;
use crate::filter::types::*;

pub async fn handle_search<B: Backend>(req: &SearchRequest, backend: &B) -> Result<Vec<SearchResultEntry>> {
    let mut results = Vec::new();
    let all_entries = backend.get_all_entries().await?;
    let base_dn = req.base_object.to_lowercase();
    
    for entry_arc in all_entries {
        let entry = entry_arc.read();
        
        let dn_lower = entry.dn.to_lowercase();
        
        // Scope check
        let in_scope = match req.scope {
            Scope::BaseObject => dn_lower == base_dn,
            Scope::SingleLevel => {
                if base_dn.is_empty() {
                    !dn_lower.contains(',')
                } else {
                    dn_lower.ends_with(&base_dn) && dn_lower.len() > base_dn.len() && !dn_lower[..dn_lower.len() - base_dn.len() - 1].contains(',')
                }
            },
            Scope::WholeSubtree => {
                if base_dn.is_empty() {
                    true
                } else {
                    dn_lower.ends_with(&base_dn)
                }
            }
        };
        
        if in_scope && evaluate_filter(&req.filter, &entry) {
            let mut attributes = Vec::new();
            
            let return_all = req.attributes.is_empty() || req.attributes.iter().any(|a| a == "*");
            
            for (k, v) in &entry.attributes {
                if return_all || req.attributes.iter().any(|a| a.eq_ignore_ascii_case(k)) {
                    let mut vals = Vec::new();
                    if !req.types_only {
                        for val in v {
                            vals.push(val.as_bytes().to_vec());
                        }
                    }
                    attributes.push(PartialAttribute {
                        type_: k.clone(),
                        vals,
                    });
                }
            }
            
            if return_all || req.attributes.iter().any(|a| a.eq_ignore_ascii_case("objectclass")) {
                let mut vals = Vec::new();
                if !req.types_only {
                    for oc in &entry.object_classes {
                        vals.push(oc.as_bytes().to_vec());
                    }
                }
                attributes.push(PartialAttribute {
                    type_: "objectClass".to_string(),
                    vals,
                });
            }
            
            results.push(SearchResultEntry {
                object_name: entry.dn.clone(),
                attributes,
            });
        }
    }
    
    Ok(results)
}

fn evaluate_filter(filter: &Filter, entry: &DirectoryEntry) -> bool {
    match filter {
        Filter::And(filters) => filters.iter().all(|f| evaluate_filter(f, entry)),
        Filter::Or(filters) => filters.iter().any(|f| evaluate_filter(f, entry)),
        Filter::Not(f) => !evaluate_filter(f, entry),
        Filter::EqualityMatch(attr, val) => {
            let attr_lower = attr.to_lowercase();
            if attr_lower == "objectclass" {
                entry.object_classes.iter().any(|oc| oc.eq_ignore_ascii_case(val))
            } else if let Some(vals) = entry.attributes.get(&attr_lower) {
                vals.iter().any(|v| v.eq_ignore_ascii_case(val))
            } else {
                false
            }
        },
        Filter::Present(attr) => {
            let attr_lower = attr.to_lowercase();
            if attr_lower == "objectclass" {
                !entry.object_classes.is_empty()
            } else {
                entry.attributes.contains_key(&attr_lower)
            }
        },
        Filter::Substring(attr, sub) => {
            let attr_lower = attr.to_lowercase();
            let check_val = |v: &str| -> bool {
                let v_lower = v.to_lowercase();
                if let Some(initial) = &sub.initial {
                    if !v_lower.starts_with(&initial.to_lowercase()) {
                        return false;
                    }
                }
                if let Some(final_) = &sub.final_ {
                    if !v_lower.ends_with(&final_.to_lowercase()) {
                        return false;
                    }
                }
                true
            };
            
            if attr_lower == "objectclass" {
                entry.object_classes.iter().any(|oc| check_val(oc))
            } else if let Some(vals) = entry.attributes.get(&attr_lower) {
                vals.iter().any(|v| check_val(v))
            } else {
                false
            }
        },
        _ => false,
    }
}
