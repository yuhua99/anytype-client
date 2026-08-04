use crate::api::AnytypeClient;
use anyhow::{Result, anyhow};

pub(crate) async fn resolve_default_template_for_type(
    client: &AnytypeClient,
    space_id: &str,
    type_key: &str,
) -> Result<Option<String>> {
    let types = client.types(space_id).await?.data;
    for t in types {
        if t.id == type_key || t.key.eq_ignore_ascii_case(type_key) {
            if let Some(id) = t.default_template_id.clone().filter(|id| !id.is_empty()) {
                return Ok(Some(id));
            }
            let templates = client.templates(space_id, &t.id).await?.data;
            return Ok(match templates.as_slice() {
                [only] => Some(only.id.clone()),
                _ => None,
            });
        }
    }
    Err(anyhow!(
        "type not found: '{type_key}' matched no type key or id (run `anyclient types list <space>` to list keys)"
    ))
}
