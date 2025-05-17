use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for enum classes which are also decorated with `@dataclass`.
///
/// ## Why is this bad?
/// Decorating an enum with `@dataclass()` does not cause any errors at runtime,
/// but may cause erroneous results:
///
/// ```python
/// @dataclass
/// class E(Enum):
///     A = 1
///     B = 2
///
/// print(E.A == E.B)  # True
/// ```
///
/// ## Example
///
/// ```python
/// from dataclasses import dataclass
/// from enum import Enum
///
///
/// @dataclass
/// class E(Enum): ...
/// ```
///
/// Use instead:
///
/// ```python
/// from enum import Enum
///
///
/// class E(Enum): ...
/// ```
///
/// ## References
/// - [Python documentation: Enum HOWTO &sect; Dataclass support](https://docs.python.org/3/howto/enum.html#dataclass-support)
#[derive(ViolationMetadata)]
pub struct DataclassEnum;

impl Violation for DataclassEnum {
    #[derive_message_formats]
    fn message(&self) -> String {
        "An enum class should not be decorated with `@dataclass`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove either `@dataclass` or `Enum`".to_string())
    }
}