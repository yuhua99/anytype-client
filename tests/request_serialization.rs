use anyclient::models::*;
use serde_json::{Value, json};

fn assert_json<T: serde::Serialize>(value: &T, expected: Value) {
    assert_eq!(serde_json::to_value(value).unwrap(), expected);
}

#[test]
fn serializes_search_request_body() {
    let req = SearchRequest {
        query: "task".into(),
        types: vec!["page".into(), "task".into()],
        filters: Some(
            FilterExpression::and().condition(FilterItem::Select(SelectFilterItem {
                property_key: "status".into(),
                condition: FilterCondition::Eq,
                select: "done".into(),
            })),
        ),
        sort: Some(SortOptions {
            property_key: SortProperty::LastModifiedDate,
            direction: SortDirection::Desc,
        }),
    };

    assert_json(
        &req,
        json!({
            "query": "task",
            "types": ["page", "task"],
            "filters": {
                "operator": "and",
                "conditions": [
                    {"property_key":"status","condition":"eq","select":"done"}
                ]
            },
            "sort": {"property_key":"last_modified_date","direction":"desc"}
        }),
    );
}

#[test]
fn serializes_space_request_bodies() {
    assert_json(
        &CreateSpaceRequest {
            name: "Work".into(),
            description: "Team space".into(),
        },
        json!({"name":"Work","description":"Team space"}),
    );

    assert_json(
        &CreateSpaceRequest {
            name: "Work".into(),
            description: String::new(),
        },
        json!({"name":"Work"}),
    );

    assert_json(
        &UpdateSpaceRequest {
            name: Some("New".into()),
            description: None,
        },
        json!({"name":"New"}),
    );
}

#[test]
fn serializes_object_request_bodies() {
    assert_json(
        &CreateObjectRequest {
            type_key: "task".into(),
            name: "Ship".into(),
            body: "Body".into(),
            icon: Some(Icon::Emoji {
                emoji: "🚀".into()
            }),
            template_id: Some("template-id".into()),
            properties: vec![PropertyLinkValue::Text(TextPropertyLinkValue {
                key: "status".into(),
                text: "done".into(),
            })],
        },
        json!({
            "type_key":"task",
            "name":"Ship",
            "body":"Body",
            "icon":{"format":"emoji","emoji":"🚀"},
            "template_id":"template-id",
            "properties":[{"key":"status","text":"done"}]
        }),
    );

    assert_json(
        &UpdateObjectRequest {
            type_key: None,
            name: Some("Renamed".into()),
            markdown: None,
            icon: Some(None),
            properties: Vec::new(),
        },
        json!({"name":"Renamed","icon":null}),
    );
}

#[test]
fn serializes_property_and_tag_request_bodies() {
    assert_json(
        &CreatePropertyRequest {
            key: Some("status".into()),
            name: "Status".into(),
            format: PropertyFormat::Select,
            tags: vec![CreateTagRequest {
                key: Some("done".into()),
                name: "Done".into(),
                color: IconColor::Teal,
            }],
        },
        json!({
            "key":"status",
            "name":"Status",
            "format":"select",
            "tags":[{"key":"done","name":"Done","color":"teal"}]
        }),
    );

    assert_json(
        &UpdatePropertyRequest {
            key: None,
            name: "State".into(),
        },
        json!({"name":"State"}),
    );

    assert_json(
        &CreateTagRequest {
            key: None,
            name: "Blocked".into(),
            color: IconColor::Red,
        },
        json!({"name":"Blocked","color":"red"}),
    );

    assert_json(
        &UpdateTagRequest {
            key: Some("blocked".into()),
            name: None,
            color: Some(IconColor::Orange),
        },
        json!({"key":"blocked","color":"orange"}),
    );
}

#[test]
fn serializes_type_request_bodies() {
    assert_json(
        &CreateTypeRequest {
            key: Some("task".into()),
            name: "Task".into(),
            plural_name: "Tasks".into(),
            layout: ObjectLayout::Basic,
            icon: Some(Icon::Named {
                name: "check".into(),
                color: IconColor::Blue,
            }),
            properties: vec![PropertyLink {
                key: "status".into(),
                name: "Status".into(),
                format: PropertyFormat::Select,
            }],
        },
        json!({
            "key":"task",
            "name":"Task",
            "plural_name":"Tasks",
            "layout":"basic",
            "icon":{"format":"icon","name":"check","color":"blue"},
            "properties":[{"key":"status","name":"Status","format":"select"}]
        }),
    );

    assert_json(
        &UpdateTypeRequest {
            key: None,
            name: Some("Task".into()),
            plural_name: None,
            layout: None,
            icon: Some(None),
            properties: Vec::new(),
        },
        json!({"name":"Task","icon":null}),
    );
}

#[test]
fn serializes_auth_request_bodies() {
    assert_json(
        &CreateChallengeRequest {
            app_name: "anyclient".into(),
        },
        json!({"app_name":"anyclient"}),
    );

    assert_json(
        &CreateApiKeyRequest {
            challenge_id: "challenge".into(),
            code: "123456".into(),
        },
        json!({"challenge_id":"challenge","code":"123456"}),
    );
}

#[test]
fn serializes_list_request_bodies() {
    let objects = vec!["obj-1".to_string(), "obj-2".to_string()];

    assert_json(
        &AddToListRequest { objects: &objects },
        json!({"objects":["obj-1","obj-2"]}),
    );
}
