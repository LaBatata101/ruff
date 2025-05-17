use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for use of `TypeAlias` annotations and `TypeAliasType` assignments
/// for declaring type aliases.
///
/// ## Why is this bad?
/// The `type` keyword was introduced in Python 3.12 by [PEP 695] for defining
/// type aliases. The `type` keyword is easier to read and provides cleaner
/// support for generics.
///
/// ## Known problems
/// [PEP 695] uses inferred variance for type parameters, instead of the
/// `covariant` and `contravariant` keywords used by `TypeVar` variables. As
/// such, rewriting a type alias using a PEP-695 `type` statement may change
/// the variance of the alias's type parameters.
///
/// Unlike type aliases that use simple assignments, definitions created using
/// [PEP 695] `type` statements cannot be used as drop-in replacements at
/// runtime for the value on the right-hand side of the statement. This means
/// that while for some simple old-style type aliases you can use them as the
/// second argument to an `isinstance()` call (for example), doing the same
/// with a [PEP 695] `type` statement will always raise `TypeError` at
/// runtime.
///
/// ## Example
/// ```python
/// ListOfInt: TypeAlias = list[int]
/// PositiveInt = TypeAliasType("PositiveInt", Annotated[int, Gt(0)])
/// ```
///
/// Use instead:
/// ```python
/// type ListOfInt = list[int]
/// type PositiveInt = Annotated[int, Gt(0)]
/// ```
///
/// ## Fix safety
///
/// This fix is marked unsafe for `TypeAlias` assignments outside of stub files because of the
/// runtime behavior around `isinstance()` calls noted above. The fix is also unsafe for
/// `TypeAliasType` assignments if there are any comments in the replacement range that would be
/// deleted.
///
/// ## See also
///
/// This rule only applies to `TypeAlias`es and `TypeAliasType`s. See
/// [`non-pep695-generic-class`][UP046] and [`non-pep695-generic-function`][UP047] for similar
/// transformations for generic classes and functions.
///
/// This rule replaces standalone type variables in aliases but doesn't remove the corresponding
/// type variables even if they are unused after the fix. See [`unused-private-type-var`][PYI018]
/// for a rule to clean up unused private type variables.
///
/// This rule will not rename private type variables to remove leading underscores, even though the
/// new type parameters are restricted in scope to their associated aliases. See
/// [`private-type-parameter`][UP049] for a rule to update these names.
///
/// [PEP 695]: https://peps.python.org/pep-0695/
/// [PYI018]: https://docs.astral.sh/ruff/rules/unused-private-type-var/
/// [UP046]: https://docs.astral.sh/ruff/rules/non-pep695-generic-class/
/// [UP047]: https://docs.astral.sh/ruff/rules/non-pep695-generic-function/
/// [UP049]: https://docs.astral.sh/ruff/rules/private-type-parameter/
#[derive(ViolationMetadata)]
pub struct NonPEP695TypeAlias {
    name: String,
    type_alias_kind: TypeAliasKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TypeAliasKind {
    TypeAlias,
    TypeAliasType,
}

impl Violation for NonPEP695TypeAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Always;

    #[derive_message_formats]
    fn message(&self) -> String {
        let NonPEP695TypeAlias {
            name,
            type_alias_kind,
        } = self;
        let type_alias_method = match type_alias_kind {
            TypeAliasKind::TypeAlias => "`TypeAlias` annotation",
            TypeAliasKind::TypeAliasType => "`TypeAliasType` assignment",
        };
        format!("Type alias `{name}` uses {type_alias_method} instead of the `type` keyword")
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use the `type` keyword".to_string())
    }
}