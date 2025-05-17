use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for classes that only have a public `__init__` method,
/// without base classes and decorators.
///
/// ## Why is this bad?
/// Classes with just an `__init__` are possibly better off
/// being a dataclass or a namedtuple, which have less boilerplate.
///
/// ## Example
/// ```python
/// class Point:
///     def __init__(self, x: float, y: float):
///         self.x = x
///         self.y = y
/// ```
///
/// Use instead:
/// ```python
/// from dataclasses import dataclass
///
///
/// @dataclass
/// class Point:
///     x: float
///     y: float
/// ```
#[derive(ViolationMetadata)]
pub struct ClassAsDataStructure;

impl Violation for ClassAsDataStructure {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Class could be dataclass or namedtuple".to_string()
    }
}