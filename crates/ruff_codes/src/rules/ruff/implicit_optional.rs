use std::fmt;

use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::PythonVersion;

/// ## What it does
/// Checks for the use of implicit `Optional` in type annotations when the
/// default parameter value is `None`.
///
/// ## Why is this bad?
/// Implicit `Optional` is prohibited by [PEP 484]. It is confusing and
/// inconsistent with the rest of the type system.
///
/// It's recommended to use `Optional[T]` instead. For Python 3.10 and later,
/// you can also use `T | None`.
///
/// ## Example
/// ```python
/// def foo(arg: int = None):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// from typing import Optional
///
///
/// def foo(arg: Optional[int] = None):
///     pass
/// ```
///
/// Or, for Python 3.10 and later:
/// ```python
/// def foo(arg: int | None = None):
///     pass
/// ```
///
/// If you want to use the `|` operator in Python 3.9 and earlier, you can
/// use future imports:
/// ```python
/// from __future__ import annotations
///
///
/// def foo(arg: int | None = None):
///     pass
/// ```
///
/// ## Limitations
///
/// Type aliases are not supported and could result in false negatives.
/// For example, the following code will not be flagged:
/// ```python
/// Text = str | bytes
///
///
/// def foo(arg: Text = None):
///     pass
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## Fix safety
///
/// This fix is always marked as unsafe because it can change the behavior of code that relies on
/// type hints, and it assumes the default value is always appropriate—which might not be the case.
///
/// [PEP 484]: https://peps.python.org/pep-0484/#union-types
#[derive(ViolationMetadata)]
pub struct ImplicitOptional {
    conversion_type: ConversionType,
}

impl Violation for ImplicitOptional {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "PEP 484 prohibits implicit `Optional`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        let ImplicitOptional { conversion_type } = self;
        Some(format!("Convert to `{conversion_type}`"))
    }
}

// FIX: duplicated with ruff_rule_ruff::implicit_optional
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConversionType {
    /// Conversion using the `|` operator e.g., `str | None`
    BinOpOr,
    /// Conversion using the `typing.Optional` type e.g., `typing.Optional[str]`
    Optional,
}

impl fmt::Display for ConversionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BinOpOr => f.write_str("T | None"),
            Self::Optional => f.write_str("Optional[T]"),
        }
    }
}

impl From<PythonVersion> for ConversionType {
    fn from(target_version: PythonVersion) -> Self {
        if target_version >= PythonVersion::PY310 {
            Self::BinOpOr
        } else {
            Self::Optional
        }
    }
}