use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for repeated keyword arguments in function calls.
///
/// ## Why is this bad?
/// Python does not allow repeated keyword arguments in function calls. If a
/// function is called with the same keyword argument multiple times, the
/// interpreter will raise an exception.
///
/// ## Example
/// ```python
/// func(1, 2, c=3, **{"c": 4})
/// ```
///
/// ## References
/// - [Python documentation: Argument](https://docs.python.org/3/glossary.html#term-argument)
#[derive(ViolationMetadata)]
pub struct RepeatedKeywordArgument {
    duplicate_keyword: String,
}

impl Violation for RepeatedKeywordArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { duplicate_keyword } = self;
        format!("Repeated keyword argument: `{duplicate_keyword}`")
    }
}