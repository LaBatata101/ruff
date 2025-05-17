use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for parameter annotations that contain redundant unions between
/// builtin numeric types (e.g., `int | float`).
///
/// ## Why is this bad?
/// The [typing specification] states:
///
/// > Python’s numeric types `complex`, `float` and `int` are not subtypes of
/// > each other, but to support common use cases, the type system contains a
/// > straightforward shortcut: when an argument is annotated as having type
/// > `float`, an argument of type `int` is acceptable; similar, for an
/// > argument annotated as having type `complex`, arguments of type `float` or
/// > `int` are acceptable.
///
/// As such, a union that includes both `int` and `float` is redundant in the
/// specific context of a parameter annotation, as it is equivalent to a union
/// that only includes `float`. For readability and clarity, unions should omit
/// redundant elements.
///
/// ## Example
///
/// ```pyi
/// def foo(x: float | int | str) -> None: ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// def foo(x: float | str) -> None: ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe, unless the type annotation contains comments.
///
/// Note that while the fix may flatten nested unions into a single top-level union,
/// the semantics of the annotation will remain unchanged.
///
/// ## References
/// - [Python documentation: The numeric tower](https://docs.python.org/3/library/numbers.html#the-numeric-tower)
/// - [PEP 484: The numeric tower](https://peps.python.org/pep-0484/#the-numeric-tower)
///
/// [typing specification]: https://typing.python.org/en/latest/spec/special-types.html#special-cases-for-float-and-complex
#[derive(ViolationMetadata)]
pub struct RedundantNumericUnion {
    redundancy: Redundancy,
}

impl Violation for RedundantNumericUnion {
    // Always fixable, but currently under preview.
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let (subtype, supertype) = match self.redundancy {
            Redundancy::IntFloatComplex => ("int | float", "complex"),
            Redundancy::FloatComplex => ("float", "complex"),
            Redundancy::IntComplex => ("int", "complex"),
            Redundancy::IntFloat => ("int", "float"),
        };
        format!("Use `{supertype}` instead of `{subtype} | {supertype}`")
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove redundant type".to_string())
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::redundant_numeric_union
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Redundancy {
    IntFloatComplex,
    FloatComplex,
    IntComplex,
    IntFloat,
}