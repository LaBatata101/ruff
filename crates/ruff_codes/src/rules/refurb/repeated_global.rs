use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::Stmt;

/// ## What it does
/// Checks for consecutive `global` (or `nonlocal`) statements.
///
/// ## Why is this bad?
/// The `global` and `nonlocal` keywords accepts multiple comma-separated names.
/// Instead of using multiple `global` (or `nonlocal`) statements for separate
/// variables, you can use a single statement to declare multiple variables at
/// once.
///
/// ## Example
/// ```python
/// def func():
///     global x
///     global y
///
///     print(x, y)
/// ```
///
/// Use instead:
/// ```python
/// def func():
///     global x, y
///
///     print(x, y)
/// ```
///
/// ## References
/// - [Python documentation: the `global` statement](https://docs.python.org/3/reference/simple_stmts.html#the-global-statement)
/// - [Python documentation: the `nonlocal` statement](https://docs.python.org/3/reference/simple_stmts.html#the-nonlocal-statement)
#[derive(ViolationMetadata)]
pub struct RepeatedGlobal {
    global_kind: GlobalKind,
}

impl AlwaysFixableViolation for RepeatedGlobal {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use of repeated consecutive `{}`", self.global_kind)
    }

    fn fix_title(&self) -> String {
        format!("Merge `{}` statements", self.global_kind)
    }
}

// FIX: duplicated with ruff_rule_refurb::repeated_global
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GlobalKind {
    Global,
    NonLocal,
}

impl GlobalKind {
    fn from_stmt(stmt: &Stmt) -> Option<Self> {
        match stmt {
            Stmt::Global(_) => Some(GlobalKind::Global),
            Stmt::Nonlocal(_) => Some(GlobalKind::NonLocal),
            _ => None,
        }
    }
}

impl std::fmt::Display for GlobalKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalKind::Global => write!(f, "global"),
            GlobalKind::NonLocal => write!(f, "nonlocal"),
        }
    }
}