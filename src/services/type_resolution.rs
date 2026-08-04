use crate::{api::AnytypeClient, models::ObjectType, output::eprint_status, services::Match};
use anyhow::{Result, anyhow};

pub(crate) struct ResolvedType {
    id: String,
    pub(crate) key: String,
    default_template_id: Option<String>,
}

pub(crate) async fn resolve_type(
    client: &AnytypeClient,
    space_id: &str,
    name_or_key: &str,
) -> Result<ResolvedType> {
    let types = client.types(space_id).await?.data;
    let r#type = match resolve_type_from_list(&types, name_or_key)? {
        Match::Exact(r#type) => r#type,
        Match::Fuzzy(r#type) => {
            eprint_status(format!(
                "note: resolved type '{name_or_key}' by partial match to '{}' ({})",
                r#type.name, r#type.id
            ));
            r#type
        }
    };

    Ok(ResolvedType {
        id: r#type.id.clone(),
        key: r#type.key.clone(),
        default_template_id: r#type.default_template_id.clone(),
    })
}

pub(crate) async fn resolve_default_template_for_type(
    client: &AnytypeClient,
    space_id: &str,
    r#type: &ResolvedType,
) -> Result<Option<String>> {
    if let Some(id) = r#type
        .default_template_id
        .as_deref()
        .filter(|id| !id.is_empty())
    {
        return Ok(Some(id.to_string()));
    }
    let templates = client.templates(space_id, &r#type.id).await?.data;
    Ok(match templates.as_slice() {
        [only] => Some(only.id.clone()),
        _ => None,
    })
}

fn resolve_type_from_list<'a>(
    types: &'a [ObjectType],
    name_or_key: &str,
) -> Result<Match<&'a ObjectType>> {
    if let Some(r#type) = types.iter().find(|r#type| r#type.id == name_or_key) {
        return Ok(Match::Exact(r#type));
    }
    if let Some(r#type) = types
        .iter()
        .find(|r#type| r#type.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(Match::Exact(r#type));
    }
    if let Some(r#type) = types
        .iter()
        .find(|r#type| r#type.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(Match::Exact(r#type));
    }

    let needle = name_or_key.to_lowercase();
    let matches: Vec<_> = types
        .iter()
        .filter(|r#type| {
            r#type.name.to_lowercase().contains(&needle)
                || r#type.key.to_lowercase().contains(&needle)
        })
        .collect();

    match matches.len() {
        0 => Err(anyhow!(
            "type not found: '{name_or_key}' matched no type name or key (run `anyclient types list <space>` to list names and keys)"
        )),
        1 => Ok(Match::Fuzzy(matches[0])),
        _ => Err(anyhow!(
            "type ambiguous: '{name_or_key}' matched multiple: {}",
            matches
                .iter()
                .map(|r#type| format!("{} ({})", r#type.name, r#type.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ObjectLayout;

    fn object_type(id: &str, key: &str, name: &str) -> ObjectType {
        ObjectType {
            id: id.into(),
            key: key.into(),
            name: name.into(),
            layout: ObjectLayout::Basic,
            plural_name: String::new(),
            description: String::new(),
            default_template_id: None,
            archived: false,
            is_hidden: false,
            property_definitions: Vec::new(),
            icon: None,
            extra: Default::default(),
        }
    }

    #[test]
    fn resolves_by_id_key_and_name() {
        let types = vec![
            object_type("type-1", "note", "Meeting Note"),
            object_type("type-2", "task", "Task"),
        ];

        assert!(matches!(
            resolve_type_from_list(&types, "type-1").unwrap(),
            Match::Exact(r#type) if r#type.key == "note"
        ));
        assert!(matches!(
            resolve_type_from_list(&types, "TASK").unwrap(),
            Match::Exact(r#type) if r#type.id == "type-2"
        ));
        assert!(matches!(
            resolve_type_from_list(&types, "mEeTiNg NoTe").unwrap(),
            Match::Exact(r#type) if r#type.id == "type-1"
        ));
    }

    #[test]
    fn unique_partial_match_is_fuzzy() {
        let types = vec![object_type("type-1", "note", "Note")];

        assert!(matches!(
            resolve_type_from_list(&types, "not").unwrap(),
            Match::Fuzzy(r#type) if r#type.id == "type-1"
        ));
    }

    #[test]
    fn errors_on_no_match_and_ambiguity() {
        let types = vec![
            object_type("type-1", "note_one", "Note One"),
            object_type("type-2", "note_two", "Note Two"),
        ];

        assert!(
            resolve_type_from_list(&types, "missing")
                .unwrap_err()
                .to_string()
                .contains("matched no type name or key")
        );
        assert!(
            resolve_type_from_list(&types, "note_")
                .unwrap_err()
                .to_string()
                .contains(
                    "type ambiguous: 'note_' matched multiple: Note One (type-1), Note Two (type-2)"
                )
        );
    }
}
