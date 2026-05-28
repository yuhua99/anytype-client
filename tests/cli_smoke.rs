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
