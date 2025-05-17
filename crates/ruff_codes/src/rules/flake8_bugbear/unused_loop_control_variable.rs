use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::Certainty;

/// ## What it does
/// Checks for unused variables in loops (e.g., `for` and `while` statements).
///
/// ## Why is this bad?
/// Defining a variable in a loop statement that is never used can confuse
/// readers.
///
/// If the variable is intended to be unused (e.g., to facilitate
/// destructuring of a tuple or other object), prefix it with an underscore
/// to indicate the intent. Otherwise, remove the variable entirely.
///
/// ## Example
/// ```python
/// for i, j in foo:
///     bar(i)
/// ```
///
/// Use instead:
/// ```python
/// for i, _j in foo:
///     bar(i)
/// ```
///
/// ## References
/// - [PEP 8: Naming Conventions](https://peps.python.org/pep-0008/#naming-conventions)
#[derive(ViolationMetadata)]
pub struct UnusedLoopControlVariable {
    /// The name of the loop control variable.
    name: String,
    /// The name to which the variable should be renamed, if it can be
    /// safely renamed.
    rename: Option<String>,
    /// Whether the variable is certain to be unused in the loop body, or
    /// merely suspect. A variable _may_ be used, but undetectably
    /// so, if the loop incorporates by magic control flow (e.g.,
    /// `locals()`).
    certainty: Certainty,
}

impl Violation for UnusedLoopControlVariable {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedLoopControlVariable {
            name, certainty, ..
        } = self;
        match certainty {
            Certainty::Certain => {
                format!("Loop control variable `{name}` not used within loop body")
            }
            Certainty::Uncertain => {
                format!("Loop control variable `{name}` may not be used within loop body")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let UnusedLoopControlVariable { rename, name, .. } = self;

        rename
            .as_ref()
            .map(|rename| format!("Rename unused `{name}` to `{rename}`"))
    }
}