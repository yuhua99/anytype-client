use std::collections::BTreeMap;

use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::{AnytypeClient, PageOptions},
    models::{Object, SearchRequest},
};

use crate::services::space_resolution::resolve_space;

pub(crate) enum ObjectCountResult {
    Total(usize),
    Grouped {
        counts: BTreeMap<String, usize>,
        total: usize,
    },
}

pub(crate) async fn count_objects(
    client: &AnytypeClient,
    space: String,
    group_by: Option<String>,
) -> Result<ObjectCountResult> {
    let space_id = resolve_space(client, &space).await?;
    let req = SearchRequest::new(String::new());

    match group_by.as_deref() {
        // Grouping needs every object; ungrouped counts only need the total.
        None => Ok(ObjectCountResult::Total(
            count_total(client, &space_id, &req).await?,
        )),
        Some("type") => {
            let results = client.space_search_page(&space_id, &req, None).await?.data;
            let total = results.len();
            Ok(ObjectCountResult::Grouped {
                counts: count_by_type(&results),
                total,
            })
        }
        Some(group) if group.starts_with("property:") => {
            let results = client.space_search_page(&space_id, &req, None).await?.data;
            let total = results.len();
            Ok(ObjectCountResult::Grouped {
                counts: count_by_property(&results, &group["property:".len()..]),
                total,
            })
        }
        Some(other) => Err(anyhow!(
            "invalid --group-by: '{other}'. Use 'type' or 'property:<key>'"
        )),
    }
}

/// Count via `pagination.total` from a single-item page. Falls back to
/// fetching everything only when the API omits the total.
async fn count_total(client: &AnytypeClient, space_id: &str, req: &SearchRequest) -> Result<usize> {
    let page = client
        .space_search_page(
            space_id,
            req,
            Some(PageOptions {
                offset: 0,
                limit: 1,
            }),
        )
        .await?;
    if let Some(total) = page.pagination.as_ref().and_then(|page| page.total) {
        return Ok(total as usize);
    }
    let results = client.space_search_page(space_id, req, None).await?.data;
    Ok(results.len())
}

fn count_by_type(objects: &[Object]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for object in objects {
        let type_name = object
            .object_type
            .as_ref()
            .map(|object_type| {
                if object_type.key.is_empty() {
                    object_type.name.clone()
                } else {
                    object_type.key.clone()
                }
            })
            .unwrap_or_else(|| "(none)".to_string());
        *counts.entry(type_name).or_insert(0) += 1;
    }
    counts
}

fn count_by_property(objects: &[Object], prop_key: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    let mut missing = 0usize;

    for object in objects {
        let found = object.properties.iter().find(|property| {
            property
                .get("key")
                .and_then(Value::as_str)
                .is_some_and(|key| key.eq_ignore_ascii_case(prop_key))
        });
        match found {
            None => missing += 1,
            Some(property) => {
                let value = display_property_value(property);
                if value.is_empty() {
                    *counts.entry("(empty)".to_string()).or_insert(0) += 1;
                } else {
                    *counts.entry(value).or_insert(0) += 1;
                }
            }
        }
    }

    if missing > 0 {
        counts.insert("(missing)".to_string(), missing);
    }

    counts
}

/// Format a property value for count grouping.
fn display_property_value(prop: &Value) -> String {
    for key in ["text", "url", "email", "phone"] {
        if let Some(value) = prop.get(key).and_then(Value::as_str) {
            return value.to_string();
        }
    }
    if let Some(value) = prop.get("number") {
        return value.to_string();
    }
    if let Some(value) = prop.get("select").and_then(Value::as_str) {
        return value.to_string();
    }
    if let Some(arr) = prop.get("multi_select").and_then(Value::as_array) {
        let names: Vec<String> = arr
            .iter()
            .map(|value| {
                value
                    .get("name")
                    .and_then(Value::as_str)
                    .or_else(|| value.as_str())
                    .unwrap_or("?")
                    .to_string()
            })
            .collect();
        return names.join(", ");
    }
    if let Some(value) = prop.get("checkbox").and_then(Value::as_bool) {
        return value.to_string();
    }
    if let Some(value) = prop.get("date").and_then(Value::as_str) {
        return value.to_string();
    }
    serde_json::to_string(prop).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn mock_spaces(server: &MockServer) {
        Mock::given(method("GET"))
            .and(path("/v1/spaces"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{"id": "s1", "name": "Work"}],
                "pagination": {"has_more": false}
            })))
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn ungrouped_count_uses_pagination_total() {
        let server = MockServer::start().await;
        mock_spaces(&server).await;

        Mock::given(method("POST"))
            .and(path("/v1/spaces/s1/search"))
            .and(query_param("limit", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{"id": "o1", "name": "One", "space_id": "s1", "layout": "basic"}],
                "pagination": {"total": 42, "has_more": true}
            })))
            .mount(&server)
            .await;

        let client = AnytypeClient::new(server.uri(), None).unwrap();
        let result = count_objects(&client, "s1".into(), None).await.unwrap();
        assert!(matches!(result, ObjectCountResult::Total(42)));
    }

    #[tokio::test]
    async fn ungrouped_count_falls_back_to_full_fetch_without_total() {
        let server = MockServer::start().await;
        mock_spaces(&server).await;

        Mock::given(method("POST"))
            .and(path("/v1/spaces/s1/search"))
            .and(query_param("limit", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{"id": "o1", "name": "One", "space_id": "s1", "layout": "basic"}],
                "pagination": {"has_more": true}
            })))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/spaces/s1/search"))
            .and(query_param("limit", "1000"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {"id": "o1", "name": "One", "space_id": "s1", "layout": "basic"},
                    {"id": "o2", "name": "Two", "space_id": "s1", "layout": "basic"}
                ],
                "pagination": {"has_more": false}
            })))
            .mount(&server)
            .await;

        let client = AnytypeClient::new(server.uri(), None).unwrap();
        let result = count_objects(&client, "s1".into(), None).await.unwrap();
        assert!(matches!(result, ObjectCountResult::Total(2)));
    }
}
