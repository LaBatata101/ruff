use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for import aliases that do not rename the original package.
///
/// ## Why is this bad?
/// The import alias is redundant and should be removed to avoid confusion.
///
/// ## Fix safety
/// This fix is marked as always unsafe because the user may be intentionally
/// re-exporting the import. While statements like `import numpy as numpy`
/// appear redundant, they can have semantic meaning in certain contexts.
///
/// ## Example
/// ```python
/// import numpy as numpy
/// ```
///
/// Use instead:
/// ```python
/// import numpy as np
/// ```
///
/// or
///
/// ```python
/// import numpy
/// ```
#[derive(ViolationMetadata)]
pub struct UselessImportAlias {
    required_import_conflict: bool,
}

impl Violation for UselessImportAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        #[expect(clippy::if_not_else)]
        if !self.required_import_conflict {
            "Import alias does not rename original package".to_string()
        } else {
            "Required import does not rename original package.".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        if self.required_import_conflict {
            Some("Change required import or disable rule.".to_string())
        } else {
            Some("Remove import alias".to_string())
        }
    }
}