use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `Unpack[]` on Python 3.11 and above, and suggests
/// using `*` instead.
///
/// ## Why is this bad?
/// [PEP 646] introduced a new syntax for unpacking sequences based on the `*`
/// operator. This syntax is more concise and readable than the previous
/// `Unpack[]` syntax.
///
/// ## Example
/// ```python
/// from typing import Unpack
///
///
/// def foo(*args: Unpack[tuple[int, ...]]) -> None:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// def foo(*args: *tuple[int, ...]) -> None:
///     pass
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as `Unpack[T]` and `*T` are considered
/// different values when introspecting types at runtime. However, in most cases,
/// the fix should be safe to apply.
///
/// [PEP 646]: https://peps.python.org/pep-0646/
#[derive(ViolationMetadata)]
pub struct NonPEP646Unpack;

impl Violation for NonPEP646Unpack {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Always;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `*` for unpacking".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Convert to `*` for unpacking".to_string())
    }
}