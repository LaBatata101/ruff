use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for negated `==` operators.
///
/// ## Why is this bad?
/// Negated `==` operators are less readable than `!=` operators. When testing
/// for non-equality, it is more common to use `!=` than `==`.
///
/// ## Example
/// ```python
/// not a == b
/// ```
///
/// Use instead:
/// ```python
/// a != b
/// ```
///
/// ## Fix safety
/// The fix is marked as unsafe, as it might change the behaviour
/// if `a` and/or `b` overrides `__eq__`/`__ne__`
/// in such a manner that they don't return booleans.
///
/// ## References
/// - [Python documentation: Comparisons](https://docs.python.org/3/reference/expressions.html#comparisons)
#[derive(ViolationMetadata)]
pub struct NegateEqualOp {
    left: String,
    right: String,
}

impl AlwaysFixableViolation for NegateEqualOp {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NegateEqualOp { left, right } = self;
        format!("Use `{left} != {right}` instead of `not {left} == {right}`")
    }

    fn fix_title(&self) -> String {
        "Replace with `!=` operator".to_string()
    }
}

/// ## What it does
/// Checks for negated `!=` operators.
///
/// ## Why is this bad?
/// Negated `!=` operators are less readable than `==` operators, as they avoid a
/// double negation.
///
/// ## Example
/// ```python
/// not a != b
/// ```
///
/// Use instead:
/// ```python
/// a == b
/// ```
///
/// ## Fix safety
/// The fix is marked as unsafe, as it might change the behaviour
/// if `a` and/or `b` overrides `__ne__`/`__eq__`
/// in such a manner that they don't return booleans.
///
/// ## References
/// - [Python documentation: Comparisons](https://docs.python.org/3/reference/expressions.html#comparisons)
#[derive(ViolationMetadata)]
pub struct NegateNotEqualOp {
    left: String,
    right: String,
}

impl AlwaysFixableViolation for NegateNotEqualOp {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NegateNotEqualOp { left, right } = self;
        format!("Use `{left} == {right}` instead of `not {left} != {right}`")
    }

    fn fix_title(&self) -> String {
        "Replace with `==` operator".to_string()
    }
}

/// ## What it does
/// Checks for double negations (i.e., multiple `not` operators).
///
/// ## Why is this bad?
/// A double negation is redundant and less readable than omitting the `not`
/// operators entirely.
///
/// ## Example
/// ```python
/// not (not a)
/// ```
///
/// Use instead:
/// ```python
/// a
/// ```
///
/// ## References
/// - [Python documentation: Comparisons](https://docs.python.org/3/reference/expressions.html#comparisons)
#[derive(ViolationMetadata)]
pub struct DoubleNegation {
    expr: String,
}

impl AlwaysFixableViolation for DoubleNegation {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DoubleNegation { expr } = self;
        format!("Use `{expr}` instead of `not (not {expr})`")
    }

    fn fix_title(&self) -> String {
        let DoubleNegation { expr } = self;
        format!("Replace with `{expr}`")
    }
}
