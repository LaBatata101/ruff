use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

#[derive(ViolationMetadata)]
pub struct RedefinedArgumentFromLocal {
    pub(crate) name: String,
}

impl Violation for RedefinedArgumentFromLocal {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RedefinedArgumentFromLocal { name } = self;
        format!("Redefining argument with the local name `{name}`")
    }
}