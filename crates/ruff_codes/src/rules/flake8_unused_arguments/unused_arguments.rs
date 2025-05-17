use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of unused arguments in function definitions.
///
/// ## Why is this bad?
/// An argument that is defined but not used is likely a mistake, and should
/// be removed to avoid confusion.
///
/// If a variable is intentionally defined-but-not-used, it should be
/// prefixed with an underscore, or some other value that adheres to the
/// [`lint.dummy-variable-rgx`] pattern.
///
/// ## Example
/// ```python
/// def foo(bar, baz):
///     return bar * 2
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar):
///     return bar * 2
/// ```
///
/// ## Options
/// - `lint.dummy-variable-rgx`
#[derive(ViolationMetadata)]
pub struct UnusedFunctionArgument {
    name: String,
}

impl Violation for UnusedFunctionArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedFunctionArgument { name } = self;
        format!("Unused function argument: `{name}`")
    }
}

/// ## What it does
/// Checks for the presence of unused arguments in instance method definitions.
///
/// ## Why is this bad?
/// An argument that is defined but not used is likely a mistake, and should
/// be removed to avoid confusion.
///
/// If a variable is intentionally defined-but-not-used, it should be
/// prefixed with an underscore, or some other value that adheres to the
/// [`lint.dummy-variable-rgx`] pattern.
///
/// ## Example
/// ```python
/// class Class:
///     def foo(self, arg1, arg2):
///         print(arg1)
/// ```
///
/// Use instead:
/// ```python
/// class Class:
///     def foo(self, arg1):
///         print(arg1)
/// ```
///
/// ## Options
/// - `lint.dummy-variable-rgx`
#[derive(ViolationMetadata)]
pub struct UnusedMethodArgument {
    name: String,
}

impl Violation for UnusedMethodArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedMethodArgument { name } = self;
        format!("Unused method argument: `{name}`")
    }
}

/// ## What it does
/// Checks for the presence of unused arguments in class method definitions.
///
/// ## Why is this bad?
/// An argument that is defined but not used is likely a mistake, and should
/// be removed to avoid confusion.
///
/// If a variable is intentionally defined-but-not-used, it should be
/// prefixed with an underscore, or some other value that adheres to the
/// [`lint.dummy-variable-rgx`] pattern.
///
/// ## Example
/// ```python
/// class Class:
///     @classmethod
///     def foo(cls, arg1, arg2):
///         print(arg1)
/// ```
///
/// Use instead:
/// ```python
/// class Class:
///     @classmethod
///     def foo(cls, arg1):
///         print(arg1)
/// ```
///
/// ## Options
/// - `lint.dummy-variable-rgx`
#[derive(ViolationMetadata)]
pub struct UnusedClassMethodArgument {
    name: String,
}

impl Violation for UnusedClassMethodArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedClassMethodArgument { name } = self;
        format!("Unused class method argument: `{name}`")
    }
}

/// ## What it does
/// Checks for the presence of unused arguments in static method definitions.
///
/// ## Why is this bad?
/// An argument that is defined but not used is likely a mistake, and should
/// be removed to avoid confusion.
///
/// If a variable is intentionally defined-but-not-used, it should be
/// prefixed with an underscore, or some other value that adheres to the
/// [`lint.dummy-variable-rgx`] pattern.
///
/// ## Example
/// ```python
/// class Class:
///     @staticmethod
///     def foo(arg1, arg2):
///         print(arg1)
/// ```
///
/// Use instead:
/// ```python
/// class Class:
///     @staticmethod
///     def foo(arg1):
///         print(arg1)
/// ```
///
/// ## Options
/// - `lint.dummy-variable-rgx`
#[derive(ViolationMetadata)]
pub struct UnusedStaticMethodArgument {
    name: String,
}

impl Violation for UnusedStaticMethodArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedStaticMethodArgument { name } = self;
        format!("Unused static method argument: `{name}`")
    }
}

/// ## What it does
/// Checks for the presence of unused arguments in lambda expression
/// definitions.
///
/// ## Why is this bad?
/// An argument that is defined but not used is likely a mistake, and should
/// be removed to avoid confusion.
///
/// If a variable is intentionally defined-but-not-used, it should be
/// prefixed with an underscore, or some other value that adheres to the
/// [`lint.dummy-variable-rgx`] pattern.
///
/// ## Example
/// ```python
/// my_list = [1, 2, 3, 4, 5]
/// squares = map(lambda x, y: x**2, my_list)
/// ```
///
/// Use instead:
/// ```python
/// my_list = [1, 2, 3, 4, 5]
/// squares = map(lambda x: x**2, my_list)
/// ```
///
/// ## Options
/// - `lint.dummy-variable-rgx`
#[derive(ViolationMetadata)]
pub struct UnusedLambdaArgument {
    name: String,
}

impl Violation for UnusedLambdaArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedLambdaArgument { name } = self;
        format!("Unused lambda argument: `{name}`")
    }
}
