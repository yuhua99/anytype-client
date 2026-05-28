use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::models::{IconColor, ObjectLayout, PropertyFormat, SortDirection, SortProperty};

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

#[derive(Debug, Clone, Copy, Default, Args)]
pub struct PageArgs {
    #[arg(long)]
    pub limit: Option<i64>,
    #[arg(long, default_value_t = 0)]
    pub offset: i64,
}

#[derive(Debug, Clone, Args)]
pub struct IconArgs {
    #[arg(long)]
    pub clear_icon: bool,
    /// Legacy emoji shortcut. Prefer --icon-emoji.
    #[arg(long)]
    pub icon: Option<String>,
    #[arg(long)]
    pub icon_emoji: Option<String>,
    #[arg(long)]
    pub icon_file: Option<String>,
    #[arg(long)]
    pub icon_name: Option<String>,
    #[arg(long, value_enum, default_value_t = IconColor::Yellow)]
    pub icon_color: IconColor,
}

#[derive(Debug, Clone, Args)]
pub struct PropertyValueArgs {
    /// Repeatable JSON object matching Anytype property value schema.
    #[arg(long = "property")]
    pub properties: Vec<String>,
    /// JSON array matching Anytype properties schema.
    #[arg(long = "properties-json")]
    pub properties_json: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct PropertyLinkArgs {
    /// Repeatable JSON object: {"key":"...","name":"...","format":"text"}.
    #[arg(long = "property")]
    pub properties: Vec<String>,
    /// JSON array of property link objects.
    #[arg(long = "properties-json")]
    pub properties_json: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    Auth(AuthArgs),
    Spaces(SpacesArgs),
    Objects(ObjectsArgs),
    Search(SearchArgs),
    Types(TypesArgs),
    Properties(PropertiesArgs),
    Tags(TagsArgs),
    Files(FilesArgs),
    Collections(CollectionsArgs),
    Members(MembersArgs),
}

#[derive(Args)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand)]
pub enum AuthCommand {
    Desktop {
        #[arg(long, default_value = "anyclient")]
        app_name: String,
        #[arg(long)]
        force: bool,
    },
    Headless {
        #[arg(long)]
        api_key: Option<String>,
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
    List {
        #[command(flatten)]
        page: PageArgs,
    },
    Get {
        space: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "")]
        description: String,
    },
    Update {
        space: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
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
        #[command(flatten)]
        page: PageArgs,
    },
    Get {
        space: String,
        object_id: String,
        #[arg(long, default_value = "md", value_parser = ["md"])]
        format: String,
    },
    Create {
        space: String,
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "page")]
        r#type: String,
        #[arg(long, default_value = "")]
        body: String,
        #[command(flatten)]
        icon: IconArgs,
        #[arg(long)]
        template: Option<String>,
        #[command(flatten)]
        properties: PropertyValueArgs,
    },
    Update {
        space: String,
        object_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        r#type: Option<String>,
        #[arg(long)]
        markdown: Option<String>,
        #[command(flatten)]
        icon: IconArgs,
        #[command(flatten)]
        properties: PropertyValueArgs,
        /// Property name/key/id holding tags (e.g. "Tag").
        #[arg(long)]
        tag_property: Option<String>,
        /// Tag name/key/id(s) to add (repeatable).
        #[arg(long = "tag-add")]
        tag_add: Vec<String>,
        /// Tag name/key/id(s) to remove (repeatable).
        #[arg(long = "tag-remove")]
        tag_remove: Vec<String>,
    },
    Delete {
        space: String,
        object_id: String,
    },
    Export {
        space: String,
        object_id: String,
        #[arg(long, default_value = "md", value_parser = ["md"])]
        format: String,
    },
    /// Batch update multiple objects by IDs or search query.
    UpdateMany {
        space: String,
        /// File with one object ID per line.
        #[arg(long)]
        ids_file: Option<PathBuf>,
        /// Inline object IDs (comma-separated).
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
        /// Search query (used when no IDs provided).
        #[arg(long)]
        query: Option<String>,
        /// Filter by type key(s).
        #[arg(long, value_delimiter = ',')]
        types: Vec<String>,
        /// Property name/key/id holding tags.
        #[arg(long)]
        tag_property: Option<String>,
        /// Tag name/key/id(s) to add.
        #[arg(long = "tag-add")]
        tag_add: Vec<String>,
        /// Tag name/key/id(s) to remove.
        #[arg(long = "tag-remove")]
        tag_remove: Vec<String>,
        /// Preview changes without applying.
        #[arg(long)]
        dry_run: bool,
    },
    /// Find objects with simplified filters.
    Find {
        space: String,
        /// Filter by type key/name.
        #[arg(long)]
        r#type: Option<String>,
        /// Filter by tag name/key/id (requires --tag-property).
        #[arg(long)]
        tag: Option<String>,
        /// Property name for --tag filter.
        #[arg(long)]
        tag_property: Option<String>,
        /// Filter by property value: key=value.
        #[arg(long)]
        property: Option<String>,
        /// Filter by name (substring match).
        #[arg(long)]
        name: Option<String>,
        /// Find objects missing a property.
        #[arg(long)]
        missing_property: Option<String>,
        /// Output only object IDs.
        #[arg(long)]
        ids_only: bool,
        /// Output only object names.
        #[arg(long)]
        names_only: bool,
    },
    /// Count objects grouped by type or property.
    Count {
        space: String,
        /// Group by: "type" or "property:<key>".
        #[arg(long)]
        group_by: Option<String>,
    },
}

#[derive(Args)]
pub struct SearchArgs {
    #[arg(long, default_value = "")]
    pub query: String,
    #[arg(long, value_delimiter = ',')]
    pub types: Vec<String>,
    #[arg(long, value_enum)]
    pub sort: Option<SortProperty>,
    #[arg(long, value_enum, default_value = "desc")]
    pub direction: SortDirection,
    #[arg(long)]
    pub filters: Option<String>,
    #[arg(long)]
    pub space: Option<String>,
    #[command(flatten)]
    pub page: PageArgs,
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
        #[command(flatten)]
        page: PageArgs,
    },
    Get {
        space: String,
        type_id: String,
    },
    Create {
        space: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        plural_name: String,
        #[arg(long, value_enum)]
        layout: ObjectLayout,
        #[arg(long)]
        key: Option<String>,
        #[command(flatten)]
        icon: IconArgs,
        #[command(flatten)]
        properties: PropertyLinkArgs,
    },
    Update {
        space: String,
        type_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        plural_name: Option<String>,
        #[arg(long, value_enum)]
        layout: Option<ObjectLayout>,
        #[arg(long)]
        key: Option<String>,
        #[command(flatten)]
        icon: IconArgs,
        #[command(flatten)]
        properties: PropertyLinkArgs,
    },
    Delete {
        space: String,
        type_id: String,
    },
    Templates {
        space: String,
        type_id: String,
        #[command(flatten)]
        page: PageArgs,
    },
    TemplateGet {
        space: String,
        type_id: String,
        template_id: String,
    },
}

#[derive(Args)]
pub struct PropertiesArgs {
    #[command(subcommand)]
    pub command: PropertiesCommand,
}

#[derive(Subcommand)]
pub enum PropertiesCommand {
    List {
        space: String,
        #[command(flatten)]
        page: PageArgs,
    },
    Get {
        space: String,
        property_id: String,
    },
    Create {
        space: String,
        #[arg(long)]
        name: String,
        #[arg(long, value_enum)]
        format: PropertyFormat,
        #[arg(long)]
        key: Option<String>,
        #[arg(long = "tag")]
        tags: Vec<String>,
        #[arg(long = "tags-json")]
        tags_json: Option<String>,
    },
    Update {
        space: String,
        property_id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        key: Option<String>,
    },
    Delete {
        space: String,
        property_id: String,
    },
}

#[derive(Args)]
pub struct TagsArgs {
    #[command(subcommand)]
    pub command: TagsCommand,
}

#[derive(Subcommand)]
pub enum TagsCommand {
    List {
        space: String,
        property_id: String,
        #[command(flatten)]
        page: PageArgs,
    },
    Get {
        space: String,
        property_id: String,
        tag_id: String,
    },
    Create {
        space: String,
        property_id: String,
        #[arg(long)]
        name: String,
        #[arg(long, value_enum)]
        color: IconColor,
        #[arg(long)]
        key: Option<String>,
    },
    Update {
        space: String,
        property_id: String,
        tag_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        color: Option<IconColor>,
        #[arg(long)]
        key: Option<String>,
    },
    Delete {
        space: String,
        property_id: String,
        tag_id: String,
    },
}

#[derive(Args)]
pub struct FilesArgs {
    #[command(subcommand)]
    pub command: FilesCommand,
}

#[derive(Subcommand)]
pub enum FilesCommand {
    Upload {
        space: String,
        path: PathBuf,
    },
    Download {
        space: String,
        file_id: String,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long)]
        width: Option<i64>,
        #[arg(long)]
        force: bool,
    },
    Delete {
        space: String,
        file_id: String,
        #[arg(long)]
        skip_bin: bool,
    },
}

#[derive(Args)]
pub struct CollectionsArgs {
    #[command(subcommand)]
    pub command: CollectionsCommand,
}

#[derive(Subcommand)]
pub enum CollectionsCommand {
    Views {
        space: String,
        collection_id: String,
        #[command(flatten)]
        page: PageArgs,
    },
    Objects {
        space: String,
        collection_id: String,
        view_id: String,
        #[command(flatten)]
        page: PageArgs,
    },
    Add {
        space: String,
        collection_id: String,
        object_ids: Vec<String>,
    },
    Remove {
        space: String,
        collection_id: String,
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
    List {
        space: String,
        #[command(flatten)]
        page: PageArgs,
    },
    Get {
        space: String,
        member_id: String,
    },
}
