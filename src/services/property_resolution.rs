use anyhow::{Result, anyhow};

use crate::{api::AnytypeClient, models::Property};

/// Resolve a user-supplied id/key/name to the full property, so callers can
/// use the canonical `key` for object property reads/writes and the `id` for
/// tag endpoints.
pub(crate) async fn resolve_property(
    client: &AnytypeClient,
    space_id: &str,
    name_or_key: &str,
) -> Result<Property> {
    let properties = client.properties(space_id).await?.data;
    resolve_property_from_list(&properties, name_or_key).cloned()
}

fn resolve_property_from_list<'a>(
    properties: &'a [Property],
    name_or_key: &str,
) -> Result<&'a Property> {
    if let Some(property) = properties
        .iter()
        .find(|property| property.id == name_or_key)
    {
        return Ok(property);
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(property);
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(property);
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
        1 => Ok(matches[0]),
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

        assert_eq!(
            resolve_property_from_list(&properties, "prop-1")
                .unwrap()
                .key,
            "tag"
        );
        assert_eq!(
            resolve_property_from_list(&properties, "TAG").unwrap().id,
            "prop-1"
        );
        assert_eq!(
            resolve_property_from_list(&properties, "Status")
                .unwrap()
                .id,
            "prop-2"
        );
    }

    #[test]
    fn display_name_resolves_to_canonical_key() {
        let properties = vec![property("prop-1", "tag", "Tag")];

        let resolved = resolve_property_from_list(&properties, "Tag").unwrap();
        assert_eq!(resolved.key, "tag");
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
