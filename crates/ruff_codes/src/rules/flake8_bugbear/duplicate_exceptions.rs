use itertools::Itertools;
use ruff_diagnostics::{AlwaysFixableViolation, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `try-except` blocks with duplicate exception handlers.
///
/// ## Why is this bad?
/// Duplicate exception handlers are redundant, as the first handler will catch
/// the exception, making the second handler unreachable.
///
/// ## Example
/// ```python
/// try:
///     ...
/// except ValueError:
///     ...
/// except ValueError:
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
pub struct DuplicateTryBlockException {
    name: String,
    is_star: bool,
}

impl Violation for DuplicateTryBlockException {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DuplicateTryBlockException { name, is_star } = self;
        if *is_star {
            format!("try-except* block with duplicate exception `{name}`")
        } else {
            format!("try-except block with duplicate exception `{name}`")
        }
    }
}

/// ## What it does
/// Checks for exception handlers that catch duplicate exceptions.
///
/// ## Why is this bad?
/// Including the same exception multiple times in the same handler is redundant,
/// as the first exception will catch the exception, making the second exception
/// unreachable. The same applies to exception hierarchies, as a handler for a
/// parent exception (like `Exception`) will also catch child exceptions (like
/// `ValueError`).
///
/// ## Example
/// ```python
/// try:
///     ...
/// except (Exception, ValueError):  # `Exception` includes `ValueError`.
///     ...
/// ```
///
/// Use instead:
/// ```python
/// try:
///     ...
/// except Exception:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `except` clause](https://docs.python.org/3/reference/compound_stmts.html#except-clause)
/// - [Python documentation: Exception hierarchy](https://docs.python.org/3/library/exceptions.html#exception-hierarchy)
#[derive(ViolationMetadata)]
pub struct DuplicateHandlerException {
    pub names: Vec<String>,
}

impl AlwaysFixableViolation for DuplicateHandlerException {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DuplicateHandlerException { names } = self;
        if let [name] = names.as_slice() {
            format!("Exception handler with duplicate exception: `{name}`")
        } else {
            let names = names.iter().map(|name| format!("`{name}`")).join(", ");
            format!("Exception handler with duplicate exceptions: {names}")
        }
    }

    fn fix_title(&self) -> String {
        "De-duplicate exceptions".to_string()
    }
}
