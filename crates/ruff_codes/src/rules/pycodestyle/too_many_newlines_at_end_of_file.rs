use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for files with multiple trailing blank lines.
///
/// In the case of notebooks, this check is applied to
/// each cell separately.
///
/// ## Why is this bad?
/// Trailing blank lines in a file are superfluous.
///
/// However, the last line of the file should end with a newline.
///
/// ## Example
/// ```python
/// spam(1)\n\n\n
/// ```
///
/// Use instead:
/// ```python
/// spam(1)\n
/// ```
#[derive(ViolationMetadata)]
pub struct TooManyNewlinesAtEndOfFile {
    num_trailing_newlines: u32,
    in_notebook: bool,
}

impl AlwaysFixableViolation for TooManyNewlinesAtEndOfFile {
    #[derive_message_formats]
    fn message(&self) -> String {
        let domain = if self.in_notebook { "cell" } else { "file" };
        // We expect a single trailing newline; so two trailing newlines is one too many, three
        // trailing newlines is two too many, etc.
        if self.num_trailing_newlines > 2 {
            format!("Too many newlines at end of {domain}")
        } else {
            format!("Extra newline at end of {domain}")
        }
    }

    fn fix_title(&self) -> String {
        let title = if self.num_trailing_newlines > 2 {
            "Remove trailing newlines"
        } else {
            "Remove trailing newline"
        };
        title.to_string()
    }
}