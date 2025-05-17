use std::fmt;

use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary calls to `str`, `bytes`, `int`, `float`, and `bool`.
///
/// ## Why is this bad?
/// The mentioned constructors can be replaced with their respective literal
/// forms, which are more readable and idiomatic.
///
/// ## Example
/// ```python
/// str("foo")
/// ```
///
/// Use instead:
/// ```python
/// "foo"
/// ```
///
/// ## Fix safety
/// The fix is marked as unsafe if it might remove comments.
///
/// ## References
/// - [Python documentation: `str`](https://docs.python.org/3/library/stdtypes.html#str)
/// - [Python documentation: `bytes`](https://docs.python.org/3/library/stdtypes.html#bytes)
/// - [Python documentation: `int`](https://docs.python.org/3/library/functions.html#int)
/// - [Python documentation: `float`](https://docs.python.org/3/library/functions.html#float)
/// - [Python documentation: `bool`](https://docs.python.org/3/library/functions.html#bool)
#[derive(ViolationMetadata)]
pub struct NativeLiterals {
    literal_type: LiteralType,
}

impl AlwaysFixableViolation for NativeLiterals {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NativeLiterals { literal_type } = self;
        format!("Unnecessary `{literal_type}` call (rewrite as a literal)")
    }

    fn fix_title(&self) -> String {
        let NativeLiterals { literal_type } = self;
        match literal_type {
            LiteralType::Str => "Replace with string literal".to_string(),
            LiteralType::Bytes => "Replace with bytes literal".to_string(),
            LiteralType::Int => "Replace with integer literal".to_string(),
            LiteralType::Float => "Replace with float literal".to_string(),
            LiteralType::Bool => "Replace with boolean literal".to_string(),
        }
    }
}

// FIX: duplicated with ruff_rule_pyupgrade::native_literals
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum LiteralType {
    Str,
    Bytes,
    Int,
    Float,
    Bool,
}

impl fmt::Display for LiteralType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralType::Str => fmt.write_str("str"),
            LiteralType::Bytes => fmt.write_str("bytes"),
            LiteralType::Int => fmt.write_str("int"),
            LiteralType::Float => fmt.write_str("float"),
            LiteralType::Bool => fmt.write_str("bool"),
        }
    }
}
