use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of unused variables in unpacked assignments.
///
/// ## Why is this bad?
/// A variable that is defined but never used can confuse readers.
///
/// If a variable is intentionally defined-but-not-used, it should be
/// prefixed with an underscore, or some other value that adheres to the
/// [`lint.dummy-variable-rgx`] pattern.
///
/// ## Example
///
/// ```python
/// def get_pair():
///     return 1, 2
///
///
/// def foo():
///     x, y = get_pair()
///     return x
/// ```
///
/// Use instead:
///
/// ```python
/// def foo():
///     x, _ = get_pair()
///     return x
/// ```
///
/// ## Options
/// - `lint.dummy-variable-rgx`
#[derive(ViolationMetadata)]
pub struct UnusedUnpackedVariable {
    pub name: String,
}

impl Violation for UnusedUnpackedVariable {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedUnpackedVariable { name } = self;
        format!("Unpacked variable `{name}` is never used")
    }

    fn fix_title(&self) -> Option<String> {
        Some("Prefix it with an underscore or any other dummy variable pattern".to_string())
    }
}