use ruff_diagnostics::Violation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `open` and `read` that can be replaced by `pathlib`
/// methods, like `Path.read_text` and `Path.read_bytes`.
///
/// ## Why is this bad?
/// When reading the entire contents of a file into a variable, it's simpler
/// and more concise to use `pathlib` methods like `Path.read_text` and
/// `Path.read_bytes` instead of `open` and `read` calls via `with` statements.
///
/// ## Example
/// ```python
/// with open(filename) as f:
///     contents = f.read()
/// ```
///
/// Use instead:
/// ```python
/// from pathlib import Path
///
/// contents = Path(filename).read_text()
/// ```
///
/// ## References
/// - [Python documentation: `Path.read_bytes`](https://docs.python.org/3/library/pathlib.html#pathlib.Path.read_bytes)
/// - [Python documentation: `Path.read_text`](https://docs.python.org/3/library/pathlib.html#pathlib.Path.read_text)
#[derive(ViolationMetadata)]
pub struct ReadWholeFile {
    filename: SourceCodeSnippet,
    suggestion: SourceCodeSnippet,
}

impl Violation for ReadWholeFile {
    #[derive_message_formats]
    fn message(&self) -> String {
        let filename = self.filename.truncated_display();
        let suggestion = self.suggestion.truncated_display();
        format!("`open` and `read` should be replaced by `Path({filename}).{suggestion}`")
    }
}