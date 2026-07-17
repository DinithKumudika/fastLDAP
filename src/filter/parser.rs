use crate::utils::error::{Result, LdapError};
use super::types::*;

pub fn parse_filter(input: &str) -> Result<Filter> {
    let (filter, remaining) = parse_filter_node(input.trim())?;
    if !remaining.is_empty() {
        return Err(LdapError::InvalidFilter("Extra characters after filter".into()));
    }
    Ok(filter)
}

fn parse_filter_node(input: &str) -> Result<(Filter, &str)> {
    if !input.starts_with('(') {
        return Err(LdapError::InvalidFilter("Filter must start with '('".into()));
    }
    
    let inner = &input[1..];
    
    match inner.chars().next() {
        Some('&') => parse_list(&inner[1..], Filter::And),
        Some('|') => parse_list(&inner[1..], Filter::Or),
        Some('!') => {
            let (filter, remaining) = parse_filter_node(&inner[1..].trim_start())?;
            if !remaining.starts_with(')') {
                return Err(LdapError::InvalidFilter("Missing closing ')' for NOT".into()));
            }
            Ok((Filter::Not(Box::new(filter)), &remaining[1..]))
        }
        Some(_) => parse_item(inner),
        None => Err(LdapError::InvalidFilter("Unexpected EOF".into()))
    }
}

fn parse_list<F>(mut input: &str, constructor: F) -> Result<(Filter, &str)>
where
    F: Fn(Vec<Filter>) -> Filter
{
    let mut filters = Vec::new();
    input = input.trim_start();
    while input.starts_with('(') {
        let (filter, remaining) = parse_filter_node(input)?;
        filters.push(filter);
        input = remaining.trim_start();
    }
    if !input.starts_with(')') {
        return Err(LdapError::InvalidFilter("Missing closing ')' for list".into()));
    }
    Ok((constructor(filters), &input[1..]))
}

fn parse_item(input: &str) -> Result<(Filter, &str)> {
    let end_idx = input.find(')').ok_or_else(|| LdapError::InvalidFilter("Missing closing ')'".into()))?;
    let item_str = &input[..end_idx];
    let remaining = &input[end_idx + 1..];
    
    if let Some(eq_idx) = item_str.find('=') {
        let attr = item_str[..eq_idx].trim().to_string();
        let val = item_str[eq_idx + 1..].trim().to_string();
        
        if val == "*" {
            return Ok((Filter::Present(attr), remaining));
        } else if val.contains('*') {
            let parts: Vec<&str> = val.split('*').collect();
            let initial = if !parts[0].is_empty() { Some(parts[0].to_string()) } else { None };
            let final_ = if !parts.last().unwrap().is_empty() { Some(parts.last().unwrap().to_string()) } else { None };
            let mut any = Vec::new();
            for i in 1..parts.len()-1 {
                if !parts[i].is_empty() {
                    any.push(parts[i].to_string());
                }
            }
            return Ok((Filter::Substring(attr, SubstringFilter { initial, any, final_ }), remaining));
        }
        Ok((Filter::EqualityMatch(attr, val), remaining))
    } else {
        Err(LdapError::InvalidFilter("Invalid filter item (no '=')".into()))
    }
}
