use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for an invalid `mode` argument in `open` calls.
///
/// ## Why is this bad?
/// The `open` function accepts a `mode` argument that specifies how the file
/// should be opened (e.g., read-only, write-only, append-only, etc.).
///
/// Python supports a variety of open modes: `r`, `w`, `a`, and `x`, to control
/// reading, writing, appending, and creating, respectively, along with
/// `b` (binary mode), `+` (read and write), and `U` (universal newlines),
/// the latter of which is only valid alongside `r`. This rule detects both
/// invalid combinations of modes and invalid characters in the mode string
/// itself.
///
/// ## Example
/// ```python
/// with open("file", "rwx") as f:
///     return f.read()
/// ```
///
/// Use instead:
/// ```python
/// with open("file", "r") as f:
///     return f.read()
/// ```
///
/// ## References
/// - [Python documentation: `open`](https://docs.python.org/3/library/functions.html#open)
#[derive(ViolationMetadata)]
pub struct BadOpenMode {
    mode: String,
}

impl Violation for BadOpenMode {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BadOpenMode { mode } = self;
        format!("`{mode}` is not a valid mode for `open`")
    }
}