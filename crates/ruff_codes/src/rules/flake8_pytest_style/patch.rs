use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for mocked calls that use a dummy `lambda` function instead of
/// `return_value`.
///
/// ## Why is this bad?
/// When patching calls, an explicit `return_value` better conveys the intent
/// than a `lambda` function, assuming the `lambda` does not use the arguments
/// passed to it.
///
/// `return_value` is also robust to changes in the patched function's
/// signature, and enables additional assertions to verify behavior. For
/// example, `return_value` allows for verification of the number of calls or
/// the arguments passed to the patched function via `assert_called_once_with`
/// and related methods.
///
/// ## Example
/// ```python
/// def test_foo(mocker):
///     mocker.patch("module.target", lambda x, y: 7)
/// ```
///
/// Use instead:
/// ```python
/// def test_foo(mocker):
///     mocker.patch("module.target", return_value=7)
///
///     # If the lambda makes use of the arguments, no diagnostic is emitted.
///     mocker.patch("module.other_target", lambda x, y: x)
/// ```
///
/// ## References
/// - [Python documentation: `unittest.mock.patch`](https://docs.python.org/3/library/unittest.mock.html#unittest.mock.patch)
/// - [PyPI: `pytest-mock`](https://pypi.org/project/pytest-mock/)
#[derive(ViolationMetadata)]
pub struct PytestPatchWithLambda;

impl Violation for PytestPatchWithLambda {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `return_value=` instead of patching with `lambda`".to_string()
    }
}