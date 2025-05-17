use ruff_diagnostics::{AlwaysFixableViolation, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::TypingModule;

/// ## What it does
/// Checks for typed function arguments in stubs with complex default values.
///
/// ## Why is this bad?
/// Stub (`.pyi`) files exist as "data files" for static analysis tools, and
/// are not evaluated at runtime. While simple default values may be useful for
/// some tools that consume stubs, such as IDEs, they are ignored by type
/// checkers.
///
/// Instead of including and reproducing a complex value, use `...` to indicate
/// that the assignment has a default value, but that the value is "complex" or
/// varies according to the current platform or Python version. For the
/// purposes of this rule, any default value counts as "complex" unless it is
/// a literal `int`, `float`, `complex`, `bytes`, `str`, `bool`, `None`, `...`,
/// or a simple container literal.
///
/// ## Example
///
/// ```pyi
/// def foo(arg: list[int] = list(range(10_000))) -> None: ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// def foo(arg: list[int] = ...) -> None: ...
/// ```
///
/// ## References
/// - [`flake8-pyi`](https://github.com/PyCQA/flake8-pyi/blob/main/ERRORCODES.md)
#[derive(ViolationMetadata)]
pub struct TypedArgumentDefaultInStub;

impl AlwaysFixableViolation for TypedArgumentDefaultInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Only simple default values allowed for typed arguments".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace default value with `...`".to_string()
    }
}

/// ## What it does
/// Checks for untyped function arguments in stubs with default values that
/// are not "simple" /// (i.e., `int`, `float`, `complex`, `bytes`, `str`,
/// `bool`, `None`, `...`, or simple container literals).
///
/// ## Why is this bad?
/// Stub (`.pyi`) files exist to define type hints, and are not evaluated at
/// runtime. As such, function arguments in stub files should not have default
/// values, as they are ignored by type checkers.
///
/// However, the use of default values may be useful for IDEs and other
/// consumers of stub files, and so "simple" values may be worth including and
/// are permitted by this rule.
///
/// Instead of including and reproducing a complex value, use `...` to indicate
/// that the assignment has a default value, but that the value is non-simple
/// or varies according to the current platform or Python version.
///
/// ## Example
///
/// ```pyi
/// def foo(arg=[]) -> None: ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// def foo(arg=...) -> None: ...
/// ```
///
/// ## References
/// - [`flake8-pyi`](https://github.com/PyCQA/flake8-pyi/blob/main/ERRORCODES.md)
#[derive(ViolationMetadata)]
pub struct ArgumentDefaultInStub;

impl AlwaysFixableViolation for ArgumentDefaultInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Only simple default values allowed for arguments".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace default value with `...`".to_string()
    }
}

/// ## What it does
/// Checks for assignments in stubs with default values that are not "simple"
/// (i.e., `int`, `float`, `complex`, `bytes`, `str`, `bool`, `None`, `...`, or
/// simple container literals).
///
/// ## Why is this bad?
/// Stub (`.pyi`) files exist to define type hints, and are not evaluated at
/// runtime. As such, assignments in stub files should not include values,
/// as they are ignored by type checkers.
///
/// However, the use of such values may be useful for IDEs and other consumers
/// of stub files, and so "simple" values may be worth including and are
/// permitted by this rule.
///
/// Instead of including and reproducing a complex value, use `...` to indicate
/// that the assignment has a default value, but that the value is non-simple
/// or varies according to the current platform or Python version.
///
/// ## Example
/// ```pyi
/// foo: str = "..."
/// ```
///
/// Use instead:
/// ```pyi
/// foo: str = ...
/// ```
///
/// ## References
/// - [`flake8-pyi`](https://github.com/PyCQA/flake8-pyi/blob/main/ERRORCODES.md)
#[derive(ViolationMetadata)]
pub struct AssignmentDefaultInStub;

impl AlwaysFixableViolation for AssignmentDefaultInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Only simple default values allowed for assignments".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace default value with `...`".to_string()
    }
}

/// ## What it does
/// Checks for unannotated assignments in stub (`.pyi`) files.
///
/// ## Why is this bad?
/// Stub files exist to provide type hints, and are never executed. As such,
/// all assignments in stub files should be annotated with a type.
#[derive(ViolationMetadata)]
pub struct UnannotatedAssignmentInStub {
    name: String,
}

impl Violation for UnannotatedAssignmentInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnannotatedAssignmentInStub { name } = self;
        format!("Need type annotation for `{name}`")
    }
}

/// ## What it does
/// Checks that `__all__`, `__match_args__`, and `__slots__` variables are
/// assigned to values when defined in stub files.
///
/// ## Why is this bad?
/// Special variables like `__all__` have the same semantics in stub files
/// as they do in Python modules, and so should be consistent with their
/// runtime counterparts.
///
/// ## Example
/// ```pyi
/// __all__: list[str]
/// ```
///
/// Use instead:
/// ```pyi
/// __all__: list[str] = ["foo", "bar"]
/// ```
#[derive(ViolationMetadata)]
pub struct UnassignedSpecialVariableInStub {
    name: String,
}

impl Violation for UnassignedSpecialVariableInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnassignedSpecialVariableInStub { name } = self;
        format!("`{name}` in a stub file must have a value, as it has the same semantics as `{name}` at runtime")
    }
}

/// ## What it does
/// Checks for type alias definitions that are not annotated with
/// `typing.TypeAlias`.
///
/// ## Why is this bad?
/// In Python, a type alias is defined by assigning a type to a variable (e.g.,
/// `Vector = list[float]`).
///
/// It's best to annotate type aliases with the `typing.TypeAlias` type to
/// make it clear that the statement is a type alias declaration, as opposed
/// to a normal variable assignment.
///
/// ## Example
/// ```pyi
/// Vector = list[float]
/// ```
///
/// Use instead:
/// ```pyi
/// from typing import TypeAlias
///
/// Vector: TypeAlias = list[float]
/// ```
///
/// ## Availability
///
/// Because this rule relies on the third-party `typing_extensions` module for Python versions
/// before 3.10, its diagnostic will not be emitted, and no fix will be offered, if
/// `typing_extensions` imports have been disabled by the [`lint.typing-extensions`] linter option.
///
/// ## Options
///
/// - `lint.typing-extensions`
#[derive(ViolationMetadata)]
pub struct TypeAliasWithoutAnnotation {
    module: TypingModule,
    name: String,
    value: String,
}

impl AlwaysFixableViolation for TypeAliasWithoutAnnotation {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TypeAliasWithoutAnnotation {
            module,
            name,
            value,
        } = self;
        format!("Use `{module}.TypeAlias` for type alias, e.g., `{name}: TypeAlias = {value}`")
    }

    fn fix_title(&self) -> String {
        "Add `TypeAlias` annotation".to_string()
    }
}
