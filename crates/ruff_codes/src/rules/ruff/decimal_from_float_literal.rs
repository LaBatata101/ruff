use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `Decimal` calls passing a float literal.
///
/// ## Why is this bad?
/// Float literals have limited precision that can lead to unexpected results.
/// The `Decimal` class is designed to handle numbers with fixed-point precision,
/// so a string literal should be used instead.
///
/// ## Example
///
/// ```python
/// num = Decimal(1.2345)
/// ```
///
/// Use instead:
/// ```python
/// num = Decimal("1.2345")
/// ```
///
/// ## Fix Safety
/// This rule's fix is marked as unsafe because it changes the underlying value
/// of the `Decimal` instance that is constructed. This can lead to unexpected
/// behavior if your program relies on the previous value (whether deliberately or not).
#[derive(ViolationMetadata)]
pub struct DecimalFromFloatLiteral;

impl AlwaysFixableViolation for DecimalFromFloatLiteral {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`Decimal()` called with float literal argument".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with string literal".to_string()
    }
}