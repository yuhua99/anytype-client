use anyhow::{Result, anyhow};

use crate::{api::AnytypeClient, models::Space};

pub(crate) async fn resolve_space(client: &AnytypeClient, id_or_name: &str) -> Result<String> {
    let spaces = client.spaces().await?.data;
    resolve_space_from_list(&spaces, id_or_name)
}

fn resolve_space_from_list(spaces: &[Space], id_or_name: &str) -> Result<String> {
    if spaces.iter().any(|space| space.id == id_or_name) {
        return Ok(id_or_name.to_string());
    }
    if let Some(space) = spaces
        .iter()
        .find(|space| space.name.eq_ignore_ascii_case(id_or_name))
    {
        return Ok(space.id.clone());
    }

    let needle = id_or_name.to_lowercase();
    let matches: Vec<_> = spaces
        .iter()
        .filter(|space| space.name.to_lowercase().contains(&needle))
        .collect();

    match matches.len() {
        0 => Err(anyhow!(
            "space not found: '{}' (use exact ID or name; partial matches: none)",
            id_or_name
        )),
        1 => Ok(matches[0].id.clone()),
        _ => Err(anyhow!(
            "space not found: multiple spaces matched '{}': {}",
            id_or_name,
            matches
                .iter()
                .map(|space| format!("{} ({})", space.name, space.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}
