use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::Reason;

/// ## What it does
/// Checks for files with overly permissive permissions.
///
/// ## Why is this bad?
/// Overly permissive file permissions may allow unintended access and
/// arbitrary code execution.
///
/// ## Example
/// ```python
/// import os
///
/// os.chmod("/etc/secrets.txt", 0o666)  # rw-rw-rw-
/// ```
///
/// Use instead:
/// ```python
/// import os
///
/// os.chmod("/etc/secrets.txt", 0o600)  # rw-------
/// ```
///
/// ## References
/// - [Python documentation: `os.chmod`](https://docs.python.org/3/library/os.html#os.chmod)
/// - [Python documentation: `stat`](https://docs.python.org/3/library/stat.html)
/// - [Common Weakness Enumeration: CWE-732](https://cwe.mitre.org/data/definitions/732.html)
#[derive(ViolationMetadata)]
pub struct BadFilePermissions {
    reason: Reason,
}

impl Violation for BadFilePermissions {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BadFilePermissions { reason } = self;
        match reason {
            Reason::Permissive(mask) => {
                format!("`os.chmod` setting a permissive mask `{mask:#o}` on file or directory")
            }
            Reason::Invalid => {
                "`os.chmod` setting an invalid mask on file or directory".to_string()
            }
        }
    }
}