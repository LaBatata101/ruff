use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for function arguments that use the same names as builtins.
///
/// ## Why is this bad?
/// Reusing a builtin name for the name of an argument increases the
/// difficulty of reading and maintaining the code, and can cause
/// non-obvious errors, as readers may mistake the argument for the
/// builtin and vice versa.
///
/// Builtins can be marked as exceptions to this rule via the
/// [`lint.flake8-builtins.ignorelist`] configuration option.
///
/// ## Example
/// ```python
/// def remove_duplicates(list, list2):
///     result = set()
///     for value in list:
///         result.add(value)
///     for value in list2:
///         result.add(value)
///     return list(result)  # TypeError: 'list' object is not callable
/// ```
///
/// Use instead:
/// ```python
/// def remove_duplicates(list1, list2):
///     result = set()
///     for value in list1:
///         result.add(value)
///     for value in list2:
///         result.add(value)
///     return list(result)
/// ```
///
/// ## Options
/// - `lint.flake8-builtins.ignorelist`
///
/// ## References
/// - [_Is it bad practice to use a built-in function name as an attribute or method identifier?_](https://stackoverflow.com/questions/9109333/is-it-bad-practice-to-use-a-built-in-function-name-as-an-attribute-or-method-ide)
/// - [_Why is it a bad idea to name a variable `id` in Python?_](https://stackoverflow.com/questions/77552/id-is-a-bad-variable-name-in-python)
#[derive(ViolationMetadata)]
pub struct BuiltinArgumentShadowing {
    name: String,
}

impl Violation for BuiltinArgumentShadowing {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BuiltinArgumentShadowing { name } = self;
        format!("Function argument `{name}` is shadowing a Python builtin")
    }
}