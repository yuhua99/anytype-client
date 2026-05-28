use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, SearchArgs},
    models::{FilterExpression, SearchFilters, SearchRequest, SortOptions},
    output::print_data,
};

use super::{page_options, resolve_space};

pub async fn run(client: &AnytypeClient, args: SearchArgs, output: &OutputFormat) -> Result<()> {
    let page = page_options(args.page)?;
    let req = SearchRequest {
        query: args.query,
        types: args.types,
        filters: parse_filters(args.filters)?,
        sort: args.sort.map(|property_key| SortOptions {
            property_key,
            direction: args.direction,
        }),
    };
    let resp = if let Some(space) = args.space {
        let id = resolve_space(client, &space).await?;
        client.space_search_page(&id, &req, page).await?
    } else {
        client.search_page(&req, page).await?
    };
    print_data(resp.data, output)
}

/// Parses --filters as JSON.
/// Typed FilterExpression input is kept typed; legacy/raw JSON is passed through unchanged.
fn parse_filters(filters: Option<String>) -> Result<Option<SearchFilters>> {
    let Some(filters) = filters else {
        return Ok(None);
    };

    let value: Value = serde_json::from_str(&filters).map_err(|err| {
        anyhow!(
            "invalid JSON for --filters: {err}\n\
             Expected a JSON object.\n\
             Typed example: {{\"operator\":\"and\",\"conditions\":[{{\"property_key\":\"status\",\"condition\":\"eq\",\"select\":\"<tag-id>\"}}]}}"
        )
    })?;

    if !value.is_object() {
        return Err(anyhow!("invalid --filters: expected a JSON object"));
    }

    match serde_json::from_value::<FilterExpression>(value.clone()) {
        Ok(expr) => Ok(Some(SearchFilters::Expression(expr))),
        Err(err) if value.get("operator").is_some() || value.get("conditions").is_some() => Err(
            anyhow!("invalid typed filter expression for --filters: {err}"),
        ),
        Err(_) => Ok(Some(SearchFilters::Raw(value))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_filters_accepts_typed_expression() {
        let filters = parse_filters(Some(
            r#"{"operator":"and","conditions":[{"property_key":"status","condition":"eq","select":"done"}]}"#
                .into(),
        ))
        .unwrap();

        assert!(matches!(filters, Some(SearchFilters::Expression(_))));
    }

    #[test]
    fn parse_filters_preserves_legacy_raw_object() {
        let filters = parse_filters(Some(
            r#"{"type":"and","filters":[{"key":"type","condition":"equal","value":"task"}]}"#
                .into(),
        ))
        .unwrap();

        assert!(matches!(filters, Some(SearchFilters::Raw(_))));
    }

    #[test]
    fn parse_filters_rejects_invalid_typed_expression() {
        let err = parse_filters(Some(
            r#"{"operator":"and","conditions":[{"property_key":"status","condition":"eq","select":"done","unexpected":true}]}"#
                .into(),
        ))
        .unwrap_err();

        assert!(err.to_string().contains("invalid typed filter expression"));
    }

    #[test]
    fn parse_filters_rejects_invalid_json() {
        assert!(parse_filters(Some("{".into())).is_err());
    }

    #[test]
    fn parse_filters_rejects_non_object_json() {
        assert!(parse_filters(Some("[]".into())).is_err());
    }
}
