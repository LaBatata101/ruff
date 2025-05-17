use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::name::Name;

/// ## What it does
/// Checks for loops in which a `set` is modified during iteration.
///
/// ## Why is this bad?
/// If a `set` is modified during iteration, it will cause a `RuntimeError`.
///
/// If you need to modify a `set` within a loop, consider iterating over a copy
/// of the `set` instead.
///
/// ## Known problems
/// This rule favors false negatives over false positives. Specifically, it
/// will only detect variables that can be inferred to be a `set` type based on
/// local type inference, and will only detect modifications that are made
/// directly on the variable itself (e.g., `set.add()`), as opposed to
/// modifications within other function calls (e.g., `some_function(set)`).
///
/// ## Example
/// ```python
/// nums = {1, 2, 3}
/// for num in nums:
///     nums.add(num + 5)
/// ```
///
/// Use instead:
/// ```python
/// nums = {1, 2, 3}
/// for num in nums.copy():
///     nums.add(num + 5)
/// ```
///
/// ## Fix safety
/// This fix is always unsafe because it changes the program’s behavior. Replacing the
/// original set with a copy during iteration allows code that would previously raise a
/// `RuntimeError` to run without error.
///
/// ## References
/// - [Python documentation: `set`](https://docs.python.org/3/library/stdtypes.html#set)
#[derive(ViolationMetadata)]
pub struct ModifiedIteratingSet {
    name: Name,
}

impl AlwaysFixableViolation for ModifiedIteratingSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ModifiedIteratingSet { name } = self;
        format!("Iterated set `{name}` is modified within the `for` loop",)
    }

    fn fix_title(&self) -> String {
        let ModifiedIteratingSet { name } = self;
        format!("Iterate over a copy of `{name}`")
    }
}