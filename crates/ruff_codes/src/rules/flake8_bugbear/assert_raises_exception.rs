use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::ExceptionKind;

/// ## What it does
/// Checks for `assertRaises` and `pytest.raises` context managers that catch
/// `Exception` or `BaseException`.
///
/// ## Why is this bad?
/// These forms catch every `Exception`, which can lead to tests passing even
/// if, e.g., the code under consideration raises a `SyntaxError` or
/// `IndentationError`.
///
/// Either assert for a more specific exception (builtin or custom), or use
/// `assertRaisesRegex` or `pytest.raises(..., match=<REGEX>)` respectively.
///
/// ## Example
/// ```python
/// self.assertRaises(Exception, foo)
/// ```
///
/// Use instead:
/// ```python
/// self.assertRaises(SomeSpecificException, foo)
/// ```
#[derive(ViolationMetadata)]
pub struct AssertRaisesException {
    exception: ExceptionKind,
}

impl Violation for AssertRaisesException {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AssertRaisesException { exception } = self;
        format!("Do not assert blind exception: `{exception}`")
    }
}