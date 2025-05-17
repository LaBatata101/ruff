use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for function calls in default function arguments.
///
/// ## Why is this bad?
/// Any function call that's used in a default argument will only be performed
/// once, at definition time. The returned value will then be reused by all
/// calls to the function, which can lead to unexpected behaviour.
///
/// Parameters with immutable type annotations will be ignored by this rule.
/// Those whose default arguments are `NewType` calls where the original type
/// is immutable are also ignored.
///
/// Calls and types outside of the standard library can be marked as an exception
/// to this rule with the [`lint.flake8-bugbear.extend-immutable-calls`] configuration option.
///
/// ## Example
///
/// ```python
/// def create_list() -> list[int]:
///     return [1, 2, 3]
///
///
/// def mutable_default(arg: list[int] = create_list()) -> list[int]:
///     arg.append(4)
///     return arg
/// ```
///
/// Use instead:
///
/// ```python
/// def better(arg: list[int] | None = None) -> list[int]:
///     if arg is None:
///         arg = create_list()
///
///     arg.append(4)
///     return arg
/// ```
///
/// If the use of a singleton is intentional, assign the result call to a
/// module-level variable, and use that variable in the default argument:
///
/// ```python
/// ERROR = ValueError("Hosts weren't successfully added")
///
///
/// def add_host(error: Exception = ERROR) -> None: ...
/// ```
///
/// ## Options
/// - `lint.flake8-bugbear.extend-immutable-calls`
#[derive(ViolationMetadata)]
pub struct FunctionCallInDefaultArgument {
    name: Option<String>,
}

impl Violation for FunctionCallInDefaultArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(name) = &self.name {
            format!("Do not perform function call `{name}` in argument defaults; instead, perform the call within the function, or read the default from a module-level singleton variable")
        } else {
            "Do not perform function call in argument defaults; instead, perform the call within the function, or read the default from a module-level singleton variable".to_string()
        }
    }
}