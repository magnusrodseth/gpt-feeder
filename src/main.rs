extern crate glob;

use std::{
    io::{self, Write},
    path::Path,
};

use clap::Parser;
use glob::glob;
use walkdir::WalkDir;

/// A command-line application that scans the entire codebase,
/// and produces one string consisting of all filenames and file contents.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// An optional output file. If not present, stdout is used.
    #[arg(short, long)]
    out: Option<String>,

    /// The pattern to include in the scan.
    /// This is a glob pattern.
    /// Example: `*.rs`
    #[arg(short, long)]
    include: String,

    /// The pattern to exclude from the scan. If not present, no filetypes are excluded.
    /// This is a glob pattern.
    /// Example: `*.rs`
    #[arg(short, long)]
    exclude: Option<String>,

    /// A boolean flag for whether to include hidden files.
    /// If not present, hidden files are not included.
    #[arg(long, default_value = "false")]
    hidden_files: bool,
}

fn is_hidden_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let include_regex = &format!("r\"{}\"", args.include)[..];
    // TODO: Extract exclude_regex here
    println!("include_regex: {}", include_regex);

    // Recursively walk the current directory (including subdirectories)
    for entry in WalkDir::new(".") {
        let entry = entry?;
        let name = entry.path().display();

        if entry.file_type().is_dir() {
            continue;
        }

        // If the file fulfills the include glob pattern, and does not fulfill the exclude pattern,
        // then we include it in the output.
        if glo

        if let Some(exclude_pattern) = &args.exclude {
            let exclude_regex = &format!("r\"{}\"", exclude_pattern)[..];
            if regex::Regex::new(exclude_regex)?.is_match(&name.to_string()) {
                continue;
            }
        }

        // If the file is hidden, and we don't want to include hidden files, skip it.
        if !args.hidden_files && is_hidden_file(entry.path()) {
            continue;
        }

        // Read content of file
        let content = std::fs::read_to_string(entry.path()).unwrap_or_default();

        // Print filename and content
        let output = format!("`{}`\n\n```\n{}```", name, content);
        println!("{}", output);

        // Print to stdout or to file
        match &args.out {
            Some(out) => std::fs::write(out, output)?,
            None => io::stdout().write_all(output.as_bytes())?,
        }
    }

    Ok(())
}
