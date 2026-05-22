use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "anyclient",
    version,
    about = "Rust CLI/client for Anytype HTTP API"
)]
pub struct Cli {
    #[arg(long, global = true, env = "ANYTYPE_BASE_URL")]
    pub base_url: Option<String>,

    #[arg(long, global = true, env = "ANYTYPE_API_KEY")]
    pub api_key: Option<String>,

    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(short, long, global = true, value_enum, default_value_t = OutputFormat::Table)]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

#[derive(Subcommand)]
pub enum Command {
    Auth(AuthArgs),
    Spaces(SpacesArgs),
    Objects(ObjectsArgs),
    Search(SearchArgs),
    Types(TypesArgs),
    Lists(ListsArgs),
    Members(MembersArgs),
}

#[derive(Args)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Authenticate against Anytype desktop app local API (default: http://127.0.0.1:31009).
    Desktop {
        /// App name used for challenge auth.
        #[arg(long, default_value = "anyclient")]
        app_name: String,

        /// Force re-auth even if config has API key.
        #[arg(long)]
        force: bool,
    },
    /// Store API key for Anytype headless server (default: http://127.0.0.1:31012).
    Headless {
        /// API key from `anytype auth apikey create <name>`.
        #[arg(long)]
        api_key: Option<String>,

        /// Force overwrite even if config has API key.
        #[arg(long)]
        force: bool,
    },
}

#[derive(Args)]
pub struct SpacesArgs {
    #[command(subcommand)]
    pub command: SpacesCommand,
}

#[derive(Subcommand)]
pub enum SpacesCommand {
    List,
    Get {
        space: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "")]
        description: String,
    },
}

#[derive(Args)]
pub struct ObjectsArgs {
    #[command(subcommand)]
    pub command: ObjectsCommand,
}

#[derive(Subcommand)]
pub enum ObjectsCommand {
    List {
        space: String,
    },
    Get {
        space: String,
        object_id: String,
    },
    Create {
        space: String,
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "page")]
        r#type: String,
        #[arg(long, default_value = "")]
        body: String,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        template: Option<String>,
    },
    Delete {
        space: String,
        object_id: String,
    },
    Export {
        space: String,
        object_id: String,
    },
}

#[derive(Args)]
pub struct SearchArgs {
    #[arg(long, default_value = "")]
    pub query: String,
    #[arg(long, value_delimiter = ',')]
    pub types: Vec<String>,
    #[arg(long)]
    pub sort: Option<String>,
    #[arg(long, default_value = "desc")]
    pub direction: String,
    #[arg(long)]
    pub space: Option<String>,
}

#[derive(Args)]
pub struct TypesArgs {
    #[command(subcommand)]
    pub command: TypesCommand,
}

#[derive(Subcommand)]
pub enum TypesCommand {
    List {
        space: String,
    },
    Get {
        space: String,
        type_id: String,
    },
    Templates {
        space: String,
        type_id: String,
    },
    TemplateGet {
        space: String,
        type_id: String,
        template_id: String,
    },
}

#[derive(Args)]
pub struct ListsArgs {
    #[command(subcommand)]
    pub command: ListsCommand,
}

#[derive(Subcommand)]
pub enum ListsCommand {
    Views {
        space: String,
        list_id: String,
    },
    Objects {
        space: String,
        list_id: String,
        view_id: String,
    },
    Add {
        space: String,
        list_id: String,
        object_ids: Vec<String>,
    },
    Remove {
        space: String,
        list_id: String,
        object_id: String,
    },
}

#[derive(Args)]
pub struct MembersArgs {
    #[command(subcommand)]
    pub command: MembersCommand,
}

#[derive(Subcommand)]
pub enum MembersCommand {
    List { space: String },
    Get { space: String, member_id: String },
}
