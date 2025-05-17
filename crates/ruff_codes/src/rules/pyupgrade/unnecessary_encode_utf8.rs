use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary calls to `encode` as UTF-8.
///
/// ## Why is this bad?
/// UTF-8 is the default encoding in Python, so there is no need to call
/// `encode` when UTF-8 is the desired encoding. Instead, use a bytes literal.
///
/// ## Example
/// ```python
/// "foo".encode("utf-8")
/// ```
///
/// Use instead:
/// ```python
/// b"foo"
/// ```
///
/// ## References
/// - [Python documentation: `str.encode`](https://docs.python.org/3/library/stdtypes.html#str.encode)
#[derive(ViolationMetadata)]
pub struct UnnecessaryEncodeUTF8 {
    reason: Reason,
}

impl AlwaysFixableViolation for UnnecessaryEncodeUTF8 {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.reason {
            Reason::BytesLiteral => "Unnecessary call to `encode` as UTF-8".to_string(),
            Reason::DefaultArgument => {
                "Unnecessary UTF-8 `encoding` argument to `encode`".to_string()
            }
        }
    }

    fn fix_title(&self) -> String {
        match self.reason {
            Reason::BytesLiteral => "Rewrite as bytes literal".to_string(),
            Reason::DefaultArgument => "Remove unnecessary `encoding` argument".to_string(),
        }
    }
}

// FIX: duplicated with ruff_rule_pyupgrade::unnecessary_encode_utf8
#[derive(Debug, PartialEq, Eq)]
enum Reason {
    BytesLiteral,
    DefaultArgument,
}