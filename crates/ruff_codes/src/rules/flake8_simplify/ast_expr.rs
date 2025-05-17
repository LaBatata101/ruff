use ruff_diagnostics::{AlwaysFixableViolation, FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for environment variables that are not capitalized.
///
/// ## Why is this bad?
/// By convention, environment variables should be capitalized.
///
/// On Windows, environment variables are case-insensitive and are converted to
/// uppercase, so using lowercase environment variables can lead to subtle bugs.
///
/// ## Example
/// ```python
/// import os
///
/// os.environ["foo"]
/// ```
///
/// Use instead:
/// ```python
/// import os
///
/// os.environ["FOO"]
/// ```
///
/// ## Fix safety
///
/// This fix is always marked as unsafe because automatically capitalizing environment variable names
/// can change program behavior in environments where the variable names are case-sensitive, such as most
/// Unix-like systems.
///
/// ## References
/// - [Python documentation: `os.environ`](https://docs.python.org/3/library/os.html#os.environ)
#[derive(ViolationMetadata)]
pub struct UncapitalizedEnvironmentVariables {
    expected: SourceCodeSnippet,
    actual: SourceCodeSnippet,
}

impl Violation for UncapitalizedEnvironmentVariables {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UncapitalizedEnvironmentVariables { expected, actual } = self;
        if let (Some(expected), Some(actual)) = (expected.full_display(), actual.full_display()) {
            format!("Use capitalized environment variable `{expected}` instead of `{actual}`")
        } else {
            "Use capitalized environment variable".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let UncapitalizedEnvironmentVariables { expected, actual } = self;
        if let (Some(expected), Some(actual)) = (expected.full_display(), actual.full_display()) {
            Some(format!("Replace `{actual}` with `{expected}`"))
        } else {
            Some("Capitalize environment variable".to_string())
        }
    }
}

/// ## What it does
/// Checks for `dict.get()` calls that pass `None` as the default value.
///
/// ## Why is this bad?
/// `None` is the default value for `dict.get()`, so it is redundant to pass it
/// explicitly.
///
/// ## Example
/// ```python
/// ages = {"Tom": 23, "Maria": 23, "Dog": 11}
/// age = ages.get("Cat", None)
/// ```
///
/// Use instead:
/// ```python
/// ages = {"Tom": 23, "Maria": 23, "Dog": 11}
/// age = ages.get("Cat")
/// ```
///
/// ## References
/// - [Python documentation: `dict.get`](https://docs.python.org/3/library/stdtypes.html#dict.get)
#[derive(ViolationMetadata)]
pub struct DictGetWithNoneDefault {
    expected: SourceCodeSnippet,
    actual: SourceCodeSnippet,
}

impl AlwaysFixableViolation for DictGetWithNoneDefault {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DictGetWithNoneDefault { expected, actual } = self;
        if let (Some(expected), Some(actual)) = (expected.full_display(), actual.full_display()) {
            format!("Use `{expected}` instead of `{actual}`")
        } else {
            "Use `dict.get()` without default value".to_string()
        }
    }

    fn fix_title(&self) -> String {
        let DictGetWithNoneDefault { expected, actual } = self;
        if let (Some(expected), Some(actual)) = (expected.full_display(), actual.full_display()) {
            format!("Replace `{actual}` with `{expected}`")
        } else {
            "Remove default value".to_string()
        }
    }
}
