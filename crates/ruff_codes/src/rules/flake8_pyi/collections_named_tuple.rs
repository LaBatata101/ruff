use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `collections.namedtuple` in stub files.
///
/// ## Why is this bad?
/// `typing.NamedTuple` is the "typed version" of `collections.namedtuple`.
///
/// Inheriting from `typing.NamedTuple` creates a custom `tuple` subclass in
/// the same way as using the `collections.namedtuple` factory function.
/// However, using `typing.NamedTuple` allows you to provide a type annotation
/// for each field in the class. This means that type checkers will have more
/// information to work with, and will be able to analyze your code more
/// precisely.
///
/// ## Example
/// ```pyi
/// from collections import namedtuple
///
/// person = namedtuple("Person", ["name", "age"])
/// ```
///
/// Use instead:
/// ```pyi
/// from typing import NamedTuple
///
/// class Person(NamedTuple):
///     name: str
///     age: int
/// ```
#[derive(ViolationMetadata)]
pub struct CollectionsNamedTuple;

impl Violation for CollectionsNamedTuple {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `typing.NamedTuple` instead of `collections.namedtuple`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `typing.NamedTuple`".to_string())
    }
}