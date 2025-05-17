use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for function calls in dataclass attribute defaults.
///
/// ## Why is this bad?
/// Function calls are only performed once, at definition time. The returned
/// value is then reused by all instances of the dataclass. This can lead to
/// unexpected behavior when the function call returns a mutable object, as
/// changes to the object will be shared across all instances.
///
/// If a field needs to be initialized with a mutable object, use the
/// `field(default_factory=...)` pattern.
///
/// Attributes whose default arguments are `NewType` calls
/// where the original type is immutable are ignored.
///
/// ## Example
/// ```python
/// from dataclasses import dataclass
///
///
/// def simple_list() -> list[int]:
///     return [1, 2, 3, 4]
///
///
/// @dataclass
/// class A:
///     mutable_default: list[int] = simple_list()
/// ```
///
/// Use instead:
/// ```python
/// from dataclasses import dataclass, field
///
///
/// def creating_list() -> list[int]:
///     return [1, 2, 3, 4]
///
///
/// @dataclass
/// class A:
///     mutable_default: list[int] = field(default_factory=creating_list)
/// ```
///
/// ## Options
/// - `lint.flake8-bugbear.extend-immutable-calls`
#[derive(ViolationMetadata)]
pub struct FunctionCallInDataclassDefaultArgument {
    name: Option<String>,
}

impl Violation for FunctionCallInDataclassDefaultArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(name) = &self.name {
            format!("Do not perform function call `{name}` in dataclass defaults")
        } else {
            "Do not perform function call in dataclass defaults".to_string()
        }
    }
}