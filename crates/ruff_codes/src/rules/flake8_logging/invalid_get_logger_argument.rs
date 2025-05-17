use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for any usage of `__cached__` and `__file__` as an argument to
/// `logging.getLogger()`.
///
/// ## Why is this bad?
/// The [logging documentation] recommends this pattern:
///
/// ```python
/// logging.getLogger(__name__)
/// ```
///
/// Here, `__name__` is the fully qualified module name, such as `foo.bar`,
/// which is the intended format for logger names.
///
/// This rule detects probably-mistaken usage of similar module-level dunder constants:
///
/// * `__cached__` - the pathname of the module's compiled version, such as `foo/__pycache__/bar.cpython-311.pyc`.
/// * `__file__` - the pathname of the module, such as `foo/bar.py`.
///
/// ## Example
/// ```python
/// import logging
///
/// logger = logging.getLogger(__file__)
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
/// logger = logging.getLogger(__name__)
/// ```
///
/// [logging documentation]: https://docs.python.org/3/library/logging.html#logger-objects
#[derive(ViolationMetadata)]
pub struct InvalidGetLoggerArgument;

impl Violation for InvalidGetLoggerArgument {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `__name__` with `logging.getLogger()`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `__name__`".to_string())
    }
}