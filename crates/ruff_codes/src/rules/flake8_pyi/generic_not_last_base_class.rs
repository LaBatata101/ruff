use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for classes inheriting from `typing.Generic[]` where `Generic[]` is
/// not the last base class in the bases tuple.
///
/// ## Why is this bad?
/// If `Generic[]` is not the final class in the bases tuple, unexpected
/// behaviour can occur at runtime (See [this CPython issue][1] for an example).
/// The rule is also applied to stub files, but, unlike at runtime,
/// in stubs it is purely enforced for stylistic consistency.
///
/// For example:
/// ```python
/// class LinkedList(Generic[T], Sized):
///     def push(self, item: T) -> None:
///         self._items.append(item)
///
/// class MyMapping(
///     Generic[K, V],
///     Iterable[Tuple[K, V]],
///     Container[Tuple[K, V]],
/// ):
///     ...
/// ```
///
/// Use instead:
/// ```python
/// class LinkedList(Sized, Generic[T]):
///     def push(self, item: T) -> None:
///         self._items.append(item)
///
/// class MyMapping(
///     Iterable[Tuple[K, V]],
///     Container[Tuple[K, V]],
///     Generic[K, V],
/// ):
///     ...
/// ```
/// ## References
/// - [`typing.Generic` documentation](https://docs.python.org/3/library/typing.html#typing.Generic)
///
/// [1]: https://github.com/python/cpython/issues/106102
#[derive(ViolationMetadata)]
pub struct GenericNotLastBaseClass;

impl Violation for GenericNotLastBaseClass {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`Generic[]` should always be the last base class".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Move `Generic[]` to the end".to_string())
    }
}