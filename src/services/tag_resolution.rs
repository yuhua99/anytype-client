use anyhow::{Result, anyhow};

use crate::models::Tag;

pub(crate) fn resolve_tag_from_list(tags: &[Tag], name_or_key: &str) -> Result<String> {
    if let Some(tag) = tags.iter().find(|tag| tag.id == name_or_key) {
        return Ok(tag.id.clone());
    }
    if let Some(tag) = tags
        .iter()
        .find(|tag| tag.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(tag.id.clone());
    }
    if let Some(tag) = tags
        .iter()
        .find(|tag| tag.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(tag.id.clone());
    }

    let needle = name_or_key.to_lowercase();
    let matches: Vec<_> = tags
        .iter()
        .filter(|tag| {
            tag.name.to_lowercase().contains(&needle) || tag.key.to_lowercase().contains(&needle)
        })
        .collect();

    match matches.len() {
        0 => Err(anyhow!(
            "tag not found: '{name_or_key}' matched no tag name or key"
        )),
        1 => Ok(matches[0].id.clone()),
        _ => Err(anyhow!(
            "tag ambiguous: '{name_or_key}' matched multiple: {}",
            matches
                .iter()
                .map(|tag| format!("{} ({})", tag.name, tag.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}
