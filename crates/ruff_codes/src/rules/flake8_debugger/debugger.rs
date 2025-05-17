use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of debugger calls and imports.
///
/// ## Why is this bad?
/// Debugger calls and imports should be used for debugging purposes only. The
/// presence of a debugger call or import in production code is likely a
/// mistake and may cause unintended behavior, such as exposing sensitive
/// information or causing the program to hang.
///
/// Instead, consider using a logging library to log information about the
/// program's state, and writing tests to verify that the program behaves
/// as expected.
///
/// ## Example
/// ```python
/// def foo():
///     breakpoint()
/// ```
///
/// ## References
/// - [Python documentation: `pdb` — The Python Debugger](https://docs.python.org/3/library/pdb.html)
/// - [Python documentation: `logging` — Logging facility for Python](https://docs.python.org/3/library/logging.html)
#[derive(ViolationMetadata)]
pub struct Debugger {
    using_type: DebuggerUsingType,
}

impl Violation for Debugger {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Debugger { using_type } = self;
        match using_type {
            DebuggerUsingType::Call(name) => format!("Trace found: `{name}` used"),
            DebuggerUsingType::Import(name) => format!("Import for `{name}` found"),
        }
    }
}

// FIX: duplicate with ruff_rule_flake8_debugger::types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DebuggerUsingType {
    Call(String),
    Import(String),
}