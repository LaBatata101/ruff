use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for "dummy variables" (variables that are named as if to indicate they are unused)
/// that are in fact used.
///
/// By default, "dummy variables" are any variables with names that start with leading
/// underscores. However, this is customisable using the [`lint.dummy-variable-rgx`] setting).
///
/// ## Why is this bad?
/// Marking a variable with a leading underscore conveys that it is intentionally unused within the function or method.
/// When these variables are later referenced in the code, it causes confusion and potential misunderstandings about
/// the code's intention. If a variable marked as "unused" is subsequently used, it suggests that either the variable
/// could be given a clearer name, or that the code is accidentally making use of the wrong variable.
///
/// Sometimes leading underscores are used to avoid variables shadowing other variables, Python builtins, or Python
/// keywords. However, [PEP 8] recommends to use trailing underscores for this rather than leading underscores.
///
/// Dunder variables are ignored by this rule, as are variables named `_`.
/// Only local variables in function scopes are flagged by the rule.
///
/// ## Example
/// ```python
/// def function():
///     _variable = 3
///     # important: avoid shadowing the builtin `id()` function!
///     _id = 4
///     return _variable + _id
/// ```
///
/// Use instead:
/// ```python
/// def function():
///     variable = 3
///     # important: avoid shadowing the builtin `id()` function!
///     id_ = 4
///     return variable + id_
/// ```
///
/// ## Fix availability
/// The rule's fix is only available for variables that start with leading underscores.
/// It will also only be available if the "obvious" new name for the variable
/// would not shadow any other known variables already accessible from the scope
/// in which the variable is defined.
///
/// ## Fix safety
/// This rule's fix is marked as unsafe.
///
/// For this rule's fix, Ruff renames the variable and fixes up all known references to
/// it so they point to the renamed variable. However, some renamings also require other
/// changes such as different arguments to constructor calls or alterations to comments.
/// Ruff is aware of some of these cases: `_T = TypeVar("_T")` will be fixed to
/// `T = TypeVar("T")` if the `_T` binding is flagged by this rule. However, in general,
/// cases like these are hard to detect and hard to automatically fix.
///
/// ## Options
/// - [`lint.dummy-variable-rgx`]
///
/// [PEP 8]: https://peps.python.org/pep-0008/
#[derive(ViolationMetadata)]
pub struct UsedDummyVariable {
    name: String,
    shadowed_kind: Option<ShadowedKind>,
}

impl Violation for UsedDummyVariable {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Local dummy variable `{}` is accessed", self.name)
    }

    fn fix_title(&self) -> Option<String> {
        self.shadowed_kind.map(|kind| match kind {
            ShadowedKind::BuiltIn => {
                "Prefer using trailing underscores to avoid shadowing a built-in".to_string()
            }
            ShadowedKind::Keyword => {
                "Prefer using trailing underscores to avoid shadowing a keyword".to_string()
            }
            ShadowedKind::Some => {
                "Prefer using trailing underscores to avoid shadowing a variable".to_string()
            }
            ShadowedKind::None => "Remove leading underscores".to_string(),
        })
    }
}

// FIX: this is temporary hack to make it the project compile. `ShadowedKind`
// is duplicated in ruff_linter_checkers::shadowed. We can't add a dependency to
// ruff_linter_checkers here because it would cause a cyclic dependency and
// `ShadowedKind` needs types defined in ruff_linter_checkers.
//
/// Enumeration of various ways in which a binding can shadow other variables
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ShadowedKind {
    /// The variable shadows a global, nonlocal or local symbol
    Some,
    /// The variable shadows a builtin symbol
    BuiltIn,
    /// The variable shadows a keyword
    Keyword,
    /// The variable does not shadow any other symbols
    None,
}