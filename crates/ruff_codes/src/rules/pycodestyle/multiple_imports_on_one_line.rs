use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for multiple imports on one line.
///
/// ## Why is this bad?
/// According to [PEP 8], "imports should usually be on separate lines."
///
/// ## Example
/// ```python
/// import sys, os
/// ```
///
/// Use instead:
/// ```python
/// import os
/// import sys
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#imports
#[derive(ViolationMetadata)]
pub struct MultipleImportsOnOneLine;

impl Violation for MultipleImportsOnOneLine {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Multiple imports on one line".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Split imports".to_string())
    }
}