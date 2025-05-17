use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `noqa` directives that use redirected rule codes.
///
/// ## Why is this bad?
/// When one of Ruff's rule codes has been redirected, the implication is that the rule has
/// been deprecated in favor of another rule or code. To keep your codebase
/// consistent and up-to-date, prefer the canonical rule code over the deprecated
/// code.
///
/// ## Example
/// ```python
/// x = eval(command)  # noqa: PGH001
/// ```
///
/// Use instead:
/// ```python
/// x = eval(command)  # noqa: S307
/// ```
#[derive(ViolationMetadata)]
pub struct RedirectedNOQA {
    original: String,
    target: String,
}

impl AlwaysFixableViolation for RedirectedNOQA {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RedirectedNOQA { original, target } = self;
        format!("`{original}` is a redirect to `{target}`")
    }

    fn fix_title(&self) -> String {
        let RedirectedNOQA { target, .. } = self;
        format!("Replace with `{target}`")
    }
}