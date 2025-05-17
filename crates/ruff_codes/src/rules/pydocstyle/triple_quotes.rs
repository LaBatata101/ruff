use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::str::Quote;

/// ## What it does
/// Checks for docstrings that use `'''triple single quotes'''` instead of
/// `"""triple double quotes"""`.
///
/// ## Why is this bad?
/// [PEP 257](https://peps.python.org/pep-0257/#what-is-a-docstring) recommends
/// the use of `"""triple double quotes"""` for docstrings, to ensure
/// consistency.
///
/// ## Example
/// ```python
/// def kos_root():
///     '''Return the pathname of the KOS root directory.'''
/// ```
///
/// Use instead:
/// ```python
/// def kos_root():
///     """Return the pathname of the KOS root directory."""
/// ```
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces consistent quotes, making the rule redundant.
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
///
/// [formatter]: https://docs.astral.sh/ruff/formatter/
#[derive(ViolationMetadata)]
pub struct TripleSingleQuotes {
    expected_quote: Quote,
}

impl Violation for TripleSingleQuotes {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.expected_quote {
            Quote::Double => r#"Use triple double quotes `"""`"#.to_string(),
            Quote::Single => r"Use triple single quotes `'''`".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.expected_quote {
            Quote::Double => "Convert to triple double quotes",
            Quote::Single => "Convert to triple single quotes",
        };
        Some(title.to_string())
    }
}