use anyhow::Result;
use serde::Serialize;
use tabled::{Table, Tabled, settings::Style};

use crate::cli::OutputFormat;

/// Output rendering contract (Phase 5):
/// - `Json`: machine-stable, pretty-printed JSON for scripting/automation/CI.
/// - `Yaml`: machine-stable YAML for human+machine or config use.
/// - `Table`: human-friendly tabular output using `tabled` with sharp style for CLI display.
///
/// All formatting lives here. Commands call these after fetching data; services/API never render.
pub fn render_data<T>(data: Vec<T>, output: &OutputFormat) -> Result<String>
where
    T: Serialize + Tabled,
{
    match output {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&data)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&data)?),
        OutputFormat::Table => Ok(Table::new(data).with(Style::sharp()).to_string()),
    }
}

pub fn render_one<T>(data: T, output: &OutputFormat) -> Result<String>
where
    T: Serialize + Tabled,
{
    match output {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&data)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&data)?),
        OutputFormat::Table => Ok(Table::new([data]).with(Style::sharp()).to_string()),
    }
}

pub fn print_data<T>(data: Vec<T>, output: &OutputFormat) -> Result<()>
where
    T: Serialize + Tabled,
{
    println!("{}", render_data(data, output)?);
    Ok(())
}

pub fn print_one<T>(data: T, output: &OutputFormat) -> Result<()>
where
    T: Serialize + Tabled,
{
    println!("{}", render_one(data, output)?);
    Ok(())
}

/// Print a non-data success/status message to stdout.
/// Commands use for post-operation feedback (e.g. "Type deleted", "Downloaded...").
/// This keeps simple messages out of data paths while output layer owns all printing.
pub fn print_success(msg: impl std::fmt::Display) {
    println!("{msg}");
}

/// Print an operation status or result summary to stderr.
/// Used for bulk update summaries, dry-runs, etc. to separate from data output.
pub fn eprint_status(msg: impl std::fmt::Display) {
    eprintln!("{msg}");
}

/// Print a simple total count respecting the output format.
/// Used by object count command (non-grouped case).
pub fn print_count_total(total: usize, output: &OutputFormat) -> Result<()> {
    match output {
        OutputFormat::Json => println!("{{\"total\": {total}}}"),
        OutputFormat::Yaml => println!("total: {total}"),
        OutputFormat::Table => println!("{total}"),
    }
    Ok(())
}

/// Print grouped counts (by type or property) respecting the output format.
/// Used by object count command (grouped case). Exact output shape preserved for CLI compatibility.
pub fn print_grouped_counts(
    counts: &std::collections::BTreeMap<String, usize>,
    total: usize,
    output: &OutputFormat,
) -> Result<()> {
    match output {
        OutputFormat::Json => {
            let mut map = serde_json::Map::new();
            for (key, count) in counts {
                map.insert(key.clone(), serde_json::Value::Number((*count).into()));
            }
            map.insert("total".to_string(), serde_json::Value::Number(total.into()));
            println!("{}", serde_json::to_string_pretty(&map)?);
        }
        OutputFormat::Yaml => {
            let mut map = serde_json::Map::new();
            for (key, count) in counts {
                map.insert(key.clone(), serde_json::Value::Number((*count).into()));
            }
            map.insert("total".to_string(), serde_json::Value::Number(total.into()));
            println!("{}", serde_yaml::to_string(&map)?);
        }
        OutputFormat::Table => {
            for (name, count) in counts {
                println!("{name}: {count}");
            }
            println!("---");
            println!("total: {total}");
        }
    }
    Ok(())
}
