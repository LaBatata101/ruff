use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary list or set comprehensions passed to builtin functions that take an iterable.
///
/// Set comprehensions are only a violation in the case where the builtin function does not care about
/// duplication of elements in the passed iterable.
///
/// ## Why is this bad?
/// Many builtin functions (this rule currently covers `any` and `all` in stable, along with `min`,
/// `max`, and `sum` in [preview]) accept any iterable, including a generator. Constructing a
/// temporary list via list comprehension is unnecessary and wastes memory for large iterables.
///
/// `any` and `all` can also short-circuit iteration, saving a lot of time. The unnecessary
/// comprehension forces a full iteration of the input iterable, giving up the benefits of
/// short-circuiting. For example, compare the performance of `all` with a list comprehension
/// against that of a generator in a case where an early short-circuit is possible (almost 40x
/// faster):
///
/// ```console
/// In [1]: %timeit all([i for i in range(1000)])
/// 8.14 µs ± 25.4 ns per loop (mean ± std. dev. of 7 runs, 100,000 loops each)
///
/// In [2]: %timeit all(i for i in range(1000))
/// 212 ns ± 0.892 ns per loop (mean ± std. dev. of 7 runs, 1,000,000 loops each)
/// ```
///
/// This performance improvement is due to short-circuiting. If the entire iterable has to be
/// traversed, the comprehension version may even be a bit faster: list allocation overhead is not
/// necessarily greater than generator overhead.
///
/// Applying this rule simplifies the code and will usually save memory, but in the absence of
/// short-circuiting it may not improve performance. (It may even slightly regress performance,
/// though the difference will usually be small.)
///
/// ## Example
/// ```python
/// any([x.id for x in bar])
/// all([x.id for x in bar])
/// sum([x.val for x in bar])
/// min([x.val for x in bar])
/// max([x.val for x in bar])
/// ```
///
/// Use instead:
/// ```python
/// any(x.id for x in bar)
/// all(x.id for x in bar)
/// sum(x.val for x in bar)
/// min(x.val for x in bar)
/// max(x.val for x in bar)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it can change the behavior of the code if the iteration
/// has side effects (due to laziness and short-circuiting). The fix may also drop comments when
/// rewriting some comprehensions.
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct UnnecessaryComprehensionInCall {
    comprehension_kind: ComprehensionKind,
}

impl Violation for UnnecessaryComprehensionInCall {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.comprehension_kind {
            ComprehensionKind::List => "Unnecessary list comprehension".to_string(),
            ComprehensionKind::Set => "Unnecessary set comprehension".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove unnecessary comprehension".to_string())
    }
}

// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_comprehension_in_call
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ComprehensionKind {
    List,
    Set,
}