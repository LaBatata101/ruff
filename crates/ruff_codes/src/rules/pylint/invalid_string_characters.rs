use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for strings that contain the control character `BS`.
///
/// ## Why is this bad?
/// Control characters are displayed differently by different text editors and
/// terminals.
///
/// By using the `\b` sequence in lieu of the `BS` control character, the
/// string will contain the same value, but will render visibly in all editors.
///
/// ## Example
/// ```python
/// x = ""
/// ```
///
/// Use instead:
/// ```python
/// x = "\b"
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidCharacterBackspace;

impl Violation for InvalidCharacterBackspace {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid unescaped character backspace, use \"\\b\" instead".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with escape sequence".to_string())
    }
}

/// ## What it does
/// Checks for strings that contain the raw control character `SUB`.
///
/// ## Why is this bad?
/// Control characters are displayed differently by different text editors and
/// terminals.
///
/// By using the `\x1A` sequence in lieu of the `SUB` control character, the
/// string will contain the same value, but will render visibly in all editors.
///
/// ## Example
/// ```python
/// x = ""
/// ```
///
/// Use instead:
/// ```python
/// x = "\x1a"
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidCharacterSub;

impl Violation for InvalidCharacterSub {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid unescaped character SUB, use \"\\x1A\" instead".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with escape sequence".to_string())
    }
}

/// ## What it does
/// Checks for strings that contain the raw control character `ESC`.
///
/// ## Why is this bad?
/// Control characters are displayed differently by different text editors and
/// terminals.
///
/// By using the `\x1B` sequence in lieu of the `SUB` control character, the
/// string will contain the same value, but will render visibly in all editors.
///
/// ## Example
/// ```python
/// x = ""
/// ```
///
/// Use instead:
/// ```python
/// x = "\x1b"
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidCharacterEsc;

impl Violation for InvalidCharacterEsc {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid unescaped character ESC, use \"\\x1B\" instead".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with escape sequence".to_string())
    }
}

/// ## What it does
/// Checks for strings that contain the raw control character `NUL` (0 byte).
///
/// ## Why is this bad?
/// Control characters are displayed differently by different text editors and
/// terminals.
///
/// By using the `\0` sequence in lieu of the `NUL` control character, the
/// string will contain the same value, but will render visibly in all editors.
///
/// ## Example
/// ```python
/// x = ""
/// ```
///
/// Use instead:
/// ```python
/// x = "\0"
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidCharacterNul;

impl Violation for InvalidCharacterNul {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid unescaped character NUL, use \"\\0\" instead".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with escape sequence".to_string())
    }
}

/// ## What it does
/// Checks for strings that contain the zero width space character.
///
/// ## Why is this bad?
/// This character is rendered invisibly in some text editors and terminals.
///
/// By using the `\u200B` sequence, the string will contain the same value,
/// but will render visibly in all editors.
///
/// ## Example
/// ```python
/// x = "Dear Sir/Madam"
/// ```
///
/// Use instead:
/// ```python
/// x = "Dear Sir\u200b/\u200bMadam"  # zero width space
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidCharacterZeroWidthSpace;

impl Violation for InvalidCharacterZeroWidthSpace {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid unescaped character zero-width-space, use \"\\u200B\" instead".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with escape sequence".to_string())
    }
}
