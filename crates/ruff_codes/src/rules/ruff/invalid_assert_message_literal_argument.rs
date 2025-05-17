use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for invalid use of literals in assert message arguments.
///
/// ## Why is this bad?
/// An assert message which is a non-string literal was likely intended
/// to be used in a comparison assertion, rather than as a message.
///
/// ## Example
/// ```python
/// fruits = ["apples", "plums", "pears"]
/// fruits.filter(lambda fruit: fruit.startwith("p"))
/// assert len(fruits), 2  # True unless the list is empty
/// ```
///
/// Use instead:
/// ```python
/// fruits = ["apples", "plums", "pears"]
/// fruits.filter(lambda fruit: fruit.startwith("p"))
/// assert len(fruits) == 2
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidAssertMessageLiteralArgument;

impl Violation for InvalidAssertMessageLiteralArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Non-string literal used as assert message".to_string()
    }
}