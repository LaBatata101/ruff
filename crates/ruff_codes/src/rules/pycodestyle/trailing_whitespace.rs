use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for superfluous trailing whitespace.
///
/// ## Why is this bad?
/// According to [PEP 8], "avoid trailing whitespace anywhere. Because it’s usually
/// invisible, it can be confusing"
///
/// ## Example
/// ```python
/// spam(1) \n#
/// ```
///
/// Use instead:
/// ```python
/// spam(1)\n#
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#other-recommendations
#[derive(ViolationMetadata)]
pub struct TrailingWhitespace;

impl AlwaysFixableViolation for TrailingWhitespace {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Trailing whitespace".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove trailing whitespace".to_string()
    }
}

/// ## What it does
/// Checks for superfluous whitespace in blank lines.
///
/// ## Why is this bad?
/// According to [PEP 8], "avoid trailing whitespace anywhere. Because it’s usually
/// invisible, it can be confusing"
///
/// ## Example
/// ```python
/// class Foo(object):\n    \n    bang = 12
/// ```
///
/// Use instead:
/// ```python
/// class Foo(object):\n\n    bang = 12
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#other-recommendations
#[derive(ViolationMetadata)]
pub struct BlankLineWithWhitespace;

impl AlwaysFixableViolation for BlankLineWithWhitespace {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Blank line contains whitespace".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove whitespace from blank line".to_string()
    }
}
