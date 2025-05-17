use ruff_diagnostics::{AlwaysFixableViolation, FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for multiple `isinstance` calls on the same target.
///
/// ## Why is this bad?
/// To check if an object is an instance of any one of multiple types
/// or classes, it is unnecessary to use multiple `isinstance` calls, as
/// the second argument of the `isinstance` built-in function accepts a
/// tuple of types and classes.
///
/// Using a single `isinstance` call implements the same behavior with more
/// concise code and clearer intent.
///
/// ## Example
/// ```python
/// if isinstance(obj, int) or isinstance(obj, float):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// if isinstance(obj, (int, float)):
///     pass
/// ```
///
/// ## References
/// - [Python documentation: `isinstance`](https://docs.python.org/3/library/functions.html#isinstance)
#[derive(ViolationMetadata)]
pub struct DuplicateIsinstanceCall {
    name: Option<String>,
}

impl Violation for DuplicateIsinstanceCall {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(name) = &self.name {
            format!("Multiple `isinstance` calls for `{name}`, merge into a single call")
        } else {
            "Multiple `isinstance` calls for expression, merge into a single call".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some(if let Some(name) = &self.name {
            format!("Merge `isinstance` calls for `{name}`")
        } else {
            "Merge `isinstance` calls".to_string()
        })
    }
}

/// ## What it does
/// Checks for boolean expressions that contain multiple equality comparisons
/// to the same value.
///
/// ## Why is this bad?
/// To check if an object is equal to any one of multiple values, it's more
/// concise to use the `in` operator with a tuple of values.
///
/// ## Example
/// ```python
/// if foo == x or foo == y:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// if foo in (x, y):
///     ...
/// ```
///
/// ## References
/// - [Python documentation: Membership test operations](https://docs.python.org/3/reference/expressions.html#membership-test-operations)
#[derive(ViolationMetadata)]
pub struct CompareWithTuple {
    replacement: String,
}

impl AlwaysFixableViolation for CompareWithTuple {
    #[derive_message_formats]
    fn message(&self) -> String {
        let CompareWithTuple { replacement } = self;
        format!("Use `{replacement}` instead of multiple equality comparisons")
    }

    fn fix_title(&self) -> String {
        let CompareWithTuple { replacement } = self;
        format!("Replace with `{replacement}`")
    }
}

/// ## What it does
/// Checks for `and` expressions that include both an expression and its
/// negation.
///
/// ## Why is this bad?
/// An `and` expression that includes both an expression and its negation will
/// always evaluate to `False`.
///
/// ## Example
/// ```python
/// x and not x
/// ```
///
/// ## References
/// - [Python documentation: Boolean operations](https://docs.python.org/3/reference/expressions.html#boolean-operations)
#[derive(ViolationMetadata)]
pub struct ExprAndNotExpr {
    name: String,
}

impl AlwaysFixableViolation for ExprAndNotExpr {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ExprAndNotExpr { name } = self;
        format!("Use `False` instead of `{name} and not {name}`")
    }

    fn fix_title(&self) -> String {
        "Replace with `False`".to_string()
    }
}

/// ## What it does
/// Checks for `or` expressions that include both an expression and its
/// negation.
///
/// ## Why is this bad?
/// An `or` expression that includes both an expression and its negation will
/// always evaluate to `True`.
///
/// ## Example
/// ```python
/// x or not x
/// ```
///
/// ## References
/// - [Python documentation: Boolean operations](https://docs.python.org/3/reference/expressions.html#boolean-operations)
#[derive(ViolationMetadata)]
pub struct ExprOrNotExpr {
    name: String,
}

impl AlwaysFixableViolation for ExprOrNotExpr {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ExprOrNotExpr { name } = self;
        format!("Use `True` instead of `{name} or not {name}`")
    }

    fn fix_title(&self) -> String {
        "Replace with `True`".to_string()
    }
}

/// ## What it does
/// Checks for `or` expressions that contain truthy values.
///
/// ## Why is this bad?
/// If the expression is used as a condition, it can be replaced in-full with
/// `True`.
///
/// In other cases, the expression can be short-circuited to the first truthy
/// value.
///
/// By using `True` (or the first truthy value), the code is more concise
/// and easier to understand, since it no longer contains redundant conditions.
///
/// ## Example
/// ```python
/// if x or [1] or y:
///     pass
///
/// a = x or [1] or y
/// ```
///
/// Use instead:
/// ```python
/// if True:
///     pass
///
/// a = x or [1]
/// ```
#[derive(ViolationMetadata)]
pub struct ExprOrTrue {
    expr: String,
    remove: ContentAround,
}

impl AlwaysFixableViolation for ExprOrTrue {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ExprOrTrue { expr, remove } = self;
        let replaced = match remove {
            ContentAround::After => format!("{expr} or ..."),
            ContentAround::Before => format!("... or {expr}"),
            ContentAround::Both => format!("... or {expr} or ..."),
        };
        format!("Use `{expr}` instead of `{replaced}`")
    }

    fn fix_title(&self) -> String {
        let ExprOrTrue { expr, .. } = self;
        format!("Replace with `{expr}`")
    }
}

/// ## What it does
/// Checks for `and` expressions that contain falsey values.
///
/// ## Why is this bad?
/// If the expression is used as a condition, it can be replaced in-full with
/// `False`.
///
/// In other cases, the expression can be short-circuited to the first falsey
/// value.
///
/// By using `False` (or the first falsey value), the code is more concise
/// and easier to understand, since it no longer contains redundant conditions.
///
/// ## Example
/// ```python
/// if x and [] and y:
///     pass
///
/// a = x and [] and y
/// ```
///
/// Use instead:
/// ```python
/// if False:
///     pass
///
/// a = x and []
/// ```
#[derive(ViolationMetadata)]
pub struct ExprAndFalse {
    expr: String,
    remove: ContentAround,
}

impl AlwaysFixableViolation for ExprAndFalse {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ExprAndFalse { expr, remove } = self;
        let replaced = match remove {
            ContentAround::After => format!(r#"{expr} and ..."#),
            ContentAround::Before => format!("... and {expr}"),
            ContentAround::Both => format!("... and {expr} and ..."),
        };
        format!("Use `{expr}` instead of `{replaced}`")
    }

    fn fix_title(&self) -> String {
        let ExprAndFalse { expr, .. } = self;
        format!("Replace with `{expr}`")
    }
}

// FIX: duplicated with ruff_rule_flake8_simplify::ast_bool_op
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ContentAround {
    Before,
    After,
    Both,
}