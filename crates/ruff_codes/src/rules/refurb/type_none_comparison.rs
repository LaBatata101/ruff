use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::CmpOp;

/// ## What it does
/// Checks for uses of `type` that compare the type of an object to the type of `None`.
///
/// ## Why is this bad?
/// There is only ever one instance of `None`, so it is more efficient and
/// readable to use the `is` operator to check if an object is `None`.
///
/// ## Example
/// ```python
/// type(obj) is type(None)
/// ```
///
/// Use instead:
/// ```python
/// obj is None
/// ```
///
/// ## Fix safety
/// If the fix might remove comments, it will be marked as unsafe.
///
/// ## References
/// - [Python documentation: `isinstance`](https://docs.python.org/3/library/functions.html#isinstance)
/// - [Python documentation: `None`](https://docs.python.org/3/library/constants.html#None)
/// - [Python documentation: `type`](https://docs.python.org/3/library/functions.html#type)
/// - [Python documentation: Identity comparisons](https://docs.python.org/3/reference/expressions.html#is-not)
#[derive(ViolationMetadata)]
pub struct TypeNoneComparison {
    replacement: IdentityCheck,
}

impl AlwaysFixableViolation for TypeNoneComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "When checking against `None`, use `{}` instead of comparison with `type(None)`",
            self.replacement.op()
        )
    }

    fn fix_title(&self) -> String {
        format!("Replace with `{} None`", self.replacement.op())
    }
}

// FIX: duplicated with ruff_rule_refurb::type_none_comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IdentityCheck {
    Is,
    IsNot,
}

impl IdentityCheck {
    fn op(self) -> CmpOp {
        match self {
            Self::Is => CmpOp::Is,
            Self::IsNot => CmpOp::IsNot,
        }
    }
}