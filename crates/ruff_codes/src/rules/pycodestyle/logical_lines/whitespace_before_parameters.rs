use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_parser::TokenKind;

/// ## What it does
/// Checks for extraneous whitespace immediately preceding an open parenthesis
/// or bracket.
///
/// ## Why is this bad?
/// According to [PEP 8], open parentheses and brackets should not be preceded
/// by any trailing whitespace.
///
/// ## Example
/// ```python
/// spam (1)
/// ```
///
/// Use instead:
/// ```python
/// spam(1)
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#pet-peeves
#[derive(ViolationMetadata)]
pub struct WhitespaceBeforeParameters {
    bracket: TokenKind,
}

impl WhitespaceBeforeParameters {
    fn bracket_text(&self) -> char {
        match self.bracket {
            TokenKind::Lpar => '(',
            TokenKind::Lsqb => '[',
            _ => unreachable!(),
        }
    }
}

impl AlwaysFixableViolation for WhitespaceBeforeParameters {
    #[derive_message_formats]
    fn message(&self) -> String {
        let bracket = self.bracket_text();
        format!("Whitespace before '{bracket}'")
    }

    fn fix_title(&self) -> String {
        let bracket = self.bracket_text();
        format!("Removed whitespace before '{bracket}'")
    }
}
