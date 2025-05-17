use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for calls of the form `map(int, __version__.split("."))`.
///
/// ## Why is this bad?
/// `__version__` does not always contain integral-like elements.
///
/// ```python
/// import matplotlib  # `__version__ == "3.9.1.post-1"` in our environment
///
/// # ValueError: invalid literal for int() with base 10: 'post1'
/// tuple(map(int, matplotlib.__version__.split(".")))
/// ```
///
/// See also [*Version specifiers* | Packaging spec][version-specifier].
///
/// ## Example
/// ```python
/// tuple(map(int, matplotlib.__version__.split(".")))
/// ```
///
/// Use instead:
/// ```python
/// import packaging.version as version
///
/// version.parse(matplotlib.__version__)
/// ```
///
/// [version-specifier]: https://packaging.python.org/en/latest/specifications/version-specifiers/#version-specifiers
#[derive(ViolationMetadata)]
pub struct MapIntVersionParsing;

impl Violation for MapIntVersionParsing {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__version__` may contain non-integral-like elements".to_string()
    }
}