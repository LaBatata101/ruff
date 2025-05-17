use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of NumPy functions and constants that were removed from
/// the main namespace in NumPy 2.0.
///
/// ## Why is this bad?
/// NumPy 2.0 includes an overhaul of NumPy's Python API, intended to remove
/// redundant aliases and routines, and establish unambiguous mechanisms for
/// accessing constants, dtypes, and functions.
///
/// As part of this overhaul, a variety of deprecated NumPy functions and
/// constants were removed from the main namespace.
///
/// The majority of these functions and constants can be automatically replaced
/// by other members of the NumPy API or by equivalents from the Python
/// standard library. With the exception of renaming `numpy.byte_bounds` to
/// `numpy.lib.array_utils.byte_bounds`, all such replacements are backwards
/// compatible with earlier versions of NumPy.
///
/// This rule flags all uses of removed members, along with automatic fixes for
/// any backwards-compatible replacements.
///
/// ## Example
/// ```python
/// import numpy as np
///
/// arr1 = [np.Infinity, np.NaN, np.nan, np.PINF, np.inf]
/// arr2 = [np.float_(1.5), np.float64(5.1)]
/// np.round_(arr2)
/// ```
///
/// Use instead:
/// ```python
/// import numpy as np
///
/// arr1 = [np.inf, np.nan, np.nan, np.inf, np.inf]
/// arr2 = [np.float64(1.5), np.float64(5.1)]
/// np.round(arr2)
/// ```
#[derive(ViolationMetadata)]
pub struct Numpy2Deprecation {
    existing: String,
    migration_guide: Option<String>,
    code_action: Option<String>,
}

impl Violation for Numpy2Deprecation {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Numpy2Deprecation {
            existing,
            migration_guide,
            code_action: _,
        } = self;
        match migration_guide {
            Some(migration_guide) => {
                format!("`np.{existing}` will be removed in NumPy 2.0. {migration_guide}",)
            }
            None => format!("`np.{existing}` will be removed without replacement in NumPy 2.0"),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Numpy2Deprecation {
            existing: _,
            migration_guide: _,
            code_action,
        } = self;
        code_action.clone()
    }
}