use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `else` clauses on loops without a `break` statement.
///
/// ## Why is this bad?
/// When a loop includes an `else` statement, the code inside the `else` clause
/// will be executed if the loop terminates "normally" (i.e., without a
/// `break`).
///
/// If a loop _always_ terminates "normally" (i.e., does _not_ contain a
/// `break`), then the `else` clause is redundant, as the code inside the
/// `else` clause will always be executed.
///
/// In such cases, the code inside the `else` clause can be moved outside the
/// loop entirely, and the `else` clause can be removed.
///
/// ## Example
/// ```python
/// for item in items:
///     print(item)
/// else:
///     print("All items printed")
/// ```
///
/// Use instead:
/// ```python
/// for item in items:
///     print(item)
/// print("All items printed")
/// ```
///
/// ## References
/// - [Python documentation: `break` and `continue` Statements, and `else` Clauses on Loops](https://docs.python.org/3/tutorial/controlflow.html#break-and-continue-statements-and-else-clauses-on-loops)
#[derive(ViolationMetadata)]
pub struct UselessElseOnLoop;

impl Violation for UselessElseOnLoop {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`else` clause on loop without a `break` statement; remove the `else` and dedent its contents".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove `else`".to_string())
    }
}