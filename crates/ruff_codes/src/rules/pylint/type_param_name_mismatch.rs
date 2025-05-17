use std::fmt;

use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `TypeVar`, `TypeVarTuple`, `ParamSpec`, and `NewType`
/// definitions in which the name of the type parameter does not match the name
/// of the variable to which it is assigned.
///
/// ## Why is this bad?
/// When defining a `TypeVar` or a related type parameter, Python allows you to
/// provide a name for the type parameter. According to [PEP 484], the name
/// provided to the `TypeVar` constructor must be equal to the name of the
/// variable to which it is assigned.
///
/// ## Example
/// ```python
/// from typing import TypeVar
///
/// T = TypeVar("U")
/// ```
///
/// Use instead:
/// ```python
/// from typing import TypeVar
///
/// T = TypeVar("T")
/// ```
///
/// ## References
/// - [Python documentation: `typing` — Support for type hints](https://docs.python.org/3/library/typing.html)
/// - [PEP 484 – Type Hints: Generics](https://peps.python.org/pep-0484/#generics)
///
/// [PEP 484]:https://peps.python.org/pep-0484/#generics
#[derive(ViolationMetadata)]
pub struct TypeParamNameMismatch {
    kind: VarKind,
    var_name: String,
    param_name: String,
}

impl Violation for TypeParamNameMismatch {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TypeParamNameMismatch {
            kind,
            var_name,
            param_name,
        } = self;
        format!("`{kind}` name `{param_name}` does not match assigned variable name `{var_name}`")
    }
}

// FIX: duplicated with rufe_rule_pylint::type_param_name_mismatch
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum VarKind {
    TypeVar,
    ParamSpec,
    TypeVarTuple,
    NewType,
}

impl fmt::Display for VarKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarKind::TypeVar => fmt.write_str("TypeVar"),
            VarKind::ParamSpec => fmt.write_str("ParamSpec"),
            VarKind::TypeVarTuple => fmt.write_str("TypeVarTuple"),
            VarKind::NewType => fmt.write_str("NewType"),
        }
    }
}
