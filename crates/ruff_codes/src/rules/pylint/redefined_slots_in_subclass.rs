use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for a re-defined slot in a subclass.
///
/// ## Why is this bad?
/// If a class defines a slot also defined in a base class, the
/// instance variable defined by the base class slot is inaccessible
/// (except by retrieving its descriptor directly from the base class).
///
/// ## Example
/// ```python
/// class Base:
///     __slots__ = ("a", "b")
///
///
/// class Subclass(Base):
///     __slots__ = ("a", "d")  # slot "a" redefined
/// ```
///
/// Use instead:
/// ```python
/// class Base:
///     __slots__ = ("a", "b")
///
///
/// class Subclass(Base):
///     __slots__ = "d"
/// ```
#[derive(ViolationMetadata)]
pub struct RedefinedSlotsInSubclass {
    base: String,
    slot_name: String,
}

impl Violation for RedefinedSlotsInSubclass {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RedefinedSlotsInSubclass { base, slot_name } = self;
        format!("Slot `{slot_name}` redefined from base class `{base}`")
    }
}