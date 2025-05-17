use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

// FIX: this is duplicated with ruff_rule_pycodestyle::invalid_escape_sequence
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum EqCmpOp {
    Eq,
    NotEq,
}

/// ## What it does
/// Checks for comparisons to `None` which are not using the `is` operator.
///
/// ## Why is this bad?
/// According to [PEP 8], "Comparisons to singletons like None should always be done with
/// `is` or `is not`, never the equality operators."
///
/// ## Example
/// ```python
/// if arg != None:
///     pass
/// if None == arg:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// if arg is not None:
///     pass
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe, as it may alter runtime behavior when
/// used with libraries that override the `==`/`__eq__` or `!=`/`__ne__` operators.
/// In these cases, `is`/`is not` may not be equivalent to `==`/`!=`. For more
/// information, see [this issue].
///
/// [PEP 8]: https://peps.python.org/pep-0008/#programming-recommendations
/// [this issue]: https://github.com/astral-sh/ruff/issues/4560
#[derive(ViolationMetadata)]
pub struct NoneComparison(EqCmpOp);

impl AlwaysFixableViolation for NoneComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NoneComparison(op) = self;
        match op {
            EqCmpOp::Eq => "Comparison to `None` should be `cond is None`".to_string(),
            EqCmpOp::NotEq => "Comparison to `None` should be `cond is not None`".to_string(),
        }
    }

    fn fix_title(&self) -> String {
        let NoneComparison(op) = self;
        let title = match op {
            EqCmpOp::Eq => "Replace with `cond is None`",
            EqCmpOp::NotEq => "Replace with `cond is not None`",
        };
        title.to_string()
    }
}

/// ## What it does
/// Checks for equality comparisons to boolean literals.
///
/// ## Why is this bad?
/// [PEP 8] recommends against using the equality operators `==` and `!=` to
/// compare values to `True` or `False`.
///
/// Instead, use `if cond:` or `if not cond:` to check for truth values.
///
/// If you intend to check if a value is the boolean literal `True` or `False`,
/// consider using `is` or `is not` to check for identity instead.
///
/// ## Example
/// ```python
/// if foo == True:
///     ...
///
/// if bar == False:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// if foo:
///     ...
///
/// if not bar:
///     ...
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe, as it may alter runtime behavior when
/// used with libraries that override the `==`/`__eq__` or `!=`/`__ne__` operators.
/// In these cases, `is`/`is not` may not be equivalent to `==`/`!=`. For more
/// information, see [this issue].
///
/// [PEP 8]: https://peps.python.org/pep-0008/#programming-recommendations
/// [this issue]: https://github.com/astral-sh/ruff/issues/4560
#[derive(ViolationMetadata)]
pub struct TrueFalseComparison {
    value: bool,
    op: EqCmpOp,
    cond: Option<SourceCodeSnippet>,
}

impl AlwaysFixableViolation for TrueFalseComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TrueFalseComparison { value, op, cond } = self;
        let Some(cond) = cond else {
            return "Avoid equality comparisons to `True` or `False`".to_string();
        };
        let cond = cond.truncated_display();
        match (value, op) {
            (true, EqCmpOp::Eq) => {
                format!("Avoid equality comparisons to `True`; use `if {cond}:` for truth checks")
            }
            (true, EqCmpOp::NotEq) => {
                format!(
                    "Avoid inequality comparisons to `True`; use `if not {cond}:` for false checks"
                )
            }
            (false, EqCmpOp::Eq) => {
                format!(
                    "Avoid equality comparisons to `False`; use `if not {cond}:` for false checks"
                )
            }
            (false, EqCmpOp::NotEq) => {
                format!(
                    "Avoid inequality comparisons to `False`; use `if {cond}:` for truth checks"
                )
            }
        }
    }

    fn fix_title(&self) -> String {
        let TrueFalseComparison { value, op, cond } = self;
        let Some(cond) = cond.as_ref().and_then(|cond| cond.full_display()) else {
            return "Replace comparison".to_string();
        };
        match (value, op) {
            (true, EqCmpOp::Eq) => format!("Replace with `{cond}`"),
            (true, EqCmpOp::NotEq) => format!("Replace with `not {cond}`"),
            (false, EqCmpOp::Eq) => format!("Replace with `not {cond}`"),
            (false, EqCmpOp::NotEq) => format!("Replace with `{cond}`"),
        }
    }
}
