use anyclient::models::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

fn round_trip<T>(value: T) -> T
where
    T: Serialize + DeserializeOwned,
{
    serde_json::from_value(serde_json::to_value(value).unwrap()).unwrap()
}

#[test]
fn round_trips_icons() {
    let emoji = round_trip(Icon::Emoji {
        emoji: "🚀".into()
    });
    assert!(matches!(emoji, Icon::Emoji { emoji } if emoji == "🚀"));

    let file = round_trip(Icon::File {
        file: "file-id".into(),
    });
    assert!(matches!(file, Icon::File { file } if file == "file-id"));

    let named = round_trip(Icon::Named {
        name: "check".into(),
        color: IconColor::Teal,
    });
    assert!(matches!(named, Icon::Named { name, color: IconColor::Teal } if name == "check"));
}

#[test]
fn round_trips_space_models() {
    let space = round_trip(Space {
        id: "space-id".into(),
        name: "Work".into(),
        description: "Team".into(),
        home_id: Some("home-id".into()),
        icon: Some(Icon::Emoji {
            emoji: "📌".into()
        }),
        extra: [("custom".into(), json!(true))].into(),
    });

    assert_eq!(space.id, "space-id");
    assert_eq!(space.extra["custom"], json!(true));
}

#[test]
fn round_trips_object_models() {
    let object = round_trip(Object {
        id: "object-id".into(),
        name: "Task".into(),
        space_id: "space-id".into(),
        object_type: Some(ObjectTypeRef {
            id: "type-id".into(),
            key: "task".into(),
            name: "Task".into(),
            extra: Default::default(),
        }),
        layout: "basic".into(),
        archived: false,
        markdown: Some("# Task".into()),
        icon: None,
        properties: vec![json!({"key":"status","select":"done"})],
        extra: Default::default(),
    });

    assert_eq!(object.id, "object-id");
    assert_eq!(object.properties, [json!({"key":"status","select":"done"})]);
}

#[test]
fn round_trips_property_and_tag_models() {
    let property = round_trip(Property {
        id: "property-id".into(),
        key: "status".into(),
        name: "Status".into(),
        format: PropertyFormat::Select,
        object: "object-id".into(),
        extra: Default::default(),
    });

    assert_eq!(property.format.to_string(), "select");

    let tag = round_trip(Tag {
        id: "tag-id".into(),
        key: "done".into(),
        name: "Done".into(),
        color: IconColor::Teal,
        object: "object-id".into(),
        extra: Default::default(),
    });

    assert_eq!(tag.color.to_string(), "teal");
}

#[test]
fn round_trips_type_and_list_models() {
    let object_type = round_trip(ObjectType {
        id: "type-id".into(),
        key: "task".into(),
        name: "Task".into(),
        layout: "basic".into(),
        plural_name: "Tasks".into(),
        description: "Things to do".into(),
        archived: false,
        is_hidden: false,
        property_definitions: vec![json!({"key":"status"})],
        icon: None,
        extra: Default::default(),
    });

    assert_eq!(object_type.property_definitions, [json!({"key":"status"})]);

    let view = round_trip(ListView {
        id: "view-id".into(),
        name: "All".into(),
        layout: "table".into(),
        filters: vec![json!({"operator":"and"})],
        sorts: vec![json!({"property_key":"name"})],
        extra: Default::default(),
    });

    assert_eq!(view.filters.len(), 1);
    assert_eq!(view.sorts.len(), 1);
}

#[test]
fn deserializes_nullable_list_filters_and_sorts_as_empty() {
    let view: ListView = serde_json::from_value(json!({
        "id": "view-id",
        "name": "All",
        "layout": "table",
        "filters": null,
        "sorts": null
    }))
    .unwrap();

    assert!(view.filters.is_empty());
    assert!(view.sorts.is_empty());
}

#[test]
fn round_trips_data_response_pagination() {
    let response = round_trip(DataResponse {
        data: vec![Space {
            id: "space-id".into(),
            name: "Work".into(),
            description: String::new(),
            home_id: None,
            icon: None,
            extra: Default::default(),
        }],
        pagination: Some(Pagination {
            limit: Some(10),
            offset: Some(20),
            total: Some(30),
            has_more: Some(false),
        }),
    });

    assert_eq!(response.data.len(), 1);
    assert_eq!(response.pagination.unwrap().total, Some(30));
}
