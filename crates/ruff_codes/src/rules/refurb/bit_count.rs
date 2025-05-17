use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `bin(...).count("1")` to perform a population count.
///
/// ## Why is this bad?
/// In Python 3.10, a `bit_count()` method was added to the `int` class,
/// which is more concise and efficient than converting to a binary
/// representation via `bin(...)`.
///
/// ## Example
/// ```python
/// x = bin(123).count("1")
/// y = bin(0b1111011).count("1")
/// ```
///
/// Use instead:
/// ```python
/// x = (123).bit_count()
/// y = 0b1111011.bit_count()
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe unless the argument to `bin` can be inferred as
/// an instance of a type that implements the `__index__` and `bit_count` methods because this can
/// change the exception raised at runtime for an invalid argument.
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation:`int.bit_count`](https://docs.python.org/3/library/stdtypes.html#int.bit_count)
#[derive(ViolationMetadata)]
pub struct BitCount {
    existing: SourceCodeSnippet,
    replacement: SourceCodeSnippet,
}

impl AlwaysFixableViolation for BitCount {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BitCount { existing, .. } = self;
        let existing = existing.truncated_display();
        format!("Use of `bin({existing}).count('1')`")
    }

    fn fix_title(&self) -> String {
        let BitCount { replacement, .. } = self;
        if let Some(replacement) = replacement.full_display() {
            format!("Replace with `{replacement}`")
        } else {
            "Replace with `.bit_count()`".to_string()
        }
    }
}