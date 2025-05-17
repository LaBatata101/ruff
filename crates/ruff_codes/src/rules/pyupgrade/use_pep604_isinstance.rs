use std::fmt;

use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## Deprecation
/// This rule was deprecated as using [PEP 604] syntax in `isinstance` and `issubclass` calls
/// isn't recommended practice, and it incorrectly suggests that other typing syntaxes like [PEP 695]
/// would be supported by `isinstance` and `issubclass`. Using the [PEP 604] syntax
/// is also slightly slower.
///
/// ## What it does
/// Checks for uses of `isinstance` and `issubclass` that take a tuple
/// of types for comparison.
///
/// ## Why is this bad?
/// Since Python 3.10, `isinstance` and `issubclass` can be passed a
/// `|`-separated union of types, which is consistent
/// with the union operator introduced in [PEP 604].
///
/// Note that this results in slower code. Ignore this rule if the
/// performance of an `isinstance` or `issubclass` check is a
/// concern, e.g., in a hot loop.
///
/// ## Example
/// ```python
/// isinstance(x, (int, float))
/// ```
///
/// Use instead:
/// ```python
/// isinstance(x, int | float)
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `isinstance`](https://docs.python.org/3/library/functions.html#isinstance)
/// - [Python documentation: `issubclass`](https://docs.python.org/3/library/functions.html#issubclass)
///
/// [PEP 604]: https://peps.python.org/pep-0604/
/// [PEP 695]: https://peps.python.org/pep-0695/
#[derive(ViolationMetadata)]
pub struct NonPEP604Isinstance {
    kind: CallKind,
}

impl AlwaysFixableViolation for NonPEP604Isinstance {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use `X | Y` in `{}` call instead of `(X, Y)`", self.kind)
    }

    fn fix_title(&self) -> String {
        "Convert to `X | Y`".to_string()
    }
}

// FIX: duplicated with ruff_rule_pyupgrade::use_pep604_isinstance
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum CallKind {
    Isinstance,
    Issubclass,
}

impl fmt::Display for CallKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CallKind::Isinstance => fmt.write_str("isinstance"),
            CallKind::Issubclass => fmt.write_str("issubclass"),
        }
    }
}