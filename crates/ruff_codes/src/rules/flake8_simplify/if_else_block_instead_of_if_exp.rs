use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for `if`-`else`-blocks that can be replaced with a ternary operator.
/// Moreover, in [preview], check if these ternary expressions can be
/// further simplified to binary expressions.
///
/// ## Why is this bad?
/// `if`-`else`-blocks that assign a value to a variable in both branches can
/// be expressed more concisely by using a ternary or binary operator.
///
/// ## Example
///
/// ```python
/// if foo:
///     bar = x
/// else:
///     bar = y
/// ```
///
/// Use instead:
/// ```python
/// bar = x if foo else y
/// ```
///
/// Or, in [preview]:
///
/// ```python
/// if cond:
///     z = cond
/// else:
///     z = other_cond
/// ```
///
/// Use instead:
///
/// ```python
/// z = cond or other_cond
/// ```
///
/// ## Known issues
/// This is an opinionated style rule that may not always be to everyone's
/// taste, especially for code that makes use of complex `if` conditions.
/// Ternary operators can also make it harder to measure [code coverage]
/// with tools that use line profiling.
///
/// ## References
/// - [Python documentation: Conditional expressions](https://docs.python.org/3/reference/expressions.html#conditional-expressions)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
/// [code coverage]: https://github.com/nedbat/coveragepy/issues/509
#[derive(ViolationMetadata)]
pub struct IfElseBlockInsteadOfIfExp {
    /// The ternary or binary expression to replace the `if`-`else`-block.
    contents: String,
    /// Whether to use a binary or ternary assignment.
    kind: AssignmentKind,
}

impl Violation for IfElseBlockInsteadOfIfExp {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let IfElseBlockInsteadOfIfExp { contents, kind } = self;
        match kind {
            AssignmentKind::Ternary => {
                format!("Use ternary operator `{contents}` instead of `if`-`else`-block")
            }
            AssignmentKind::Binary => {
                format!("Use binary operator `{contents}` instead of `if`-`else`-block")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let IfElseBlockInsteadOfIfExp { contents, .. } = self;
        Some(format!("Replace `if`-`else`-block with `{contents}`"))
    }
}

// FIX: duplicated with ruff_rule_flake8_simplify::if_else_block_instead_of_if_exp
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssignmentKind {
    Binary,
    Ternary,
}