use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `assert` statements that use a string literal as the first
/// argument.
///
/// ## Why is this bad?
/// An `assert` on a non-empty string literal will always pass, while an
/// `assert` on an empty string literal will always fail.
///
/// ## Example
/// ```python
/// assert "always true"
/// ```
#[derive(ViolationMetadata)]
pub struct AssertOnStringLiteral {
    kind: Kind,
}

impl Violation for AssertOnStringLiteral {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.kind {
            Kind::Empty => "Asserting on an empty string literal will never pass".to_string(),
            Kind::NonEmpty => {
                "Asserting on a non-empty string literal will always pass".to_string()
            }
            Kind::Unknown => {
                "Asserting on a string literal may have unintended results".to_string()
            }
        }
    }
}

// FIX: duplicated with rufe_rule_pylint::assert_on_string_literal
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Kind {
    Empty,
    NonEmpty,
    Unknown,
}