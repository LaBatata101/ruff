use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for type annotations that can be rewritten based on [PEP 604] syntax.
///
/// ## Why is this bad?
/// [PEP 604] introduced a new syntax for union type annotations based on the
/// `|` operator. This syntax is more concise and readable than the previous
/// `typing.Union` and `typing.Optional` syntaxes.
///
/// This rule is enabled when targeting Python 3.10 or later (see:
/// [`target-version`]). By default, it's _also_ enabled for earlier Python
/// versions if `from __future__ import annotations` is present, as
/// `__future__` annotations are not evaluated at runtime. If your code relies
/// on runtime type annotations (either directly or via a library like
/// Pydantic), you can disable this behavior for Python versions prior to 3.10
/// by setting [`lint.pyupgrade.keep-runtime-typing`] to `true`.
///
/// ## Example
/// ```python
/// from typing import Union
///
/// foo: Union[int, str] = 1
/// ```
///
/// Use instead:
/// ```python
/// foo: int | str = 1
/// ```
///
/// ## Preview
/// In preview mode, this rule only checks for usages of `typing.Union`,
/// while `UP045` checks for `typing.Optional`.
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may lead to runtime errors when
/// alongside libraries that rely on runtime type annotations, like Pydantic,
/// on Python versions prior to Python 3.10. It may also lead to runtime errors
/// in unusual and likely incorrect type annotations where the type does not
/// support the `|` operator.
///
/// ## Options
/// - `target-version`
/// - `lint.pyupgrade.keep-runtime-typing`
///
/// [PEP 604]: https://peps.python.org/pep-0604/
#[derive(ViolationMetadata)]
pub struct NonPEP604AnnotationUnion;

impl Violation for NonPEP604AnnotationUnion {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `X | Y` for type annotations".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Convert to `X | Y`".to_string())
    }
}

/// ## What it does
/// Check for `typing.Optional` annotations that can be rewritten based on [PEP 604] syntax.
///
/// ## Why is this bad?
/// [PEP 604] introduced a new syntax for union type annotations based on the
/// `|` operator. This syntax is more concise and readable than the previous
/// `typing.Optional` syntax.
///
/// This rule is enabled when targeting Python 3.10 or later (see:
/// [`target-version`]). By default, it's _also_ enabled for earlier Python
/// versions if `from __future__ import annotations` is present, as
/// `__future__` annotations are not evaluated at runtime. If your code relies
/// on runtime type annotations (either directly or via a library like
/// Pydantic), you can disable this behavior for Python versions prior to 3.10
/// by setting [`lint.pyupgrade.keep-runtime-typing`] to `true`.
///
/// ## Example
/// ```python
/// from typing import Optional
///
/// foo: Optional[int] = None
/// ```
///
/// Use instead:
/// ```python
/// foo: int | None = None
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may lead to runtime errors when
/// alongside libraries that rely on runtime type annotations, like Pydantic,
/// on Python versions prior to Python 3.10. It may also lead to runtime errors
/// in unusual and likely incorrect type annotations where the type does not
/// support the `|` operator.
///
/// ## Options
/// - `target-version`
/// - `lint.pyupgrade.keep-runtime-typing`
///
/// [PEP 604]: https://peps.python.org/pep-0604/
#[derive(ViolationMetadata)]
pub struct NonPEP604AnnotationOptional;

impl Violation for NonPEP604AnnotationOptional {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `X | None` for type annotations".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Convert to `X | None`".to_string())
    }
}
