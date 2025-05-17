use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks that `append`, `extend` and `remove` methods are not called on
/// `__all__`.
///
/// ## Why is this bad?
/// Different type checkers have varying levels of support for calling these
/// methods on `__all__`. Instead, use the `+=` operator to add items to
/// `__all__`, which is known to be supported by all major type checkers.
///
/// ## Example
/// ```pyi
/// import sys
///
/// __all__ = ["A", "B"]
///
/// if sys.version_info >= (3, 10):
///     __all__.append("C")
///
/// if sys.version_info >= (3, 11):
///     __all__.remove("B")
/// ```
///
/// Use instead:
/// ```pyi
/// import sys
///
/// __all__ = ["A"]
///
/// if sys.version_info < (3, 11):
///     __all__ += ["B"]
///
/// if sys.version_info >= (3, 10):
///     __all__ += ["C"]
/// ```
#[derive(ViolationMetadata)]
pub struct UnsupportedMethodCallOnAll {
    name: String,
}

impl Violation for UnsupportedMethodCallOnAll {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnsupportedMethodCallOnAll { name } = self;
        format!("Calling `.{name}()` on `__all__` may not be supported by all type checkers (use `+=` instead)")
    }
}