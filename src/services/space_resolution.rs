use anyhow::{Result, anyhow};

use crate::{api::AnytypeClient, models::Space, output::eprint_status, services::Match};

pub(crate) async fn resolve_space(client: &AnytypeClient, id_or_name: &str) -> Result<String> {
    // Fast path: inputs shaped like a space ID are verified with a direct GET,
    // skipping the full space listing. On failure we fall back to list-based
    // resolution, which produces the proper "space not found" error.
    if looks_like_space_id(id_or_name) && client.space(id_or_name).await.is_ok() {
        return Ok(id_or_name.to_string());
    }

    let spaces = client.spaces().await?.data;
    match resolve_space_from_list(&spaces, id_or_name)? {
        Match::Exact(space_id) => Ok(space_id),
        Match::Fuzzy(space_id) => {
            let name = spaces
                .iter()
                .find(|space| space.id == space_id)
                .map(|space| space.name.as_str())
                .unwrap_or_default();
            eprint_status(format!(
                "note: resolved space '{id_or_name}' by partial match to '{name}' ({space_id})"
            ));
            Ok(space_id)
        }
    }
}

/// Anytype space IDs are long CID-style strings (e.g. `bafyrei…`); names that
/// long without whitespace are unlikely, so this only gates the fast path.
fn looks_like_space_id(input: &str) -> bool {
    input.len() >= 32 && !input.chars().any(char::is_whitespace)
}

fn resolve_space_from_list(spaces: &[Space], id_or_name: &str) -> Result<Match<String>> {
    if spaces.iter().any(|space| space.id == id_or_name) {
        return Ok(Match::Exact(id_or_name.to_string()));
    }
    if let Some(space) = spaces
        .iter()
        .find(|space| space.name.eq_ignore_ascii_case(id_or_name))
    {
        return Ok(Match::Exact(space.id.clone()));
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
        1 => Ok(Match::Fuzzy(matches[0].id.clone())),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn space(id: &str, name: &str) -> Space {
        Space {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            home_id: None,
            icon: None,
            extra: Default::default(),
        }
    }

    #[test]
    fn space_id_heuristic_accepts_cids_and_rejects_names() {
        assert!(looks_like_space_id(
            "bafyreialsvyqzgf3xx2nyvzq6gie5mhg77ngfczzgkbeyvirmi3v"
        ));
        assert!(!looks_like_space_id("Work"));
        assert!(!looks_like_space_id(
            "a long space name with enough characters"
        ));
    }

    #[test]
    fn exact_id_and_name_win_over_fuzzy() {
        let spaces = vec![space("s1", "Work"), space("s2", "Workshop")];

        assert!(matches!(
            resolve_space_from_list(&spaces, "s1").unwrap(),
            Match::Exact(id) if id == "s1"
        ));
        assert!(matches!(
            resolve_space_from_list(&spaces, "work").unwrap(),
            Match::Exact(id) if id == "s1"
        ));
    }

    #[test]
    fn unique_partial_match_is_fuzzy() {
        let spaces = vec![space("s1", "Work"), space("s2", "Archive")];

        assert!(matches!(
            resolve_space_from_list(&spaces, "arch").unwrap(),
            Match::Fuzzy(id) if id == "s2"
        ));
    }

    #[test]
    fn errors_on_no_match_and_ambiguity() {
        let spaces = vec![space("s1", "Work"), space("s2", "Workshop")];

        assert!(
            resolve_space_from_list(&spaces, "missing")
                .unwrap_err()
                .to_string()
                .contains("space not found")
        );
        assert!(
            resolve_space_from_list(&spaces, "wor")
                .unwrap_err()
                .to_string()
                .contains("multiple spaces matched")
        );
    }
}
