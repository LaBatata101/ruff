use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for single-element tuples in exception handlers (e.g.,
/// `except (ValueError,):`).
///
/// Note: Single-element tuples consisting of a starred expression
/// are allowed.
///
/// ## Why is this bad?
/// A tuple with a single element can be more concisely and idiomatically
/// expressed as a single value.
///
/// ## Example
/// ```python
/// try:
///     ...
/// except (ValueError,):
///     ...
/// ```
///
/// Use instead:
/// ```python
/// try:
///     ...
/// except ValueError:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `except` clause](https://docs.python.org/3/reference/compound_stmts.html#except-clause)
#[derive(ViolationMetadata)]
pub struct RedundantTupleInExceptionHandler {
    name: String,
}

impl AlwaysFixableViolation for RedundantTupleInExceptionHandler {
    #[derive_message_formats]
    fn message(&self) -> String {
        "A length-one tuple literal is redundant in exception handlers".to_string()
    }

    fn fix_title(&self) -> String {
        let RedundantTupleInExceptionHandler { name } = self;
        format!("Replace with `except {name}`")
    }
}