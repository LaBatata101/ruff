use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::{Expr, ExprNumberLiteral, Number};
use serde::{Deserialize, Serialize};

/// ## What it does
/// Checks for uses of `type` that take a primitive as an argument.
///
/// ## Why is this bad?
/// `type()` returns the type of a given object. A type of a primitive can
/// always be known in advance and accessed directly, which is more concise
/// and explicit than using `type()`.
///
/// ## Example
/// ```python
/// type(1)
/// ```
///
/// Use instead:
/// ```python
/// int
/// ```
///
/// ## References
/// - [Python documentation: `type()`](https://docs.python.org/3/library/functions.html#type)
/// - [Python documentation: Built-in types](https://docs.python.org/3/library/stdtypes.html)
#[derive(ViolationMetadata)]
pub struct TypeOfPrimitive {
    primitive: Primitive,
}

impl Violation for TypeOfPrimitive {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let TypeOfPrimitive { primitive } = self;
        format!("Use `{}` instead of `type(...)`", primitive.builtin())
    }

    fn fix_title(&self) -> Option<String> {
        let TypeOfPrimitive { primitive } = self;
        Some(format!(
            "Replace `type(...)` with `{}`",
            primitive.builtin()
        ))
    }
}

// FIX: duplicated with ruff_rule_pyupgrade::types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Primitive {
    Bool,
    Str,
    Bytes,
    Int,
    Float,
    Complex,
}

impl Primitive {
    pub const fn from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::BooleanLiteral(_) => Some(Self::Bool),
            Expr::StringLiteral(_) => Some(Self::Str),
            Expr::BytesLiteral(_) => Some(Self::Bytes),
            Expr::NumberLiteral(ExprNumberLiteral { value, .. }) => match value {
                Number::Int(_) => Some(Self::Int),
                Number::Float(_) => Some(Self::Float),
                Number::Complex { .. } => Some(Self::Complex),
            },
            _ => None,
        }
    }

    pub fn builtin(self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Str => "str".to_string(),
            Self::Bytes => "bytes".to_string(),
            Self::Int => "int".to_string(),
            Self::Float => "float".to_string(),
            Self::Complex => "complex".to_string(),
        }
    }
}