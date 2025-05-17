use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if` expressions that can be replaced with `min()` or `max()`
/// calls.
///
/// ## Why is this bad?
/// An `if` expression that selects the lesser or greater of two
/// sub-expressions can be replaced with a `min()` or `max()` call
/// respectively. When possible, prefer `min()` and `max()`, as they're more
/// concise and readable than the equivalent `if` expression.
///
/// ## Example
/// ```python
/// highest_score = score1 if score1 > score2 else score2
/// ```
///
/// Use instead:
/// ```python
/// highest_score = max(score2, score1)
/// ```
///
/// ## References
/// - [Python documentation: `min`](https://docs.python.org/3.11/library/functions.html#min)
/// - [Python documentation: `max`](https://docs.python.org/3.11/library/functions.html#max)
#[derive(ViolationMetadata)]
pub struct IfExprMinMax {
    min_max: MinMax,
    expression: SourceCodeSnippet,
    replacement: SourceCodeSnippet,
}

impl Violation for IfExprMinMax {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self {
            min_max,
            expression,
            replacement,
        } = self;

        match (expression.full_display(), replacement.full_display()) {
            (_, None) => {
                format!("Replace `if` expression with `{min_max}` call")
            }
            (None, Some(replacement)) => {
                format!("Replace `if` expression with `{replacement}`")
            }
            (Some(expression), Some(replacement)) => {
                format!("Replace `{expression}` with `{replacement}`")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Self {
            replacement,
            min_max,
            ..
        } = self;
        if let Some(replacement) = replacement.full_display() {
            Some(format!("Replace with `{replacement}`"))
        } else {
            Some(format!("Replace with `{min_max}` call"))
        }
    }
}

// FIX: duplicated with ruff_rule_refurb::if_expr_min_max
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MinMax {
    Min,
    Max,
}

impl MinMax {
    #[must_use]
    const fn reverse(self) -> Self {
        match self {
            Self::Min => Self::Max,
            Self::Max => Self::Min,
        }
    }

    #[must_use]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Min => "min",
            Self::Max => "max",
        }
    }
}

impl std::fmt::Display for MinMax {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.as_str())
    }
}
