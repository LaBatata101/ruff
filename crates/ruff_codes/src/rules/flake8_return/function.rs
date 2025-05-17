use std::fmt;

use ruff_diagnostics::{AlwaysFixableViolation, FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of a `return None` statement when `None` is the only
/// possible return value.
///
/// ## Why is this bad?
/// Python implicitly assumes `return None` if an explicit `return` value is
/// omitted. Therefore, explicitly returning `None` is redundant and should be
/// avoided when it is the only possible `return` value across all code paths
/// in a given function.
///
/// ## Example
/// ```python
/// def foo(bar):
///     if not bar:
///         return
///     return None
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar):
///     if not bar:
///         return
///     return
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessaryReturnNone;

impl AlwaysFixableViolation for UnnecessaryReturnNone {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not explicitly `return None` in function if it is the only possible return value"
            .to_string()
    }

    fn fix_title(&self) -> String {
        "Remove explicit `return None`".to_string()
    }
}

/// ## What it does
/// Checks for the presence of a `return` statement with no explicit value,
/// for functions that return non-`None` values elsewhere.
///
/// ## Why is this bad?
/// Including a `return` statement with no explicit value can cause confusion
/// when other `return` statements in the function return non-`None` values.
/// Python implicitly assumes return `None` if no other return value is present.
/// Adding an explicit `return None` can make the code more readable by clarifying
/// intent.
///
/// ## Example
/// ```python
/// def foo(bar):
///     if not bar:
///         return
///     return 1
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar):
///     if not bar:
///         return None
///     return 1
/// ```
#[derive(ViolationMetadata)]
pub struct ImplicitReturnValue;

impl AlwaysFixableViolation for ImplicitReturnValue {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not implicitly `return None` in function able to return non-`None` value".to_string()
    }

    fn fix_title(&self) -> String {
        "Add explicit `None` return value".to_string()
    }
}

/// ## What it does
/// Checks for missing explicit `return` statements at the end of functions
/// that can return non-`None` values.
///
/// ## Why is this bad?
/// The lack of an explicit `return` statement at the end of a function that
/// can return non-`None` values can cause confusion. Python implicitly returns
/// `None` if no other return value is present. Adding an explicit
/// `return None` can make the code more readable by clarifying intent.
///
/// ## Example
/// ```python
/// def foo(bar):
///     if not bar:
///         return 1
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar):
///     if not bar:
///         return 1
///     return None
/// ```
#[derive(ViolationMetadata)]
pub struct ImplicitReturn;

impl AlwaysFixableViolation for ImplicitReturn {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing explicit `return` at the end of function able to return non-`None` value"
            .to_string()
    }

    fn fix_title(&self) -> String {
        "Add explicit `return` statement".to_string()
    }
}

/// ## What it does
/// Checks for variable assignments that immediately precede a `return` of the
/// assigned variable.
///
/// ## Why is this bad?
/// The variable assignment is not necessary, as the value can be returned
/// directly.
///
/// ## Example
/// ```python
/// def foo():
///     bar = 1
///     return bar
/// ```
///
/// Use instead:
/// ```python
/// def foo():
///     return 1
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessaryAssign {
    name: String,
}

impl AlwaysFixableViolation for UnnecessaryAssign {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryAssign { name } = self;
        format!("Unnecessary assignment to `{name}` before `return` statement")
    }

    fn fix_title(&self) -> String {
        "Remove unnecessary assignment".to_string()
    }
}

/// ## What it does
/// Checks for `else` statements with a `return` statement in the preceding
/// `if` block.
///
/// ## Why is this bad?
/// The `else` statement is not needed as the `return` statement will always
/// break out of the enclosing function. Removing the `else` will reduce
/// nesting and make the code more readable.
///
/// ## Example
/// ```python
/// def foo(bar, baz):
///     if bar:
///         return 1
///     else:
///         return baz
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar, baz):
///     if bar:
///         return 1
///     return baz
/// ```
#[derive(ViolationMetadata)]
pub struct SuperfluousElseReturn {
    branch: Branch,
}

impl Violation for SuperfluousElseReturn {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;
    #[derive_message_formats]
    fn message(&self) -> String {
        let SuperfluousElseReturn { branch } = self;
        format!("Unnecessary `{branch}` after `return` statement")
    }

    fn fix_title(&self) -> Option<String> {
        let SuperfluousElseReturn { branch } = self;
        Some(format!("Remove unnecessary `{branch}`"))
    }
}

/// ## What it does
/// Checks for `else` statements with a `raise` statement in the preceding `if`
/// block.
///
/// ## Why is this bad?
/// The `else` statement is not needed as the `raise` statement will always
/// break out of the current scope. Removing the `else` will reduce nesting
/// and make the code more readable.
///
/// ## Example
/// ```python
/// def foo(bar, baz):
///     if bar == "Specific Error":
///         raise Exception(bar)
///     else:
///         raise Exception(baz)
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar, baz):
///     if bar == "Specific Error":
///         raise Exception(bar)
///     raise Exception(baz)
/// ```
#[derive(ViolationMetadata)]
pub struct SuperfluousElseRaise {
    branch: Branch,
}

impl Violation for SuperfluousElseRaise {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;
    #[derive_message_formats]
    fn message(&self) -> String {
        let SuperfluousElseRaise { branch } = self;
        format!("Unnecessary `{branch}` after `raise` statement")
    }

    fn fix_title(&self) -> Option<String> {
        let SuperfluousElseRaise { branch } = self;
        Some(format!("Remove unnecessary `{branch}`"))
    }
}

/// ## What it does
/// Checks for `else` statements with a `continue` statement in the preceding
/// `if` block.
///
/// ## Why is this bad?
/// The `else` statement is not needed, as the `continue` statement will always
/// continue onto the next iteration of a loop. Removing the `else` will reduce
/// nesting and make the code more readable.
///
/// ## Example
/// ```python
/// def foo(bar, baz):
///     for i in bar:
///         if i < baz:
///             continue
///         else:
///             x = 0
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar, baz):
///     for i in bar:
///         if i < baz:
///             continue
///         x = 0
/// ```
#[derive(ViolationMetadata)]
pub struct SuperfluousElseContinue {
    branch: Branch,
}

impl Violation for SuperfluousElseContinue {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;
    #[derive_message_formats]
    fn message(&self) -> String {
        let SuperfluousElseContinue { branch } = self;
        format!("Unnecessary `{branch}` after `continue` statement")
    }

    fn fix_title(&self) -> Option<String> {
        let SuperfluousElseContinue { branch } = self;
        Some(format!("Remove unnecessary `{branch}`"))
    }
}

/// ## What it does
/// Checks for `else` statements with a `break` statement in the preceding `if`
/// block.
///
/// ## Why is this bad?
/// The `else` statement is not needed, as the `break` statement will always
/// break out of the loop. Removing the `else` will reduce nesting and make the
/// code more readable.
///
/// ## Example
/// ```python
/// def foo(bar, baz):
///     for i in bar:
///         if i > baz:
///             break
///         else:
///             x = 0
/// ```
///
/// Use instead:
/// ```python
/// def foo(bar, baz):
///     for i in bar:
///         if i > baz:
///             break
///         x = 0
/// ```
#[derive(ViolationMetadata)]
pub struct SuperfluousElseBreak {
    branch: Branch,
}

impl Violation for SuperfluousElseBreak {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;
    #[derive_message_formats]
    fn message(&self) -> String {
        let SuperfluousElseBreak { branch } = self;
        format!("Unnecessary `{branch}` after `break` statement")
    }

    fn fix_title(&self) -> Option<String> {
        let SuperfluousElseBreak { branch } = self;
        Some(format!("Remove unnecessary `{branch}`"))
    }
}

// FIX: duplicated with ruff_rule_flake8_return::branch
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum Branch {
    Elif,
    Else,
}

impl fmt::Display for Branch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branch::Elif => fmt.write_str("elif"),
            Branch::Else => fmt.write_str("else"),
        }
    }
}