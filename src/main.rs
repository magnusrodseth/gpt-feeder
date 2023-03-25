extern crate glob;

use std::path::Path;

use clap::Parser;
use glob::glob;
use std::io::Write;

/// A command-line application that scans the entire codebase,
/// and produces one string consisting of all filenames and file contents.
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

    /// A comma-separated list of patterns to exclude from the scan. If not present, no filetypes are excluded.
    /// This is a glob pattern, or a list of glob patterns.
    /// Example: `*.rs`, `*.rs,*.ts`
    #[arg(short, long)]
    exclude: Option<String>,

    /// A boolean flag for whether to include hidden files.
    /// If not present, hidden files are not included.
    #[arg(long, default_value = "false")]
    hidden_files_included: bool,
}

fn is_hidden_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let includes = args
        .include
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|include| glob(include))
        .collect::<Vec<_>>();

    let excludes = args
        .exclude
        .unwrap_or_default()
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|include| glob(include))
        .collect::<Vec<_>>();

    let mut output = String::new();

    for include in includes {
        for entry in include? {
            let entry = entry?;

            if !args.hidden_files_included && is_hidden_file(&entry) {
                continue;
            }

            let filename = entry.display().to_string();
            let contents = std::fs::read_to_string(&entry)?;
            let formatted = format!("`{}`\n\n```\n{}\n```\n\n", filename, contents);

            output.push_str(&formatted);
        }
    }

    // Output to file or stdout
    if let Some(out) = args.out {
        std::fs::write(out, output)?;
    } else {
        writeln!(std::io::stdout(), "{}", output)?;
    }

    Ok(())
}
