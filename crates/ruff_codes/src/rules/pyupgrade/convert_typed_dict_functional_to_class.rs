use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `TypedDict` declarations that use functional syntax.
///
/// ## Why is this bad?
/// `TypedDict` types can be defined either through a functional syntax
/// (`Foo = TypedDict(...)`) or a class syntax (`class Foo(TypedDict): ...`).
///
/// The class syntax is more readable and generally preferred over the
/// functional syntax.
///
/// Nonetheless, there are some situations in which it is impossible to use
/// the class-based syntax. This rule will not apply to those cases. Namely,
/// it is impossible to use the class-based syntax if any `TypedDict` fields are:
/// - Not valid [python identifiers] (for example, `@x`)
/// - [Python keywords] such as `in`
/// - [Private names] such as `__id` that would undergo [name mangling] at runtime
///   if the class-based syntax was used
/// - [Dunder names] such as `__int__` that can confuse type checkers if they're used
///   with the class-based syntax.
///
/// ## Example
/// ```python
/// from typing import TypedDict
///
/// Foo = TypedDict("Foo", {"a": int, "b": str})
/// ```
///
/// Use instead:
/// ```python
/// from typing import TypedDict
///
///
/// class Foo(TypedDict):
///     a: int
///     b: str
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe if there are any comments within the
/// range of the `TypedDict` definition, as these will be dropped by the
/// autofix.
///
/// ## References
/// - [Python documentation: `typing.TypedDict`](https://docs.python.org/3/library/typing.html#typing.TypedDict)
///
/// [Private names]: https://docs.python.org/3/tutorial/classes.html#private-variables
/// [name mangling]: https://docs.python.org/3/reference/expressions.html#private-name-mangling
/// [python identifiers]: https://docs.python.org/3/reference/lexical_analysis.html#identifiers
/// [Python keywords]: https://docs.python.org/3/reference/lexical_analysis.html#keywords
/// [Dunder names]: https://docs.python.org/3/reference/lexical_analysis.html#reserved-classes-of-identifiers
#[derive(ViolationMetadata)]
pub struct ConvertTypedDictFunctionalToClass {
    name: String,
}

impl Violation for ConvertTypedDictFunctionalToClass {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let ConvertTypedDictFunctionalToClass { name } = self;
        format!("Convert `{name}` from `TypedDict` functional to class syntax")
    }

    fn fix_title(&self) -> Option<String> {
        let ConvertTypedDictFunctionalToClass { name } = self;
        Some(format!("Convert `{name}` to class syntax"))
    }
}