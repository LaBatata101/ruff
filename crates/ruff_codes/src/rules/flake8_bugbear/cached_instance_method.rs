use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the `functools.lru_cache` and `functools.cache`
/// decorators on methods.
///
/// ## Why is this bad?
/// Using the `functools.lru_cache` and `functools.cache` decorators on methods
/// can lead to memory leaks, as the global cache will retain a reference to
/// the instance, preventing it from being garbage collected.
///
/// Instead, refactor the method to depend only on its arguments and not on the
/// instance of the class, or use the `@lru_cache` decorator on a function
/// outside of the class.
///
/// This rule ignores instance methods on enumeration classes, as enum members
/// are singletons.
///
/// ## Example
/// ```python
/// from functools import lru_cache
///
///
/// def square(x: int) -> int:
///     return x * x
///
///
/// class Number:
///     value: int
///
///     @lru_cache
///     def squared(self):
///         return square(self.value)
/// ```
///
/// Use instead:
/// ```python
/// from functools import lru_cache
///
///
/// @lru_cache
/// def square(x: int) -> int:
///     return x * x
///
///
/// class Number:
///     value: int
///
///     def squared(self):
///         return square(self.value)
/// ```
///
/// ## References
/// - [Python documentation: `functools.lru_cache`](https://docs.python.org/3/library/functools.html#functools.lru_cache)
/// - [Python documentation: `functools.cache`](https://docs.python.org/3/library/functools.html#functools.cache)
/// - [don't lru_cache methods!](https://www.youtube.com/watch?v=sVjtp6tGo0g)
#[derive(ViolationMetadata)]
pub struct CachedInstanceMethod;

impl Violation for CachedInstanceMethod {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of `functools.lru_cache` or `functools.cache` on methods can lead to memory leaks"
            .to_string()
    }
}