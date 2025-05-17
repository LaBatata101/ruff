use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for local variables that are annotated but never used.
///
/// ## Why is this bad?
/// Annotations are used to provide type hints to static type checkers. If a
/// variable is annotated but never used, the annotation is unnecessary.
///
/// ## Example
/// ```python
/// def foo():
///     bar: int
/// ```
///
/// ## References
/// - [PEP 484 – Type Hints](https://peps.python.org/pep-0484/)
#[derive(ViolationMetadata)]
pub struct UnusedAnnotation {
    name: String,
}

impl Violation for UnusedAnnotation {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedAnnotation { name } = self;
        format!("Local variable `{name}` is annotated but never used")
    }
}