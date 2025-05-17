use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `pathlib.Path.with_suffix()` calls where
/// the given suffix does not have a leading dot
/// or the given suffix is a single dot `"."`.
///
/// ## Why is this bad?
/// `Path.with_suffix()` will raise an error at runtime
/// if the given suffix is not prefixed with a dot
/// or it is a single dot `"."`.
///
/// ## Example
///
/// ```python
/// path.with_suffix("py")
/// ```
///
/// Use instead:
///
/// ```python
/// path.with_suffix(".py")
/// ```
///
/// ## Known problems
/// This rule is likely to have false negatives, as Ruff can only emit the
/// lint if it can say for sure that a binding refers to a `Path` object at
/// runtime. Due to type inference limitations, Ruff is currently only
/// confident about this if it can see that the binding originates from a
/// function parameter annotated with `Path` or from a direct assignment to a
/// `Path()` constructor call.
///
/// ## Fix safety
/// The fix for this rule adds a leading period to the string passed
/// to the `with_suffix()` call. This fix is marked as unsafe, as it
/// changes runtime behaviour: the call would previously always have
/// raised an exception, but no longer will.
///
/// Moreover, it's impossible to determine if this is the correct fix
/// for a given situation (it's possible that the string was correct
/// but was being passed to the wrong method entirely, for example).
///
/// No fix is offered if the suffix `"."` is given, since the intent is unclear.
#[derive(ViolationMetadata)]
pub struct InvalidPathlibWithSuffix {
    // TODO: Since "." is a correct suffix in Python 3.14,
    // we will need to update this rule and documentation
    // once Ruff supports Python 3.14.
    single_dot: bool,
}

impl Violation for InvalidPathlibWithSuffix {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        if self.single_dot {
            "Invalid suffix passed to `.with_suffix()`".to_string()
        } else {
            "Dotless suffix passed to `.with_suffix()`".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = if self.single_dot {
            "Remove \".\" or extend to valid suffix"
        } else {
            "Add a leading dot"
        };
        Some(title.to_string())
    }
}