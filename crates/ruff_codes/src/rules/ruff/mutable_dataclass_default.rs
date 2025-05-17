use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for mutable default values in dataclass attributes.
///
/// ## Why is this bad?
/// Mutable default values share state across all instances of the dataclass.
/// This can lead to bugs when the attributes are changed in one instance, as
/// those changes will unexpectedly affect all other instances.
///
/// Instead of sharing mutable defaults, use the `field(default_factory=...)`
/// pattern.
///
/// If the default value is intended to be mutable, it must be annotated with
/// `typing.ClassVar`; otherwise, a `ValueError` will be raised.
///
/// ## Example
/// ```python
/// from dataclasses import dataclass
///
///
/// @dataclass
/// class A:
///     # A list without a `default_factory` or `ClassVar` annotation
///     # will raise a `ValueError`.
///     mutable_default: list[int] = []
/// ```
///
/// Use instead:
/// ```python
/// from dataclasses import dataclass, field
///
///
/// @dataclass
/// class A:
///     mutable_default: list[int] = field(default_factory=list)
/// ```
///
/// Or:
/// ```python
/// from dataclasses import dataclass
/// from typing import ClassVar
///
///
/// @dataclass
/// class A:
///     mutable_default: ClassVar[list[int]] = []
/// ```
#[derive(ViolationMetadata)]
pub struct MutableDataclassDefault;

impl Violation for MutableDataclassDefault {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not use mutable default values for dataclass attributes".to_string()
    }
}