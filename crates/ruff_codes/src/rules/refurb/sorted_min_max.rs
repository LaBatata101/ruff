use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `sorted()` to retrieve the minimum or maximum value in
/// a sequence.
///
/// ## Why is this bad?
/// Using `sorted()` to compute the minimum or maximum value in a sequence is
/// inefficient and less readable than using `min()` or `max()` directly.
///
/// ## Example
/// ```python
/// nums = [3, 1, 4, 1, 5]
/// lowest = sorted(nums)[0]
/// highest = sorted(nums)[-1]
/// highest = sorted(nums, reverse=True)[0]
/// ```
///
/// Use instead:
/// ```python
/// nums = [3, 1, 4, 1, 5]
/// lowest = min(nums)
/// highest = max(nums)
/// ```
///
/// ## Fix safety
/// In some cases, migrating to `min` or `max` can lead to a change in behavior,
/// notably when breaking ties.
///
/// As an example, `sorted(data, key=itemgetter(0), reverse=True)[0]` will return
/// the _last_ "minimum" element in the list, if there are multiple elements with
/// the same key. However, `min(data, key=itemgetter(0))` will return the _first_
/// "minimum" element in the list in the same scenario.
///
/// As such, this rule's fix is marked as unsafe when the `reverse` keyword is used.
///
/// ## References
/// - [Python documentation: `min`](https://docs.python.org/3/library/functions.html#min)
/// - [Python documentation: `max`](https://docs.python.org/3/library/functions.html#max)
#[derive(ViolationMetadata)]
pub struct SortedMinMax {
    min_max: MinMax,
}

impl Violation for SortedMinMax {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.min_max {
            MinMax::Min => {
                "Prefer `min` over `sorted()` to compute the minimum value in a sequence"
                    .to_string()
            }
            MinMax::Max => {
                "Prefer `max` over `sorted()` to compute the maximum value in a sequence"
                    .to_string()
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.min_max {
            MinMax::Min => "Replace with `min`",
            MinMax::Max => "Replace with `max`",
        };
        Some(title.to_string())
    }
}

// FIX: duplicated with ruff_rule_refurb::sorted_min_max
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MinMax {
    /// E.g., `min(nums)`
    Min,
    /// E.g., `max(nums)`
    Max,
}

impl MinMax {
    fn as_str(self) -> &'static str {
        match self {
            Self::Min => "min",
            Self::Max => "max",
        }
    }
}

impl std::fmt::Display for MinMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
        }
    }
}