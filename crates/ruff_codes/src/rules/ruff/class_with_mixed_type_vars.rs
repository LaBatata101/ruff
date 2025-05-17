use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for classes that have [PEP 695] [type parameter lists]
/// while also inheriting from `typing.Generic` or `typing_extensions.Generic`.
///
/// ## Why is this bad?
/// Such classes cause errors at runtime:
///
/// ```python
/// from typing import Generic, TypeVar
///
/// U = TypeVar("U")
///
/// # TypeError: Cannot inherit from Generic[...] multiple times.
/// class C[T](Generic[U]): ...
/// ```
///
/// ## Example
///
/// ```python
/// from typing import Generic, ParamSpec, TypeVar, TypeVarTuple
///
/// U = TypeVar("U")
/// P = ParamSpec("P")
/// Ts = TypeVarTuple("Ts")
///
///
/// class C[T](Generic[U, P, *Ts]): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class C[T, U, **P, *Ts]: ...
/// ```
///
/// ## Fix safety
/// As the fix changes runtime behaviour, it is always marked as unsafe.
/// Additionally, comments within the fix range will not be preserved.
///
/// ## References
/// - [Python documentation: User-defined generic types](https://docs.python.org/3/library/typing.html#user-defined-generic-types)
/// - [Python documentation: type parameter lists](https://docs.python.org/3/reference/compound_stmts.html#type-params)
/// - [PEP 695 - Type Parameter Syntax](https://peps.python.org/pep-0695/)
///
/// [PEP 695]: https://peps.python.org/pep-0695/
/// [type parameter lists]: https://docs.python.org/3/reference/compound_stmts.html#type-params
#[derive(ViolationMetadata)]
pub struct ClassWithMixedTypeVars;

impl Violation for ClassWithMixedTypeVars {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Class with type parameter list inherits from `Generic`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove `Generic` base class".to_string())
    }
}