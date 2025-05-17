use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for parameters of test functions with default arguments.
///
/// ## Why is this bad?
/// Such a parameter will always have the default value during the test
/// regardless of whether a fixture with the same name is defined.
///
/// ## Example
///
/// ```python
/// def test_foo(a=1): ...
/// ```
///
/// Use instead:
///
/// ```python
/// def test_foo(a): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as modifying a function signature can
/// change the behavior of the code.
///
/// ## References
/// - [Original Pytest issue](https://github.com/pytest-dev/pytest/issues/12693)
#[derive(ViolationMetadata)]
pub struct PytestParameterWithDefaultArgument {
    parameter_name: String,
}

impl Violation for PytestParameterWithDefaultArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Test function parameter `{}` has default argument",
            self.parameter_name
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove default argument".to_string())
    }
}