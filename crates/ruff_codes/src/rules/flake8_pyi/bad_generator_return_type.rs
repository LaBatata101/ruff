use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for simple `__iter__` methods that return `Generator`, and for
/// simple `__aiter__` methods that return `AsyncGenerator`.
///
/// ## Why is this bad?
/// Using `(Async)Iterator` for these methods is simpler and more elegant. More
/// importantly, it also reflects the fact that the precise kind of iterator
/// returned from an `__iter__` method is usually an implementation detail that
/// could change at any time. Type annotations help define a contract for a
/// function; implementation details should not leak into that contract.
///
/// For example:
/// ```python
/// from collections.abc import AsyncGenerator, Generator
/// from typing import Any
///
///
/// class CustomIterator:
///     def __iter__(self) -> Generator:
///         yield from range(42)
///
///
/// class CustomIterator2:
///     def __iter__(self) -> Generator[str, Any, None]:
///         yield from "abcdefg"
/// ```
///
/// Use instead:
/// ```python
/// from collections.abc import Iterator
///
///
/// class CustomIterator:
///     def __iter__(self) -> Iterator:
///         yield from range(42)
///
///
/// class CustomIterator2:
///     def __iter__(self) -> Iterator[str]:
///         yield from "abdefg"
/// ```
///
/// ## Fix safety
/// This rule tries hard to avoid false-positive errors, and the rule's fix
/// should always be safe for `.pyi` stub files. However, there is a slightly
/// higher chance that a false positive might be emitted by this rule when
/// applied to runtime Python (`.py` files). As such, the fix is marked as
/// unsafe for any `__iter__` or `__aiter__` method in a `.py` file that has
/// more than two statements (including docstrings) in its body.
#[derive(ViolationMetadata)]
pub struct GeneratorReturnFromIterMethod {
    return_type: Iterator,
    method: Method,
}

impl Violation for GeneratorReturnFromIterMethod {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let GeneratorReturnFromIterMethod {
            return_type,
            method,
        } = self;
        format!("Use `{return_type}` as the return value for simple `{method}` methods")
    }

    fn fix_title(&self) -> Option<String> {
        let GeneratorReturnFromIterMethod {
            return_type,
            method,
        } = self;
        Some(format!(
            "Convert the return annotation of your `{method}` method to `{return_type}`"
        ))
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::bad_generator_return_type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Iterator {
    Iterator,
    AsyncIterator,
}

impl std::fmt::Display for Iterator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Iterator::Iterator => write!(f, "Iterator"),
            Iterator::AsyncIterator => write!(f, "AsyncIterator"),
        }
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::bad_generator_return_type
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Method {
    Iter,
    AIter,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Iter => write!(f, "__iter__"),
            Method::AIter => write!(f, "__aiter__"),
        }
    }
}