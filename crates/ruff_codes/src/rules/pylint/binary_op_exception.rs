use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::BoolOp;

/// ## What it does
/// Checks for `except` clauses that attempt to catch multiple
/// exceptions with a binary operation (`and` or `or`).
///
/// ## Why is this bad?
/// A binary operation will not catch multiple exceptions. Instead, the binary
/// operation will be evaluated first, and the result of _that_ operation will
/// be caught (for an `or` operation, this is typically the first exception in
/// the list). This is almost never the desired behavior.
///
/// ## Example
/// ```python
/// try:
///     pass
/// except A or B:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// try:
///     pass
/// except (A, B):
///     pass
/// ```
#[derive(ViolationMetadata)]
pub struct BinaryOpException {
    op: BoolOp,
}

impl Violation for BinaryOpException {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.op {
            BoolOp::And => {
                "Exception to catch is the result of a binary `and` operation".to_string()
            }
            BoolOp::Or => "Exception to catch is the result of a binary `or` operation".to_string(),
        }
    }
}