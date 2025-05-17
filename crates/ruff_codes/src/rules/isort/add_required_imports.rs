use ruff_diagnostics::{AlwaysFixableViolation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Adds any required imports, as specified by the user, to the top of the
/// file.
///
/// ## Why is this bad?
/// In some projects, certain imports are required to be present in all
/// files. For example, some projects assume that
/// `from __future__ import annotations` is enabled,
/// and thus require that import to be
/// present in all files. Omitting a "required" import (as specified by
/// the user) can cause errors or unexpected behavior.
///
/// ## Example
/// ```python
/// import typing
/// ```
///
/// Use instead:
/// ```python
/// from __future__ import annotations
///
/// import typing
/// ```
///
/// ## Options
/// - `lint.isort.required-imports`
#[derive(ViolationMetadata)]
pub struct MissingRequiredImport(pub String);

impl AlwaysFixableViolation for MissingRequiredImport {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MissingRequiredImport(name) = self;
        format!("Missing required import: `{name}`")
    }

    fn fix_title(&self) -> String {
        let MissingRequiredImport(name) = self;
        format!("Insert required import: `{name}`")
    }
}