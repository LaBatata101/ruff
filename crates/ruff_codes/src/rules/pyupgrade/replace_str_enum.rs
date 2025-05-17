use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

#[derive(ViolationMetadata)]
pub struct ReplaceStrEnum {
    name: String,
}

impl Violation for ReplaceStrEnum {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let ReplaceStrEnum { name } = self;
        format!("Class {name} inherits from both `str` and `enum.Enum`")
    }

    fn fix_title(&self) -> Option<String> {
        Some("Inherit from `enum.StrEnum`".to_string())
    }
}