use anyclient::{
    cli::OutputFormat,
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
