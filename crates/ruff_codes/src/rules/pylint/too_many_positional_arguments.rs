use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for function definitions that include too many positional arguments.
///
/// By default, this rule allows up to five arguments, as configured by the
/// [`lint.pylint.max-positional-args`] option.
///
/// ## Why is this bad?
/// Functions with many arguments are harder to understand, maintain, and call.
/// This is especially true for functions with many positional arguments, as
/// providing arguments positionally is more error-prone and less clear to
/// readers than providing arguments by name.
///
/// Consider refactoring functions with many arguments into smaller functions
/// with fewer arguments, using objects to group related arguments, or migrating to
/// [keyword-only arguments](https://docs.python.org/3/tutorial/controlflow.html#special-parameters).
///
/// ## Example
///
/// ```python
/// def plot(x, y, z, color, mark, add_trendline): ...
///
///
/// plot(1, 2, 3, "r", "*", True)
/// ```
///
/// Use instead:
///
/// ```python
/// def plot(x, y, z, *, color, mark, add_trendline): ...
///
///
/// plot(1, 2, 3, color="r", mark="*", add_trendline=True)
/// ```
///
/// ## Options
/// - `lint.pylint.max-positional-args`
#[derive(ViolationMetadata)]
pub struct TooManyPositionalArguments {
    c_pos: usize,
    max_pos: usize,
}

impl Violation for TooManyPositionalArguments {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyPositionalArguments { c_pos, max_pos } = self;
        format!("Too many positional arguments ({c_pos}/{max_pos})")
    }
}