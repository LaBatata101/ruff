use ruff_diagnostics::{AlwaysFixableViolation, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for compound statements (multiple statements on the same line).
///
/// ## Why is this bad?
/// According to [PEP 8], "compound statements are generally discouraged".
///
/// ## Example
/// ```python
/// if foo == "blah": do_blah_thing()
/// ```
///
/// Use instead:
/// ```python
/// if foo == "blah":
///     do_blah_thing()
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#other-recommendations
#[derive(ViolationMetadata)]
pub struct MultipleStatementsOnOneLineColon;

impl Violation for MultipleStatementsOnOneLineColon {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple statements on one line (colon)".to_string()
    }
}

/// ## What it does
/// Checks for multiline statements on one line.
///
/// ## Why is this bad?
/// According to [PEP 8], including multi-clause statements on the same line is
/// discouraged.
///
/// ## Example
/// ```python
/// do_one(); do_two(); do_three()
/// ```
///
/// Use instead:
/// ```python
/// do_one()
/// do_two()
/// do_three()
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#other-recommendations
#[derive(ViolationMetadata)]
pub struct MultipleStatementsOnOneLineSemicolon;

impl Violation for MultipleStatementsOnOneLineSemicolon {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple statements on one line (semicolon)".to_string()
    }
}

/// ## What it does
/// Checks for statements that end with an unnecessary semicolon.
///
/// ## Why is this bad?
/// A trailing semicolon is unnecessary and should be removed.
///
/// ## Example
/// ```python
/// do_four();  # useless semicolon
/// ```
///
/// Use instead:
/// ```python
/// do_four()
/// ```
#[derive(ViolationMetadata)]
pub struct UselessSemicolon;

impl AlwaysFixableViolation for UselessSemicolon {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Statement ends with an unnecessary semicolon".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove unnecessary semicolon".to_string()
    }
}
