use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for type aliases that do not use the CamelCase naming convention.
///
/// ## Why is this bad?
/// It's conventional to use the CamelCase naming convention for type aliases,
/// to distinguish them from other variables.
///
/// ## Example
/// ```pyi
/// type_alias_name: TypeAlias = int
/// ```
///
/// Use instead:
/// ```pyi
/// TypeAliasName: TypeAlias = int
/// ```
#[derive(ViolationMetadata)]
pub struct SnakeCaseTypeAlias {
    name: String,
}

impl Violation for SnakeCaseTypeAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Type alias `{name}` should be CamelCase")
    }
}

/// ## What it does
/// Checks for private type alias definitions suffixed with 'T'.
///
/// ## Why is this bad?
/// It's conventional to use the 'T' suffix for type variables; the use of
/// such a suffix implies that the object is a `TypeVar`.
///
/// Adding the 'T' suffix to a non-`TypeVar`, it can be misleading and should
/// be avoided.
///
/// ## Example
/// ```pyi
/// from typing import TypeAlias
///
/// _MyTypeT: TypeAlias = int
/// ```
///
/// Use instead:
/// ```pyi
/// from typing import TypeAlias
///
/// _MyType: TypeAlias = int
/// ```
///
/// ## References
/// - [PEP 484: Type Aliases](https://peps.python.org/pep-0484/#type-aliases)
#[derive(ViolationMetadata)]
pub struct TSuffixedTypeAlias {
    name: String,
}

impl Violation for TSuffixedTypeAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Private type alias `{name}` should not be suffixed with `T` (the `T` suffix implies that an object is a `TypeVar`)")
    }
}
