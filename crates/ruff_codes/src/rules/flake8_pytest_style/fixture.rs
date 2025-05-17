use ruff_diagnostics::{AlwaysFixableViolation, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use super::types::Parentheses;

/// ## What it does
/// Checks for argument-free `@pytest.fixture()` decorators with or without
/// parentheses, depending on the [`lint.flake8-pytest-style.fixture-parentheses`]
/// setting.
///
/// ## Why is this bad?
/// If a `@pytest.fixture()` doesn't take any arguments, the parentheses are
/// optional.
///
/// Either removing those unnecessary parentheses _or_ requiring them for all
/// fixtures is fine, but it's best to be consistent. The rule defaults to
/// removing unnecessary parentheses, to match the documentation of the
/// official pytest projects.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def my_fixture(): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture
/// def my_fixture(): ...
/// ```
///
/// ## Options
/// - `lint.flake8-pytest-style.fixture-parentheses`
///
/// ## References
/// - [`pytest` documentation: API Reference: Fixtures](https://docs.pytest.org/en/latest/reference/reference.html#fixtures-api)
#[derive(ViolationMetadata)]
pub struct PytestFixtureIncorrectParenthesesStyle {
    expected: Parentheses,
    actual: Parentheses,
}

impl AlwaysFixableViolation for PytestFixtureIncorrectParenthesesStyle {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestFixtureIncorrectParenthesesStyle { expected, actual } = self;
        format!("Use `@pytest.fixture{expected}` over `@pytest.fixture{actual}`")
    }

    fn fix_title(&self) -> String {
        let PytestFixtureIncorrectParenthesesStyle { expected, .. } = self;
        match expected {
            Parentheses::None => "Remove parentheses".to_string(),
            Parentheses::Empty => "Add parentheses".to_string(),
        }
    }
}

/// ## What it does
/// Checks for `pytest.fixture` calls with positional arguments.
///
/// ## Why is this bad?
/// For clarity and consistency, prefer using keyword arguments to specify
/// fixture configuration.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture("module")
/// def my_fixture(): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture(scope="module")
/// def my_fixture(): ...
/// ```
///
/// ## References
/// - [`pytest` documentation: `@pytest.fixture` functions](https://docs.pytest.org/en/latest/reference/reference.html#pytest-fixture)
#[derive(ViolationMetadata)]
pub struct PytestFixturePositionalArgs {
    function: String,
}

impl Violation for PytestFixturePositionalArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestFixturePositionalArgs { function } = self;
        format!("Configuration for fixture `{function}` specified via positional args, use kwargs")
    }
}

/// ## What it does
/// Checks for `pytest.fixture` calls with `scope="function"`.
///
/// ## Why is this bad?
/// `scope="function"` can be omitted, as it is the default.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture(scope="function")
/// def my_fixture(): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def my_fixture(): ...
/// ```
///
/// ## References
/// - [`pytest` documentation: `@pytest.fixture` functions](https://docs.pytest.org/en/latest/reference/reference.html#pytest-fixture)
#[derive(ViolationMetadata)]
pub struct PytestExtraneousScopeFunction;

impl AlwaysFixableViolation for PytestExtraneousScopeFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`scope='function'` is implied in `@pytest.fixture()`".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove implied `scope` argument".to_string()
    }
}

/// ## Removal
/// This rule has been removed because marking fixtures that do not return a value with an underscore
/// isn't a practice recommended by the pytest community.
///
/// ## What it does
/// Checks for `pytest` fixtures that do not return a value, but are not named
/// with a leading underscore.
///
/// ## Why is this bad?
/// By convention, fixtures that don't return a value should be named with a
/// leading underscore, while fixtures that do return a value should not.
///
/// This rule ignores abstract fixtures and generators.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def patch_something(mocker):
///     mocker.patch("module.object")
///
///
/// @pytest.fixture()
/// def use_context():
///     with create_context():
///         yield
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def _patch_something(mocker):
///     mocker.patch("module.object")
///
///
/// @pytest.fixture()
/// def _use_context():
///     with create_context():
///         yield
/// ```
///
/// ## References
/// - [`pytest` documentation: `@pytest.fixture` functions](https://docs.pytest.org/en/latest/reference/reference.html#pytest-fixture)
#[derive(ViolationMetadata)]
#[deprecated(note = "PT004 has been removed")]
pub struct PytestMissingFixtureNameUnderscore;

#[expect(deprecated)]
impl Violation for PytestMissingFixtureNameUnderscore {
    fn message(&self) -> String {
        unreachable!("PT004 has been removed");
    }

    fn message_formats() -> &'static [&'static str] {
        &["Fixture `{function}` does not return anything, add leading underscore"]
    }
}

/// ## Removal
/// This rule has been removed because marking fixtures that do not return a value with an underscore
/// isn't a practice recommended by the pytest community.
///
/// ## What it does
/// Checks for `pytest` fixtures that return a value, but are named with a
/// leading underscore.
///
/// ## Why is this bad?
/// By convention, fixtures that don't return a value should be named with a
/// leading underscore, while fixtures that do return a value should not.
///
/// This rule ignores abstract fixtures.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def _some_object():
///     return SomeClass()
///
///
/// @pytest.fixture()
/// def _some_object_with_cleanup():
///     obj = SomeClass()
///     yield obj
///     obj.cleanup()
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def some_object():
///     return SomeClass()
///
///
/// @pytest.fixture()
/// def some_object_with_cleanup():
///     obj = SomeClass()
///     yield obj
///     obj.cleanup()
/// ```
///
/// ## References
/// - [`pytest` documentation: `@pytest.fixture` functions](https://docs.pytest.org/en/latest/reference/reference.html#pytest-fixture)
#[derive(ViolationMetadata)]
#[deprecated(note = "PT005 has been removed")]
pub struct PytestIncorrectFixtureNameUnderscore;

#[expect(deprecated)]
impl Violation for PytestIncorrectFixtureNameUnderscore {
    fn message(&self) -> String {
        unreachable!("PT005 has been removed");
    }

    fn message_formats() -> &'static [&'static str] {
        &["Fixture `{function}` returns a value, remove leading underscore"]
    }
}

/// ## What it does
/// Checks for `pytest` test functions that should be decorated with
/// `@pytest.mark.usefixtures`.
///
/// ## Why is this bad?
/// In `pytest`, fixture injection is used to activate fixtures in a test
/// function.
///
/// Fixtures can be injected either by passing them as parameters to the test
/// function, or by using the `@pytest.mark.usefixtures` decorator.
///
/// If the test function depends on the fixture being activated, but does not
/// use it in the test body or otherwise rely on its return value, prefer
/// the `@pytest.mark.usefixtures` decorator, to make the dependency explicit
/// and avoid the confusion caused by unused arguments.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture
/// def _patch_something(): ...
///
///
/// def test_foo(_patch_something): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.fixture
/// def _patch_something(): ...
///
///
/// @pytest.mark.usefixtures("_patch_something")
/// def test_foo(): ...
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.mark.usefixtures`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-mark-usefixtures)
#[derive(ViolationMetadata)]
pub struct PytestFixtureParamWithoutValue {
    name: String,
}

impl Violation for PytestFixtureParamWithoutValue {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestFixtureParamWithoutValue { name } = self;
        format!(
            "Fixture `{name}` without value is injected as parameter, use \
             `@pytest.mark.usefixtures` instead"
        )
    }
}

/// ## What it does
/// Checks for `pytest.yield_fixture` usage.
///
/// ## Why is this bad?
/// `pytest.yield_fixture` is deprecated. `pytest.fixture` should be used instead.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// @pytest.yield_fixture()
/// def my_fixture():
///     obj = SomeClass()
///     yield obj
///     obj.cleanup()
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def my_fixture():
///     obj = SomeClass()
///     yield obj
///     obj.cleanup()
/// ```
///
/// ## References
/// - [`pytest` documentation: `yield_fixture` functions](https://docs.pytest.org/en/latest/yieldfixture.html)
#[derive(ViolationMetadata)]
pub struct PytestDeprecatedYieldFixture;

impl Violation for PytestDeprecatedYieldFixture {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`@pytest.yield_fixture` is deprecated, use `@pytest.fixture`".to_string()
    }
}

/// ## What it does
/// Checks for unnecessary `request.addfinalizer` usages in `pytest` fixtures.
///
/// ## Why is this bad?
/// `pytest` offers two ways to perform cleanup in fixture code. The first is
/// sequential (via the `yield` statement), the second callback-based (via
/// `request.addfinalizer`).
///
/// The sequential approach is more readable and should be preferred, unless
/// the fixture uses the "factory as fixture" pattern.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def my_fixture(request):
///     resource = acquire_resource()
///     request.addfinalizer(resource.release)
///     return resource
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def my_fixture():
///     resource = acquire_resource()
///     yield resource
///     resource.release()
///
///
/// # "factory-as-fixture" pattern
/// @pytest.fixture()
/// def my_factory(request):
///     def create_resource(arg):
///         resource = acquire_resource(arg)
///         request.addfinalizer(resource.release)
///         return resource
///
///     return create_resource
/// ```
///
/// ## References
/// - [`pytest` documentation: Adding finalizers directly](https://docs.pytest.org/en/latest/how-to/fixtures.html#adding-finalizers-directly)
/// - [`pytest` documentation: Factories as fixtures](https://docs.pytest.org/en/latest/how-to/fixtures.html#factories-as-fixtures)
#[derive(ViolationMetadata)]
pub struct PytestFixtureFinalizerCallback;

impl Violation for PytestFixtureFinalizerCallback {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `yield` instead of `request.addfinalizer`".to_string()
    }
}

/// ## What it does
/// Checks for unnecessary `yield` expressions in `pytest` fixtures.
///
/// ## Why is this bad?
/// In `pytest` fixtures, the `yield` expression should only be used for fixtures
/// that include teardown code, to clean up the fixture after the test function
/// has finished executing.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def my_fixture():
///     resource = acquire_resource()
///     yield resource
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def my_fixture_with_teardown():
///     resource = acquire_resource()
///     yield resource
///     resource.release()
///
///
/// @pytest.fixture()
/// def my_fixture_without_teardown():
///     resource = acquire_resource()
///     return resource
/// ```
///
/// ## References
/// - [`pytest` documentation: Teardown/Cleanup](https://docs.pytest.org/en/latest/how-to/fixtures.html#teardown-cleanup-aka-fixture-finalization)
#[derive(ViolationMetadata)]
pub struct PytestUselessYieldFixture {
    name: String,
}

impl AlwaysFixableViolation for PytestUselessYieldFixture {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestUselessYieldFixture { name } = self;
        format!("No teardown in fixture `{name}`, use `return` instead of `yield`")
    }

    fn fix_title(&self) -> String {
        "Replace `yield` with `return`".to_string()
    }
}

/// ## What it does
/// Checks for `pytest.mark.usefixtures` decorators applied to `pytest`
/// fixtures.
///
/// ## Why is this bad?
/// The `pytest.mark.usefixtures` decorator has no effect on `pytest` fixtures.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def a():
///     pass
///
///
/// @pytest.mark.usefixtures("a")
/// @pytest.fixture()
/// def b(a):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// def a():
///     pass
///
///
/// @pytest.fixture()
/// def b(a):
///     pass
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.mark.usefixtures`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-mark-usefixtures)
#[derive(ViolationMetadata)]
pub struct PytestErroneousUseFixturesOnFixture;

impl AlwaysFixableViolation for PytestErroneousUseFixturesOnFixture {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`pytest.mark.usefixtures` has no effect on fixtures".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `pytest.mark.usefixtures`".to_string()
    }
}

/// ## What it does
/// Checks for unnecessary `@pytest.mark.asyncio` decorators applied to fixtures.
///
/// ## Why is this bad?
/// `pytest.mark.asyncio` is unnecessary for fixtures.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// @pytest.mark.asyncio()
/// @pytest.fixture()
/// async def my_fixture():
///     return 0
/// ```
///
/// Use instead:
/// ```python
/// import pytest
///
///
/// @pytest.fixture()
/// async def my_fixture():
///     return 0
/// ```
///
/// ## References
/// - [PyPI: `pytest-asyncio`](https://pypi.org/project/pytest-asyncio/)
#[derive(ViolationMetadata)]
pub struct PytestUnnecessaryAsyncioMarkOnFixture;

impl AlwaysFixableViolation for PytestUnnecessaryAsyncioMarkOnFixture {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`pytest.mark.asyncio` is unnecessary for fixtures".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `pytest.mark.asyncio`".to_string()
    }
}
