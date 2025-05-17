use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of exceptions that alias `OSError`.
///
/// ## Why is this bad?
/// `OSError` is the builtin error type used for exceptions that relate to the
/// operating system.
///
/// In Python 3.3, a variety of other exceptions, like `WindowsError` were
/// aliased to `OSError`. These aliases remain in place for compatibility with
/// older versions of Python, but may be removed in future versions.
///
/// Prefer using `OSError` directly, as it is more idiomatic and future-proof.
///
/// ## Example
/// ```python
/// raise IOError
/// ```
///
/// Use instead:
/// ```python
/// raise OSError
/// ```
///
/// ## References
/// - [Python documentation: `OSError`](https://docs.python.org/3/library/exceptions.html#OSError)
#[derive(ViolationMetadata)]
pub struct OSErrorAlias {
    name: Option<String>,
}

impl AlwaysFixableViolation for OSErrorAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Replace aliased errors with `OSError`".to_string()
    }

    fn fix_title(&self) -> String {
        let OSErrorAlias { name } = self;
        match name {
            None => "Replace with builtin `OSError`".to_string(),
            Some(name) => format!("Replace `{name}` with builtin `OSError`"),
        }
    }
}