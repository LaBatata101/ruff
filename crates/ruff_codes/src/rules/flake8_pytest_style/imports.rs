use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for incorrect import of pytest.
///
/// ## Why is this bad?
/// For consistency, `pytest` should be imported as `import pytest` and its members should be
/// accessed in the form of `pytest.xxx.yyy` for consistency
///
/// ## Example
/// ```python
/// import pytest as pt
/// from pytest import fixture
/// ```
///
/// Use instead:
/// ```python
/// import pytest
/// ```
#[derive(ViolationMetadata)]
pub struct PytestIncorrectPytestImport;

impl Violation for PytestIncorrectPytestImport {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Incorrect import of `pytest`; use `import pytest` instead".to_string()
    }
}