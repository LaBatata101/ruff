use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for abstract classes without abstract methods or properties.
/// Annotated but unassigned class variables are regarded as abstract.
///
/// ## Why is this bad?
/// Abstract base classes are used to define interfaces. If an abstract base
/// class has no abstract methods or properties, you may have forgotten
/// to add an abstract method or property to the class,
/// or omitted an `@abstractmethod` decorator.
///
/// If the class is _not_ meant to be used as an interface, consider removing
/// the `ABC` base class from the class definition.
///
/// ## Example
/// ```python
/// from abc import ABC
/// from typing import ClassVar
///
///
/// class Foo(ABC):
///     class_var: ClassVar[str] = "assigned"
///
///     def method(self):
///         bar()
/// ```
///
/// Use instead:
/// ```python
/// from abc import ABC, abstractmethod
/// from typing import ClassVar
///
///
/// class Foo(ABC):
///     class_var: ClassVar[str]  # unassigned
///
///     @abstractmethod
///     def method(self):
///         bar()
/// ```
///
/// ## References
/// - [Python documentation: `abc`](https://docs.python.org/3/library/abc.html)
/// - [Python documentation: `typing.ClassVar`](https://docs.python.org/3/library/typing.html#typing.ClassVar)
#[derive(ViolationMetadata)]
pub struct AbstractBaseClassWithoutAbstractMethod {
    name: String,
}

impl Violation for AbstractBaseClassWithoutAbstractMethod {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AbstractBaseClassWithoutAbstractMethod { name } = self;
        format!("`{name}` is an abstract base class, but it has no abstract methods or properties")
    }
}

/// ## What it does
/// Checks for empty methods in abstract base classes without an abstract
/// decorator.
///
/// ## Why is this bad?
/// Empty methods in abstract base classes without an abstract decorator may be
/// be indicative of a mistake. If the method is meant to be abstract, add an
/// `@abstractmethod` decorator to the method.
///
/// ## Example
///
/// ```python
/// from abc import ABC
///
///
/// class Foo(ABC):
///     def method(self): ...
/// ```
///
/// Use instead:
///
/// ```python
/// from abc import ABC, abstractmethod
///
///
/// class Foo(ABC):
///     @abstractmethod
///     def method(self): ...
/// ```
///
/// ## References
/// - [Python documentation: `abc`](https://docs.python.org/3/library/abc.html)
#[derive(ViolationMetadata)]
pub struct EmptyMethodWithoutAbstractDecorator {
    name: String,
}

impl Violation for EmptyMethodWithoutAbstractDecorator {
    #[derive_message_formats]
    fn message(&self) -> String {
        let EmptyMethodWithoutAbstractDecorator { name } = self;
        format!(
            "`{name}` is an empty method in an abstract base class, but has no abstract decorator"
        )
    }
}
