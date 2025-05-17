use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `functools.lru_cache` that set `maxsize=None`.
///
/// ## Why is this bad?
/// Since Python 3.9, `functools.cache` can be used as a drop-in replacement
/// for `functools.lru_cache(maxsize=None)`. When possible, prefer
/// `functools.cache` as it is more readable and idiomatic.
///
/// ## Example
///
/// ```python
/// import functools
///
///
/// @functools.lru_cache(maxsize=None)
/// def foo(): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import functools
///
///
/// @functools.cache
/// def foo(): ...
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `@functools.cache`](https://docs.python.org/3/library/functools.html#functools.cache)
#[derive(ViolationMetadata)]
pub struct LRUCacheWithMaxsizeNone;

impl AlwaysFixableViolation for LRUCacheWithMaxsizeNone {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `@functools.cache` instead of `@functools.lru_cache(maxsize=None)`".to_string()
    }

    fn fix_title(&self) -> String {
        "Rewrite with `@functools.cache".to_string()
    }
}