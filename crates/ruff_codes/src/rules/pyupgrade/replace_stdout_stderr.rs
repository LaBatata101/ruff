use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `subprocess.run` that send `stdout` and `stderr` to a
/// pipe.
///
/// ## Why is this bad?
/// As of Python 3.7, `subprocess.run` has a `capture_output` keyword argument
/// that can be set to `True` to capture `stdout` and `stderr` outputs. This is
/// equivalent to setting `stdout` and `stderr` to `subprocess.PIPE`, but is
/// more explicit and readable.
///
/// ## Example
/// ```python
/// import subprocess
///
/// subprocess.run(["foo"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
/// ```
///
/// Use instead:
/// ```python
/// import subprocess
///
/// subprocess.run(["foo"], capture_output=True)
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe because replacing `stdout=subprocess.PIPE` and
/// `stderr=subprocess.PIPE` with `capture_output=True` may delete comments attached
/// to the original arguments.
///
/// ## References
/// - [Python 3.7 release notes](https://docs.python.org/3/whatsnew/3.7.html#subprocess)
/// - [Python documentation: `subprocess.run`](https://docs.python.org/3/library/subprocess.html#subprocess.run)
#[derive(ViolationMetadata)]
pub struct ReplaceStdoutStderr;

impl Violation for ReplaceStdoutStderr {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Prefer `capture_output` over sending `stdout` and `stderr` to `PIPE`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `capture_output` keyword argument".to_string())
    }
}