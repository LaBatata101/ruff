use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if` statements that can be replaced with `min()` or `max()`
/// calls.
///
/// ## Why is this bad?
/// An `if` statement that selects the lesser or greater of two sub-expressions
/// can be replaced with a `min()` or `max()` call respectively. Where possible,
/// prefer `min()` and `max()`, as they're more concise and readable than the
/// equivalent `if` statements.
///
/// ## Example
/// ```python
/// if score > highest_score:
///     highest_score = score
/// ```
///
/// Use instead:
/// ```python
/// highest_score = max(highest_score, score)
/// ```
///
/// ## Fix safety
/// This fix is marked unsafe if it would delete any comments within the replacement range.
///
/// An example to illustrate where comments are preserved and where they are not:
///
/// ```py
/// a, b = 0, 10
///
/// if a >= b: # deleted comment
///     # deleted comment
///     a = b # preserved comment
/// ```
///
/// ## References
/// - [Python documentation: `max`](https://docs.python.org/3/library/functions.html#max)
/// - [Python documentation: `min`](https://docs.python.org/3/library/functions.html#min)
#[derive(ViolationMetadata)]
pub struct IfStmtMinMax {
    min_max: MinMax,
    replacement: SourceCodeSnippet,
}

impl Violation for IfStmtMinMax {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self {
            min_max,
            replacement,
        } = self;
        if let Some(replacement) = replacement.full_display() {
            format!("Replace `if` statement with `{replacement}`")
        } else {
            format!("Replace `if` statement with `{min_max}` call")
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Self {
            min_max,
            replacement,
        } = self;
        if let Some(replacement) = replacement.full_display() {
            Some(format!("Replace with `{replacement}`"))
        } else {
            Some(format!("Replace with `{min_max}` call"))
        }
    }
}

// FIX: duplicated with rufe_rule_pylint::if_stmt_min_max
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MinMax {
    Min,
    Max,
}

impl MinMax {
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