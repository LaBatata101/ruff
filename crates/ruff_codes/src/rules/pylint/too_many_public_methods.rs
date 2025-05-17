use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for classes with too many public methods
///
/// By default, this rule allows up to 20 public methods, as configured by
/// the [`lint.pylint.max-public-methods`] option.
///
/// ## Why is this bad?
/// Classes with many public methods are harder to understand
/// and maintain.
///
/// Instead, consider refactoring the class into separate classes.
///
/// ## Example
/// Assuming that `lint.pylint.max-public-methods` is set to 5:
/// ```python
/// class Linter:
///     def __init__(self):
///         pass
///
///     def pylint(self):
///         pass
///
///     def pylint_settings(self):
///         pass
///
///     def flake8(self):
///         pass
///
///     def flake8_settings(self):
///         pass
///
///     def pydocstyle(self):
///         pass
///
///     def pydocstyle_settings(self):
///         pass
/// ```
///
/// Use instead:
/// ```python
/// class Linter:
///     def __init__(self):
///         self.pylint = Pylint()
///         self.flake8 = Flake8()
///         self.pydocstyle = Pydocstyle()
///
///     def lint(self):
///         pass
///
///
/// class Pylint:
///     def lint(self):
///         pass
///
///     def settings(self):
///         pass
///
///
/// class Flake8:
///     def lint(self):
///         pass
///
///     def settings(self):
///         pass
///
///
/// class Pydocstyle:
///     def lint(self):
///         pass
///
///     def settings(self):
///         pass
/// ```
///
/// ## Options
/// - `lint.pylint.max-public-methods`
#[derive(ViolationMetadata)]
pub struct TooManyPublicMethods {
    methods: usize,
    max_methods: usize,
}

impl Violation for TooManyPublicMethods {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyPublicMethods {
            methods,
            max_methods,
        } = self;
        format!("Too many public methods ({methods} > {max_methods})")
    }
}