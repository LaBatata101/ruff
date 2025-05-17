use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for imports that use non-standard naming conventions, like
/// `import tensorflow.keras.backend as K`.
///
/// ## Why is this bad?
/// Consistency is good. Avoid using a non-standard naming convention for
/// imports, and, in particular, choosing import aliases that violate PEP 8.
///
/// For example, aliasing via `import tensorflow.keras.backend as K` violates
/// the guidance of PEP 8, and is thus avoided in some projects.
///
/// ## Example
/// ```python
/// import tensorflow.keras.backend as K
/// ```
///
/// Use instead:
/// ```python
/// import tensorflow as tf
///
/// tf.keras.backend
/// ```
///
/// ## Options
/// - `lint.flake8-import-conventions.banned-aliases`
#[derive(ViolationMetadata)]
pub struct BannedImportAlias {
    name: String,
    asname: String,
}

impl Violation for BannedImportAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BannedImportAlias { name, asname } = self;
        format!("`{name}` should not be imported as `{asname}`")
    }
}