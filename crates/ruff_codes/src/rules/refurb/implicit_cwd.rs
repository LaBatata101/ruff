use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

#[derive(ViolationMetadata)]
pub struct ImplicitCwd;

impl Violation for ImplicitCwd {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Prefer `Path.cwd()` over `Path().resolve()` for current-directory lookups".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace `Path().resolve()` with `Path.cwd()`".to_string())
    }
}