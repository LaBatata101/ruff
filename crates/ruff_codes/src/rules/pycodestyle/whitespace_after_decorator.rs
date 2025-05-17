use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

#[derive(ViolationMetadata)]
pub struct WhitespaceAfterDecorator;

impl AlwaysFixableViolation for WhitespaceAfterDecorator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Whitespace after decorator".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove whitespace".to_string()
    }
}
