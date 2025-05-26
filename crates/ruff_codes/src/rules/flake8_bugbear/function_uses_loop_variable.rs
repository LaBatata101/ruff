use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for function definitions that use a loop variable.
///
/// ## Why is this bad?
/// The loop variable is not bound in the function definition, so it will always
/// have the value it had in the last iteration when the function is called.
///
/// Instead, consider using a default argument to bind the loop variable at
/// function definition time. Or, use `functools.partial`.
///
/// ## Example
/// ```python
/// adders = [lambda x: x + i for i in range(3)]
/// values = [adder(1) for adder in adders]  # [3, 3, 3]
/// ```
///
/// Use instead:
/// ```python
/// adders = [lambda x, i=i: x + i for i in range(3)]
/// values = [adder(1) for adder in adders]  # [1, 2, 3]
/// ```
///
/// Or:
/// ```python
/// from functools import partial
///
/// adders = [partial(lambda x, i: x + i, i=i) for i in range(3)]
/// values = [adder(1) for adder in adders]  # [1, 2, 3]
/// ```
///
/// ## References
/// - [The Hitchhiker's Guide to Python: Late Binding Closures](https://docs.python-guide.org/writing/gotchas/#late-binding-closures)
/// - [Python documentation: `functools.partial`](https://docs.python.org/3/library/functools.html#functools.partial)
#[derive(ViolationMetadata)]
pub struct FunctionUsesLoopVariable {
    name: String,
}

impl Violation for FunctionUsesLoopVariable {
    #[derive_message_formats]
    fn message(&self) -> String {
        let FunctionUsesLoopVariable { name } = self;
        format!("Function definition does not bind loop variable `{name}`")
    }
}