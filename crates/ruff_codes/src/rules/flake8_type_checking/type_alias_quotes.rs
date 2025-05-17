use ruff_diagnostics::{AlwaysFixableViolation, FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks if [PEP 613] explicit type aliases contain references to
/// symbols that are not available at runtime.
///
/// ## Why is this bad?
/// Referencing type-checking only symbols results in a `NameError` at runtime.
///
/// ## Example
/// ```python
/// from typing import TYPE_CHECKING, TypeAlias
///
/// if TYPE_CHECKING:
///     from foo import Foo
/// OptFoo: TypeAlias = Foo | None
/// ```
///
/// Use instead:
/// ```python
/// from typing import TYPE_CHECKING, TypeAlias
///
/// if TYPE_CHECKING:
///     from foo import Foo
/// OptFoo: TypeAlias = "Foo | None"
/// ```
///
/// ## Fix safety
/// This rule's fix is currently always marked as unsafe, since runtime
/// typing libraries may try to access/resolve the type alias in a way
/// that we can't statically determine during analysis and relies on the
/// type alias not containing any forward references.
///
/// ## References
/// - [PEP 613 – Explicit Type Aliases](https://peps.python.org/pep-0613/)
///
/// [PEP 613]: https://peps.python.org/pep-0613/
#[derive(ViolationMetadata)]
pub struct UnquotedTypeAlias;

impl Violation for UnquotedTypeAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Add quotes to type alias".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Add quotes".to_string())
    }
}

/// ## What it does
/// Checks for unnecessary quotes in [PEP 613] explicit type aliases
/// and [PEP 695] type statements.
///
/// ## Why is this bad?
/// Unnecessary string forward references can lead to additional overhead
/// in runtime libraries making use of type hints. They can also have bad
/// interactions with other runtime uses like [PEP 604] type unions.
///
/// PEP-613 type aliases are only flagged by the rule if Ruff can have high
/// confidence that the quotes are unnecessary. Specifically, any PEP-613
/// type alias where the type expression on the right-hand side contains
/// subscripts or attribute accesses will not be flagged. This is because
/// type aliases can reference types that are, for example, generic in stub
/// files but not at runtime. That can mean that a type checker expects the
/// referenced type to be subscripted with type arguments despite the fact
/// that doing so would fail at runtime if the type alias value was not
/// quoted. Similarly, a type alias might need to reference a module-level
/// attribute that exists in a stub file but not at runtime, meaning that
/// the type alias value would need to be quoted to avoid a runtime error.
///
/// ## Example
/// Given:
/// ```python
/// OptInt: TypeAlias = "int | None"
/// ```
///
/// Use instead:
/// ```python
/// OptInt: TypeAlias = int | None
/// ```
///
/// Given:
/// ```python
/// type OptInt = "int | None"
/// ```
///
/// Use instead:
/// ```python
/// type OptInt = int | None
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe, unless the type annotation contains comments.
///
/// ## See also
/// This rule only applies to type aliases in non-stub files. For removing quotes in other
/// contexts or in stub files, see:
///
/// - [`quoted-annotation-in-stub`][PYI020]: A rule that
///   removes all quoted annotations from stub files
/// - [`quoted-annotation`][UP037]: A rule that removes unnecessary quotes
///   from *annotations* in runtime files.
///
/// ## References
/// - [PEP 613 – Explicit Type Aliases](https://peps.python.org/pep-0613/)
/// - [PEP 695: Generic Type Alias](https://peps.python.org/pep-0695/#generic-type-alias)
/// - [PEP 604 – Allow writing union types as `X | Y`](https://peps.python.org/pep-0604/)
///
/// [PEP 604]: https://peps.python.org/pep-0604/
/// [PEP 613]: https://peps.python.org/pep-0613/
/// [PEP 695]: https://peps.python.org/pep-0695/#generic-type-alias
/// [PYI020]: https://docs.astral.sh/ruff/rules/quoted-annotation-in-stub/
/// [UP037]: https://docs.astral.sh/ruff/rules/quoted-annotation/
#[derive(ViolationMetadata)]
pub struct QuotedTypeAlias;

impl AlwaysFixableViolation for QuotedTypeAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Remove quotes from type alias".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove quotes".to_string()
    }
}
