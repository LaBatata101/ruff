use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for function docstrings that include the function's signature in
/// the summary line.
///
/// ## Why is this bad?
/// [PEP 257] recommends against including a function's signature in its
/// docstring. Instead, consider using type annotations as a form of
/// documentation for the function's parameters and return value.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is enabled when using the `google` and
/// `pep257` conventions, and disabled when using the `numpy` convention.
///
/// ## Example
/// ```python
/// def foo(a, b):
///     """foo(a: int, b: int) -> list[int]"""
/// ```
///
/// Use instead:
/// ```python
/// def foo(a: int, b: int) -> list[int]:
///     """Return a list of a and b."""
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
///
/// [PEP 257]: https://peps.python.org/pep-0257/
#[derive(ViolationMetadata)]
pub struct SignatureInDocstring;

impl Violation for SignatureInDocstring {
    #[derive_message_formats]
    fn message(&self) -> String {
        "First line should not be the function's signature".to_string()
    }
}