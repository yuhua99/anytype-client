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
