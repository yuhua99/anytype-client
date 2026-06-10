use anyhow::{Result, anyhow};

use crate::{models::Tag, output::eprint_status};

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
        1 => {
            eprint_status(format!(
                "note: resolved tag '{name_or_key}' by partial match to '{}' ({})",
                matches[0].name, matches[0].id
            ));
            Ok(matches[0].id.clone())
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::IconColor;

    fn tag(id: &str, key: &str, name: &str) -> Tag {
        Tag {
            id: id.into(),
            key: key.into(),
            name: name.into(),
            color: IconColor::Yellow,
            object: String::new(),
            extra: Default::default(),
        }
    }

    #[test]
    fn resolves_by_id_key_name_then_fuzzy() {
        let tags = vec![tag("t1", "done", "Done"), tag("t2", "blocked", "Blocked")];

        assert_eq!(resolve_tag_from_list(&tags, "t1").unwrap(), "t1");
        assert_eq!(resolve_tag_from_list(&tags, "DONE").unwrap(), "t1");
        assert_eq!(resolve_tag_from_list(&tags, "Blocked").unwrap(), "t2");
        assert_eq!(resolve_tag_from_list(&tags, "block").unwrap(), "t2");
    }

    #[test]
    fn errors_on_no_match_and_ambiguity() {
        let tags = vec![tag("t1", "done", "Done"), tag("t2", "redone", "Redone")];

        assert!(
            resolve_tag_from_list(&tags, "missing")
                .unwrap_err()
                .to_string()
                .contains("tag not found")
        );
        assert!(
            resolve_tag_from_list(&tags, "don")
                .unwrap_err()
                .to_string()
                .contains("tag ambiguous")
        );
    }
}
