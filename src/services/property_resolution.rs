use anyhow::{Result, anyhow};

use crate::api::AnytypeClient;

pub(crate) async fn resolve_property(
    client: &AnytypeClient,
    space_id: &str,
    name_or_key: &str,
) -> Result<String> {
    let properties = client.properties(space_id).await?.data;
    if let Some(property) = properties
        .iter()
        .find(|property| property.id == name_or_key)
    {
        return Ok(property.id.clone());
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(property.id.clone());
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(property.id.clone());
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
        1 => Ok(matches[0].id.clone()),
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
