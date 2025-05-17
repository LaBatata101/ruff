use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for comparisons against NaN values.
///
/// ## Why is this bad?
/// Comparing against a NaN value can lead to unexpected results. For example,
/// `float("NaN") == float("NaN")` will return `False` and, in general,
/// `x == float("NaN")` will always return `False`, even if `x` is `NaN`.
///
/// To determine whether a value is `NaN`, use `math.isnan` or `np.isnan`
/// instead of comparing against `NaN` directly.
///
/// ## Example
/// ```python
/// if x == float("NaN"):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// import math
///
/// if math.isnan(x):
///     pass
/// ```
///
#[derive(ViolationMetadata)]
pub struct NanComparison {
    nan: Nan,
}

impl Violation for NanComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.nan {
            Nan::Math => "Comparing against a NaN value; use `math.isnan` instead".to_string(),
            Nan::NumPy => "Comparing against a NaN value; use `np.isnan` instead".to_string(),
        }
    }
}

// FIX: duplicated with rufe_rule_pylint::nan_comparison
#[derive(Debug, PartialEq, Eq)]
enum Nan {
    /// `math.isnan`
    Math,
    /// `np.isnan`
    NumPy,
}

impl std::fmt::Display for Nan {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Nan::Math => fmt.write_str("math"),
            Nan::NumPy => fmt.write_str("numpy"),
        }
    }
}