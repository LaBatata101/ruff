use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for imports that are typically imported using a common convention,
/// like `import pandas as pd`, and enforces that convention.
///
/// ## Why is this bad?
/// Consistency is good. Use a common convention for imports to make your code
/// more readable and idiomatic.
///
/// For example, `import pandas as pd` is a common
/// convention for importing the `pandas` library, and users typically expect
/// Pandas to be aliased as `pd`.
///
/// ## Example
/// ```python
/// import pandas
/// ```
///
/// Use instead:
/// ```python
/// import pandas as pd
/// ```
///
/// ## Options
/// - `lint.flake8-import-conventions.aliases`
/// - `lint.flake8-import-conventions.extend-aliases`
#[derive(ViolationMetadata)]
pub struct UnconventionalImportAlias {
    name: String,
    asname: String,
}

impl Violation for UnconventionalImportAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnconventionalImportAlias { name, asname } = self;
        format!("`{name}` should be imported as `{asname}`")
    }

    fn fix_title(&self) -> Option<String> {
        let UnconventionalImportAlias { name, asname } = self;
        Some(format!("Alias `{name}` to `{asname}`"))
    }
}