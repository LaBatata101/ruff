use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

// FIX: this is duplicated with ruff_rule_pycodestyle::invalid_escape_sequence
#[derive(Debug, PartialEq, Eq)]
enum FixTitle {
    AddBackslash,
    UseRawStringLiteral,
}

/// ## What it does
/// Checks for invalid escape sequences.
///
/// ## Why is this bad?
/// Invalid escape sequences are deprecated in Python 3.6.
///
/// ## Example
/// ```python
/// regex = "\.png$"
/// ```
///
/// Use instead:
/// ```python
/// regex = r"\.png$"
/// ```
///
/// Or, if the string already contains a valid escape sequence:
/// ```python
/// value = "new line\nand invalid escape \_ here"
/// ```
///
/// Use instead:
/// ```python
/// value = "new line\nand invalid escape \\_ here"
/// ```
///
/// ## References
/// - [Python documentation: String and Bytes literals](https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals)
#[derive(ViolationMetadata)]
pub struct InvalidEscapeSequence {
    ch: char,
    fix_title: FixTitle,
}

impl AlwaysFixableViolation for InvalidEscapeSequence {
    #[derive_message_formats]
    fn message(&self) -> String {
        let InvalidEscapeSequence { ch, .. } = self;
        format!("Invalid escape sequence: `\\{ch}`")
    }

    fn fix_title(&self) -> String {
        match self.fix_title {
            FixTitle::AddBackslash => "Add backslash to escape sequence".to_string(),
            FixTitle::UseRawStringLiteral => "Use a raw string literal".to_string(),
        }
    }
}