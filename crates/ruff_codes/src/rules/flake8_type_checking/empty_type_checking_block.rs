use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for an empty type-checking block.
///
/// ## Why is this bad?
/// The type-checking block does not do anything and should be removed to avoid
/// confusion.
///
/// ## Example
/// ```python
/// from typing import TYPE_CHECKING
///
/// if TYPE_CHECKING:
///     pass
///
/// print("Hello, world!")
/// ```
///
/// Use instead:
/// ```python
/// print("Hello, world!")
/// ```
///
/// ## References
/// - [PEP 563: Runtime annotation resolution and `TYPE_CHECKING`](https://peps.python.org/pep-0563/#runtime-annotation-resolution-and-type-checking)
#[derive(ViolationMetadata)]
pub struct EmptyTypeCheckingBlock;

impl AlwaysFixableViolation for EmptyTypeCheckingBlock {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Found empty type-checking block".to_string()
    }

    fn fix_title(&self) -> String {
        "Delete empty type-checking block".to_string()
    }
}