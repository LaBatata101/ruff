use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `subprocess.run` without an explicit `check` argument.
///
/// ## Why is this bad?
/// By default, `subprocess.run` does not check the return code of the process
/// it runs. This can lead to silent failures.
///
/// Instead, consider using `check=True` to raise an exception if the process
/// fails, or set `check=False` explicitly to mark the behavior as intentional.
///
/// ## Example
/// ```python
/// import subprocess
///
/// subprocess.run(["ls", "nonexistent"])  # No exception raised.
/// ```
///
/// Use instead:
/// ```python
/// import subprocess
///
/// subprocess.run(["ls", "nonexistent"], check=True)  # Raises exception.
/// ```
///
/// Or:
/// ```python
/// import subprocess
///
/// subprocess.run(["ls", "nonexistent"], check=False)  # Explicitly no check.
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe for function calls that contain
/// `**kwargs`, as adding a `check` keyword argument to such a call may lead
/// to a duplicate keyword argument error.
///
/// ## References
/// - [Python documentation: `subprocess.run`](https://docs.python.org/3/library/subprocess.html#subprocess.run)
#[derive(ViolationMetadata)]
pub struct SubprocessRunWithoutCheck;

impl AlwaysFixableViolation for SubprocessRunWithoutCheck {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`subprocess.run` without explicit `check` argument".to_string()
    }

    fn fix_title(&self) -> String {
        "Add explicit `check=False`".to_string()
    }
}