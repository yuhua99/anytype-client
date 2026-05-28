use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Logical operator for combining filter expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    #[default]
    And,
    Or,
}

/// Common condition operators supported by Anytype filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterCondition {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    Ncontains,
    In,
    Nin,
    All,
    Empty,
    Nempty,
}

/// A single filter condition targeting a specific property.
///
/// This is a tagged union matching Anytype's `FilterItem` oneOf schema.
/// Each variant corresponds to a property format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterItem {
    Text(TextFilterItem),
    Number(NumberFilterItem),
    Select(SelectFilterItem),
    MultiSelect(MultiSelectFilterItem),
    Date(DateFilterItem),
    Checkbox(CheckboxFilterItem),
    Files(FilesFilterItem),
    Url(UrlFilterItem),
    Email(EmailFilterItem),
    Phone(PhoneFilterItem),
    Objects(ObjectsFilterItem),
    Empty(EmptyFilterItem),
}

/// Text property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    pub text: String,
}

/// Number property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumberFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    pub number: f64,
}

/// Select (single tag) property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    /// Tag ID for eq/ne/in conditions.
    pub select: String,
}

/// Multi-select (multiple tags) property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiSelectFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    /// List of tag IDs.
    pub multi_select: Vec<String>,
}

/// Date property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DateFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    /// Accepts RFC3339 or date-only (YYYY-MM-DD).
    pub date: String,
}

/// Checkbox property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckboxFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    pub checkbox: bool,
}

/// Files property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilesFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    /// File IDs.
    pub files: Vec<String>,
}

/// URL property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UrlFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    pub url: String,
}

/// Email property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmailFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    pub email: String,
}

/// Phone property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhoneFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    pub phone: String,
}

/// Objects (relation) property filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectsFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
    /// Related object IDs.
    pub objects: Vec<String>,
}

/// Filter for checking empty / non-empty property values.
/// Does not carry an extra value field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmptyFilterItem {
    pub property_key: String,
    pub condition: FilterCondition,
}

/// A filter expression supporting logical grouping and nesting.
///
/// This directly models Anytype's `FilterExpression` schema and supports
/// recursive nesting via the `filters` field.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FilterExpression {
    pub operator: FilterOperator,

    /// Individual conditions to apply with the operator.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<FilterItem>,

    /// Nested filter expressions (for building complex trees).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub filters: Vec<FilterExpression>,
}

impl FilterExpression {
    pub fn new(operator: FilterOperator) -> Self {
        Self {
            operator,
            ..Default::default()
        }
    }

    pub fn and() -> Self {
        Self::new(FilterOperator::And)
    }

    pub fn or() -> Self {
        Self::new(FilterOperator::Or)
    }

    pub fn with_conditions(mut self, conditions: Vec<FilterItem>) -> Self {
        self.conditions = conditions;
        self
    }

    pub fn with_nested(mut self, filters: Vec<FilterExpression>) -> Self {
        self.filters = filters;
        self
    }

    /// Convenience: add a single condition.
    pub fn condition(mut self, item: FilterItem) -> Self {
        self.conditions.push(item);
        self
    }

    /// Convenience: add a nested expression.
    pub fn nested(mut self, expr: FilterExpression) -> Self {
        self.filters.push(expr);
        self
    }
}

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Default)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum SortDirection {
    #[default]
    Desc,
    Asc,
}

/// Known sortable property keys for search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Default)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum SortProperty {
    CreatedDate,
    #[default]
    LastModifiedDate,
    LastOpenedDate,
    Name,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_each_filter_item_variant() {
        let cases = [
            (
                json!({"property_key":"title","condition":"contains","text":"foo"}),
                "text",
            ),
            (
                json!({"property_key":"score","condition":"gte","number":10.0}),
                "number",
            ),
            (
                json!({"property_key":"status","condition":"eq","select":"done"}),
                "select",
            ),
            (
                json!({"property_key":"tags","condition":"all","multi_select":["a","b"]}),
                "multi_select",
            ),
            (
                json!({"property_key":"due","condition":"lt","date":"2024-01-01"}),
                "date",
            ),
            (
                json!({"property_key":"done","condition":"eq","checkbox":true}),
                "checkbox",
            ),
            (
                json!({"property_key":"attachments","condition":"in","files":["f1"]}),
                "files",
            ),
            (
                json!({"property_key":"site","condition":"eq","url":"https://example.com"}),
                "url",
            ),
            (
                json!({"property_key":"email","condition":"eq","email":"a@example.com"}),
                "email",
            ),
            (
                json!({"property_key":"phone","condition":"eq","phone":"123"}),
                "phone",
            ),
            (
                json!({"property_key":"links","condition":"in","objects":["o1"]}),
                "objects",
            ),
            (
                json!({"property_key":"empty_prop","condition":"empty"}),
                "empty",
            ),
        ];

        for (value, expected) in cases {
            let item: FilterItem = serde_json::from_value(value).unwrap();
            let actual = match item {
                FilterItem::Text(_) => "text",
                FilterItem::Number(_) => "number",
                FilterItem::Select(_) => "select",
                FilterItem::MultiSelect(_) => "multi_select",
                FilterItem::Date(_) => "date",
                FilterItem::Checkbox(_) => "checkbox",
                FilterItem::Files(_) => "files",
                FilterItem::Url(_) => "url",
                FilterItem::Email(_) => "email",
                FilterItem::Phone(_) => "phone",
                FilterItem::Objects(_) => "objects",
                FilterItem::Empty(_) => "empty",
            };
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn deserializes_nested_filter_expression() {
        let expr: FilterExpression = serde_json::from_value(json!({
            "operator": "or",
            "filters": [
                {
                    "operator": "and",
                    "conditions": [
                        {"property_key":"status","condition":"eq","select":"done"},
                        {"property_key":"priority","condition":"eq","select":"high"}
                    ]
                },
                {
                    "operator": "and",
                    "conditions": [
                        {"property_key":"created_date","condition":"gt","date":"2024-01-01"}
                    ]
                }
            ]
        }))
        .unwrap();

        assert_eq!(expr.operator, FilterOperator::Or);
        assert_eq!(expr.filters.len(), 2);
        assert_eq!(expr.filters[0].conditions.len(), 2);
    }

    #[test]
    fn round_trips_filter_expression() {
        let expr = FilterExpression::and()
            .condition(FilterItem::Text(TextFilterItem {
                property_key: "title".into(),
                condition: FilterCondition::Contains,
                text: "foo".into(),
            }))
            .nested(
                FilterExpression::or().condition(FilterItem::Checkbox(CheckboxFilterItem {
                    property_key: "done".into(),
                    condition: FilterCondition::Eq,
                    checkbox: false,
                })),
            );

        let value = serde_json::to_value(&expr).unwrap();
        let decoded: FilterExpression = serde_json::from_value(value).unwrap();

        assert_eq!(decoded.operator, FilterOperator::And);
        assert_eq!(decoded.conditions.len(), 1);
        assert_eq!(decoded.filters.len(), 1);
    }

    #[test]
    fn rejects_unknown_filter_item_fields() {
        let err = serde_json::from_value::<FilterItem>(json!({
            "property_key": "status",
            "condition": "eq",
            "select": "done",
            "unexpected": true
        }))
        .unwrap_err();

        assert!(err.to_string().contains("data did not match any variant"));
    }

    #[test]
    fn rejects_ambiguous_filter_items() {
        assert!(
            serde_json::from_value::<FilterItem>(json!({
                "property_key": "mixed",
                "condition": "eq",
                "text": "foo",
                "number": 1.0
            }))
            .is_err()
        );
    }
}
