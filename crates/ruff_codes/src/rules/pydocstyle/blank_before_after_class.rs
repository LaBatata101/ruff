use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstrings on class definitions that are not preceded by a
/// blank line.
///
/// ## Why is this bad?
/// Use a blank line to separate the docstring from the class definition, for
/// consistency.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is disabled when using the `google`,
/// `numpy`, and `pep257` conventions.
///
/// For an alternative, see [D211].
///
/// ## Example
///
/// ```python
/// class PhotoMetadata:
///     """Metadata about a photo."""
/// ```
///
/// Use instead:
///
/// ```python
/// class PhotoMetadata:
///
///     """Metadata about a photo."""
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// [D211]: https://docs.astral.sh/ruff/rules/blank-line-before-class
#[derive(ViolationMetadata)]
pub struct IncorrectBlankLineBeforeClass;

impl AlwaysFixableViolation for IncorrectBlankLineBeforeClass {
    #[derive_message_formats]
    fn message(&self) -> String {
        "1 blank line required before class docstring".to_string()
    }

    fn fix_title(&self) -> String {
        "Insert 1 blank line before class docstring".to_string()
    }
}

/// ## What it does
/// Checks for class methods that are not separated from the class's docstring
/// by a blank line.
///
/// ## Why is this bad?
/// [PEP 257] recommends the use of a blank line to separate a class's
/// docstring from its methods.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is enabled when using the `numpy` and `pep257`
/// conventions, and disabled when using the `google` convention.
///
/// ## Example
/// ```python
/// class PhotoMetadata:
///     """Metadata about a photo."""
///     def __init__(self, file: Path):
///         ...
/// ```
///
/// Use instead:
/// ```python
/// class PhotoMetadata:
///     """Metadata about a photo."""
///
///     def __init__(self, file: Path):
///         ...
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
///
/// [PEP 257]: https://peps.python.org/pep-0257/
#[derive(ViolationMetadata)]
pub struct IncorrectBlankLineAfterClass;

impl AlwaysFixableViolation for IncorrectBlankLineAfterClass {
    #[derive_message_formats]
    fn message(&self) -> String {
        "1 blank line required after class docstring".to_string()
    }

    fn fix_title(&self) -> String {
        "Insert 1 blank line after class docstring".to_string()
    }
}

/// ## What it does
/// Checks for docstrings on class definitions that are preceded by a blank
/// line.
///
/// ## Why is this bad?
/// Avoid introducing any blank lines between a class definition and its
/// docstring, for consistency.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is enabled when using the `google`,
/// `numpy`, and `pep257` conventions.
///
/// For an alternative, see [D203].
///
/// ## Example
///
/// ```python
/// class PhotoMetadata:
///
///     """Metadata about a photo."""
/// ```
///
/// Use instead:
///
/// ```python
/// class PhotoMetadata:
///     """Metadata about a photo."""
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// [D203]: https://docs.astral.sh/ruff/rules/incorrect-blank-line-before-class
#[derive(ViolationMetadata)]
pub struct BlankLineBeforeClass;

impl AlwaysFixableViolation for BlankLineBeforeClass {
    #[derive_message_formats]
    fn message(&self) -> String {
        "No blank lines allowed before class docstring".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove blank line(s) before class docstring".to_string()
    }
}
