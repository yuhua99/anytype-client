use std::{fs::OpenOptions, io::Write};

use anyhow::{Context, Result, anyhow};

use crate::{
    api::AnytypeClient,
    cli::{FilesArgs, FilesCommand, OutputFormat},
    output::{print_one, print_success},
};

use super::resolve_space;

pub async fn run(client: &AnytypeClient, args: FilesArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        FilesCommand::Upload { space, path } => {
            if !path.is_file() {
                return Err(anyhow!("file not found: {}", path.display()));
            }
            let id = resolve_space(client, &space).await?;
            print_one(client.upload_file(&id, &path).await?, output)
        }
        FilesCommand::Download {
            space,
            file_id,
            output,
            width,
            force,
        } => {
            if width.is_some_and(|width| width <= 0) {
                return Err(anyhow!("--width must be > 0"));
            }
            let id = resolve_space(client, &space).await?;
            let bytes = client.download_file(&id, &file_id, width).await?;
            write_file(&output, &bytes, force)?;
            print_success(format!(
                "Downloaded {} bytes to {}",
                bytes.len(),
                output.display()
            ));
            Ok(())
        }
        FilesCommand::Delete {
            space,
            file_id,
            skip_bin,
        } => {
            let id = resolve_space(client, &space).await?;
            client.delete_file(&id, &file_id, skip_bin).await?;
            print_success(format!("Deleted file {file_id}"));
            Ok(())
        }
    }
}

fn write_file(path: &std::path::Path, bytes: &[u8], force: bool) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create(true);
    if force {
        options.truncate(true);
    } else {
        options.create_new(true);
    }
    let mut file = options.open(path).with_context(|| {
        format!(
            "failed to create {} (use --force to overwrite)",
            path.display()
        )
    })?;
    file.write_all(bytes)
        .with_context(|| format!("failed to write {}", path.display()))
}
