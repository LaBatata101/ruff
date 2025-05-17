use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for imports of the `mock` module that should be replaced with
/// `unittest.mock`.
///
/// ## Why is this bad?
/// Since Python 3.3, `mock` has been a part of the standard library as
/// `unittest.mock`. The `mock` package is deprecated; use `unittest.mock`
/// instead.
///
/// ## Example
/// ```python
/// import mock
/// ```
///
/// Use instead:
/// ```python
/// from unittest import mock
/// ```
///
/// ## References
/// - [Python documentation: `unittest.mock`](https://docs.python.org/3/library/unittest.mock.html)
/// - [PyPI: `mock`](https://pypi.org/project/mock/)
#[derive(ViolationMetadata)]
pub struct DeprecatedMockImport {
    reference_type: MockReference,
}

impl AlwaysFixableViolation for DeprecatedMockImport {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`mock` is deprecated, use `unittest.mock`".to_string()
    }

    fn fix_title(&self) -> String {
        let DeprecatedMockImport { reference_type } = self;
        match reference_type {
            MockReference::Import => "Import from `unittest.mock` instead".to_string(),
            MockReference::Attribute => "Replace `mock.mock` with `mock`".to_string(),
        }
    }
}

// FIX: duplicated with ruff_rule_pyupgrade::deprecated_mock_import
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MockReference {
    Import,
    Attribute,
}