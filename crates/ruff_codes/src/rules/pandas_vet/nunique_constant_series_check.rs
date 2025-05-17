use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for uses of `.nunique()` to check if a Pandas Series is constant
/// (i.e., contains only one unique value).
///
/// ## Why is this bad?
/// `.nunique()` is computationally inefficient for checking if a Series is
/// constant.
///
/// Consider, for example, a Series of length `n` that consists of increasing
/// integer values (e.g., 1, 2, 3, 4). The `.nunique()` method will iterate
/// over the entire Series to count the number of unique values. But in this
/// case, we can detect that the Series is non-constant after visiting the
/// first two values, which are non-equal.
///
/// In general, `.nunique()` requires iterating over the entire Series, while a
/// more efficient approach allows short-circuiting the operation as soon as a
/// non-equal value is found.
///
/// Instead of calling `.nunique()`, convert the Series to a NumPy array, and
/// check if all values in the array are equal to the first observed value.
///
/// ## Example
/// ```python
/// import pandas as pd
///
/// data = pd.Series(range(1000))
/// if data.nunique() <= 1:
///     print("Series is constant")
/// ```
///
/// Use instead:
/// ```python
/// import pandas as pd
///
/// data = pd.Series(range(1000))
/// array = data.to_numpy()
/// if array.shape[0] == 0 or (array[0] == array).all():
///     print("Series is constant")
/// ```
///
/// ## References
/// - [Pandas Cookbook: "Constant Series"](https://pandas.pydata.org/docs/user_guide/cookbook.html#constant-series)
/// - [Pandas documentation: `nunique`](https://pandas.pydata.org/docs/reference/api/pandas.Series.nunique.html)
#[derive(ViolationMetadata)]
pub struct PandasNuniqueConstantSeriesCheck;

impl Violation for PandasNuniqueConstantSeriesCheck {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `series.nunique()` for checking that a series is constant is inefficient".to_string()
    }
}