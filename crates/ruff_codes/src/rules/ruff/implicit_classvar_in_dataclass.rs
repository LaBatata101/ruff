use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for implicit class variables in dataclasses.
///
/// Variables matching the [`lint.dummy-variable-rgx`] are excluded
/// from this rule.
///
/// ## Why is this bad?
/// Class variables are shared between all instances of that class.
/// In dataclasses, fields with no annotations at all
/// are implicitly considered class variables, and a `TypeError` is
/// raised if a user attempts to initialize an instance of the class
/// with this field.
///
///
/// ```python
/// @dataclass
/// class C:
///     a = 1
///     b: str = ""
///
/// C(a = 42)  # TypeError: C.__init__() got an unexpected keyword argument 'a'
/// ```
///
/// ## Example
///
/// ```python
/// @dataclass
/// class C:
///     a = 1
/// ```
///
/// Use instead:
///
/// ```python
/// from typing import ClassVar
///
///
/// @dataclass
/// class C:
///     a: ClassVar[int] = 1
/// ```
///
/// ## Options
/// - [`lint.dummy-variable-rgx`]
#[derive(ViolationMetadata)]
pub struct ImplicitClassVarInDataclass;

impl Violation for ImplicitClassVarInDataclass {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Assignment without annotation found in dataclass body".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use `ClassVar[...]`".to_string())
    }
}