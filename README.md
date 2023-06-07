# GPT Feeder

## What is it?

One problem with ChatGPT 3.5 is the lack of context that ChatGPT 3.5 has of your code base, due to the low word limit for the input. GPT-4 is capable of handling over 25,000 words of text, allowing for use cases like long form content creation, extended conversations, and document search and analysis [[1]](https://openai.com/product/gpt-4). Enter GPT Feeder.

GPT Feeder is a command-line application that scans the entire codebase, and produces one string consisting of all filenames and file contents you want to be included. This string can then be fed into ChatGPT-4, and the model can generate code based on the context of your code base.

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
gpt-feeder --include "*.rs,*.md"

# If you want to output the result to a file, use the `--out` flag
gpt-feeder --include "*.rs,*.md" --out "output.txt"

# Print help
gpt-feeder --help
```

Note that `gpt-feeder` automatically copies the content to your clipboard.

You can now paste this string into ChatGPT-4, and generate code based on the context of your code base. 🚀

### Example

![Demo](/static/demo.png)

## Final Notes

For more information on how to contribute and run the application, see [CONTRIBUTING.md](CONTRIBUTING.md).
