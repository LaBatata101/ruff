use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `typing.ByteString` or `collections.abc.ByteString`.
///
/// ## Why is this bad?
/// `ByteString` has been deprecated since Python 3.9 and will be removed in
/// Python 3.14. The Python documentation recommends using either
/// `collections.abc.Buffer` (or the `typing_extensions` backport
/// on Python <3.12) or a union like `bytes | bytearray | memoryview` instead.
///
/// ## Example
/// ```python
/// from typing import ByteString
/// ```
///
/// Use instead:
/// ```python
/// from collections.abc import Buffer
/// ```
///
/// ## References
/// - [Python documentation: The `ByteString` type](https://docs.python.org/3/library/typing.html#typing.ByteString)
#[derive(ViolationMetadata)]
pub struct ByteStringUsage {
    origin: ByteStringOrigin,
}

impl Violation for ByteStringUsage {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::None;

    #[derive_message_formats]
    fn message(&self) -> String {
        let ByteStringUsage { origin } = self;
        format!("Do not use `{origin}.ByteString`, which has unclear semantics and is deprecated")
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::bytestring_usage
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ByteStringOrigin {
    Typing,
    CollectionsAbc,
}

impl std::fmt::Display for ByteStringOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Typing => "typing",
            Self::CollectionsAbc => "collections.abc",
        })
    }
}