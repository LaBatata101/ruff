use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary parentheses on raised exceptions.
///
/// ## Why is this bad?
/// If an exception is raised without any arguments, parentheses are not
/// required, as the `raise` statement accepts either an exception instance
/// or an exception class (which is then implicitly instantiated).
///
/// Removing the parentheses makes the code more concise.
///
/// ## Known problems
/// Parentheses can only be omitted if the exception is a class, as opposed to
/// a function call. This rule isn't always capable of distinguishing between
/// the two.
///
/// For example, if you import a function `module.get_exception` from another
/// module, and `module.get_exception` returns an exception object, this rule will
/// incorrectly mark the parentheses in `raise module.get_exception()` as
/// unnecessary.
///
/// ## Example
/// ```python
/// raise TypeError()
/// ```
///
/// Use instead:
/// ```python
/// raise TypeError
/// ```
///
/// ## References
/// - [Python documentation: The `raise` statement](https://docs.python.org/3/reference/simple_stmts.html#the-raise-statement)
#[derive(ViolationMetadata)]
pub struct UnnecessaryParenOnRaiseException;

impl AlwaysFixableViolation for UnnecessaryParenOnRaiseException {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary parentheses on raised exception".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove unnecessary parentheses".to_string()
    }
}