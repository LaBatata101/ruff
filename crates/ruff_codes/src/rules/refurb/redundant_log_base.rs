use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `math.log` calls with a redundant base.
///
/// ## Why is this bad?
/// The default base of `math.log` is `e`, so specifying it explicitly is
/// redundant.
///
/// Instead of passing 2 or 10 as the base, use `math.log2` or `math.log10`
/// respectively, as these dedicated variants are typically more accurate
/// than `math.log`.
///
/// ## Example
/// ```python
/// import math
///
/// math.log(4, math.e)
/// math.log(4, 2)
/// math.log(4, 10)
/// ```
///
/// Use instead:
/// ```python
/// import math
///
/// math.log(4)
/// math.log2(4)
/// math.log10(4)
/// ```
///
/// ## References
/// - [Python documentation: `math.log`](https://docs.python.org/3/library/math.html#math.log)
/// - [Python documentation: `math.log2`](https://docs.python.org/3/library/math.html#math.log2)
/// - [Python documentation: `math.log10`](https://docs.python.org/3/library/math.html#math.log10)
/// - [Python documentation: `math.e`](https://docs.python.org/3/library/math.html#math.e)
#[derive(ViolationMetadata)]
pub struct RedundantLogBase {
    base: Base,
    arg: String,
}

impl Violation for RedundantLogBase {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let RedundantLogBase { base, arg } = self;
        let log_function = base.to_log_function();
        format!("Prefer `math.{log_function}({arg})` over `math.log` with a redundant base")
    }

    fn fix_title(&self) -> Option<String> {
        let RedundantLogBase { base, arg } = self;
        let log_function = base.to_log_function();
        Some(format!("Replace with `math.{log_function}({arg})`"))
    }
}

// FIX: duplicated with ruff_rule_refurb::redundant_log_base
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Base {
    E,
    Two,
    Ten,
}

impl Base {
    fn to_log_function(self) -> &'static str {
        match self {
            Base::E => "log",
            Base::Two => "log2",
            Base::Ten => "log10",
        }
    }
}