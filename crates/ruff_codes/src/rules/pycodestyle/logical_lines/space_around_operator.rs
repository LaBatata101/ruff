use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for extraneous tabs before an operator.
///
/// ## Why is this bad?
/// According to [PEP 8], operators should be surrounded by at most a single space on either
/// side.
///
/// ## Example
/// ```python
/// a = 4\t+ 5
/// ```
///
/// Use instead:
/// ```python
/// a = 4 + 5
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#whitespace-in-expressions-and-statements
#[derive(ViolationMetadata)]
pub struct TabBeforeOperator;

impl AlwaysFixableViolation for TabBeforeOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Tab before operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous whitespace before an operator.
///
/// ## Why is this bad?
/// According to [PEP 8], operators should be surrounded by at most a single space on either
/// side.
///
/// ## Example
/// ```python
/// a = 4  + 5
/// ```
///
/// Use instead:
/// ```python
/// a = 4 + 5
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#whitespace-in-expressions-and-statements
#[derive(ViolationMetadata)]
pub struct MultipleSpacesBeforeOperator;

impl AlwaysFixableViolation for MultipleSpacesBeforeOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple spaces before operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous tabs after an operator.
///
/// ## Why is this bad?
/// According to [PEP 8], operators should be surrounded by at most a single space on either
/// side.
///
/// ## Example
/// ```python
/// a = 4 +\t5
/// ```
///
/// Use instead:
/// ```python
/// a = 4 + 5
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#whitespace-in-expressions-and-statements
#[derive(ViolationMetadata)]
pub struct TabAfterOperator;

impl AlwaysFixableViolation for TabAfterOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Tab after operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous whitespace after an operator.
///
/// ## Why is this bad?
/// According to [PEP 8], operators should be surrounded by at most a single space on either
/// side.
///
/// ## Example
/// ```python
/// a = 4 +  5
/// ```
///
/// Use instead:
/// ```python
/// a = 4 + 5
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#whitespace-in-expressions-and-statements
#[derive(ViolationMetadata)]
pub struct MultipleSpacesAfterOperator;

impl AlwaysFixableViolation for MultipleSpacesAfterOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple spaces after operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous tabs after a comma.
///
/// ## Why is this bad?
/// Commas should be followed by one space, never tabs.
///
/// ## Example
/// ```python
/// a = 4,\t5
/// ```
///
/// Use instead:
/// ```python
/// a = 4, 5
/// ```
///
#[derive(ViolationMetadata)]
pub struct TabAfterComma;

impl AlwaysFixableViolation for TabAfterComma {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Tab after comma".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous whitespace after a comma.
///
/// ## Why is this bad?
/// Consistency is good. This rule helps ensure you have a consistent
/// formatting style across your project.
///
/// ## Example
/// ```python
/// a = 4,    5
/// ```
///
/// Use instead:
/// ```python
/// a = 4, 5
/// ```
#[derive(ViolationMetadata)]
pub struct MultipleSpacesAfterComma;

impl AlwaysFixableViolation for MultipleSpacesAfterComma {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple spaces after comma".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}
