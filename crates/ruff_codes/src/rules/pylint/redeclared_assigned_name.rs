use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for declared assignments to the same variable multiple times
/// in the same assignment.
///
/// ## Why is this bad?
/// Assigning a variable multiple times in the same assignment is redundant,
/// as the final assignment to the variable is what the value will be.
///
/// ## Example
/// ```python
/// a, b, a = (1, 2, 3)
/// print(a)  # 3
/// ```
///
/// Use instead:
/// ```python
/// # this is assuming you want to assign 3 to `a`
/// _, b, a = (1, 2, 3)
/// print(a)  # 3
/// ```
///
#[derive(ViolationMetadata)]
pub struct RedeclaredAssignedName {
    name: String,
}

impl Violation for RedeclaredAssignedName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RedeclaredAssignedName { name } = self;
        format!("Redeclared variable `{name}` in assignment")
    }
}