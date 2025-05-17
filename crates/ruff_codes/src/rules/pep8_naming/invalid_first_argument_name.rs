use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for instance methods that use a name other than `self` for their
/// first argument.
///
/// ## Why is this bad?
/// [PEP 8] recommends the use of `self` as first argument for all instance
/// methods:
///
/// > Always use self for the first argument to instance methods.
/// >
/// > If a function argument’s name clashes with a reserved keyword, it is generally better to
/// > append a single trailing underscore rather than use an abbreviation or spelling corruption.
/// > Thus `class_` is better than `clss`. (Perhaps better is to avoid such clashes by using a synonym.)
///
/// Names can be excluded from this rule using the [`lint.pep8-naming.ignore-names`]
/// or [`lint.pep8-naming.extend-ignore-names`] configuration options. For example,
/// to allow the use of `this` as the first argument to instance methods, set
/// the [`lint.pep8-naming.extend-ignore-names`] option to `["this"]`.
///
/// ## Example
///
/// ```python
/// class Example:
///     def function(cls, data): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Example:
///     def function(self, data): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as renaming a method parameter
/// can change the behavior of the program.
///
/// ## Options
/// - `lint.pep8-naming.classmethod-decorators`
/// - `lint.pep8-naming.staticmethod-decorators`
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-method-arguments
#[derive(ViolationMetadata)]
pub struct InvalidFirstArgumentNameForMethod {
    argument_name: String,
}

impl Violation for InvalidFirstArgumentNameForMethod {
    const FIX_AVAILABILITY: ruff_diagnostics::FixAvailability =
        ruff_diagnostics::FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "First argument of a method should be named `self`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        let Self { argument_name } = self;
        Some(format!("Rename `{argument_name}` to `self`"))
    }
}

/// ## What it does
/// Checks for class methods that use a name other than `cls` for their
/// first argument.
///
/// The method `__new__` is exempted from this
/// check and the corresponding violation is caught by
/// [`bad-staticmethod-argument`][PLW0211].
///
/// ## Why is this bad?
/// [PEP 8] recommends the use of `cls` as the first argument for all class
/// methods:
///
/// > Always use `cls` for the first argument to class methods.
/// >
/// > If a function argument’s name clashes with a reserved keyword, it is generally better to
/// > append a single trailing underscore rather than use an abbreviation or spelling corruption.
/// > Thus `class_` is better than `clss`. (Perhaps better is to avoid such clashes by using a synonym.)
///
/// Names can be excluded from this rule using the [`lint.pep8-naming.ignore-names`]
/// or [`lint.pep8-naming.extend-ignore-names`] configuration options. For example,
/// to allow the use of `klass` as the first argument to class methods, set
/// the [`lint.pep8-naming.extend-ignore-names`] option to `["klass"]`.
///
/// ## Example
///
/// ```python
/// class Example:
///     @classmethod
///     def function(self, data): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Example:
///     @classmethod
///     def function(cls, data): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as renaming a method parameter
/// can change the behavior of the program.
///
/// ## Options
/// - `lint.pep8-naming.classmethod-decorators`
/// - `lint.pep8-naming.staticmethod-decorators`
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-method-arguments
/// [PLW0211]: https://docs.astral.sh/ruff/rules/bad-staticmethod-argument/
#[derive(ViolationMetadata)]
pub struct InvalidFirstArgumentNameForClassMethod {
    argument_name: String,
    // Whether the method is `__new__`
    is_new: bool,
}

impl Violation for InvalidFirstArgumentNameForClassMethod {
    const FIX_AVAILABILITY: ruff_diagnostics::FixAvailability =
        ruff_diagnostics::FixAvailability::Sometimes;

    #[derive_message_formats]
    // The first string below is what shows up in the documentation
    // in the rule table, and it is the more common case.
    #[expect(clippy::if_not_else)]
    fn message(&self) -> String {
        if !self.is_new {
            "First argument of a class method should be named `cls`".to_string()
        } else {
            "First argument of `__new__` method should be named `cls`".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Self { argument_name, .. } = self;
        Some(format!("Rename `{argument_name}` to `cls`"))
    }
}
