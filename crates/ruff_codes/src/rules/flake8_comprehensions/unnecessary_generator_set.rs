use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary generators that can be rewritten as set
/// comprehensions (or with `set()` directly).
///
/// ## Why is this bad?
/// It is unnecessary to use `set` around a generator expression, since
/// there are equivalent comprehensions for these types. Using a
/// comprehension is clearer and more idiomatic.
///
/// Further, if the comprehension can be removed entirely, as in the case of
/// `set(x for x in foo)`, it's better to use `set(foo)` directly, since it's
/// even more direct.
///
/// ## Example
/// ```python
/// set(f(x) for x in foo)
/// set(x for x in foo)
/// set((x for x in foo))
/// ```
///
/// Use instead:
/// ```python
/// {f(x) for x in foo}
/// set(foo)
/// set(foo)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryGeneratorSet {
    short_circuit: bool,
}

impl AlwaysFixableViolation for UnnecessaryGeneratorSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.short_circuit {
            "Unnecessary generator (rewrite using `set()`)".to_string()
        } else {
            "Unnecessary generator (rewrite as a set comprehension)".to_string()
        }
    }

    fn fix_title(&self) -> String {
        if self.short_circuit {
            "Rewrite using `set()`".to_string()
        } else {
            "Rewrite as a set comprehension".to_string()
        }
    }
}