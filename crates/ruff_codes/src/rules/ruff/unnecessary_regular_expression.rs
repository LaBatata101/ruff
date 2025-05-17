use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
///
/// Checks for uses of the `re` module that can be replaced with builtin `str` methods.
///
/// ## Why is this bad?
///
/// Performing checks on strings directly can make the code simpler, may require
/// less escaping, and will often be faster.
///
/// ## Example
///
/// ```python
/// re.sub("abc", "", s)
/// ```
///
/// Use instead:
///
/// ```python
/// s.replace("abc", "")
/// ```
///
/// ## Details
///
/// The rule reports the following calls when the first argument to the call is
/// a plain string literal, and no additional flags are passed:
///
/// - `re.sub`
/// - `re.match`
/// - `re.search`
/// - `re.fullmatch`
/// - `re.split`
///
/// For `re.sub`, the `repl` (replacement) argument must also be a string literal,
/// not a function. For `re.match`, `re.search`, and `re.fullmatch`, the return
/// value must also be used only for its truth value.
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe if the affected expression contains comments. Otherwise,
/// the fix can be applied safely.
///
/// ## References
/// - [Python Regular Expression HOWTO: Common Problems - Use String Methods](https://docs.python.org/3/howto/regex.html#use-string-methods)
#[derive(ViolationMetadata)]
pub struct UnnecessaryRegularExpression {
    replacement: Option<String>,
}

impl Violation for UnnecessaryRegularExpression {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Plain string pattern passed to `re` function".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some(format!("Replace with `{}`", self.replacement.as_ref()?))
    }
}