use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for indentation that uses tabs.
///
/// ## Why is this bad?
/// According to [PEP 8], spaces are preferred over tabs (unless used to remain
/// consistent with code that is already indented with tabs).
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces consistent indentation, making the rule redundant.
///
/// The rule is also incompatible with the [formatter] when using
/// `format.indent-style="tab"`.
///
/// [PEP 8]: https://peps.python.org/pep-0008/#tabs-or-spaces
/// [formatter]: https://docs.astral.sh/ruff/formatter
#[derive(ViolationMetadata)]
pub struct TabIndentation;

impl Violation for TabIndentation {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Indentation contains tabs".to_string()
    }
}