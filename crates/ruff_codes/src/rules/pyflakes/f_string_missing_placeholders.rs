use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for f-strings that do not contain any placeholder expressions.
///
/// ## Why is this bad?
/// f-strings are a convenient way to format strings, but they are not
/// necessary if there are no placeholder expressions to format. In this
/// case, a regular string should be used instead, as an f-string without
/// placeholders can be confusing for readers, who may expect such a
/// placeholder to be present.
///
/// An f-string without any placeholders could also indicate that the
/// author forgot to add a placeholder expression.
///
/// ## Example
/// ```python
/// f"Hello, world!"
/// ```
///
/// Use instead:
/// ```python
/// "Hello, world!"
/// ```
///
/// **Note:** to maintain compatibility with PyFlakes, this rule only flags
/// f-strings that are part of an implicit concatenation if _none_ of the
/// f-string segments contain placeholder expressions.
///
/// For example:
///
/// ```python
/// # Will not be flagged.
/// (
///     f"Hello,"
///     f" {name}!"
/// )
///
/// # Will be flagged.
/// (
///     f"Hello,"
///     f" World!"
/// )
/// ```
///
/// See [#10885](https://github.com/astral-sh/ruff/issues/10885) for more.
///
/// ## References
/// - [PEP 498 – Literal String Interpolation](https://peps.python.org/pep-0498/)
#[derive(ViolationMetadata)]
pub struct FStringMissingPlaceholders;

impl AlwaysFixableViolation for FStringMissingPlaceholders {
    #[derive_message_formats]
    fn message(&self) -> String {
        "f-string without any placeholders".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove extraneous `f` prefix".to_string()
    }
}