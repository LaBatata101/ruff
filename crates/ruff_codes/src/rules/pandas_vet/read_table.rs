use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `pd.read_table` to read CSV files.
///
/// ## Why is this bad?
/// In the Pandas API, `pd.read_csv` and `pd.read_table` are equivalent apart
/// from their default separator: `pd.read_csv` defaults to a comma (`,`),
/// while `pd.read_table` defaults to a tab (`\t`) as the default separator.
///
/// Prefer `pd.read_csv` over `pd.read_table` when reading comma-separated
/// data (like CSV files), as it is more idiomatic.
///
/// ## Example
/// ```python
/// import pandas as pd
///
/// cities_df = pd.read_table("cities.csv", sep=",")
/// ```
///
/// Use instead:
/// ```python
/// import pandas as pd
///
/// cities_df = pd.read_csv("cities.csv")
/// ```
///
/// ## References
/// - [Pandas documentation: `read_csv`](https://pandas.pydata.org/docs/reference/api/pandas.read_csv.html#pandas.read_csv)
/// - [Pandas documentation: `read_table`](https://pandas.pydata.org/docs/reference/api/pandas.read_table.html#pandas.read_table)
#[derive(ViolationMetadata)]
pub struct PandasUseOfDotReadTable;

impl Violation for PandasUseOfDotReadTable {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `.read_csv` instead of `.read_table` to read CSV files".to_string()
    }
}