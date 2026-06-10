use anyhow::{Result, anyhow};

use crate::{
    api::AnytypeClient,
    models::{
        CheckboxFilterItem, DateFilterItem, EmailFilterItem, EmptyFilterItem, FilesFilterItem,
        FilterCondition, FilterExpression, FilterItem, MultiSelectFilterItem, NumberFilterItem,
        Object, ObjectsFilterItem, PhoneFilterItem, Property, PropertyFormat, SearchRequest,
        SelectFilterItem, Tag, TextFilterItem, UrlFilterItem,
    },
    services::{
        property_resolution::resolve_property, space_resolution::resolve_space,
        tag_resolution::resolve_tag_from_list,
    },
};

pub(crate) struct FindObjectsParams {
    pub space: String,
    pub type_key: Option<String>,
    pub tag: Option<String>,
    pub tag_property: Option<String>,
    pub property: Option<String>,
    pub name: Option<String>,
    pub missing_property: Option<String>,
}

pub(crate) async fn find_objects(
    client: &AnytypeClient,
    params: FindObjectsParams,
) -> Result<Vec<Object>> {
    let space_id = resolve_space(client, &params.space).await?;
    let search_types = params
        .type_key
        .as_ref()
        .map(|r#type| vec![r#type.clone()])
        .unwrap_or_default();

    let mut conditions = Vec::new();

    if let Some(tag_value) = &params.tag {
        let prop = params
            .tag_property
            .as_deref()
            .ok_or_else(|| anyhow!("--tag-property is required when using --tag"))?;
        let property = resolve_property(client, &space_id, prop).await?;
        let all_tags = client.tags(&space_id, &property.id).await?.data;
        let tag_id = resolve_tag_from_list(&all_tags, tag_value)?;
        conditions.push(tag_condition(&property, tag_id)?);
    }

    if let Some(prop_expr) = &params.property {
        let (key, value) = prop_expr
            .split_once('=')
            .ok_or_else(|| anyhow!("--property must be key=value"))?;
        let property = resolve_property(client, &space_id, key).await?;
        let tags = match property.format {
            PropertyFormat::Select | PropertyFormat::MultiSelect => {
                client.tags(&space_id, &property.id).await?.data
            }
            _ => Vec::new(),
        };
        conditions.push(property_value_condition(&property, value, &tags)?);
    }

    if let Some(missing_prop) = &params.missing_property {
        let property = resolve_property(client, &space_id, missing_prop).await?;
        conditions.push(FilterItem::Empty(EmptyFilterItem {
            property_key: property.key,
            condition: FilterCondition::Empty,
        }));
    }

    let filters = if conditions.is_empty() {
        None
    } else {
        Some(FilterExpression::and().with_conditions(conditions))
    };

    let req = SearchRequest::new(String::new())
        .with_types(search_types)
        .with_filters(filters);
    let mut results = client.space_search_page(&space_id, &req, None).await?.data;

    if let Some(name) = &params.name {
        let needle = name.to_lowercase();
        results.retain(|obj| obj.name.to_lowercase().contains(&needle));
    }

    Ok(results)
}

/// Build a server-side condition matching objects that carry `tag_id` in the
/// given select/multi-select property.
fn tag_condition(property: &Property, tag_id: String) -> Result<FilterItem> {
    match property.format {
        PropertyFormat::Select => Ok(FilterItem::Select(SelectFilterItem {
            property_key: property.key.clone(),
            condition: FilterCondition::Eq,
            select: tag_id,
        })),
        PropertyFormat::MultiSelect => Ok(FilterItem::MultiSelect(MultiSelectFilterItem {
            property_key: property.key.clone(),
            condition: FilterCondition::In,
            multi_select: vec![tag_id],
        })),
        ref other => Err(anyhow!(
            "--tag requires a select or multi_select property; '{}' has format {other}",
            property.name
        )),
    }
}

/// Build a server-side equality condition for `--property key=value`,
/// typed according to the property's format.
fn property_value_condition(property: &Property, value: &str, tags: &[Tag]) -> Result<FilterItem> {
    let key = property.key.clone();
    match property.format {
        PropertyFormat::Text => Ok(FilterItem::Text(TextFilterItem {
            property_key: key,
            condition: FilterCondition::Eq,
            text: value.into(),
        })),
        PropertyFormat::Number => {
            let number = value.parse().map_err(|_| {
                anyhow!("--property: '{value}' is not a number (property '{key}' is numeric)")
            })?;
            Ok(FilterItem::Number(NumberFilterItem {
                property_key: key,
                condition: FilterCondition::Eq,
                number,
            }))
        }
        PropertyFormat::Select => Ok(FilterItem::Select(SelectFilterItem {
            property_key: key,
            condition: FilterCondition::Eq,
            select: resolve_tag_from_list(tags, value)?,
        })),
        PropertyFormat::MultiSelect => Ok(FilterItem::MultiSelect(MultiSelectFilterItem {
            property_key: key,
            condition: FilterCondition::In,
            multi_select: vec![resolve_tag_from_list(tags, value)?],
        })),
        PropertyFormat::Date => Ok(FilterItem::Date(DateFilterItem {
            property_key: key,
            condition: FilterCondition::Eq,
            date: value.into(),
        })),
        PropertyFormat::Checkbox => {
            let checkbox = value.parse().map_err(|_| {
                anyhow!(
                    "--property: '{value}' must be true or false (property '{key}' is a checkbox)"
                )
            })?;
            Ok(FilterItem::Checkbox(CheckboxFilterItem {
                property_key: key,
                condition: FilterCondition::Eq,
                checkbox,
            }))
        }
        PropertyFormat::Url => Ok(FilterItem::Url(UrlFilterItem {
            property_key: key,
            condition: FilterCondition::Eq,
            url: value.into(),
        })),
        PropertyFormat::Email => Ok(FilterItem::Email(EmailFilterItem {
            property_key: key,
            condition: FilterCondition::Eq,
            email: value.into(),
        })),
        PropertyFormat::Phone => Ok(FilterItem::Phone(PhoneFilterItem {
            property_key: key,
            condition: FilterCondition::Eq,
            phone: value.into(),
        })),
        PropertyFormat::Files => Ok(FilterItem::Files(FilesFilterItem {
            property_key: key,
            condition: FilterCondition::In,
            files: vec![value.into()],
        })),
        PropertyFormat::Objects => Ok(FilterItem::Objects(ObjectsFilterItem {
            property_key: key,
            condition: FilterCondition::In,
            objects: vec![value.into()],
        })),
        PropertyFormat::Unknown => Err(anyhow!(
            "--property: property '{}' has an unrecognized format; cannot build a filter",
            property.name
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::IconColor;

    fn property(key: &str, format: PropertyFormat) -> Property {
        Property {
            id: format!("id-{key}"),
            key: key.into(),
            name: key.to_uppercase(),
            format,
            object: String::new(),
            extra: Default::default(),
        }
    }

    fn tag(id: &str, name: &str) -> Tag {
        Tag {
            id: id.into(),
            key: name.to_lowercase(),
            name: name.into(),
            color: IconColor::Yellow,
            object: String::new(),
            extra: Default::default(),
        }
    }

    #[test]
    fn tag_condition_uses_property_format() {
        let select = tag_condition(&property("status", PropertyFormat::Select), "t1".into());
        assert!(matches!(select.unwrap(), FilterItem::Select(item)
            if item.property_key == "status" && item.select == "t1"));

        let multi = tag_condition(&property("tags", PropertyFormat::MultiSelect), "t1".into());
        assert!(matches!(multi.unwrap(), FilterItem::MultiSelect(item)
            if item.property_key == "tags" && item.multi_select == ["t1"]));

        assert!(tag_condition(&property("title", PropertyFormat::Text), "t1".into()).is_err());
    }

    #[test]
    fn property_value_condition_types_by_format() {
        let text = property_value_condition(&property("title", PropertyFormat::Text), "foo", &[]);
        assert!(matches!(text.unwrap(), FilterItem::Text(item) if item.text == "foo"));

        let number =
            property_value_condition(&property("score", PropertyFormat::Number), "4.5", &[]);
        assert!(matches!(number.unwrap(), FilterItem::Number(item) if item.number == 4.5));

        let checkbox =
            property_value_condition(&property("done", PropertyFormat::Checkbox), "true", &[]);
        assert!(matches!(checkbox.unwrap(), FilterItem::Checkbox(item) if item.checkbox));

        let select = property_value_condition(
            &property("status", PropertyFormat::Select),
            "Done",
            &[tag("t1", "Done")],
        );
        assert!(matches!(select.unwrap(), FilterItem::Select(item) if item.select == "t1"));
    }

    #[test]
    fn property_value_condition_rejects_bad_values() {
        assert!(
            property_value_condition(&property("score", PropertyFormat::Number), "abc", &[])
                .is_err()
        );
        assert!(
            property_value_condition(&property("done", PropertyFormat::Checkbox), "yep", &[])
                .is_err()
        );
        assert!(
            property_value_condition(&property("x", PropertyFormat::Unknown), "v", &[]).is_err()
        );
    }
}
