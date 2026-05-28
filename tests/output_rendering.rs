use anyclient::{
    cli::OutputFormat,
    models::{Property, PropertyFormat, Space},
    output::{render_data, render_one},
};
use serde::Serialize;
use tabled::Tabled;

#[derive(Serialize, Tabled)]
struct Row {
    id: String,
    name: String,
}

#[test]
fn renders_json_data() {
    let output = render_data(
        vec![Row {
            id: "1".into(),
            name: "Alpha".into(),
        }],
        &OutputFormat::Json,
    )
    .unwrap();

    assert_eq!(
        output,
        r#"[
  {
    "id": "1",
    "name": "Alpha"
  }
]"#
    );
}

#[test]
fn renders_yaml_one() {
    let output = render_one(
        Row {
            id: "1".into(),
            name: "Alpha".into(),
        },
        &OutputFormat::Yaml,
    )
    .unwrap();

    assert_eq!(output, "id: '1'\nname: Alpha\n");
}

#[test]
fn renders_table_data() {
    let output = render_data(
        vec![Row {
            id: "1".into(),
            name: "Alpha".into(),
        }],
        &OutputFormat::Table,
    )
    .unwrap();

    assert_eq!(
        output,
        "┌────┬───────┐\n│ id │ name  │\n├────┼───────┤\n│ 1  │ Alpha │\n└────┴───────┘"
    );
}

#[test]
fn renders_json_space_data() {
    let spaces = vec![Space {
        id: "space-1".into(),
        name: "Personal".into(),
        description: "My personal space".into(),
        home_id: None,
        icon: None,
        extra: Default::default(),
    }];
    let output = render_data(spaces, &OutputFormat::Json).unwrap();
    assert_eq!(
        output,
        "[\n  {\n    \"id\": \"space-1\",\n    \"name\": \"Personal\",\n    \"description\": \"My personal space\",\n    \"home_id\": null,\n    \"icon\": null\n  }\n]"
    );
}

#[test]
fn renders_table_property() {
    let props = vec![Property {
        id: "prop-1".into(),
        key: "title".into(),
        name: "Title".into(),
        format: PropertyFormat::Text,
        object: String::new(),
        extra: Default::default(),
    }];
    let output = render_data(props, &OutputFormat::Table).unwrap();
    assert_eq!(
        output,
        "┌────────┬───────┬───────┬────────┬────────┐\n│ id     │ key   │ name  │ format │ object │\n├────────┼───────┼───────┼────────┼────────┤\n│ prop-1 │ title │ Title │ text   │        │\n└────────┴───────┴───────┴────────┴────────┘"
    );
}
