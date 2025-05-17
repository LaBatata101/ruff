use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for self-assignment of variables.
///
/// ## Why is this bad?
/// Self-assignment of variables is redundant and likely a mistake.
///
/// ## Example
/// ```python
/// country = "Poland"
/// country = country
/// ```
///
/// Use instead:
/// ```python
/// country = "Poland"
/// ```
#[derive(ViolationMetadata)]
pub struct SelfAssigningVariable {
    name: String,
}

impl Violation for SelfAssigningVariable {
    #[derive_message_formats]
    fn message(&self) -> String {
        let SelfAssigningVariable { name } = self;
        format!("Self-assignment of variable `{name}`")
    }
}