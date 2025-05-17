use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary `from_float` and `from_decimal` usages to construct
/// `Decimal` and `Fraction` instances.
///
/// ## Why is this bad?
/// Since Python 3.2, the `Fraction` and `Decimal` classes can be constructed
/// by passing float or decimal instances to the constructor directly. As such,
/// the use of `from_float` and `from_decimal` methods is unnecessary, and
/// should be avoided in favor of the more concise constructor syntax.
///
/// ## Example
/// ```python
/// Decimal.from_float(4.2)
/// Decimal.from_float(float("inf"))
/// Fraction.from_float(4.2)
/// Fraction.from_decimal(Decimal("4.2"))
/// ```
///
/// Use instead:
/// ```python
/// Decimal(4.2)
/// Decimal("inf")
/// Fraction(4.2)
/// Fraction(Decimal(4.2))
/// ```
///
/// ## References
/// - [Python documentation: `decimal`](https://docs.python.org/3/library/decimal.html)
/// - [Python documentation: `fractions`](https://docs.python.org/3/library/fractions.html)
#[derive(ViolationMetadata)]
pub struct UnnecessaryFromFloat {
    method_name: MethodName,
    constructor: Constructor,
}

impl Violation for UnnecessaryFromFloat {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryFromFloat {
            method_name,
            constructor,
        } = self;
        format!("Verbose method `{method_name}` in `{constructor}` construction",)
    }

    fn fix_title(&self) -> Option<String> {
        let UnnecessaryFromFloat { constructor, .. } = self;
        Some(format!("Replace with `{constructor}` constructor"))
    }
}

// FIX: duplicated with ruff_rule_refurb::unnecessary_from_float
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MethodName {
    FromFloat,
    FromDecimal,
}

impl std::fmt::Display for MethodName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MethodName::FromFloat => fmt.write_str("from_float"),
            MethodName::FromDecimal => fmt.write_str("from_decimal"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Constructor {
    Decimal,
    Fraction,
}

impl std::fmt::Display for Constructor {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Constructor::Decimal => fmt.write_str("Decimal"),
            Constructor::Fraction => fmt.write_str("Fraction"),
        }
    }
}