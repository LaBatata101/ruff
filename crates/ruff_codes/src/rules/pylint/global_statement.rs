use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of `global` statements to update identifiers.
///
/// ## Why is this bad?
/// Pylint discourages the use of `global` variables as global mutable
/// state is a common source of bugs and confusing behavior.
///
/// ## Example
/// ```python
/// var = 1
///
///
/// def foo():
///     global var  # [global-statement]
///     var = 10
///     print(var)
///
///
/// foo()
/// print(var)
/// ```
///
/// Use instead:
/// ```python
/// var = 1
///
///
/// def foo():
///     print(var)
///     return 10
///
///
/// var = foo()
/// print(var)
/// ```
#[derive(ViolationMetadata)]
pub struct GlobalStatement {
    name: String,
}

impl Violation for GlobalStatement {
    #[derive_message_formats]
    fn message(&self) -> String {
        let GlobalStatement { name } = self;
        format!("Using the global statement to update `{name}` is discouraged")
    }
}