use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::Collection;

/// ## What it does
/// Checks for unnecessary `dict()`, `list()` or `tuple()` calls that can be
/// rewritten as empty literals.
///
/// ## Why is this bad?
/// It's unnecessary to call, e.g., `dict()` as opposed to using an empty
/// literal (`{}`). The former is slower because the name `dict` must be
/// looked up in the global scope in case it has been rebound.
///
/// ## Example
/// ```python
/// dict()
/// dict(a=1, b=2)
/// list()
/// tuple()
/// ```
///
/// Use instead:
/// ```python
/// {}
/// {"a": 1, "b": 2}
/// []
/// ()
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
///
/// ## Options
/// - `lint.flake8-comprehensions.allow-dict-calls-with-keyword-arguments`
#[derive(ViolationMetadata)]
pub struct UnnecessaryCollectionCall {
    kind: Collection,
}

impl AlwaysFixableViolation for UnnecessaryCollectionCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryCollectionCall { kind } = self;
        format!("Unnecessary `{kind}()` call (rewrite as a literal)")
    }

    fn fix_title(&self) -> String {
        "Rewrite as a literal".to_string()
    }
}