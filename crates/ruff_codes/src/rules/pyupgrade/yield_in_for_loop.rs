use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `for` loops that can be replaced with `yield from` expressions.
///
/// ## Why is this bad?
/// If a `for` loop only contains a `yield` statement, it can be replaced with a
/// `yield from` expression, which is more concise and idiomatic.
///
/// ## Example
/// ```python
/// for x in foo:
///     yield x
///
/// global y
/// for y in foo:
///     yield y
/// ```
///
/// Use instead:
/// ```python
/// yield from foo
///
/// for _element in foo:
///     y = _element
///     yield y
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as converting a `for` loop to a `yield
/// from` expression can change the behavior of the program in rare cases.
/// For example, if a generator is being sent values via `send`, then rewriting
/// to a `yield from` could lead to an attribute error if the underlying
/// generator does not implement the `send` method.
///
/// Additionally, if at least one target is `global` or `nonlocal`,
/// no fix will be offered.
///
/// In most cases, however, the fix is safe, and such a modification should have
/// no effect on the behavior of the program.
///
/// ## References
/// - [Python documentation: The `yield` statement](https://docs.python.org/3/reference/simple_stmts.html#the-yield-statement)
/// - [PEP 380 – Syntax for Delegating to a Subgenerator](https://peps.python.org/pep-0380/)
#[derive(ViolationMetadata)]
pub struct YieldInForLoop;

impl Violation for YieldInForLoop {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Replace `yield` over `for` loop with `yield from`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `yield from`".to_string())
    }
}