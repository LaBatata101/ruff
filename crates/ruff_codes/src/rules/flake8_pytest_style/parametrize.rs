use ruff_diagnostics::{ FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use super::types;

/// ## What it does
/// Checks for the type of parameter names passed to `pytest.mark.parametrize`.
///
/// ## Why is this bad?
/// The `argnames` argument of `pytest.mark.parametrize` takes a string or
/// a sequence of strings. For a single parameter, it's preferable to use a
/// string. For multiple parameters, it's preferable to use the style
/// configured via the [`lint.flake8-pytest-style.parametrize-names-type`] setting.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// # single parameter, always expecting string
/// @pytest.mark.parametrize(("param",), [1, 2, 3])
/// def test_foo(param): ...
///
///
/// # multiple parameters, expecting tuple
/// @pytest.mark.parametrize(["param1", "param2"], [(1, 2), (3, 4)])
/// def test_bar(param1, param2): ...
///
///
/// # multiple parameters, expecting tuple
/// @pytest.mark.parametrize("param1,param2", [(1, 2), (3, 4)])
/// def test_baz(param1, param2): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.mark.parametrize("param", [1, 2, 3])
/// def test_foo(param): ...
///
///
/// @pytest.mark.parametrize(("param1", "param2"), [(1, 2), (3, 4)])
/// def test_bar(param1, param2): ...
/// ```
///
/// ## Options
/// - `lint.flake8-pytest-style.parametrize-names-type`
///
/// ## References
/// - [`pytest` documentation: How to parametrize fixtures and test functions](https://docs.pytest.org/en/latest/how-to/parametrize.html#pytest-mark-parametrize)
#[derive(ViolationMetadata)]
pub struct PytestParametrizeNamesWrongType {
    single_argument: bool,
    expected: types::ParametrizeNameType,
}

impl Violation for PytestParametrizeNamesWrongType {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestParametrizeNamesWrongType {
            single_argument,
            expected,
        } = self;
        let expected_string = {
            if *single_argument {
                "`str`".to_string()
            } else {
                match expected {
                    types::ParametrizeNameType::Csv => format!("a {expected}"),
                    types::ParametrizeNameType::Tuple | types::ParametrizeNameType::List => {
                        format!("`{expected}`")
                    }
                }
            }
        };
        format!("Wrong type passed to first argument of `pytest.mark.parametrize`; expected {expected_string}")
    }

    fn fix_title(&self) -> Option<String> {
        let PytestParametrizeNamesWrongType {
            single_argument,
            expected,
        } = self;
        let expected_string = {
            if *single_argument {
                "string".to_string()
            } else {
                match expected {
                    types::ParametrizeNameType::Csv => format!("{expected}"),
                    types::ParametrizeNameType::Tuple | types::ParametrizeNameType::List => {
                        format!("`{expected}`")
                    }
                }
            }
        };
        Some(format!("Use a {expected_string} for the first argument"))
    }
}

/// ## What it does
/// Checks for the type of parameter values passed to `pytest.mark.parametrize`.
///
/// ## Why is this bad?
/// The `argvalues` argument of `pytest.mark.parametrize` takes an iterator of
/// parameter values, which can be provided as lists or tuples.
///
/// To aid in readability, it's recommended to use a consistent style for the
/// list of values rows, and, in the case of multiple parameters, for each row
/// of values.
///
/// The style for the list of values rows can be configured via the
/// [`lint.flake8-pytest-style.parametrize-values-type`] setting, while the
/// style for each row of values can be configured via the
/// [`lint.flake8-pytest-style.parametrize-values-row-type`] setting.
///
/// For example, [`lint.flake8-pytest-style.parametrize-values-type`] will lead to
/// the following expectations:
///
/// - `tuple`: `@pytest.mark.parametrize("value", ("a", "b", "c"))`
/// - `list`: `@pytest.mark.parametrize("value", ["a", "b", "c"])`
///
/// Similarly, [`lint.flake8-pytest-style.parametrize-values-row-type`] will lead to
/// the following expectations:
///
/// - `tuple`: `@pytest.mark.parametrize(("key", "value"), [("a", "b"), ("c", "d")])`
/// - `list`: `@pytest.mark.parametrize(("key", "value"), [["a", "b"], ["c", "d"]])`
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// # expected list, got tuple
/// @pytest.mark.parametrize("param", (1, 2))
/// def test_foo(param): ...
///
///
/// # expected top-level list, got tuple
/// @pytest.mark.parametrize(
///     ("param1", "param2"),
///     (
///         (1, 2),
///         (3, 4),
///     ),
/// )
/// def test_bar(param1, param2): ...
///
///
/// # expected individual rows to be tuples, got lists
/// @pytest.mark.parametrize(
///     ("param1", "param2"),
///     [
///         [1, 2],
///         [3, 4],
///     ],
/// )
/// def test_baz(param1, param2): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.mark.parametrize("param", [1, 2, 3])
/// def test_foo(param): ...
///
///
/// @pytest.mark.parametrize(("param1", "param2"), [(1, 2), (3, 4)])
/// def test_bar(param1, param2): ...
/// ```
///
/// ## Options
/// - `lint.flake8-pytest-style.parametrize-values-type`
/// - `lint.flake8-pytest-style.parametrize-values-row-type`
///
/// ## References
/// - [`pytest` documentation: How to parametrize fixtures and test functions](https://docs.pytest.org/en/latest/how-to/parametrize.html#pytest-mark-parametrize)
#[derive(ViolationMetadata)]
pub struct PytestParametrizeValuesWrongType {
    values: types::ParametrizeValuesType,
    row: types::ParametrizeValuesRowType,
}

impl Violation for PytestParametrizeValuesWrongType {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestParametrizeValuesWrongType { values, row } = self;
        format!("Wrong values type in `pytest.mark.parametrize` expected `{values}` of `{row}`")
    }

    fn fix_title(&self) -> Option<String> {
        let PytestParametrizeValuesWrongType { values, row } = self;
        Some(format!("Use `{values}` of `{row}` for parameter values"))
    }
}

/// ## What it does
/// Checks for duplicate test cases in `pytest.mark.parametrize`.
///
/// ## Why is this bad?
/// Duplicate test cases are redundant and should be removed.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// @pytest.mark.parametrize(
///     ("param1", "param2"),
///     [
///         (1, 2),
///         (1, 2),
///     ],
/// )
/// def test_foo(param1, param2): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.mark.parametrize(
///     ("param1", "param2"),
///     [
///         (1, 2),
///     ],
/// )
/// def test_foo(param1, param2): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as tests that rely on mutable global
/// state may be affected by removing duplicate test cases.
///
/// ## References
/// - [`pytest` documentation: How to parametrize fixtures and test functions](https://docs.pytest.org/en/latest/how-to/parametrize.html#pytest-mark-parametrize)
#[derive(ViolationMetadata)]
pub struct PytestDuplicateParametrizeTestCases {
    index: usize,
}

impl Violation for PytestDuplicateParametrizeTestCases {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestDuplicateParametrizeTestCases { index } = self;
        format!("Duplicate of test case at index {index} in `pytest.mark.parametrize`")
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove duplicate test case".to_string())
    }
}
