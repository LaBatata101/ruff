use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for incorrect usages of `default_factory` as a keyword argument when
/// initializing a `defaultdict`.
///
/// ## Why is this bad?
/// The `defaultdict` constructor accepts a callable as its first argument.
/// For example, it's common to initialize a `defaultdict` with `int` or `list`
/// via `defaultdict(int)` or `defaultdict(list)`, to create a dictionary that
/// returns `0` or `[]` respectively when a key is missing.
///
/// The default factory _must_ be provided as a positional argument, as all
/// keyword arguments to `defaultdict` are interpreted as initial entries in
/// the dictionary. For example, `defaultdict(foo=1, bar=2)` will create a
/// dictionary with `{"foo": 1, "bar": 2}` as its initial entries.
///
/// As such, `defaultdict(default_factory=list)` will create a dictionary with
/// `{"default_factory": list}` as its initial entry, instead of a dictionary
/// that returns `[]` when a key is missing. Specifying a `default_factory`
/// keyword argument is almost always a mistake, and one that type checkers
/// can't reliably detect.
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as converting `default_factory` from a
/// keyword to a positional argument will change the behavior of the code, even
/// if the keyword argument was used erroneously.
///
/// ## Example
/// ```python
/// defaultdict(default_factory=int)
/// defaultdict(default_factory=list)
/// ```
///
/// Use instead:
/// ```python
/// defaultdict(int)
/// defaultdict(list)
/// ```
#[derive(ViolationMetadata)]
pub struct DefaultFactoryKwarg {
    default_factory: SourceCodeSnippet,
}

impl Violation for DefaultFactoryKwarg {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`default_factory` is a positional-only argument to `defaultdict`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        let DefaultFactoryKwarg { default_factory } = self;
        if let Some(default_factory) = default_factory.full_display() {
            Some(format!("Replace with `defaultdict({default_factory})`"))
        } else {
            Some("Use positional argument".to_string())
        }
    }
}