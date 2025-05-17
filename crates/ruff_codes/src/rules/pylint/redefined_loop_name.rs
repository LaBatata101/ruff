use std::fmt;

use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for variables defined in `for` loops and `with` statements that
/// get overwritten within the body, for example by another `for` loop or
/// `with` statement or by direct assignment.
///
/// ## Why is this bad?
/// Redefinition of a loop variable inside the loop's body causes its value
/// to differ from the original loop iteration for the remainder of the
/// block, in a way that will likely cause bugs.
///
/// In Python, unlike many other languages, `for` loops and `with`
/// statements don't define their own scopes. Therefore, a nested loop that
/// uses the same target variable name as an outer loop will reuse the same
/// actual variable, and the value from the last iteration will "leak out"
/// into the remainder of the enclosing loop.
///
/// While this mistake is easy to spot in small examples, it can be hidden
/// in larger blocks of code, where the definition and redefinition of the
/// variable may not be visible at the same time.
///
/// ## Example
/// ```python
/// for i in range(10):
///     i = 9
///     print(i)  # prints 9 every iteration
///
/// for i in range(10):
///     for i in range(10):  # original value overwritten
///         pass
///     print(i)  # also prints 9 every iteration
///
/// with path1.open() as f:
///     with path2.open() as f:
///         f = path2.open()
///     print(f.readline())  # prints a line from path2
/// ```
#[derive(ViolationMetadata)]
pub struct RedefinedLoopName {
    name: String,
    outer_kind: OuterBindingKind,
    inner_kind: InnerBindingKind,
}

impl Violation for RedefinedLoopName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RedefinedLoopName {
            name,
            outer_kind,
            inner_kind,
        } = self;
        // Prefix the nouns describing the outer and inner kinds with "outer" and "inner"
        // to better distinguish them, but to avoid confusion, only do so if the outer and inner
        // kinds are equal. For example, instead of:
        //
        //    "Outer `for` loop variable `i` overwritten by inner assignment target."
        //
        // We have:
        //
        //    "`for` loop variable `i` overwritten by assignment target."
        //
        // While at the same time, we have:
        //
        //    "Outer `for` loop variable `i` overwritten by inner `for` loop target."
        //    "Outer `with` statement variable `f` overwritten by inner `with` statement target."

        if outer_kind == inner_kind {
            format!("Outer {outer_kind} variable `{name}` overwritten by inner {inner_kind} target")
        } else {
            format!("{outer_kind} variable `{name}` overwritten by {inner_kind} target")
        }
    }
}

// FIX: duplicated with rufe_rule_pylint::redefined_loop_name
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OuterBindingKind {
    For,
    With,
}

impl fmt::Display for OuterBindingKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OuterBindingKind::For => fmt.write_str("`for` loop"),
            OuterBindingKind::With => fmt.write_str("`with` statement"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum InnerBindingKind {
    For,
    With,
    Assignment,
}

impl fmt::Display for InnerBindingKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InnerBindingKind::For => fmt.write_str("`for` loop"),
            InnerBindingKind::With => fmt.write_str("`with` statement"),
            InnerBindingKind::Assignment => fmt.write_str("assignment"),
        }
    }
}

impl PartialEq<InnerBindingKind> for OuterBindingKind {
    fn eq(&self, other: &InnerBindingKind) -> bool {
        matches!(
            (self, other),
            (OuterBindingKind::For, InnerBindingKind::For)
                | (OuterBindingKind::With, InnerBindingKind::With)
        )
    }
}