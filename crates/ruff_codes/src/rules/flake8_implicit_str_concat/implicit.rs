use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for implicitly concatenated strings on a single line.
///
/// ## Why is this bad?
/// While it is valid Python syntax to concatenate multiple string or byte
/// literals implicitly (via whitespace delimiters), it is unnecessary and
/// negatively affects code readability.
///
/// In some cases, the implicit concatenation may also be unintentional, as
/// code formatters are capable of introducing single-line implicit
/// concatenations when collapsing long lines.
///
/// ## Example
/// ```python
/// z = "The quick " "brown fox."
/// ```
///
/// Use instead:
/// ```python
/// z = "The quick brown fox."
/// ```
#[derive(ViolationMetadata)]
pub struct SingleLineImplicitStringConcatenation;

impl Violation for SingleLineImplicitStringConcatenation {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Implicitly concatenated string literals on one line".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Combine string literals".to_string())
    }
}

/// ## What it does
/// Checks for implicitly concatenated strings that span multiple lines.
///
/// ## Why is this bad?
/// For string literals that wrap across multiple lines, [PEP 8] recommends
/// the use of implicit string concatenation within parentheses instead of
/// using a backslash for line continuation, as the former is more readable
/// than the latter.
///
/// By default, this rule will only trigger if the string literal is
/// concatenated via a backslash. To disallow implicit string concatenation
/// altogether, set the [`lint.flake8-implicit-str-concat.allow-multiline`] option
/// to `false`.
///
/// ## Example
/// ```python
/// z = "The quick brown fox jumps over the lazy "\
///     "dog."
/// ```
///
/// Use instead:
/// ```python
/// z = (
///     "The quick brown fox jumps over the lazy "
///     "dog."
/// )
/// ```
///
/// ## Options
/// - `lint.flake8-implicit-str-concat.allow-multiline`
///
/// ## Formatter compatibility
/// Using this rule with `allow-multiline = false` can be incompatible with the
/// formatter because the [formatter] can introduce new multi-line implicitly
/// concatenated strings. We recommend to either:
///
/// * Enable `ISC001` to disallow all implicit concatenated strings
/// * Setting `allow-multiline = true`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#maximum-line-length
/// [formatter]:https://docs.astral.sh/ruff/formatter/
#[derive(ViolationMetadata)]
pub struct MultiLineImplicitStringConcatenation;

impl Violation for MultiLineImplicitStringConcatenation {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Implicitly concatenated string literals over multiple lines".to_string()
    }
}
