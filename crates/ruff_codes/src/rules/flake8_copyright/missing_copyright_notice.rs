use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the absence of copyright notices within Python files.
///
/// Note that this check only searches within the first 4096 bytes of the file.
///
/// ## Why is this bad?
/// In some codebases, it's common to have a license header at the top of every
/// file. This rule ensures that the license header is present.
///
/// ## Options
/// - `lint.flake8-copyright.author`
/// - `lint.flake8-copyright.min-file-size`
/// - `lint.flake8-copyright.notice-rgx`
#[derive(ViolationMetadata)]
pub struct MissingCopyrightNotice;

impl Violation for MissingCopyrightNotice {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing copyright notice at top of file".to_string()
    }
}