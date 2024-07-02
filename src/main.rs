extern crate glob;

use arboard::Clipboard;
use glob::Pattern;
use std::env;
use std::io::Write;
use std::path::Path;

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

fn print_help() {
    println!(
        "Usage: gpt-feeder [OPTIONS] --include <FILES...>

A command-line application that scans the entire codebase,
and produces one string consisting of all filenames and file contents that you want included.

Options:
    --help                       Prints help information
    --out <FILE>                 An optional output file. If not present, stdout is used.
    --include <FILES...>         A list of patterns to include in the scan. This is a glob pattern, or a list of glob patterns.
                                 Example: `*.rs`, `*.rs,*.ts`
    --hidden_files_included      A boolean flag for whether to include hidden files. Defaults to false.
    --copy                       A boolean flag for whether to copy the generated content to the clipboard. Defaults to true."
    );
}

fn main() -> anyhow::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check for --help flag
    if args.contains(&String::from("--help")) {
        print_help();
        std::process::exit(0);
    }

    // Define default values
    let mut copy = true;
    let mut hidden_files_included = false;
    let mut out: Option<String> = None;
    let mut file_paths: Vec<String> = vec![];

    // Process arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--copy" => copy = true,
            "--hidden_files_included" => hidden_files_included = true,
            "--out" => {
                if i + 1 < args.len() {
                    out = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --out flag requires a file path argument");
                    print_help();
                    std::process::exit(1);
                }
            }
            "--include" => {
                if i + 1 < args.len() {
                    while i + 1 < args.len() && !args[i + 1].starts_with("--") {
                        file_paths.push(args[i + 1].clone());
                        i += 1;
                    }
                } else {
                    eprintln!("Error: --include flag requires at least one file path");
                    print_help();
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unexpected argument {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // Check if mandatory --include argument is present
    if file_paths.is_empty() {
        eprintln!("Error: --include flag is required");
        print_help();
        std::process::exit(1);
    }

    // Debug print for included files
    dbg!(&file_paths);

    let mut output = String::new();
    let feedignore = feedignore();

    for file_path in file_paths {
        let entry = Path::new(&file_path);

        if entry.is_dir() {
            continue;
        }

        if !hidden_files_included && is_hidden_file(&entry) {
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

    // Write to the output file if provided, otherwise to stdout
    if let Some(out) = out {
        std::fs::write(out, &output)?;
    } else {
        writeln!(std::io::stdout(), "{}", output)?;
    }

    // Copy to clipboard if the flag is set
    if copy {
        let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
        clipboard
            .set_text(&output)
            .expect("Failed to copy to clipboard");
    }

    Ok(())
}
