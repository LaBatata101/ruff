use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for lambda arguments that use the same names as Python builtins.
///
/// ## Why is this bad?
/// Reusing a builtin name for the name of a lambda argument increases the
/// difficulty of reading and maintaining the code and can cause
/// non-obvious errors. Readers may mistake the variable for the
/// builtin, and vice versa.
///
/// Builtins can be marked as exceptions to this rule via the
/// [`lint.flake8-builtins.ignorelist`] configuration option.
///
/// ## Options
/// - `lint.flake8-builtins.ignorelist`
#[derive(ViolationMetadata)]
pub struct BuiltinLambdaArgumentShadowing {
    name: String,
}

impl Violation for BuiltinLambdaArgumentShadowing {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BuiltinLambdaArgumentShadowing { name } = self;
        format!("Lambda argument `{name}` is shadowing a Python builtin")
    }
}