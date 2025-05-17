use ruff_diagnostics::Violation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `open` and `write` that can be replaced by `pathlib`
/// methods, like `Path.write_text` and `Path.write_bytes`.
///
/// ## Why is this bad?
/// When writing a single string to a file, it's simpler and more concise
/// to use `pathlib` methods like `Path.write_text` and `Path.write_bytes`
/// instead of `open` and `write` calls via `with` statements.
///
/// ## Example
/// ```python
/// with open(filename, "w") as f:
///     f.write(contents)
/// ```
///
/// Use instead:
/// ```python
/// from pathlib import Path
///
/// Path(filename).write_text(contents)
/// ```
///
/// ## References
/// - [Python documentation: `Path.write_bytes`](https://docs.python.org/3/library/pathlib.html#pathlib.Path.write_bytes)
/// - [Python documentation: `Path.write_text`](https://docs.python.org/3/library/pathlib.html#pathlib.Path.write_text)
#[derive(ViolationMetadata)]
pub struct WriteWholeFile {
    filename: SourceCodeSnippet,
    suggestion: SourceCodeSnippet,
}

impl Violation for WriteWholeFile {
    #[derive_message_formats]
    fn message(&self) -> String {
        let filename = self.filename.truncated_display();
        let suggestion = self.suggestion.truncated_display();
        format!("`open` and `write` should be replaced by `Path({filename}).{suggestion}`")
    }
}