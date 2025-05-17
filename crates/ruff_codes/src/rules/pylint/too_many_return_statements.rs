use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions or methods with too many return statements.
///
/// By default, this rule allows up to six return statements, as configured by
/// the [`lint.pylint.max-returns`] option.
///
/// ## Why is this bad?
/// Functions or methods with many return statements are harder to understand
/// and maintain, and often indicative of complex logic.
///
/// ## Example
/// ```python
/// def capital(country: str) -> str | None:
///     if country == "England":
///         return "London"
///     elif country == "France":
///         return "Paris"
///     elif country == "Poland":
///         return "Warsaw"
///     elif country == "Romania":
///         return "Bucharest"
///     elif country == "Spain":
///         return "Madrid"
///     elif country == "Thailand":
///         return "Bangkok"
///     else:
///         return None
/// ```
///
/// Use instead:
/// ```python
/// def capital(country: str) -> str | None:
///     capitals = {
///         "England": "London",
///         "France": "Paris",
///         "Poland": "Warsaw",
///         "Romania": "Bucharest",
///         "Spain": "Madrid",
///         "Thailand": "Bangkok",
///     }
///     return capitals.get(country)
/// ```
///
/// ## Options
/// - `lint.pylint.max-returns`
#[derive(ViolationMetadata)]
pub struct TooManyReturnStatements {
    returns: usize,
    max_returns: usize,
}

impl Violation for TooManyReturnStatements {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyReturnStatements {
            returns,
            max_returns,
        } = self;
        format!("Too many return statements ({returns} > {max_returns})")
    }
}