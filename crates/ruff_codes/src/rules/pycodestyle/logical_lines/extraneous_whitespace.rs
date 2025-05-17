use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of extraneous whitespace after "(", "[" or "{".
///
/// ## Why is this bad?
/// [PEP 8] recommends the omission of whitespace in the following cases:
/// - "Immediately inside parentheses, brackets or braces."
/// - "Immediately before a comma, semicolon, or colon."
///
/// ## Example
/// ```python
/// spam( ham[1], {eggs: 2})
/// spam(ham[ 1], {eggs: 2})
/// spam(ham[1], { eggs: 2})
/// ```
///
/// Use instead:
/// ```python
/// spam(ham[1], {eggs: 2})
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#pet-peeves
#[derive(ViolationMetadata)]
pub struct WhitespaceAfterOpenBracket {
    symbol: char,
}

impl AlwaysFixableViolation for WhitespaceAfterOpenBracket {
    #[derive_message_formats]
    fn message(&self) -> String {
        let WhitespaceAfterOpenBracket { symbol } = self;
        format!("Whitespace after '{symbol}'")
    }

    fn fix_title(&self) -> String {
        let WhitespaceAfterOpenBracket { symbol } = self;
        format!("Remove whitespace before '{symbol}'")
    }
}

/// ## What it does
/// Checks for the use of extraneous whitespace before ")", "]" or "}".
///
/// ## Why is this bad?
/// [PEP 8] recommends the omission of whitespace in the following cases:
/// - "Immediately inside parentheses, brackets or braces."
/// - "Immediately before a comma, semicolon, or colon."
///
/// ## Example
/// ```python
/// spam(ham[1], {eggs: 2} )
/// spam(ham[1 ], {eggs: 2})
/// spam(ham[1], {eggs: 2 })
/// ```
///
/// Use instead:
/// ```python
/// spam(ham[1], {eggs: 2})
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#pet-peeves
#[derive(ViolationMetadata)]
pub struct WhitespaceBeforeCloseBracket {
    symbol: char,
}

impl AlwaysFixableViolation for WhitespaceBeforeCloseBracket {
    #[derive_message_formats]
    fn message(&self) -> String {
        let WhitespaceBeforeCloseBracket { symbol } = self;
        format!("Whitespace before '{symbol}'")
    }

    fn fix_title(&self) -> String {
        let WhitespaceBeforeCloseBracket { symbol } = self;
        format!("Remove whitespace before '{symbol}'")
    }
}

/// ## What it does
/// Checks for the use of extraneous whitespace before ",", ";" or ":".
///
/// ## Why is this bad?
/// [PEP 8] recommends the omission of whitespace in the following cases:
/// - "Immediately inside parentheses, brackets or braces."
/// - "Immediately before a comma, semicolon, or colon."
///
/// ## Example
/// ```python
/// if x == 4: print(x, y); x, y = y , x
/// ```
///
/// Use instead:
/// ```python
/// if x == 4: print(x, y); x, y = y, x
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#pet-peeves
#[derive(ViolationMetadata)]
pub struct WhitespaceBeforePunctuation {
    symbol: char,
}

impl AlwaysFixableViolation for WhitespaceBeforePunctuation {
    #[derive_message_formats]
    fn message(&self) -> String {
        let WhitespaceBeforePunctuation { symbol } = self;
        format!("Whitespace before '{symbol}'")
    }

    fn fix_title(&self) -> String {
        let WhitespaceBeforePunctuation { symbol } = self;
        format!("Remove whitespace before '{symbol}'")
    }
}
