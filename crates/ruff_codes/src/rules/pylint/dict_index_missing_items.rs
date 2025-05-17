use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

#[derive(ViolationMetadata)]
pub struct DictIndexMissingItems;

impl Violation for DictIndexMissingItems {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Extracting value from dictionary without calling `.items()`".to_string()
    }
}