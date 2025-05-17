use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `pytest.fail` calls without a message.
///
/// ## Why is this bad?
/// `pytest.fail` calls without a message make it harder to understand and debug test failures.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_foo():
///     pytest.fail()
///
///
/// def test_bar():
///     pytest.fail("")
///
///
/// def test_baz():
///     pytest.fail(reason="")
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     pytest.fail("...")
///
///
/// def test_bar():
///     pytest.fail(reason="...")
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.fail`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-fail)
#[derive(ViolationMetadata)]
pub struct PytestFailWithoutMessage;

impl Violation for PytestFailWithoutMessage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "No message passed to `pytest.fail()`".to_string()
    }
}