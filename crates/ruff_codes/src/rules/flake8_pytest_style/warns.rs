use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `pytest.warns` context managers with multiple statements.
///
/// This rule allows `pytest.warns` bodies to contain `for`
/// loops with empty bodies (e.g., `pass` or `...` statements), to test
/// iterator behavior.
///
/// ## Why is this bad?
/// When `pytest.warns` is used as a context manager and contains multiple
/// statements, it can lead to the test passing when it should instead fail.
///
/// A `pytest.warns` context manager should only contain a single
/// simple statement that triggers the expected warning.
///
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_foo_warns():
///     with pytest.warns(Warning):
///         setup()  # False negative if setup triggers a warning but foo does not.
///         foo()
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo_warns():
///     setup()
///     with pytest.warns(Warning):
///         foo()
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.warns`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-warns)
#[derive(ViolationMetadata)]
pub struct PytestWarnsWithMultipleStatements;

impl Violation for PytestWarnsWithMultipleStatements {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`pytest.warns()` block should contain a single simple statement".to_string()
    }
}

/// ## What it does
/// Checks for `pytest.warns` calls without a `match` parameter.
///
/// ## Why is this bad?
/// `pytest.warns(Warning)` will catch any `Warning` and may catch warnings that
/// are unrelated to the code under test. To avoid this, `pytest.warns` should
/// be called with a `match` parameter. The warning names that require a `match`
/// parameter can be configured via the
/// [`lint.flake8-pytest-style.warns-require-match-for`] and
/// [`lint.flake8-pytest-style.warns-extend-require-match-for`] settings.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.warns(RuntimeWarning):
///         ...
///
///     # empty string is also an error
///     with pytest.warns(RuntimeWarning, match=""):
///         ...
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.warns(RuntimeWarning, match="expected message"):
///         ...
/// ```
///
/// ## Options
/// - `lint.flake8-pytest-style.warns-require-match-for`
/// - `lint.flake8-pytest-style.warns-extend-require-match-for`
///
/// ## References
/// - [`pytest` documentation: `pytest.warns`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-warns)
#[derive(ViolationMetadata)]
pub struct PytestWarnsTooBroad {
    warning: String,
}

impl Violation for PytestWarnsTooBroad {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestWarnsTooBroad { warning } = self;
        format!(
            "`pytest.warns({warning})` is too broad, set the `match` parameter or use a more \
             specific warning"
        )
    }
}

/// ## What it does
/// Checks for `pytest.warns` calls without an expected warning.
///
/// ## Why is this bad?
/// `pytest.warns` expects to receive an expected warning as its first
/// argument. If omitted, the `pytest.warns` call will fail at runtime.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.warns():
///         do_something()
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.warns(SomeWarning):
///         do_something()
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.warns`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-warns)
#[derive(ViolationMetadata)]
pub struct PytestWarnsWithoutWarning;

impl Violation for PytestWarnsWithoutWarning {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Set the expected warning in `pytest.warns()`".to_string()
    }
}
