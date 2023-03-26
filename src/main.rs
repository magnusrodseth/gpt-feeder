extern crate glob;

use std::path::Path;

use clap::Parser;
use glob::glob;
use std::io::Write;

/// A command-line application that scans the entire codebase,
/// and produces one string consisting of all filenames and file contents that you want included
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// An optional output file. If not present, stdout is used.
    #[arg(short, long)]
    out: Option<String>,

    /// A comma-separated list of patterns to include in the scan.
    /// This is a glob pattern, or a list of glob patterns.
    /// Example: `*.rs`, `*.rs,*.ts`
    #[arg(short, long)]
    include: String,

    /// A boolean flag for whether to include hidden files.
    /// If not present, hidden files are not included.
    #[arg(long, default_value = "false")]
    hidden_files_included: bool,
}

/// Determines if a file is categorized as hidden.
///
/// If the file is in a directory that starts with a dot, it's hidden.
/// If file starts with a dot, it's hidden.
fn is_hidden_file(path: &Path) -> bool {
    path.components().any(|component| match component {
        std::path::Component::Normal(name) => {
            let name = name
                .to_str()
                .unwrap_or_else(|| panic!("Invalid path: {:?}", path));

            name.starts_with('.')
        }
        _ => false,
    })
}

/// Parses and cleans up the include patterns.
fn includes(includes: String) -> Vec<Result<glob::Paths, glob::PatternError>> {
    includes
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        // Add `**/` to the beginning of each pattern, so that it matches
        // files in subdirectories.
        .map(|s| format!("**/{}", s))
        .collect::<Vec<_>>()
        .iter()
        .map(|include| glob(include))
        .collect::<Vec<_>>()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut output = String::new();
    let includes = includes(args.include);

    for include in includes {
        for entry in include? {
            let entry = entry?;

            if entry.is_dir() {
                continue;
            }

            if !args.hidden_files_included && is_hidden_file(&entry) {
                continue;
            }

            let filename = entry.display().to_string();
            let contents = match std::fs::read_to_string(&entry) {
                Ok(contents) => contents,
                Err(_) => continue,
            };
            let extension = entry.extension().and_then(|s| s.to_str()).unwrap_or("");
            let formatted = format!("`{}`\n\n```{}\n{}\n```\n\n", filename, extension, contents);

            output.push_str(&formatted);
        }
    }

    if let Some(out) = args.out {
        std::fs::write(out, output)?;
    } else {
        writeln!(std::io::stdout(), "{}", output)?;
    }

    Ok(())
}
