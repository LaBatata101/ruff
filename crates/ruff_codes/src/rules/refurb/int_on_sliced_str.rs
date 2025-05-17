use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `int` with an explicit base in which a string expression
/// is stripped of its leading prefix (i.e., `0b`, `0o`, or `0x`).
///
/// ## Why is this bad?
/// Given an integer string with a prefix (e.g., `0xABC`), Python can automatically
/// determine the base of the integer by the prefix without needing to specify
/// it explicitly.
///
/// Instead of `int(num[2:], 16)`, use `int(num, 0)`, which will automatically
/// deduce the base based on the prefix.
///
/// ## Example
/// ```python
/// num = "0xABC"
///
/// if num.startswith("0b"):
///     i = int(num[2:], 2)
/// elif num.startswith("0o"):
///     i = int(num[2:], 8)
/// elif num.startswith("0x"):
///     i = int(num[2:], 16)
///
/// print(i)
/// ```
///
/// Use instead:
/// ```python
/// num = "0xABC"
///
/// i = int(num, 0)
///
/// print(i)
/// ```
///
/// ## Fix safety
/// The rule's fix is marked as unsafe, as Ruff cannot guarantee that the
/// argument to `int` will remain valid when its base is included in the
/// function call.
///
/// ## References
/// - [Python documentation: `int`](https://docs.python.org/3/library/functions.html#int)
#[derive(ViolationMetadata)]
pub struct IntOnSlicedStr {
    base: u8,
}

impl AlwaysFixableViolation for IntOnSlicedStr {
    #[derive_message_formats]
    fn message(&self) -> String {
        let IntOnSlicedStr { base } = self;
        format!("Use of `int` with explicit `base={base}` after removing prefix")
    }

    fn fix_title(&self) -> String {
        "Replace with `base=0`".to_string()
    }
}