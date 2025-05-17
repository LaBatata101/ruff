use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary `pass` statements and ellipsis (`...`) literals in
/// functions, classes, and other blocks.
///
/// ## Why is this bad?
/// In Python, the `pass` statement and ellipsis (`...`) literal serve as
/// placeholders, allowing for syntactically correct empty code blocks. The
/// primary purpose of these nodes is to avoid syntax errors in situations
/// where a statement or expression is syntactically required, but no code
/// needs to be executed.
///
/// If a `pass` or ellipsis is present in a code block that includes at least
/// one other statement (even, e.g., a docstring), it is unnecessary and should
/// be removed.
///
/// ## Example
/// ```python
/// def func():
///     """Placeholder docstring."""
///     pass
/// ```
///
/// Use instead:
/// ```python
/// def func():
///     """Placeholder docstring."""
/// ```
///
/// Or, given:
/// ```python
/// def func():
///     """Placeholder docstring."""
///     ...
/// ```
///
/// Use instead:
/// ```python
/// def func():
///     """Placeholder docstring."""
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe in the rare case that the `pass` or ellipsis
/// is followed by a string literal, since removal of the placeholder would convert the
/// subsequent string literal into a docstring.
///
/// ## References
/// - [Python documentation: The `pass` statement](https://docs.python.org/3/reference/simple_stmts.html#the-pass-statement)
#[derive(ViolationMetadata)]
pub struct UnnecessaryPlaceholder {
    kind: Placeholder,
}

impl AlwaysFixableViolation for UnnecessaryPlaceholder {
    #[derive_message_formats]
    fn message(&self) -> String {
        match &self.kind {
            Placeholder::Pass => "Unnecessary `pass` statement".to_string(),
            Placeholder::Ellipsis => "Unnecessary `...` literal".to_string(),
        }
    }

    fn fix_title(&self) -> String {
        let title = match &self.kind {
            Placeholder::Pass => "Remove unnecessary `pass`",
            Placeholder::Ellipsis => "Remove unnecessary `...`",
        };
        title.to_string()
    }
}

// FIX: duplicated with ruff_rule_flake8_pie::unnecessary_placeholder
#[derive(Debug, PartialEq, Eq)]
enum Placeholder {
    Pass,
    Ellipsis,
}

impl std::fmt::Display for Placeholder {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pass => fmt.write_str("pass"),
            Self::Ellipsis => fmt.write_str("..."),
        }
    }
}