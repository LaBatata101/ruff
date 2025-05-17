use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for hard-coded sequence accesses that are known to be out of bounds.
///
/// ## Why is this bad?
/// Attempting to access a sequence with an out-of-bounds index will cause an
/// `IndexError` to be raised at runtime. When the sequence and index are
/// defined statically (e.g., subscripts on `list` and `tuple` literals, with
/// integer indexes), such errors can be detected ahead of time.
///
/// ## Example
/// ```python
/// print([0, 1, 2][3])
/// ```
#[derive(ViolationMetadata)]
pub struct PotentialIndexError;

impl Violation for PotentialIndexError {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Expression is likely to raise `IndexError`".to_string()
    }
}