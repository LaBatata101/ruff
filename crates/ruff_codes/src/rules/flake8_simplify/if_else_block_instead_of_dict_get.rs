use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if` statements that can be replaced with `dict.get` calls.
///
/// ## Why is this bad?
/// `dict.get()` calls can be used to replace `if` statements that assign a
/// value to a variable in both branches, falling back to a default value if
/// the key is not found. When possible, using `dict.get` is more concise and
/// more idiomatic.
///
/// Under [preview mode](https://docs.astral.sh/ruff/preview), this rule will
/// also suggest replacing `if`-`else` _expressions_ with `dict.get` calls.
///
/// ## Example
/// ```python
/// if "bar" in foo:
///     value = foo["bar"]
/// else:
///     value = 0
/// ```
///
/// Use instead:
/// ```python
/// value = foo.get("bar", 0)
/// ```
///
/// If preview mode is enabled:
/// ```python
/// value = foo["bar"] if "bar" in foo else 0
/// ```
///
/// Use instead:
/// ```python
/// value = foo.get("bar", 0)
/// ```
///
/// ## References
/// - [Python documentation: Mapping Types](https://docs.python.org/3/library/stdtypes.html#mapping-types-dict)
#[derive(ViolationMetadata)]
pub struct IfElseBlockInsteadOfDictGet {
    contents: String,
}

impl Violation for IfElseBlockInsteadOfDictGet {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let IfElseBlockInsteadOfDictGet { contents } = self;
        format!("Use `{contents}` instead of an `if` block")
    }

    fn fix_title(&self) -> Option<String> {
        let IfElseBlockInsteadOfDictGet { contents } = self;
        Some(format!("Replace with `{contents}`"))
    }
}