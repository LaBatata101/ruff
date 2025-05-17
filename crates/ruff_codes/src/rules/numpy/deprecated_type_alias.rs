use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for deprecated NumPy type aliases.
///
/// ## Why is this bad?
/// NumPy's `np.int` has long been an alias of the builtin `int`; the same
/// is true of `np.float` and others. These aliases exist primarily
/// for historic reasons, and have been a cause of frequent confusion
/// for newcomers.
///
/// These aliases were deprecated in 1.20, and removed in 1.24.
/// Note, however, that `np.bool` and `np.long` were reintroduced in 2.0 with
/// different semantics, and are thus omitted from this rule.
///
/// ## Example
/// ```python
/// import numpy as np
///
/// np.int
/// ```
///
/// Use instead:
/// ```python
/// int
/// ```
#[derive(ViolationMetadata)]
pub struct NumpyDeprecatedTypeAlias {
    type_name: String,
}

impl Violation for NumpyDeprecatedTypeAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let NumpyDeprecatedTypeAlias { type_name } = self;
        format!("Type alias `np.{type_name}` is deprecated, replace with builtin type")
    }

    fn fix_title(&self) -> Option<String> {
        let NumpyDeprecatedTypeAlias { type_name } = self;
        Some(format!("Replace `np.{type_name}` with builtin type"))
    }
}