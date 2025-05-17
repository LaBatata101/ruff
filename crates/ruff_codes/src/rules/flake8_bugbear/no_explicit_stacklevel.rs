use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `warnings.warn` calls without an explicit `stacklevel` keyword
/// argument.
///
/// ## Why is this bad?
/// The `warnings.warn` method uses a `stacklevel` of 1 by default, which
/// will output a stack frame of the line on which the "warn" method
/// is called. Setting it to a higher number will output a stack frame
/// from higher up the stack.
///
/// It's recommended to use a `stacklevel` of 2 or higher, to give the caller
/// more context about the warning.
///
/// ## Example
/// ```python
/// warnings.warn("This is a warning")
/// ```
///
/// Use instead:
/// ```python
/// warnings.warn("This is a warning", stacklevel=2)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe because it changes
/// the behavior of the code. Moreover, the fix will assign
/// a stacklevel of 2, while the user may wish to assign a
/// higher stacklevel to address the diagnostic.
///
/// ## References
/// - [Python documentation: `warnings.warn`](https://docs.python.org/3/library/warnings.html#warnings.warn)
#[derive(ViolationMetadata)]
pub struct NoExplicitStacklevel;

impl AlwaysFixableViolation for NoExplicitStacklevel {
    #[derive_message_formats]
    fn message(&self) -> String {
        "No explicit `stacklevel` keyword argument found".to_string()
    }

    fn fix_title(&self) -> String {
        "Set `stacklevel=2`".to_string()
    }
}