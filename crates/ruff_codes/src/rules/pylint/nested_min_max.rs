use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::MinMax;

/// ## What it does
/// Checks for nested `min` and `max` calls.
///
/// ## Why is this bad?
/// Nested `min` and `max` calls can be flattened into a single call to improve
/// readability.
///
/// ## Example
///
/// ```python
/// minimum = min(1, 2, min(3, 4, 5))
/// maximum = max(1, 2, max(3, 4, 5))
/// diff = maximum - minimum
/// ```
///
/// Use instead:
///
/// ```python
/// minimum = min(1, 2, 3, 4, 5)
/// maximum = max(1, 2, 3, 4, 5)
/// diff = maximum - minimum
/// ```
///
/// ## Fix safety
///
/// This fix is always unsafe and may change the program's behavior for types without full
/// equivalence relations, such as float comparisons involving `NaN`.
///
/// ```python
/// print(min(2.0, min(float("nan"), 1.0)))  # before fix: 2.0
/// print(min(2.0, float("nan"), 1.0))  # after fix: 1.0
///
/// print(max(1.0, max(float("nan"), 2.0)))  # before fix: 1.0
/// print(max(1.0, float("nan"), 2.0))  # after fix: 2.0
/// ```
///
/// ## References
/// - [Python documentation: `min`](https://docs.python.org/3/library/functions.html#min)
/// - [Python documentation: `max`](https://docs.python.org/3/library/functions.html#max)
#[derive(ViolationMetadata)]
pub struct NestedMinMax {
    func: MinMax,
}

impl Violation for NestedMinMax {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]

    fn message(&self) -> String {
        let NestedMinMax { func } = self;
        format!("Nested `{func}` calls can be flattened")
    }

    fn fix_title(&self) -> Option<String> {
        let NestedMinMax { func } = self;
        Some(format!("Flatten nested `{func}` calls"))
    }
}