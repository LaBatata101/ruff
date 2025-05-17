use std::fmt::Display;

use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for formatter suppression comments that are ineffective or incompatible
/// with Ruff's formatter.
///
/// ## Why is this bad?
/// Suppression comments that do not actually prevent formatting could cause unintended changes
/// when the formatter is run.
///
/// ## Example
/// In the following example, all suppression comments would cause
/// a rule violation.
///
/// ```python
/// def decorator():
///     pass
///
///
/// @decorator
/// # fmt: off
/// def example():
///     if True:
///         # fmt: skip
///         expression = [
///             # fmt: off
///             1,
///             2,
///         ]
///         # yapf: disable
///     # fmt: on
///     # yapf: enable
/// ```
///
/// ## Fix safety
///
/// This fix is always marked as unsafe because it deletes the invalid suppression comment,
/// rather than trying to move it to a valid position, which the user more likely intended.
///
#[derive(ViolationMetadata)]
pub struct InvalidFormatterSuppressionComment {
    reason: IgnoredReason,
}

impl AlwaysFixableViolation for InvalidFormatterSuppressionComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "This suppression comment is invalid because {}",
            self.reason
        )
    }

    fn fix_title(&self) -> String {
        "Remove this comment".to_string()
    }
}

// FIX: duplicated with ruff_rule_ruff::invalid_formatter_suppression_comment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IgnoredReason {
    InNonStatement,
    AfterDecorator,
    SkipHasToBeTrailing,
    FmtOnCannotBeTrailing,
    FmtOffAboveBlock,
}

impl Display for IgnoredReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InNonStatement => write!(
                f,
                "it cannot be in an expression, pattern, argument list, or other non-statement"
            ),
            Self::AfterDecorator => {
                write!(f, "it cannot be after a decorator")
            }
            Self::SkipHasToBeTrailing => {
                write!(f, "it cannot be on its own line")
            }
            Self::FmtOnCannotBeTrailing => {
                write!(f, "it cannot be at the end of a line")
            }
            Self::FmtOffAboveBlock => {
                write!(f, "it cannot be directly above an alternate body")
            }
        }
    }
}