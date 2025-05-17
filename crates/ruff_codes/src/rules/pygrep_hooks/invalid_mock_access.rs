use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for common mistakes when using mock objects.
///
/// ## Why is this bad?
/// The `mock` module exposes an assertion API that can be used to verify that
/// mock objects undergo expected interactions. This rule checks for common
/// mistakes when using this API.
///
/// For example, it checks for mock attribute accesses that should be replaced
/// with mock method calls.
///
/// ## Example
/// ```python
/// my_mock.assert_called
/// ```
///
/// Use instead:
/// ```python
/// my_mock.assert_called()
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidMockAccess {
    reason: Reason,
}

impl Violation for InvalidMockAccess {
    #[derive_message_formats]
    fn message(&self) -> String {
        let InvalidMockAccess { reason } = self;
        match reason {
            Reason::UncalledMethod(name) => format!("Mock method should be called: `{name}`"),
            Reason::NonExistentMethod(name) => format!("Non-existent mock method: `{name}`"),
        }
    }
}

// FIX: duplicated with ruff_rule_pygrep_hooks::invalid_mock_access
#[derive(Debug, PartialEq, Eq)]
enum Reason {
    UncalledMethod(String),
    NonExistentMethod(String),
}