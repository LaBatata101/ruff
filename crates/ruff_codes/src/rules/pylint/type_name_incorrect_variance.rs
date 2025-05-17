use std::fmt;

use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::VarKind;

/// ## What it does
/// Checks for type names that do not match the variance of their associated
/// type parameter.
///
/// ## Why is this bad?
/// [PEP 484] recommends the use of the `_co` and `_contra` suffixes for
/// covariant and contravariant type parameters, respectively (while invariant
/// type parameters should not have any such suffix).
///
/// ## Example
/// ```python
/// from typing import TypeVar
///
/// T = TypeVar("T", covariant=True)
/// U = TypeVar("U", contravariant=True)
/// V_co = TypeVar("V_co")
/// ```
///
/// Use instead:
/// ```python
/// from typing import TypeVar
///
/// T_co = TypeVar("T_co", covariant=True)
/// U_contra = TypeVar("U_contra", contravariant=True)
/// V = TypeVar("V")
/// ```
///
/// ## References
/// - [Python documentation: `typing` — Support for type hints](https://docs.python.org/3/library/typing.html)
/// - [PEP 483 – The Theory of Type Hints: Covariance and Contravariance](https://peps.python.org/pep-0483/#covariance-and-contravariance)
/// - [PEP 484 – Type Hints: Covariance and contravariance](https://peps.python.org/pep-0484/#covariance-and-contravariance)
///
/// [PEP 484]: https://peps.python.org/pep-0484/
#[derive(ViolationMetadata)]
pub struct TypeNameIncorrectVariance {
    kind: VarKind,
    param_name: String,
    variance: VarVariance,
    replacement_name: String,
}

impl Violation for TypeNameIncorrectVariance {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TypeNameIncorrectVariance {
            kind,
            param_name,
            variance,
            replacement_name,
        } = self;
        format!("`{kind}` name \"{param_name}\" does not reflect its {variance}; consider renaming it to \"{replacement_name}\"")
    }
}

// FIX: duplicated with rufe_rule_pylint::type_name_incorrect_variance
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum VarVariance {
    Bivariance,
    Covariance,
    Contravariance,
    Invariance,
}

impl fmt::Display for VarVariance {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarVariance::Bivariance => fmt.write_str("bivariance"),
            VarVariance::Covariance => fmt.write_str("covariance"),
            VarVariance::Contravariance => fmt.write_str("contravariance"),
            VarVariance::Invariance => fmt.write_str("invariance"),
        }
    }
}