use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for extraneous whitespace after keywords.
///
/// ## Why is this bad?
///
///
/// ## Example
/// ```python
/// True and  False
/// ```
///
/// Use instead:
/// ```python
/// True and False
/// ```
#[derive(ViolationMetadata)]
pub struct MultipleSpacesAfterKeyword;

impl AlwaysFixableViolation for MultipleSpacesAfterKeyword {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple spaces after keyword".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous whitespace before keywords.
///
/// ## Why is this bad?
///
///
/// ## Example
/// ```python
/// True  and False
/// ```
///
/// Use instead:
/// ```python
/// True and False
/// ```
#[derive(ViolationMetadata)]
pub struct MultipleSpacesBeforeKeyword;

impl AlwaysFixableViolation for MultipleSpacesBeforeKeyword {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple spaces before keyword".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous tabs after keywords.
///
/// ## Why is this bad?
///
///
/// ## Example
/// ```python
/// True and\tFalse
/// ```
///
/// Use instead:
/// ```python
/// True and False
/// ```
#[derive(ViolationMetadata)]
pub struct TabAfterKeyword;

impl AlwaysFixableViolation for TabAfterKeyword {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Tab after keyword".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}

/// ## What it does
/// Checks for extraneous tabs before keywords.
///
/// ## Why is this bad?
///
///
/// ## Example
/// ```python
/// True\tand False
/// ```
///
/// Use instead:
/// ```python
/// True and False
/// ```
#[derive(ViolationMetadata)]
pub struct TabBeforeKeyword;

impl AlwaysFixableViolation for TabBeforeKeyword {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Tab before keyword".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with single space".to_string()
    }
}
