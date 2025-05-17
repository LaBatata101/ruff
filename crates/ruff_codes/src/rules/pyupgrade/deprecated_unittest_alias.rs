use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of deprecated methods from the `unittest` module.
///
/// ## Why is this bad?
/// The `unittest` module has deprecated aliases for some of its methods.
/// The deprecated aliases were removed in Python 3.12. Instead of aliases,
/// use their non-deprecated counterparts.
///
/// ## Example
/// ```python
/// from unittest import TestCase
///
///
/// class SomeTest(TestCase):
///     def test_something(self):
///         self.assertEquals(1, 1)
/// ```
///
/// Use instead:
/// ```python
/// from unittest import TestCase
///
///
/// class SomeTest(TestCase):
///     def test_something(self):
///         self.assertEqual(1, 1)
/// ```
///
/// ## References
/// - [Python 3.11 documentation: Deprecated aliases](https://docs.python.org/3.11/library/unittest.html#deprecated-aliases)
#[derive(ViolationMetadata)]
pub struct DeprecatedUnittestAlias {
    alias: String,
    target: String,
}

impl AlwaysFixableViolation for DeprecatedUnittestAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DeprecatedUnittestAlias { alias, target } = self;
        format!("`{alias}` is deprecated, use `{target}`")
    }

    fn fix_title(&self) -> String {
        let DeprecatedUnittestAlias { alias, target } = self;
        format!("Replace `{target}` with `{alias}`")
    }
}