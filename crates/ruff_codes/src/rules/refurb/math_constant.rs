use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for literals that are similar to constants in `math` module.
///
/// ## Why is this bad?
/// Hard-coding mathematical constants like π increases code duplication,
/// reduces readability, and may lead to a lack of precision.
///
/// ## Example
/// ```python
/// A = 3.141592 * r**2
/// ```
///
/// Use instead:
/// ```python
/// A = math.pi * r**2
/// ```
///
/// ## References
/// - [Python documentation: `math` constants](https://docs.python.org/3/library/math.html#constants)
#[derive(ViolationMetadata)]
pub struct MathConstant {
    literal: String,
    constant: &'static str,
}

impl Violation for MathConstant {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let MathConstant { literal, constant } = self;
        format!("Replace `{literal}` with `math.{constant}`")
    }

    fn fix_title(&self) -> Option<String> {
        let MathConstant { constant, .. } = self;
        Some(format!("Use `math.{constant}`"))
    }
}