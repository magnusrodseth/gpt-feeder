extern crate glob;

use std::path::Path;

use clap::Parser;
use glob::{glob, Pattern};
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

/// Reads the `.feedignore` file, and parses the patterns.
///
/// This file acts much like a `.gitignore` file, using glob patterns to ignore files and directories.
fn feedignore() -> Vec<Pattern> {
    let feedignore = std::fs::read_to_string(".feedignore").unwrap_or_default();

    if feedignore.is_empty() {
        return vec![];
    }

    feedignore
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|s| {
            if std::path::Path::new(s).is_dir() {
                format!("**/{}*", s)
            } else {
                format!("**/{}", s)
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|include| Pattern::new(include).expect("Invalid pattern in `.feedignore`"))
        .collect::<Vec<_>>()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut output = String::new();
    let includes = includes(args.include);
    let feedignore = feedignore();

    for include in includes {
        for entry in include? {
            let entry = entry?;

            if entry.is_dir() {
                continue;
            }

            if !args.hidden_files_included && is_hidden_file(&entry) {
                continue;
            }

            if feedignore
                .iter()
                .any(|pattern| pattern.matches_path(&entry))
            {
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;

    const TEST_FILENAME: &str = "out.test";

    #[test]
    fn test_is_hidden_file() {
        assert!(is_hidden_file(Path::new(".git")));
        assert!(is_hidden_file(Path::new("src/.git")));
        assert!(is_hidden_file(Path::new("src/.git/")));
        assert!(is_hidden_file(Path::new("src/.git/README.md")));
        assert!(is_hidden_file(Path::new("src/.git/README.md/")));
        assert!(is_hidden_file(Path::new(".gitignore")));

        assert!(!is_hidden_file(Path::new("src/README.md")));
        assert!(!is_hidden_file(Path::new("src/README.md/")));
    }

    #[test]
    fn test_includes() {
        let actual = includes("*.rs".to_string());
        assert_eq!(actual.len(), 1);
        assert!(actual[0].is_ok());

        let actual = includes("*.rs,*.ts".to_string());
        assert_eq!(actual.len(), 2);
        assert!(actual[0].is_ok());
        assert!(actual[1].is_ok());
    }

    #[test]
    fn can_run_binary() {
        let mut cmd = Command::cargo_bin("gpt-feeder").expect("Failed to run binary");
        cmd.arg("--include").arg("*.rs");
        cmd.assert().success();
    }

    #[test]
    fn can_create_output_file() {
        let mut cmd = Command::cargo_bin("gpt-feeder").expect("Failed to run binary");
        cmd.arg("--include").arg("*.rs");
        cmd.arg("--out").arg(TEST_FILENAME);
        cmd.assert().success();
        assert!(std::path::Path::new(TEST_FILENAME).exists());
        let is_empty = std::fs::read_to_string(TEST_FILENAME)
            .expect("Failed to read test file")
            .is_empty();
        assert!(!is_empty);

        std::fs::remove_file(TEST_FILENAME).expect("Failed to delete test file");
    }
}
