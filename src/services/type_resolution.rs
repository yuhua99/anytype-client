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
            return Ok(t.default_template_id.clone().filter(|id| !id.is_empty()));
        }
    }
    Err(anyhow!(
        "type not found: '{type_key}' matched no type key or id (run `anyclient types list <space>` to list keys)"
    ))
}
