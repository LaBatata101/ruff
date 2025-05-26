use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `is` and `is not` comparisons against literals, like integers,
/// strings, or lists.
///
/// ## Why is this bad?
/// The `is` and `is not` comparators operate on identity, in that they check
/// whether two objects are the same object. If the objects are not the same
/// object, the comparison will always be `False`. Using `is` and `is not` with
/// constant literals often works "by accident", but are not guaranteed to produce
/// the expected result.
///
/// As of Python 3.8, using `is` and `is not` with constant literals will produce
/// a `SyntaxWarning`.
///
/// This rule will also flag `is` and `is not` comparisons against non-constant
/// literals, like lists, sets, and dictionaries. While such comparisons will
/// not raise a `SyntaxWarning`, they are still likely to be incorrect, as they
/// will compare the identities of the objects instead of their values, which
/// will always evaluate to `False`.
///
/// Instead, use `==` and `!=` to compare literals, which will compare the
/// values of the objects instead of their identities.
///
/// ## Example
/// ```python
/// x = 200
/// if x is 200:
///     print("It's 200!")
/// ```
///
/// Use instead:
/// ```python
/// x = 200
/// if x == 200:
///     print("It's 200!")
/// ```
///
/// ## References
/// - [Python documentation: Identity comparisons](https://docs.python.org/3/reference/expressions.html#is-not)
/// - [Python documentation: Value comparisons](https://docs.python.org/3/reference/expressions.html#value-comparisons)
/// - [_Why does Python log a SyntaxWarning for ‘is’ with literals?_ by Adam Johnson](https://adamj.eu/tech/2020/01/21/why-does-python-3-8-syntaxwarning-for-is-literal/)
#[derive(ViolationMetadata)]
pub struct IsLiteral {
    cmp_op: IsCmpOp,
}

impl AlwaysFixableViolation for IsLiteral {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.cmp_op {
            IsCmpOp::Is => "Use `==` to compare constant literals".to_string(),
            IsCmpOp::IsNot => "Use `!=` to compare constant literals".to_string(),
        }
    }

    fn fix_title(&self) -> String {
        let title = match self.cmp_op {
            IsCmpOp::Is => "Replace `is` with `==`",
            IsCmpOp::IsNot => "Replace `is not` with `!=`",
        };
        title.to_string()
    }
}

// FIX: duplicated with ruff_rule_pyflakes::invalid_literal_comparisons
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum IsCmpOp {
    Is,
    IsNot,
}