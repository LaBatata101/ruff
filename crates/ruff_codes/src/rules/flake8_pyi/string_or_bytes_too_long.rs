use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of string and bytes literals longer than 50 characters
/// in stub (`.pyi`) files.
///
/// ## Why is this bad?
/// If a function or variable has a default value where the string or bytes
/// representation is greater than 50 characters long, it is likely to be an
/// implementation detail or a constant that varies depending on the system
/// you're running on.
///
/// Although IDEs may find them useful, default values are ignored by type
/// checkers, the primary consumers of stub files. Replace very long constants
/// with ellipses (`...`) to simplify the stub.
///
/// ## Example
///
/// ```pyi
/// def foo(arg: str = "51 character stringgggggggggggggggggggggggggggggggg") -> None: ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// def foo(arg: str = ...) -> None: ...
/// ```
#[derive(ViolationMetadata)]
pub struct StringOrBytesTooLong;

impl AlwaysFixableViolation for StringOrBytesTooLong {
    #[derive_message_formats]
    fn message(&self) -> String {
        "String and bytes literals longer than 50 characters are not permitted".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `...`".to_string()
    }
}