use anyhow::Result;
use serde::Serialize;
use tabled::{Table, Tabled, settings::Style};

use crate::cli::OutputFormat;

pub fn print_data<T>(data: Vec<T>, output: &OutputFormat) -> Result<()>
where
    T: Serialize + Tabled,
{
    match output {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&data)?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&data)?),
        OutputFormat::Table => println!("{}", Table::new(data).with(Style::sharp())),
    }
    Ok(())
}

pub fn print_one<T>(data: T, output: &OutputFormat) -> Result<()>
where
    T: Serialize + Tabled,
{
    match output {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&data)?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&data)?),
        OutputFormat::Table => println!("{}", Table::new([data]).with(Style::sharp())),
    }
    Ok(())
}
