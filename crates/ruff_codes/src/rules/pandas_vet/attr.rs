use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `.values` on Pandas Series and Index objects.
///
/// ## Why is this bad?
/// The `.values` attribute is ambiguous as its return type is unclear. As
/// such, it is no longer recommended by the Pandas documentation.
///
/// Instead, use `.to_numpy()` to return a NumPy array, or `.array` to return a
/// Pandas `ExtensionArray`.
///
/// ## Example
/// ```python
/// import pandas as pd
///
/// animals = pd.read_csv("animals.csv").values  # Ambiguous.
/// ```
///
/// Use instead:
/// ```python
/// import pandas as pd
///
/// animals = pd.read_csv("animals.csv").to_numpy()  # Explicit.
/// ```
///
/// ## References
/// - [Pandas documentation: Accessing the values in a Series or Index](https://pandas.pydata.org/pandas-docs/stable/whatsnew/v0.24.0.html#accessing-the-values-in-a-series-or-index)
#[derive(ViolationMetadata)]
pub struct PandasUseOfDotValues;

impl Violation for PandasUseOfDotValues {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `.to_numpy()` instead of `.values`".to_string()
    }
}