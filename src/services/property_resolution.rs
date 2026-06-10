use anyhow::{Result, anyhow};

use crate::{api::AnytypeClient, models::Property, output::eprint_status, services::Match};

/// Resolve a user-supplied id/key/name to the full property, so callers can
/// use the canonical `key` for object property reads/writes and the `id` for
/// tag endpoints.
pub(crate) async fn resolve_property(
    client: &AnytypeClient,
    space_id: &str,
    name_or_key: &str,
) -> Result<Property> {
    let properties = client.properties(space_id).await?.data;
    match resolve_property_from_list(&properties, name_or_key)? {
        Match::Exact(property) => Ok(property.clone()),
        Match::Fuzzy(property) => {
            eprint_status(format!(
                "note: resolved property '{name_or_key}' by partial match to '{}' ({})",
                property.name, property.id
            ));
            Ok(property.clone())
        }
    }
}

fn resolve_property_from_list<'a>(
    properties: &'a [Property],
    name_or_key: &str,
) -> Result<Match<&'a Property>> {
    if let Some(property) = properties
        .iter()
        .find(|property| property.id == name_or_key)
    {
        return Ok(Match::Exact(property));
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(Match::Exact(property));
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(Match::Exact(property));
    }

    let needle = name_or_key.to_lowercase();
    let matches: Vec<_> = properties
        .iter()
        .filter(|property| {
            property.name.to_lowercase().contains(&needle)
                || property.key.to_lowercase().contains(&needle)
        })
        .collect();

    match matches.len() {
        0 => Err(anyhow!(
            "property not found: '{name_or_key}' matched no property name or key"
        )),
        1 => Ok(Match::Fuzzy(matches[0])),
        _ => Err(anyhow!(
            "property ambiguous: '{name_or_key}' matched multiple: {}",
            matches
                .iter()
                .map(|property| format!("{} ({})", property.name, property.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PropertyFormat;

    fn property(id: &str, key: &str, name: &str) -> Property {
        Property {
            id: id.into(),
            key: key.into(),
            name: name.into(),
            format: PropertyFormat::MultiSelect,
            object: String::new(),
            extra: Default::default(),
        }
    }

    #[test]
    fn resolves_by_id_key_and_name() {
        let properties = vec![
            property("prop-1", "tag", "Tag"),
            property("prop-2", "status", "Status"),
        ];

        assert!(matches!(
            resolve_property_from_list(&properties, "prop-1").unwrap(),
            Match::Exact(property) if property.key == "tag"
        ));
        assert!(matches!(
            resolve_property_from_list(&properties, "TAG").unwrap(),
            Match::Exact(property) if property.id == "prop-1"
        ));
        assert!(matches!(
            resolve_property_from_list(&properties, "Status").unwrap(),
            Match::Exact(property) if property.id == "prop-2"
        ));
    }

    #[test]
    fn display_name_resolves_to_canonical_key() {
        let properties = vec![property("prop-1", "tag", "Tag")];

        assert!(matches!(
            resolve_property_from_list(&properties, "Tag").unwrap(),
            Match::Exact(property) if property.key == "tag"
        ));
    }

    #[test]
    fn unique_partial_match_is_fuzzy() {
        let properties = vec![property("prop-1", "tag", "Tag")];

        assert!(matches!(
            resolve_property_from_list(&properties, "ta").unwrap(),
            Match::Fuzzy(property) if property.id == "prop-1"
        ));
    }

    #[test]
    fn errors_on_no_match_and_ambiguity() {
        let properties = vec![
            property("prop-1", "tag_one", "Tag One"),
            property("prop-2", "tag_two", "Tag Two"),
        ];

        assert!(
            resolve_property_from_list(&properties, "missing")
                .unwrap_err()
                .to_string()
                .contains("property not found")
        );
        assert!(
            resolve_property_from_list(&properties, "tag_")
                .unwrap_err()
                .to_string()
                .contains("property ambiguous")
        );
    }
}
