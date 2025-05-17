use ruff_diagnostics::{Violation, FixAvailability};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for assertions that combine multiple independent conditions.
///
/// ## Why is this bad?
/// Composite assertion statements are harder to debug upon failure, as the
/// failure message will not indicate which condition failed.
///
/// ## Example
/// ```python
/// def test_foo():
///     assert something and something_else
///
///
/// def test_bar():
///     assert not (something or something_else)
/// ```
///
/// Use instead:
/// ```python
/// def test_foo():
///     assert something
///     assert something_else
///
///
/// def test_bar():
///     assert not something
///     assert not something_else
/// ```
#[derive(ViolationMetadata)]
pub struct PytestCompositeAssertion;

impl Violation for PytestCompositeAssertion {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Assertion should be broken down into multiple parts".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Break down assertion into multiple parts".to_string())
    }
}

/// ## What it does
/// Checks for `assert` statements in `except` clauses.
///
/// ## Why is this bad?
/// When testing for exceptions, `pytest.raises()` should be used instead of
/// `assert` statements in `except` clauses, as it's more explicit and
/// idiomatic. Further, `pytest.raises()` will fail if the exception is _not_
/// raised, unlike the `assert` statement.
///
/// ## Example
/// ```python
/// def test_foo():
///     try:
///         1 / 0
///     except ZeroDivisionError as e:
///         assert e.args
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     with pytest.raises(ZeroDivisionError) as exc_info:
///         1 / 0
///     assert exc_info.value.args
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.raises`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-raises)
#[derive(ViolationMetadata)]
pub struct PytestAssertInExcept {
    name: String,
}

impl Violation for PytestAssertInExcept {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestAssertInExcept { name } = self;
        format!(
            "Found assertion on exception `{name}` in `except` block, use `pytest.raises()` instead"
        )
    }
}

/// ## What it does
/// Checks for `assert` statements whose test expression is a falsy value.
///
/// ## Why is this bad?
/// `pytest.fail` conveys the intent more clearly than `assert falsy_value`.
///
/// ## Example
/// ```python
/// def test_foo():
///     if some_condition:
///         assert False, "some_condition was True"
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// def test_foo():
///     if some_condition:
///         pytest.fail("some_condition was True")
///     ...
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.fail`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-fail)
#[derive(ViolationMetadata)]
pub struct PytestAssertAlwaysFalse;

impl Violation for PytestAssertAlwaysFalse {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Assertion always fails, replace with `pytest.fail()`".to_string()
    }
}

#[derive(ViolationMetadata)]
pub struct PytestUnittestAssertion {
    assertion: String,
}

impl Violation for PytestUnittestAssertion {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestUnittestAssertion { assertion } = self;
        format!("Use a regular `assert` instead of unittest-style `{assertion}`")
    }

    fn fix_title(&self) -> Option<String> {
        let PytestUnittestAssertion { assertion } = self;
        Some(format!("Replace `{assertion}(...)` with `assert ...`"))
    }
}

/// ## What it does
/// Checks for uses of exception-related assertion methods from the `unittest`
/// module.
///
/// ## Why is this bad?
/// To enforce the assertion style recommended by `pytest`, `pytest.raises` is
/// preferred over the exception-related assertion methods in `unittest`, like
/// `assertRaises`.
///
/// ## Example
/// ```python
/// import unittest
///
///
/// class TestFoo(unittest.TestCase):
///     def test_foo(self):
///         with self.assertRaises(ValueError):
///             raise ValueError("foo")
/// ```
///
/// Use instead:
/// ```python
/// import unittest
/// import pytest
///
///
/// class TestFoo(unittest.TestCase):
///     def test_foo(self):
///         with pytest.raises(ValueError):
///             raise ValueError("foo")
/// ```
///
/// ## References
/// - [`pytest` documentation: Assertions about expected exceptions](https://docs.pytest.org/en/latest/how-to/assert.html#assertions-about-expected-exceptions)
#[derive(ViolationMetadata)]
pub struct PytestUnittestRaisesAssertion {
    assertion: String,
}

impl Violation for PytestUnittestRaisesAssertion {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestUnittestRaisesAssertion { assertion } = self;
        format!("Use `pytest.raises` instead of unittest-style `{assertion}`")
    }

    fn fix_title(&self) -> Option<String> {
        let PytestUnittestRaisesAssertion { assertion } = self;
        Some(format!("Replace `{assertion}` with `pytest.raises`"))
    }
}
