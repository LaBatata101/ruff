use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the unnecessary nesting of multiple consecutive context
/// managers.
///
/// ## Why is this bad?
/// In Python 3, a single `with` block can include multiple context
/// managers.
///
/// Combining multiple context managers into a single `with` statement
/// will minimize the indentation depth of the code, making it more
/// readable.
///
/// The following context managers are exempt when used as standalone
/// statements:
///
///  - `anyio`.{`CancelScope`, `fail_after`, `move_on_after`}
///  - `asyncio`.{`timeout`, `timeout_at`}
///  - `trio`.{`fail_after`, `fail_at`, `move_on_after`, `move_on_at`}
///
/// ## Example
/// ```python
/// with A() as a:
///     with B() as b:
///         pass
/// ```
///
/// Use instead:
/// ```python
/// with A() as a, B() as b:
///     pass
/// ```
///
/// ## References
/// - [Python documentation: The `with` statement](https://docs.python.org/3/reference/compound_stmts.html#the-with-statement)
#[derive(ViolationMetadata)]
pub struct MultipleWithStatements;

impl Violation for MultipleWithStatements {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use a single `with` statement with multiple contexts instead of nested `with` \
            statements"
            .to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Combine `with` statements".to_string())
    }
}