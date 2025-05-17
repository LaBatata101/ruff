use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of mutable objects as `ContextVar` defaults.
///
/// ## Why is this bad?
///
/// The `ContextVar` default is evaluated once, when the `ContextVar` is defined.
///
/// The same mutable object is then shared across all `.get()` method calls to
/// the `ContextVar`. If the object is modified, those modifications will persist
/// across calls, which can lead to unexpected behavior.
///
/// Instead, prefer to use immutable data structures. Alternatively, take
/// `None` as a default, and initialize a new mutable object inside for each
/// call using the `.set()` method.
///
/// Types outside the standard library can be marked as immutable with the
/// [`lint.flake8-bugbear.extend-immutable-calls`] configuration option.
///
/// ## Example
/// ```python
/// from contextvars import ContextVar
///
///
/// cv: ContextVar[list] = ContextVar("cv", default=[])
/// ```
///
/// Use instead:
/// ```python
/// from contextvars import ContextVar
///
///
/// cv: ContextVar[list | None] = ContextVar("cv", default=None)
///
/// ...
///
/// if cv.get() is None:
///     cv.set([])
/// ```
///
/// ## Options
/// - `lint.flake8-bugbear.extend-immutable-calls`
///
/// ## References
/// - [Python documentation: `contextvars` — Context Variables](https://docs.python.org/3/library/contextvars.html)
#[derive(ViolationMetadata)]
pub struct MutableContextvarDefault;

impl Violation for MutableContextvarDefault {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not use mutable data structures for `ContextVar` defaults".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `None`; initialize with `.set()``".to_string())
    }
}