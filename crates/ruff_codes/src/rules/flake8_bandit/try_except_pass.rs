use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the `try`-`except`-`pass` pattern.
///
/// ## Why is this bad?
/// The `try`-`except`-`pass` pattern suppresses all exceptions. Suppressing
/// exceptions may hide errors that could otherwise reveal unexpected behavior,
/// security vulnerabilities, or malicious activity. Instead, consider logging
/// the exception.
///
/// ## Example
/// ```python
/// try:
///     ...
/// except Exception:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
/// try:
///     ...
/// except Exception as exc:
///     logging.exception("Exception occurred")
/// ```
///
/// ## Options
/// - `lint.flake8-bandit.check-typed-exception`
///
/// ## References
/// - [Common Weakness Enumeration: CWE-703](https://cwe.mitre.org/data/definitions/703.html)
/// - [Python documentation: `logging`](https://docs.python.org/3/library/logging.html)
#[derive(ViolationMetadata)]
pub struct TryExceptPass;

impl Violation for TryExceptPass {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`try`-`except`-`pass` detected, consider logging the exception".to_string()
    }
}