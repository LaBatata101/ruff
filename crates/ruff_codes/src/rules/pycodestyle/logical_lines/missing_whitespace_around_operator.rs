use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for missing whitespace around all operators.
///
/// ## Why is this bad?
/// According to [PEP 8], there should be one space before and after all
/// operators.
///
/// ## Example
/// ```python
/// if number==42:
///     print('you have found the meaning of life')
/// ```
///
/// Use instead:
/// ```python
/// if number == 42:
///     print('you have found the meaning of life')
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#pet-peeves
// E225
#[derive(ViolationMetadata)]
pub struct MissingWhitespaceAroundOperator;

impl AlwaysFixableViolation for MissingWhitespaceAroundOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing whitespace around operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Add missing whitespace".to_string()
    }
}

/// ## What it does
/// Checks for missing whitespace arithmetic operators.
///
/// ## Why is this bad?
/// According to [PEP 8], there should be one space before and after an
/// arithmetic operator (+, -, /, and *).
///
/// ## Example
/// ```python
/// number = 40+2
/// ```
///
/// Use instead:
/// ```python
/// number = 40 + 2
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#pet-peeves
// E226
#[derive(ViolationMetadata)]
pub struct MissingWhitespaceAroundArithmeticOperator;

impl AlwaysFixableViolation for MissingWhitespaceAroundArithmeticOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing whitespace around arithmetic operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Add missing whitespace".to_string()
    }
}

/// ## What it does
/// Checks for missing whitespace around bitwise and shift operators.
///
/// ## Why is this bad?
/// According to [PEP 8], there should be one space before and after bitwise and
/// shift operators (<<, >>, &, |, ^).
///
/// ## Example
/// ```python
/// x = 128<<1
/// ```
///
/// Use instead:
/// ```python
/// x = 128 << 1
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#pet-peeves
// E227
#[derive(ViolationMetadata)]
pub struct MissingWhitespaceAroundBitwiseOrShiftOperator;

impl AlwaysFixableViolation for MissingWhitespaceAroundBitwiseOrShiftOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing whitespace around bitwise or shift operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Add missing whitespace".to_string()
    }
}

/// ## What it does
/// Checks for missing whitespace around the modulo operator.
///
/// ## Why is this bad?
/// According to [PEP 8], the modulo operator (%) should have whitespace on
/// either side of it.
///
/// ## Example
/// ```python
/// remainder = 10%2
/// ```
///
/// Use instead:
/// ```python
/// remainder = 10 % 2
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#other-recommendations
// E228
#[derive(ViolationMetadata)]
pub struct MissingWhitespaceAroundModuloOperator;

impl AlwaysFixableViolation for MissingWhitespaceAroundModuloOperator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing whitespace around modulo operator".to_string()
    }

    fn fix_title(&self) -> String {
        "Add missing whitespace".to_string()
    }
}
