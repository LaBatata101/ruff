use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::name::Name;


/// ## What it does
/// Checks for the presence of multiple `type`s in a union.
///
/// ## Why is this bad?
/// `type[T | S]` has identical semantics to `type[T] | type[S]` in a type
/// annotation, but is cleaner and more concise.
///
/// ## Example
/// ```pyi
/// field: type[int] | type[float] | str
/// ```
///
/// Use instead:
/// ```pyi
/// field: type[int | float] | str
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe, unless the type annotation contains comments.
///
/// Note that while the fix may flatten nested unions into a single top-level union,
/// the semantics of the annotation will remain unchanged.
#[derive(ViolationMetadata)]
pub struct UnnecessaryTypeUnion {
    members: Vec<Name>,
    union_kind: UnionKind,
}

impl Violation for UnnecessaryTypeUnion {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let union_str = match self.union_kind {
            UnionKind::PEP604 => self.members.join(" | "),
            UnionKind::TypingUnion => format!("Union[{}]", self.members.join(", ")),
        };

        format!(
            "Multiple `type` members in a union. Combine them into one, e.g., `type[{union_str}]`."
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Combine multiple `type` members".to_string())
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::unnecessary_type_union
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnionKind {
    /// E.g., `typing.Union[int, str]`
    TypingUnion,
    /// E.g., `int | str`
    PEP604,
}