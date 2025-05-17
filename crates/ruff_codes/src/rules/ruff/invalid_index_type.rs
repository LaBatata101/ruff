use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for indexed access to lists, strings, tuples, bytes, and comprehensions
/// using a type other than an integer or slice.
///
/// ## Why is this bad?
/// Only integers or slices can be used as indices to these types. Using
/// other types will result in a `TypeError` at runtime and a `SyntaxWarning` at
/// import time.
///
/// ## Example
/// ```python
/// var = [1, 2, 3]["x"]
/// ```
///
/// Use instead:
/// ```python
/// var = [1, 2, 3][0]
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidIndexType {
    value_type: String,
    index_type: String,
    is_slice: bool,
}

impl Violation for InvalidIndexType {
    #[derive_message_formats]
    fn message(&self) -> String {
        let InvalidIndexType {
            value_type,
            index_type,
            is_slice,
        } = self;
        if *is_slice {
            format!("Slice in indexed access to type `{value_type}` uses type `{index_type}` instead of an integer")
        } else {
            format!(
                "Indexed access to type `{value_type}` uses type `{index_type}` instead of an integer or slice"
            )
        }
    }
}