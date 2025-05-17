use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `subprocess.run` that set the `universal_newlines`
/// keyword argument.
///
/// ## Why is this bad?
/// As of Python 3.7, the `universal_newlines` keyword argument has been
/// renamed to `text`, and now exists for backwards compatibility. The
/// `universal_newlines` keyword argument may be removed in a future version of
/// Python. Prefer `text`, which is more explicit and readable.
///
/// ## Example
/// ```python
/// import subprocess
///
/// subprocess.run(["foo"], universal_newlines=True)
/// ```
///
/// Use instead:
/// ```python
/// import subprocess
///
/// subprocess.run(["foo"], text=True)
/// ```
///
/// ## References
/// - [Python 3.7 release notes](https://docs.python.org/3/whatsnew/3.7.html#subprocess)
/// - [Python documentation: `subprocess.run`](https://docs.python.org/3/library/subprocess.html#subprocess.run)
#[derive(ViolationMetadata)]
pub struct ReplaceUniversalNewlines;

impl AlwaysFixableViolation for ReplaceUniversalNewlines {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`universal_newlines` is deprecated, use `text`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `text` keyword argument".to_string()
    }
}