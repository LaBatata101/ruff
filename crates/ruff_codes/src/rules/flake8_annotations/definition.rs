use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks that function arguments have type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the types of function arguments. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any provided arguments match expectation.
///
/// ## Example
///
/// ```python
/// def foo(x): ...
/// ```
///
/// Use instead:
///
/// ```python
/// def foo(x: int): ...
/// ```
///
/// ## Options
/// - `lint.flake8-annotations.suppress-dummy-args`
#[derive(ViolationMetadata)]
pub struct MissingTypeFunctionArgument {
    name: String,
}

impl Violation for MissingTypeFunctionArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Missing type annotation for function argument `{name}`")
    }
}

/// ## What it does
/// Checks that function `*args` arguments have type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the types of function arguments. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any provided arguments match expectation.
///
/// ## Example
///
/// ```python
/// def foo(*args): ...
/// ```
///
/// Use instead:
///
/// ```python
/// def foo(*args: int): ...
/// ```
///
/// ## Options
/// - `lint.flake8-annotations.suppress-dummy-args`
#[derive(ViolationMetadata)]
pub struct MissingTypeArgs {
    name: String,
}

impl Violation for MissingTypeArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Missing type annotation for `*{name}`")
    }
}

/// ## What it does
/// Checks that function `**kwargs` arguments have type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the types of function arguments. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any provided arguments match expectation.
///
/// ## Example
///
/// ```python
/// def foo(**kwargs): ...
/// ```
///
/// Use instead:
///
/// ```python
/// def foo(**kwargs: int): ...
/// ```
///
/// ## Options
/// - `lint.flake8-annotations.suppress-dummy-args`
#[derive(ViolationMetadata)]
pub struct MissingTypeKwargs {
    name: String,
}

impl Violation for MissingTypeKwargs {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Missing type annotation for `**{name}`")
    }
}

/// ## Removed
/// This rule has been removed because type checkers can infer this type without annotation.
///
/// ## What it does
/// Checks that instance method `self` arguments have type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the types of function arguments. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any provided arguments match expectation.
///
/// Note that many type checkers will infer the type of `self` automatically, so this
/// annotation is not strictly necessary.
///
/// ## Example
///
/// ```python
/// class Foo:
///     def bar(self): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Foo:
///     def bar(self: "Foo"): ...
/// ```
#[derive(ViolationMetadata)]
#[deprecated(note = "ANN101 has been removed")]
pub struct MissingTypeSelf;

#[expect(deprecated)]
impl Violation for MissingTypeSelf {
    fn message(&self) -> String {
        unreachable!("ANN101 has been removed");
    }

    fn message_formats() -> &'static [&'static str] {
        &["Missing type annotation for `{name}` in method"]
    }
}

/// ## Removed
/// This rule has been removed because type checkers can infer this type without annotation.
///
/// ## What it does
/// Checks that class method `cls` arguments have type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the types of function arguments. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any provided arguments match expectation.
///
/// Note that many type checkers will infer the type of `cls` automatically, so this
/// annotation is not strictly necessary.
///
/// ## Example
///
/// ```python
/// class Foo:
///     @classmethod
///     def bar(cls): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Foo:
///     @classmethod
///     def bar(cls: Type["Foo"]): ...
/// ```
#[derive(ViolationMetadata)]
#[deprecated(note = "ANN102 has been removed")]
pub struct MissingTypeCls;

#[expect(deprecated)]
impl Violation for MissingTypeCls {
    fn message(&self) -> String {
        unreachable!("ANN102 has been removed")
    }

    fn message_formats() -> &'static [&'static str] {
        &["Missing type annotation for `{name}` in classmethod"]
    }
}

/// ## What it does
/// Checks that public functions and methods have return type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the return types of functions. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any returned values, and the types expected by callers, match expectation.
///
/// ## Example
/// ```python
/// def add(a, b):
///     return a + b
/// ```
///
/// Use instead:
/// ```python
/// def add(a: int, b: int) -> int:
///     return a + b
/// ```
///
/// ## Availability
///
/// Because this rule relies on the third-party `typing_extensions` module for some Python versions,
/// its diagnostic will not be emitted, and no fix will be offered, if `typing_extensions` imports
/// have been disabled by the [`lint.typing-extensions`] linter option.
///
/// ## Options
///
/// - `lint.typing-extensions`
#[derive(ViolationMetadata)]
pub struct MissingReturnTypeUndocumentedPublicFunction {
    name: String,
    annotation: Option<String>,
}

impl Violation for MissingReturnTypeUndocumentedPublicFunction {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name, .. } = self;
        format!("Missing return type annotation for public function `{name}`")
    }

    fn fix_title(&self) -> Option<String> {
        let title = match &self.annotation {
            Some(annotation) => format!("Add return type annotation: `{annotation}`"),
            None => "Add return type annotation".to_string(),
        };
        Some(title)
    }
}

/// ## What it does
/// Checks that private functions and methods have return type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the return types of functions. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any returned values, and the types expected by callers, match expectation.
///
/// ## Example
/// ```python
/// def _add(a, b):
///     return a + b
/// ```
///
/// Use instead:
/// ```python
/// def _add(a: int, b: int) -> int:
///     return a + b
/// ```
///
/// ## Availability
///
/// Because this rule relies on the third-party `typing_extensions` module for some Python versions,
/// its diagnostic will not be emitted, and no fix will be offered, if `typing_extensions` imports
/// have been disabled by the [`lint.typing-extensions`] linter option.
///
/// ## Options
///
/// - `lint.typing-extensions`
#[derive(ViolationMetadata)]
pub struct MissingReturnTypePrivateFunction {
    name: String,
    annotation: Option<String>,
}

impl Violation for MissingReturnTypePrivateFunction {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name, .. } = self;
        format!("Missing return type annotation for private function `{name}`")
    }

    fn fix_title(&self) -> Option<String> {
        let title = match &self.annotation {
            Some(annotation) => format!("Add return type annotation: `{annotation}`"),
            None => "Add return type annotation".to_string(),
        };
        Some(title)
    }
}

/// ## What it does
/// Checks that "special" methods, like `__init__`, `__new__`, and `__call__`, have
/// return type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the return types of functions. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any returned values, and the types expected by callers, match expectation.
///
/// Note that type checkers often allow you to omit the return type annotation for
/// `__init__` methods, as long as at least one argument has a type annotation. To
/// opt in to this behavior, use the `mypy-init-return` setting in your `pyproject.toml`
/// or `ruff.toml` file:
///
/// ```toml
/// [tool.ruff.lint.flake8-annotations]
/// mypy-init-return = true
/// ```
///
/// ## Example
/// ```python
/// class Foo:
///     def __init__(self, x: int):
///         self.x = x
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def __init__(self, x: int) -> None:
///         self.x = x
/// ```
#[derive(ViolationMetadata)]
pub struct MissingReturnTypeSpecialMethod {
    name: String,
    annotation: Option<String>,
}

impl Violation for MissingReturnTypeSpecialMethod {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name, .. } = self;
        format!("Missing return type annotation for special method `{name}`")
    }

    fn fix_title(&self) -> Option<String> {
        let title = match &self.annotation {
            Some(annotation) => format!("Add return type annotation: `{annotation}`"),
            None => "Add return type annotation".to_string(),
        };
        Some(title)
    }
}

/// ## What it does
/// Checks that static methods have return type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the return types of functions. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any returned values, and the types expected by callers, match expectation.
///
/// ## Example
/// ```python
/// class Foo:
///     @staticmethod
///     def bar():
///         return 1
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     @staticmethod
///     def bar() -> int:
///         return 1
/// ```
#[derive(ViolationMetadata)]
pub struct MissingReturnTypeStaticMethod {
    name: String,
    annotation: Option<String>,
}

impl Violation for MissingReturnTypeStaticMethod {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name, .. } = self;
        format!("Missing return type annotation for staticmethod `{name}`")
    }

    fn fix_title(&self) -> Option<String> {
        let title = match &self.annotation {
            Some(annotation) => format!("Add return type annotation: `{annotation}`"),
            None => "Add return type annotation".to_string(),
        };
        Some(title)
    }
}

/// ## What it does
/// Checks that class methods have return type annotations.
///
/// ## Why is this bad?
/// Type annotations are a good way to document the return types of functions. They also
/// help catch bugs, when used alongside a type checker, by ensuring that the types of
/// any returned values, and the types expected by callers, match expectation.
///
/// ## Example
/// ```python
/// class Foo:
///     @classmethod
///     def bar(cls):
///         return 1
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     @classmethod
///     def bar(cls) -> int:
///         return 1
/// ```
#[derive(ViolationMetadata)]
pub struct MissingReturnTypeClassMethod {
    name: String,
    annotation: Option<String>,
}

impl Violation for MissingReturnTypeClassMethod {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name, .. } = self;
        format!("Missing return type annotation for classmethod `{name}`")
    }

    fn fix_title(&self) -> Option<String> {
        let title = match &self.annotation {
            Some(annotation) => format!("Add return type annotation: `{annotation}`"),
            None => "Add return type annotation".to_string(),
        };
        Some(title)
    }
}

/// ## What it does
/// Checks that function arguments are annotated with a more specific type than
/// `Any`.
///
/// ## Why is this bad?
/// `Any` is a special type indicating an unconstrained type. When an
/// expression is annotated with type `Any`, type checkers will allow all
/// operations on it.
///
/// It's better to be explicit about the type of an expression, and to use
/// `Any` as an "escape hatch" only when it is really needed.
///
/// ## Example
///
/// ```python
/// def foo(x: Any): ...
/// ```
///
/// Use instead:
///
/// ```python
/// def foo(x: int): ...
/// ```
///
/// ## Known problems
///
/// Type aliases are unsupported and can lead to false positives.
/// For example, the following will trigger this rule inadvertently:
///
/// ```python
/// from typing import Any
///
/// MyAny = Any
///
///
/// def foo(x: MyAny): ...
/// ```
///
/// ## References
/// - [Typing spec: `Any`](https://typing.python.org/en/latest/spec/special-types.html#any)
/// - [Python documentation: `typing.Any`](https://docs.python.org/3/library/typing.html#typing.Any)
/// - [Mypy documentation: The Any type](https://mypy.readthedocs.io/en/stable/kinds_of_types.html#the-any-type)
#[derive(ViolationMetadata)]
pub struct AnyType {
    name: String,
}

impl Violation for AnyType {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Dynamically typed expressions (typing.Any) are disallowed in `{name}`")
    }
}
