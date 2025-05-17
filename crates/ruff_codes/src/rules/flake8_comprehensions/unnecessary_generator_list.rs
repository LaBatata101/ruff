use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary generators that can be rewritten as list
/// comprehensions (or with `list()` directly).
///
/// ## Why is this bad?
/// It is unnecessary to use `list()` around a generator expression, since
/// there are equivalent comprehensions for these types. Using a
/// comprehension is clearer and more idiomatic.
///
/// Further, if the comprehension can be removed entirely, as in the case of
/// `list(x for x in foo)`, it's better to use `list(foo)` directly, since it's
/// even more direct.
///
/// ## Example
/// ```python
/// list(f(x) for x in foo)
/// list(x for x in foo)
/// list((x for x in foo))
/// ```
///
/// Use instead:
/// ```python
/// [f(x) for x in foo]
/// list(foo)
/// list(foo)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryGeneratorList {
    short_circuit: bool,
}

impl AlwaysFixableViolation for UnnecessaryGeneratorList {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.short_circuit {
            "Unnecessary generator (rewrite using `list()`)".to_string()
        } else {
            "Unnecessary generator (rewrite as a list comprehension)".to_string()
        }
    }

    fn fix_title(&self) -> String {
        if self.short_circuit {
            "Rewrite using `list()`".to_string()
        } else {
            "Rewrite as a list comprehension".to_string()
        }
    }
}