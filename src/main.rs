// SPDX-License-Identifier: MPL-2.0
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fmt::Write,
    fs::{self, File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};

use clap::Parser;
use color_eyre::{
    eyre::{bail, Context, Ok, Result},
    Section,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// path to the notice file
    #[arg(short, long, default_value = "./NOTICE")]
    notice: PathBuf,

    /// directory to apply the notice
    #[arg(long, default_value = ".")]
    dir: PathBuf,

    /// select files by extension, e.g. rs,js,kt. Needs to be in sync with comment_styles
    #[arg(short, long, default_values_t = ["rs".to_string()], value_delimiter = ',')]
    extensions: Vec<String>,

    /// comment styles, e.g. //,#,// Needs to be in sync with extensions
    #[arg(short, long, default_values_t = ["//".to_string()], value_delimiter = ',')]
    comment_styles: Vec<String>,

    /// remove notice in files
    #[arg(short, long)]
    remove: bool,

    /// replace notice with string
    #[arg(long)]
    replace_with_string: Option<String>,

    /// replace notice with file content
    #[arg(long)]
    replace_with_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let Args {
        notice: notice_path,
        dir,
        extensions,
        comment_styles,
        remove,
        replace_with_string,
        replace_with_file,
    } = Args::parse();

    for (extension, comment_style) in extensions.into_iter().zip(comment_styles) {
        let current_notice = fmt_notice_content(&notice_path, &comment_style)?;

        if remove {
            remove_notice(&dir, &current_notice, &extension)?;
            return Ok(());
        }

        if let Some(new_content_string) = replace_with_string {
            remove_notice(&dir, &current_notice, &extension)?;
            replace_notice_with_string(&notice_path, &new_content_string)?;
            let notice = fmt_notice_content(&notice_path, &comment_style)?;
            insert_notice(&dir, &notice, &extension)?;
            return Ok(());
        }

        if let Some(ref new_content_path) = replace_with_file {
            if notice_path.canonicalize()? == new_content_path.canonicalize()? {
                bail!("Don't do that, you will be appending the new notice over the previous one instead of replacing");
            }
            if new_content_path.exists() {
                remove_notice(&dir, &current_notice, &extension)?;
                replace_notice_with_file(&notice_path, new_content_path)?;
                let notice = fmt_notice_content(&notice_path, &comment_style)?;
                insert_notice(&dir, &notice, &extension)?;
                return Ok(());
            }
        }

        insert_notice(&dir, &current_notice, &extension)?;
    }

    Ok(())
}

fn fmt_notice_content(file_path: &Path, comment_style: &str) -> color_eyre::Result<String> {
    let mut notice_file = File::open(file_path)
        .wrap_err("Notice file not found")
        .suggestion("Use the --notice option or create a ./NOTICE file.")?;

    let mut contents = String::new();
    notice_file.read_to_string(&mut contents)?;

    let mut notice_comment = String::new();
    for line in contents.lines() {
        writeln!(&mut notice_comment, "{comment_style} {line}")?;
    }

    // Add a whole new line
    notice_comment += "\n";

    Ok(notice_comment)
}

fn insert_notice(dir: &Path, notice: &str, extension: &str) -> color_eyre::Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let file_path = entry.path();

        // Skip if the file doesn't have a extension
        let Some(file_extension) = file_path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        // Skip if the extension was not provided
        if file_extension != extension {
            continue;
        }

        // Read the existing content of the file
        let mut file = File::open(file_path)?;
        let mut existing_content = Vec::new();
        file.read_to_end(&mut existing_content)?;

        // Convert notice to bytes
        let notice_bytes = notice.trim().as_bytes();

        // Skip if the content of the header already exists
        if existing_content.starts_with(notice_bytes) {
            continue;
        }

        // Create a new file with the same path
        let mut new_file = File::create(file_path)?;

        // Write notice
        std::io::Write::write_all(&mut new_file, notice.as_bytes())?;

        // Write existing content
        std::io::Write::write_all(&mut new_file, &existing_content)?;
    }
    Ok(())
}

fn remove_notice(dir: &Path, notice: &str, extension: &str) -> color_eyre::Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let file_path = entry.path();

        // Skip if the file doesn't have a extension
        let Some(file_extension) = file_path.extension().and_then(|e| e.to_str()) else {
            continue;
        };

        // Skip if the extension was not provided
        if file_extension != extension {
            continue;
        }

        // Read the existing content of the file
        let mut file = File::open(file_path)?;
        let mut existing_content = Vec::new();
        file.read_to_end(&mut existing_content)?;

        // Convert notice to bytes
        let notice_bytes = notice.trim().as_bytes();

        // Skip if the content of the notice doesn't exists
        if !existing_content.starts_with(notice_bytes) {
            continue;
        }

        // Create a new file with the same path
        let mut new_file = File::create(file_path)?;

        // Remove notice
        existing_content.drain(0..=notice_bytes.len() + '\n'.len_utf8());

        // Write existing content
        std::io::Write::write_all(&mut new_file, &existing_content)?;
    }
    Ok(())
}

fn replace_notice_with_string(notice_path: &Path, new_content: &str) -> color_eyre::Result<()> {
    let mut notice_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(notice_path)
        .wrap_err("Notice file not found")
        .suggestion("Use the --notice option or create a ./NOTICE file.")?;

    std::io::Write::write_all(&mut notice_file, new_content.as_bytes())?;
    Ok(())
}

fn replace_notice_with_file(notice_path: &Path, new_content_path: &Path) -> color_eyre::Result<()> {
    let mut notice_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(notice_path)
        .wrap_err("Notice file not found")
        .suggestion("Use the --notice option or create a ./NOTICE file.")?;
    let new_content = fs::read_to_string(new_content_path)?;

    std::io::Write::write_all(&mut notice_file, new_content.as_bytes())?;
    Ok(())
}
