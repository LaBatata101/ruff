use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of named expressions (e.g., `a := 42`) that can be
/// replaced by regular assignment statements (e.g., `a = 42`).
///
/// ## Why is this bad?
/// While a top-level named expression is syntactically and semantically valid,
/// it's less clear than a regular assignment statement. Named expressions are
/// intended to be used in comprehensions and generator expressions, where
/// assignment statements are not allowed.
///
/// ## Example
/// ```python
/// (a := 42)
/// ```
///
/// Use instead:
/// ```python
/// a = 42
/// ```
#[derive(ViolationMetadata)]
pub struct NamedExprWithoutContext;

impl Violation for NamedExprWithoutContext {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Named expression used without context".to_string()
    }
}