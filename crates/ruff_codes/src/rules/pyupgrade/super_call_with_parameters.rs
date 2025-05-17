use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `super` calls that pass redundant arguments.
///
/// ## Why is this bad?
/// In Python 3, `super` can be invoked without any arguments when: (1) the
/// first argument is `__class__`, and (2) the second argument is equivalent to
/// the first argument of the enclosing method.
///
/// When possible, omit the arguments to `super` to make the code more concise
/// and maintainable.
///
/// ## Example
/// ```python
/// class A:
///     def foo(self):
///         pass
///
///
/// class B(A):
///     def bar(self):
///         super(B, self).foo()
/// ```
///
/// Use instead:
/// ```python
/// class A:
///     def foo(self):
///         pass
///
///
/// class B(A):
///     def bar(self):
///         super().foo()
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe because removing the arguments from a call
/// may delete comments that are attached to the arguments.
///
/// ## References
/// - [Python documentation: `super`](https://docs.python.org/3/library/functions.html#super)
/// - [super/MRO, Python's most misunderstood feature.](https://www.youtube.com/watch?v=X1PQ7zzltz4)
#[derive(ViolationMetadata)]
pub struct SuperCallWithParameters;

impl AlwaysFixableViolation for SuperCallWithParameters {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `super()` instead of `super(__class__, self)`".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `__super__` parameters".to_string()
    }
}