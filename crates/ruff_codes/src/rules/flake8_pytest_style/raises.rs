use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `pytest.raises` context managers with multiple statements.
///
/// This rule allows `pytest.raises` bodies to contain `for`
/// loops with empty bodies (e.g., `pass` or `...` statements), to test
/// iterator behavior.
///
/// ## Why is this bad?
/// When a `pytest.raises` is used as a context manager and contains multiple
/// statements, it can lead to the test passing when it actually should fail.
///
/// A `pytest.raises` context manager should only contain a single simple
/// statement that raises the expected exception.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.raises(MyError):
///         setup()
///         func_to_test()  # not executed if `setup()` raises `MyError`
///         assert foo()  # not executed
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     setup()
///     with pytest.raises(MyError):
///         func_to_test()
///     assert foo()
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.raises`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-raises)
#[derive(ViolationMetadata)]
pub struct PytestRaisesWithMultipleStatements;

impl Violation for PytestRaisesWithMultipleStatements {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`pytest.raises()` block should contain a single simple statement".to_string()
    }
}

/// ## What it does
/// Checks for `pytest.raises` calls without a `match` parameter.
///
/// ## Why is this bad?
/// `pytest.raises(Error)` will catch any `Error` and may catch errors that are
/// unrelated to the code under test. To avoid this, `pytest.raises` should be
/// called with a `match` parameter. The exception names that require a `match`
/// parameter can be configured via the
/// [`lint.flake8-pytest-style.raises-require-match-for`] and
/// [`lint.flake8-pytest-style.raises-extend-require-match-for`] settings.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.raises(ValueError):
///         ...
///
///     # empty string is also an error
///     with pytest.raises(ValueError, match=""):
///         ...
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.raises(ValueError, match="expected message"):
///         ...
/// ```
///
/// ## Options
/// - `lint.flake8-pytest-style.raises-require-match-for`
/// - `lint.flake8-pytest-style.raises-extend-require-match-for`
///
/// ## References
/// - [`pytest` documentation: `pytest.raises`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-raises)
#[derive(ViolationMetadata)]
pub struct PytestRaisesTooBroad {
    exception: String,
}

impl Violation for PytestRaisesTooBroad {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestRaisesTooBroad { exception } = self;
        format!(
            "`pytest.raises({exception})` is too broad, set the `match` parameter or use a more \
             specific exception"
        )
    }
}

/// ## What it does
/// Checks for `pytest.raises` calls without an expected exception.
///
/// ## Why is this bad?
/// `pytest.raises` expects to receive an expected exception as its first
/// argument. If omitted, the `pytest.raises` call will fail at runtime.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.raises():
///         do_something()
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.raises(SomeException):
///         do_something()
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.raises`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-raises)
#[derive(ViolationMetadata)]
pub struct PytestRaisesWithoutException;

impl Violation for PytestRaisesWithoutException {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Set the expected exception in `pytest.raises()`".to_string()
    }
}
