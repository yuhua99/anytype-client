use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    cli::{PropertyLinkArgs, PropertyValueArgs},
    models::{PropertyLink, PropertyLinkValue},
};

pub(super) fn parse_property_values(args: PropertyValueArgs) -> Result<Vec<PropertyLinkValue>> {
    parse_json_items(
        args.properties,
        args.properties_json,
        "--property",
        "--properties-json",
    )
}

pub(super) fn parse_property_links(args: PropertyLinkArgs) -> Result<Vec<PropertyLink>> {
    parse_json_items(
        args.properties,
        args.properties_json,
        "--property",
        "--properties-json",
    )
}

pub(super) fn parse_json_items<T: DeserializeOwned>(
    items: Vec<String>,
    items_json: Option<String>,
    item_arg: &str,
    array_arg: &str,
) -> Result<Vec<T>> {
    let mut parsed = Vec::new();

    if let Some(items_json) = items_json {
        let value = parse_json(&items_json, array_arg)?;
        let array = value
            .as_array()
            .ok_or_else(|| anyhow!("{array_arg} must be a JSON array"))?;
        for value in array {
            parsed.push(parse_json_value(value.clone(), array_arg)?);
        }
    }

    for item in items {
        let value = parse_json(&item, item_arg)?;
        if !value.is_object() {
            return Err(anyhow!("{item_arg} must be a JSON object"));
        }
        parsed.push(parse_json_value(value, item_arg)?);
    }

    Ok(parsed)
}

fn parse_json_value<T: DeserializeOwned>(value: Value, arg: &str) -> Result<T> {
    serde_json::from_value(value).map_err(|err| anyhow!("invalid schema for {arg}: {err}"))
}

fn parse_json(input: &str, arg: &str) -> Result<Value> {
    serde_json::from_str(input).map_err(|err| anyhow!("invalid JSON for {arg}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PropertyLinkValue;

    #[test]
    fn parses_repeatable_property_values() {
        let values = parse_property_values(PropertyValueArgs {
            properties: vec![r#"{"key":"status","select":"done"}"#.into()],
            properties_json: None,
        })
        .unwrap();

        assert!(matches!(values[0], PropertyLinkValue::Select(_)));
    }

    #[test]
    fn parses_property_value_array() {
        let values = parse_property_values(PropertyValueArgs {
            properties: Vec::new(),
            properties_json: Some(
                r#"[{"key":"done","checkbox":true},{"key":"title","text":"Task"}]"#.into(),
            ),
        })
        .unwrap();

        assert_eq!(values.len(), 2);
    }

    #[test]
    fn rejects_non_object_property_value() {
        let err = parse_property_values(PropertyValueArgs {
            properties: vec!["[]".into()],
            properties_json: None,
        })
        .unwrap_err();

        assert!(err.to_string().contains("--property must be a JSON object"));
    }

    #[test]
    fn rejects_invalid_property_schema() {
        let err = parse_property_values(PropertyValueArgs {
            properties: vec![r#"{"key":"status","select":"done","extra":true}"#.into()],
            properties_json: None,
        })
        .unwrap_err();

        assert!(err.to_string().contains("invalid schema for --property"));
    }
}
