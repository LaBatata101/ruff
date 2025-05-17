use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for assignments to the variable `df`.
///
/// ## Why is this bad?
/// Although `df` is a common variable name for a Pandas `DataFrame`, it's not a
/// great variable name for production code, as it's non-descriptive and
/// prone to name conflicts.
///
/// Instead, use a more descriptive variable name.
///
/// ## Example
/// ```python
/// import pandas as pd
///
/// df = pd.read_csv("animals.csv")
/// ```
///
/// Use instead:
/// ```python
/// import pandas as pd
///
/// animals = pd.read_csv("animals.csv")
/// ```
#[derive(ViolationMetadata)]
pub struct PandasDfVariableName;

impl Violation for PandasDfVariableName {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid using the generic variable name `df` for DataFrames".to_string()
    }
}