use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of the `from __future__ import annotations` import
/// statement in stub files.
///
/// ## Why is this bad?
/// Stub files natively support forward references in all contexts, as stubs are
/// never executed at runtime. (They should be thought of as "data files" for
/// type checkers.) As such, the `from __future__ import annotations` import
/// statement has no effect and should be omitted.
///
/// ## References
/// - [Static Typing with Python: Type Stubs](https://typing.python.org/en/latest/source/stubs.html)
#[derive(ViolationMetadata)]
pub struct FutureAnnotationsInStub;

impl Violation for FutureAnnotationsInStub {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`from __future__ import annotations` has no effect in stub files, since type checkers automatically treat stubs as having those semantics".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove `from __future__ import annotations`".to_string())
    }
}