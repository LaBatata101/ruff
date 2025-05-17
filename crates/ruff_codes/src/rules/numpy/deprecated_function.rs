use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of deprecated NumPy functions.
///
/// ## Why is this bad?
/// When NumPy functions are deprecated, they are usually replaced with
/// newer, more efficient versions, or with functions that are more
/// consistent with the rest of the NumPy API.
///
/// Prefer newer APIs over deprecated ones.
///
/// ## Example
/// ```python
/// import numpy as np
///
/// np.alltrue([True, False])
/// ```
///
/// Use instead:
/// ```python
/// import numpy as np
///
/// np.all([True, False])
/// ```
#[derive(ViolationMetadata)]
pub struct NumpyDeprecatedFunction {
    existing: String,
    replacement: String,
}

impl Violation for NumpyDeprecatedFunction {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let NumpyDeprecatedFunction {
            existing,
            replacement,
        } = self;
        format!("`np.{existing}` is deprecated; use `np.{replacement}` instead")
    }

    fn fix_title(&self) -> Option<String> {
        let NumpyDeprecatedFunction { replacement, .. } = self;
        Some(format!("Replace with `np.{replacement}`"))
    }
}