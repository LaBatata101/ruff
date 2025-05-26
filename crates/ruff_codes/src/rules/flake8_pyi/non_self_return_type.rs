use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for methods that are annotated with a fixed return type which
/// should instead be returning `Self`.
///
/// ## Why is this bad?
/// If methods that generally return `self` at runtime are annotated with a
/// fixed return type, and the class is subclassed, type checkers will not be
/// able to infer the correct return type.
///
/// For example:
/// ```python
/// class Shape:
///     def set_scale(self, scale: float) -> Shape:
///         self.scale = scale
///         return self
///
/// class Circle(Shape):
///     def set_radius(self, radius: float) -> Circle:
///         self.radius = radius
///         return self
///
/// # Type checker infers return type as `Shape`, not `Circle`.
/// Circle().set_scale(0.5)
///
/// # Thus, this expression is invalid, as `Shape` has no attribute `set_radius`.
/// Circle().set_scale(0.5).set_radius(2.7)
/// ```
///
/// Specifically, this check enforces that the return type of the following
/// methods is `Self`:
///
/// 1. In-place binary-operation dunder methods, like `__iadd__`, `__imul__`, etc.
/// 1. `__new__`, `__enter__`, and `__aenter__`, if those methods return the
///    class name.
/// 1. `__iter__` methods that return `Iterator`, despite the class inheriting
///    directly from `Iterator`.
/// 1. `__aiter__` methods that return `AsyncIterator`, despite the class
///    inheriting directly from `AsyncIterator`.
///
/// ## Example
///
/// ```pyi
/// class Foo:
///     def __new__(cls, *args: Any, **kwargs: Any) -> Foo: ...
///     def __enter__(self) -> Foo: ...
///     async def __aenter__(self) -> Foo: ...
///     def __iadd__(self, other: Foo) -> Foo: ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// from typing_extensions import Self
///
/// class Foo:
///     def __new__(cls, *args: Any, **kwargs: Any) -> Self: ...
///     def __enter__(self) -> Self: ...
///     async def __aenter__(self) -> Self: ...
///     def __iadd__(self, other: Foo) -> Self: ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe as it changes the meaning of your type annotations.
///
/// ## Availability
///
/// Because this rule relies on the third-party `typing_extensions` module for Python versions
/// before 3.11, its diagnostic will not be emitted, and no fix will be offered, if
/// `typing_extensions` imports have been disabled by the [`lint.typing-extensions`] linter option.
///
/// ## Options
///
/// - `lint.typing-extensions`
///
/// ## References
/// - [Python documentation: `typing.Self`](https://docs.python.org/3/library/typing.html#typing.Self)
#[derive(ViolationMetadata)]
pub struct NonSelfReturnType {
    class_name: String,
    method_name: String,
}

impl Violation for NonSelfReturnType {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let NonSelfReturnType {
            class_name,
            method_name,
        } = self;

        if matches!(class_name.as_str(), "__new__") {
            "`__new__` methods usually return `self` at runtime".to_string()
        } else {
            format!("`{method_name}` methods in classes like `{class_name}` usually return `self` at runtime")
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use `Self` as return type".to_string())
    }
}