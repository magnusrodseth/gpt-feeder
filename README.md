# GPT Feeder

## What is it?

GPT models today have a high token capability, allowing for use cases like long form content creation, extended conversations, and document search and analysis.

GPT Feeder is a command-line application that scans the entire codebase, and produces one string consisting of all filenames and file contents you want to be included. This string can then be fed into ChatGPT, and the model can generate code based on the context of your code base.

## Installation

### Using Cargo

Ensure you have `cargo` installed. Then, run the following command:

```bash
# Install the application
cargo install gpt-feeder
```

## Usage

Ensure you have `gpt-feeder` installed. Add all ignored file and directory patterns to `.feedignore`, just like you would with a `.gitignore`. Then, run the following commands:

```bash
# Navigate to the directory you want to scan
cd <directory>

# Run the application with the file extensions you want included
gpt-feeder --include *.rs *.md

# If you want to output the result to a file, use the `--out` flag
gpt-feeder --include *.rs *.md --out output.txt

# Print help
gpt-feeder --help
```

✂️ Note that `gpt-feeder` automatically copies the content to your clipboard.

You can now paste this string into ChatGPT, and generate code based on the context of your code base. 🚀

### Important to note

GPT Feeder relies on your shell to expand glob patterns. This is done in order to allow your shell to give you autocomplete suggestions on the patterns, in addition to making it easier for the program to handle an arbitrary amount of glob patterns whilst still being user-friendly.

### Example

![Demo](/static/demo.png)

## Final Notes

For more information on how to contribute and run the application, see [CONTRIBUTING.md](CONTRIBUTING.md).
