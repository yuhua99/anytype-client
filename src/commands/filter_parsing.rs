use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::models::{FilterExpression, SearchFilters};

/// Parses --filters as JSON into a strict typed FilterExpression.
/// Only the new typed shape (with "operator" and "conditions") is accepted.
pub(super) fn parse_filters(filters: Option<String>) -> Result<Option<SearchFilters>> {
    let Some(filters) = filters else {
        return Ok(None);
    };

    let value: Value = serde_json::from_str(&filters).map_err(|err| {
        anyhow!(
            "invalid JSON for --filters: {err}\n\
             Expected a JSON object with \"operator\" and \"conditions\".\n\
             Example: {{\"operator\":\"and\",\"conditions\":[{{\"property_key\":\"status\",\"condition\":\"eq\",\"select\":\"<tag-id>\"}}]}}"
        )
    })?;

    if !value.is_object() {
        return Err(anyhow!("invalid --filters: expected a JSON object"));
    }

    let expr: FilterExpression = serde_json::from_value(value)
        .map_err(|err| anyhow!("invalid typed filter expression for --filters: {err}"))?;

    Ok(Some(expr))
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

        assert!(filters.is_some());
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
