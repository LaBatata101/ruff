use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `print` calls with unnecessary empty strings as positional
/// arguments and unnecessary `sep` keyword arguments.
///
/// ## Why is this bad?
/// Prefer calling `print` without any positional arguments, which is
/// equivalent and more concise.
///
/// Similarly, when printing one or fewer items, the `sep` keyword argument,
/// (used to define the string that separates the `print` arguments) can be
/// omitted, as it's redundant when there are no items to separate.
///
/// ## Example
/// ```python
/// print("")
/// ```
///
/// Use instead:
/// ```python
/// print()
/// ```
///
/// ## References
/// - [Python documentation: `print`](https://docs.python.org/3/library/functions.html#print)
#[derive(ViolationMetadata)]
pub struct PrintEmptyString {
    reason: Reason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reason {
    /// Ex) `print("")`
    EmptyArgument,
    /// Ex) `print("foo", sep="\t")`
    UselessSeparator,
    /// Ex) `print("", sep="\t")`
    Both,
}

impl Violation for PrintEmptyString {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.reason {
            Reason::EmptyArgument => "Unnecessary empty string passed to `print`".to_string(),
            Reason::UselessSeparator => "Unnecessary separator passed to `print`".to_string(),
            Reason::Both => "Unnecessary empty string and separator passed to `print`".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.reason {
            Reason::EmptyArgument => "Remove empty string",
            Reason::UselessSeparator => "Remove separator",
            Reason::Both => "Remove empty string and separator",
        };
        Some(title.to_string())
    }
}