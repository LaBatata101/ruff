use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `sys.version[:3]`.
///
/// ## Why is this bad?
/// If the current major or minor version consists of multiple digits,
/// `sys.version[:3]` will truncate the version number (e.g., `"3.10"` would
/// become `"3.1"`). This is likely unintended, and can lead to subtle bugs if
/// the version string is used to test against a specific Python version.
///
/// Instead, use `sys.version_info` to access the current major and minor
/// version numbers as a tuple, which can be compared to other tuples
/// without issue.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.version[:3]  # Evaluates to "3.1" on Python 3.10.
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// sys.version_info[:2]  # Evaluates to (3, 10) on Python 3.10.
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub struct SysVersionSlice3;

impl Violation for SysVersionSlice3 {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version[:3]` referenced (python3.10), use `sys.version_info`".to_string()
    }
}

/// ## What it does
/// Checks for uses of `sys.version[2]`.
///
/// ## Why is this bad?
/// If the current major or minor version consists of multiple digits,
/// `sys.version[2]` will select the first digit of the minor number only
/// (e.g., `"3.10"` would evaluate to `"1"`). This is likely unintended, and
/// can lead to subtle bugs if the version is used to test against a minor
/// version number.
///
/// Instead, use `sys.version_info.minor` to access the current minor version
/// number.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.version[2]  # Evaluates to "1" on Python 3.10.
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// f"{sys.version_info.minor}"  # Evaluates to "10" on Python 3.10.
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub struct SysVersion2;

impl Violation for SysVersion2 {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version[2]` referenced (python3.10), use `sys.version_info`".to_string()
    }
}

/// ## What it does
/// Checks for uses of `sys.version[0]`.
///
/// ## Why is this bad?
/// If the current major or minor version consists of multiple digits,
/// `sys.version[0]` will select the first digit of the major version number
/// only (e.g., `"10.2"` would evaluate to `"1"`). This is likely unintended,
/// and can lead to subtle bugs if the version string is used to test against a
/// major version number.
///
/// Instead, use `sys.version_info.major` to access the current major version
/// number.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.version[0]  # If using Python 10, this evaluates to "1".
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// f"{sys.version_info.major}"  # If using Python 10, this evaluates to "10".
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub struct SysVersion0;

impl Violation for SysVersion0 {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version[0]` referenced (python10), use `sys.version_info`".to_string()
    }
}

/// ## What it does
/// Checks for uses of `sys.version[:1]`.
///
/// ## Why is this bad?
/// If the major version number consists of more than one digit, this will
/// select the first digit of the major version number only (e.g., `"10.0"`
/// would evaluate to `"1"`). This is likely unintended, and can lead to subtle
/// bugs in future versions of Python if the version string is used to test
/// against a specific major version number.
///
/// Instead, use `sys.version_info.major` to access the current major version
/// number.
///
/// ## Example
/// ```python
/// import sys
///
/// sys.version[:1]  # If using Python 10, this evaluates to "1".
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// f"{sys.version_info.major}"  # If using Python 10, this evaluates to "10".
/// ```
///
/// ## References
/// - [Python documentation: `sys.version`](https://docs.python.org/3/library/sys.html#sys.version)
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub struct SysVersionSlice1;

impl Violation for SysVersionSlice1 {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`sys.version[:1]` referenced (python10), use `sys.version_info`".to_string()
    }
}
