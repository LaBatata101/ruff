use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for problematic `sys.version_info`-related conditions in stubs.
///
/// ## Why is this bad?
/// Stub files support simple conditionals to test for differences in Python
/// versions using `sys.version_info`. However, there are a number of common
/// mistakes involving `sys.version_info` comparisons that should be avoided.
/// For example, comparing against a string can lead to unexpected behavior.
///
/// ## Example
/// ```pyi
/// import sys
///
/// if sys.version_info[0] == "2": ...
/// ```
///
/// Use instead:
/// ```pyi
/// import sys
///
/// if sys.version_info[0] == 2: ...
/// ```
///
/// ## References
/// - [Typing documentation: Version and Platform checking](https://typing.python.org/en/latest/spec/directives.html#version-and-platform-checks)
#[derive(ViolationMetadata)]
pub struct UnrecognizedVersionInfoCheck;

impl Violation for UnrecognizedVersionInfoCheck {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unrecognized `sys.version_info` check".to_string()
    }
}

/// ## What it does
/// Checks for Python version comparisons in stubs that compare against patch
/// versions (e.g., Python 3.8.3) instead of major and minor versions (e.g.,
/// Python 3.8).
///
/// ## Why is this bad?
/// Stub files support simple conditionals to test for differences in Python
/// versions and platforms. However, type checkers only understand a limited
/// subset of these conditionals. In particular, type checkers don't support
/// patch versions (e.g., Python 3.8.3), only major and minor versions (e.g.,
/// Python 3.8). Therefore, version checks in stubs should only use the major
/// and minor versions.
///
/// ## Example
/// ```pyi
/// import sys
///
/// if sys.version_info >= (3, 4, 3): ...
/// ```
///
/// Use instead:
/// ```pyi
/// import sys
///
/// if sys.version_info >= (3, 4): ...
/// ```
///
/// ## References
/// - [Typing documentation: Version and Platform checking](https://typing.python.org/en/latest/spec/directives.html#version-and-platform-checks)
#[derive(ViolationMetadata)]
pub struct PatchVersionComparison;

impl Violation for PatchVersionComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Version comparison must use only major and minor version".to_string()
    }
}

/// ## What it does
/// Checks for Python version comparisons that compare against a tuple of the
/// wrong length.
///
/// ## Why is this bad?
/// Stub files support simple conditionals to test for differences in Python
/// versions and platforms. When comparing against `sys.version_info`, avoid
/// comparing against tuples of the wrong length, which can lead to unexpected
/// behavior.
///
/// ## Example
/// ```pyi
/// import sys
///
/// if sys.version_info[:2] == (3,): ...
/// ```
///
/// Use instead:
/// ```pyi
/// import sys
///
/// if sys.version_info[0] == 3: ...
/// ```
///
/// ## References
/// - [Typing documentation: Version and Platform checking](https://typing.python.org/en/latest/spec/directives.html#version-and-platform-checks)
#[derive(ViolationMetadata)]
pub struct WrongTupleLengthVersionComparison {
    expected_length: usize,
}

impl Violation for WrongTupleLengthVersionComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        let WrongTupleLengthVersionComparison { expected_length } = self;
        format!("Version comparison must be against a length-{expected_length} tuple")
    }
}
