use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of unnamed numerical constants ("magic") values in
/// comparisons.
///
/// ## Why is this bad?
/// The use of "magic" values can make code harder to read and maintain, as
/// readers will have to infer the meaning of the value from the context.
/// Such values are discouraged by [PEP 8].
///
/// For convenience, this rule excludes a variety of common values from the
/// "magic" value definition, such as `0`, `1`, `""`, and `"__main__"`.
///
/// ## Example
/// ```python
/// def apply_discount(price: float) -> float:
///     if price <= 100:
///         return price / 2
///     else:
///         return price
/// ```
///
/// Use instead:
/// ```python
/// MAX_DISCOUNT = 100
///
///
/// def apply_discount(price: float) -> float:
///     if price <= MAX_DISCOUNT:
///         return price / 2
///     else:
///         return price
/// ```
///
/// ## Options
/// - `lint.pylint.allow-magic-value-types`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#constants
#[derive(ViolationMetadata)]
pub struct MagicValueComparison {
    value: String,
}

impl Violation for MagicValueComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MagicValueComparison { value } = self;
        format!(
            "Magic value used in comparison, consider replacing `{value}` with a constant variable"
        )
    }
}