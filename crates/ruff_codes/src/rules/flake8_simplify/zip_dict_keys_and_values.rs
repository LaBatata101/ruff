use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for use of `zip()` to iterate over keys and values of a dictionary at once.
///
/// ## Why is this bad?
/// The `dict` type provides an `.items()` method which is faster and more readable.
///
/// ## Example
/// ```python
/// flag_stars = {"USA": 50, "Slovenia": 3, "Panama": 2, "Australia": 6}
///
/// for country, stars in zip(flag_stars.keys(), flag_stars.values()):
///     print(f"{country}'s flag has {stars} stars.")
/// ```
///
/// Use instead:
/// ```python
/// flag_stars = {"USA": 50, "Slovenia": 3, "Panama": 2, "Australia": 6}
///
/// for country, stars in flag_stars.items():
///     print(f"{country}'s flag has {stars} stars.")
/// ```
///
/// ## References
/// - [Python documentation: `dict.items`](https://docs.python.org/3/library/stdtypes.html#dict.items)
#[derive(ViolationMetadata)]
pub struct ZipDictKeysAndValues {
    expected: SourceCodeSnippet,
    actual: SourceCodeSnippet,
}

impl AlwaysFixableViolation for ZipDictKeysAndValues {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ZipDictKeysAndValues { expected, actual } = self;
        if let (Some(expected), Some(actual)) = (expected.full_display(), actual.full_display()) {
            format!("Use `{expected}` instead of `{actual}`")
        } else {
            "Use `dict.items()` instead of `zip(dict.keys(), dict.values())`".to_string()
        }
    }

    fn fix_title(&self) -> String {
        let ZipDictKeysAndValues { expected, actual } = self;
        if let (Some(expected), Some(actual)) = (expected.full_display(), actual.full_display()) {
            format!("Replace `{actual}` with `{expected}`")
        } else {
            "Replace `zip(dict.keys(), dict.values())` with `dict.items()`".to_string()
        }
    }
}