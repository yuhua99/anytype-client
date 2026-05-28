use anyclient::cli::{Cli, Command, ObjectsCommand};
use clap::{CommandFactory, Parser};

#[test]
fn top_level_help_renders() {
    let mut cmd = Cli::command();
    let help = cmd.render_long_help().to_string();

    assert!(help.contains("Rust CLI/client for Anytype HTTP API"));
    assert!(help.contains("Usage:"));
    assert!(help.contains("Commands:"));
    assert!(help.contains("search"));
    assert!(help.contains("objects"));
}

#[test]
fn parses_search_command() {
    let cli = Cli::try_parse_from([
        "anyclient",
        "search",
        "--query",
        "task",
        "--types",
        "page,task",
        "--sort",
        "last_modified_date",
        "--direction",
        "asc",
        "--space",
        "space-id",
        "--limit",
        "10",
    ])
    .unwrap();

    match cli.command {
        Command::Search(args) => {
            assert_eq!(args.query, "task");
            assert_eq!(args.types, ["page", "task"]);
            assert_eq!(args.space.as_deref(), Some("space-id"));
            assert_eq!(args.page.limit, Some(10));
        }
        _ => panic!("expected search command"),
    }
}

#[test]
fn parses_nested_object_update_many_command() {
    let cli = Cli::try_parse_from([
        "anyclient",
        "objects",
        "update-many",
        "space-id",
        "--ids",
        "obj-1,obj-2",
        "--tag-property",
        "status",
        "--tag-add",
        "done",
        "--dry-run",
    ])
    .unwrap();

    match cli.command {
        Command::Objects(args) => match args.command {
            ObjectsCommand::UpdateMany {
                space,
                ids,
                tag_property,
                tag_add,
                dry_run,
                ..
            } => {
                assert_eq!(space, "space-id");
                assert_eq!(ids, ["obj-1", "obj-2"]);
                assert_eq!(tag_property.as_deref(), Some("status"));
                assert_eq!(tag_add, ["done"]);
                assert!(dry_run);
            }
            _ => panic!("expected update-many command"),
        },
        _ => panic!("expected objects command"),
    }
}

#[test]
fn rejects_invalid_search_direction() {
    let err = match Cli::try_parse_from(["anyclient", "search", "--direction", "sideways"]) {
        Ok(_) => panic!("expected invalid direction error"),
        Err(err) => err.to_string(),
    };

    assert!(err.contains("invalid value 'sideways'"));
}

#[test]
fn parses_search_with_typed_filters() {
    let cli = Cli::try_parse_from([
        "anyclient",
        "search",
        "--query",
        "task",
        "--space",
        "abc123",
        "--filters",
        r#"{"operator":"and","conditions":[{"property_key":"status","condition":"eq","select":"done"}]}"#,
        "--output",
        "json",
    ])
    .unwrap();

    match cli.command {
        Command::Search(args) => {
            assert_eq!(args.query, "task");
            assert_eq!(args.space.as_deref(), Some("abc123"));
            assert!(args.filters.is_some());
        }
        _ => panic!("expected search command"),
    }
}

#[test]
fn parses_objects_create_with_properties() {
    let cli = Cli::try_parse_from([
        "anyclient",
        "objects",
        "create",
        "space-id",
        "--name",
        "My Task",
        "--type",
        "task",
        "--property",
        r#"{"key":"status","text":"done"}"#,
        "--property",
        r#"{"key":"tags","multi_select":["tag1","tag2"]}"#,
    ])
    .unwrap();

    match cli.command {
        Command::Objects(args) => match args.command {
            ObjectsCommand::Create {
                space,
                name,
                r#type,
                properties,
                ..
            } => {
                assert_eq!(space, "space-id");
                assert_eq!(name, "My Task");
                assert_eq!(r#type, "task");
                assert_eq!(properties.properties.len(), 2);
            }
            _ => panic!("expected create command"),
        },
        _ => panic!("expected objects command"),
    }
}

#[test]
fn parses_objects_update_with_properties_and_tags() {
    let cli = Cli::try_parse_from([
        "anyclient",
        "objects",
        "update",
        "space-id",
        "obj-id",
        "--name",
        "Updated",
        "--tag-property",
        "status",
        "--tag-add",
        "done",
        "--property",
        r#"{"key":"title","text":"foo"}"#,
    ])
    .unwrap();

    match cli.command {
        Command::Objects(args) => match args.command {
            ObjectsCommand::Update {
                space,
                object_id,
                name,
                tag_property,
                tag_add,
                properties,
                ..
            } => {
                assert_eq!(space, "space-id");
                assert_eq!(object_id, "obj-id");
                assert_eq!(name.as_deref(), Some("Updated"));
                assert_eq!(tag_property.as_deref(), Some("status"));
                assert_eq!(tag_add, ["done"]);
                assert_eq!(properties.properties.len(), 1);
            }
            _ => panic!("expected update command"),
        },
        _ => panic!("expected objects command"),
    }
}
