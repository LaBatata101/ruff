use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::Method;

/// ## What it does
/// Checks for calls to `re.sub`, `re.subn`, and `re.split` that pass `count`,
/// `maxsplit`, or `flags` as positional arguments.
///
/// ## Why is this bad?
/// Passing `count`, `maxsplit`, or `flags` as positional arguments to
/// `re.sub`, `re.subn`, or `re.split` can lead to confusion, as most methods in
/// the `re` module accept `flags` as the third positional argument, while
/// `re.sub`, `re.subn`, and `re.split` have different signatures.
///
/// Instead, pass `count`, `maxsplit`, and `flags` as keyword arguments.
///
/// ## Example
/// ```python
/// import re
///
/// re.split("pattern", "replacement", 1)
/// ```
///
/// Use instead:
/// ```python
/// import re
///
/// re.split("pattern", "replacement", maxsplit=1)
/// ```
///
/// ## References
/// - [Python documentation: `re.sub`](https://docs.python.org/3/library/re.html#re.sub)
/// - [Python documentation: `re.subn`](https://docs.python.org/3/library/re.html#re.subn)
/// - [Python documentation: `re.split`](https://docs.python.org/3/library/re.html#re.split)
#[derive(ViolationMetadata)]
pub struct ReSubPositionalArgs {
    method: Method,
}

impl Violation for ReSubPositionalArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ReSubPositionalArgs { method } = self;
        let param_name = method.param_name();
        format!(
            "`{method}` should pass `{param_name}` and `flags` as keyword arguments to avoid confusion due to unintuitive argument positions"
        )
    }
}