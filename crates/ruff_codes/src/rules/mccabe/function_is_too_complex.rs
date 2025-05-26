use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions with a high `McCabe` complexity.
///
/// ## Why is this bad?
/// The `McCabe` complexity of a function is a measure of the complexity of
/// the control flow graph of the function. It is calculated by adding
/// one to the number of decision points in the function. A decision
/// point is a place in the code where the program has a choice of two
/// or more paths to follow.
///
/// Functions with a high complexity are hard to understand and maintain.
///
/// ## Example
/// ```python
/// def foo(a, b, c):
///     if a:
///         if b:
///             if c:
///                 return 1
///             else:
///                 return 2
///         else:
///             return 3
///     else:
///         return 4
/// ```
///
/// Use instead:
/// ```python
/// def foo(a, b, c):
///     if not a:
///         return 4
///     if not b:
///         return 3
///     if not c:
///         return 2
///     return 1
/// ```
///
/// ## Options
/// - `lint.mccabe.max-complexity`
#[derive(ViolationMetadata)]
pub struct ComplexStructure {
    name: String,
    complexity: usize,
    max_complexity: usize,
}

impl Violation for ComplexStructure {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ComplexStructure {
            name,
            complexity,
            max_complexity,
        } = self;
        format!("`{name}` is too complex ({complexity} > {max_complexity})")
    }
}