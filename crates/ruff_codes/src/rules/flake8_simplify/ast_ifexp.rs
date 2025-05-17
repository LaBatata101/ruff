use ruff_diagnostics::{AlwaysFixableViolation, FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if` expressions that can be replaced with `bool()` calls.
///
/// ## Why is this bad?
/// `if` expressions that evaluate to `True` for a truthy condition an `False`
/// for a falsey condition can be replaced with `bool()` calls, which are more
/// concise and readable.
///
/// ## Example
/// ```python
/// True if a else False
/// ```
///
/// Use instead:
/// ```python
/// bool(a)
/// ```
///
/// ## Fix safety
///
/// This fix is marked as unsafe because it may change the program’s behavior if the condition does not
/// return a proper Boolean. While the fix will try to wrap non-boolean values in a call to bool,
/// custom implementations of comparison functions like `__eq__` can avoid the bool call and still
/// lead to altered behavior. Moreover, the fix may remove comments.
///
/// ## References
/// - [Python documentation: Truth Value Testing](https://docs.python.org/3/library/stdtypes.html#truth-value-testing)
#[derive(ViolationMetadata)]
pub struct IfExprWithTrueFalse {
    is_compare: bool,
}

impl Violation for IfExprWithTrueFalse {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        if self.is_compare {
            "Remove unnecessary `True if ... else False`".to_string()
        } else {
            "Use `bool(...)` instead of `True if ... else False`".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = if self.is_compare {
            "Remove unnecessary `True if ... else False`"
        } else {
            "Replace with `bool(...)"
        };
        Some(title.to_string())
    }
}

/// ## What it does
/// Checks for `if` expressions that can be replaced by negating a given
/// condition.
///
/// ## Why is this bad?
/// `if` expressions that evaluate to `False` for a truthy condition and `True`
/// for a falsey condition can be replaced with `not` operators, which are more
/// concise and readable.
///
/// ## Example
/// ```python
/// False if a else True
/// ```
///
/// Use instead:
/// ```python
/// not a
/// ```
///
/// ## References
/// - [Python documentation: Truth Value Testing](https://docs.python.org/3/library/stdtypes.html#truth-value-testing)
#[derive(ViolationMetadata)]
pub struct IfExprWithFalseTrue;

impl AlwaysFixableViolation for IfExprWithFalseTrue {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `not ...` instead of `False if ... else True`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `not ...`".to_string()
    }
}

/// ## What it does
/// Checks for `if` expressions that check against a negated condition.
///
/// ## Why is this bad?
/// `if` expressions that check against a negated condition are more difficult
/// to read than `if` expressions that check against the condition directly.
///
/// ## Example
/// ```python
/// b if not a else a
/// ```
///
/// Use instead:
/// ```python
/// a if a else b
/// ```
///
/// ## References
/// - [Python documentation: Truth Value Testing](https://docs.python.org/3/library/stdtypes.html#truth-value-testing)
#[derive(ViolationMetadata)]
pub struct IfExprWithTwistedArms {
    expr_body: String,
    expr_else: String,
}

impl AlwaysFixableViolation for IfExprWithTwistedArms {
    #[derive_message_formats]
    fn message(&self) -> String {
        let IfExprWithTwistedArms {
            expr_body,
            expr_else,
        } = self;
        format!(
            "Use `{expr_else} if {expr_else} else {expr_body}` instead of `{expr_body} if not \
             {expr_else} else {expr_else}`"
        )
    }

    fn fix_title(&self) -> String {
        let IfExprWithTwistedArms {
            expr_body,
            expr_else,
        } = self;
        format!("Replace with `{expr_else} if {expr_else} else {expr_body}`")
    }
}
