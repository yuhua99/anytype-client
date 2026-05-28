use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{DataResponse, FilterExpression, Object, SortDirection, SortProperty};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchRequest {
    #[serde(default)]
    pub query: String,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub types: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SearchFilters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortOptions>,
}

impl SearchRequest {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            types: Vec::new(),
            filters: None,
            sort: None,
        }
    }

    pub fn with_types(mut self, types: Vec<String>) -> Self {
        self.types = types;
        self
    }

    pub fn with_filters(mut self, filters: Option<SearchFilters>) -> Self {
        self.filters = filters;
        self
    }

    pub fn with_sort(mut self, sort: Option<SortOptions>) -> Self {
        self.sort = sort;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortOptions {
    pub property_key: SortProperty,
    #[serde(default)]
    pub direction: SortDirection,
}

impl SortOptions {
    pub fn new(property_key: SortProperty, direction: SortDirection) -> Self {
        Self {
            property_key,
            direction,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchFilters {
    Expression(FilterExpression),
    Raw(Value),
}

impl From<FilterExpression> for SearchFilters {
    fn from(value: FilterExpression) -> Self {
        Self::Expression(value)
    }
}

impl From<Value> for SearchFilters {
    fn from(value: Value) -> Self {
        Self::Raw(value)
    }
}

pub type SearchResponse = DataResponse<Object>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{FilterCondition, FilterItem, FilterOperator, SelectFilterItem};
    use serde_json::json;

    #[test]
    fn serializes_typed_filters() {
        let req = SearchRequest {
            query: String::new(),
            types: Vec::new(),
            filters: Some(
                FilterExpression::new(FilterOperator::And)
                    .condition(FilterItem::Select(SelectFilterItem {
                        property_key: "status".into(),
                        condition: FilterCondition::Eq,
                        select: "done".into(),
                    }))
                    .into(),
            ),
            sort: Some(SortOptions {
                property_key: SortProperty::LastModifiedDate,
                direction: SortDirection::Desc,
            }),
        };

        assert_eq!(
            serde_json::to_value(req).unwrap(),
            json!({
                "query": "",
                "filters": {
                    "operator": "and",
                    "conditions": [
                        {"property_key":"status","condition":"eq","select":"done"}
                    ]
                },
                "sort": {
                    "property_key": "last_modified_date",
                    "direction": "desc"
                }
            })
        );
    }

    #[test]
    fn preserves_raw_legacy_filters() {
        let req = SearchRequest {
            query: String::new(),
            types: Vec::new(),
            filters: Some(
                json!({
                    "type": "and",
                    "filters": [{"key":"type","condition":"equal","value":"task"}]
                })
                .into(),
            ),
            sort: None,
        };

        assert_eq!(
            serde_json::to_value(req).unwrap()["filters"],
            json!({
                "type": "and",
                "filters": [{"key":"type","condition":"equal","value":"task"}]
            })
        );
    }
}
