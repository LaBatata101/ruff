use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for imports that are required at runtime but are only defined in
/// type-checking blocks.
///
/// ## Why is this bad?
/// The type-checking block is not executed at runtime, so if the only definition
/// of a symbol is in a type-checking block, it will not be available at runtime.
///
/// If [`lint.flake8-type-checking.quote-annotations`] is set to `true`,
/// annotations will be wrapped in quotes if doing so would enable the
/// corresponding import to remain in the type-checking block.
///
/// ## Example
/// ```python
/// from typing import TYPE_CHECKING
///
/// if TYPE_CHECKING:
///     import foo
///
///
/// def bar() -> None:
///     foo.bar()  # raises NameError: name 'foo' is not defined
/// ```
///
/// Use instead:
/// ```python
/// import foo
///
///
/// def bar() -> None:
///     foo.bar()
/// ```
///
/// ## Options
/// - `lint.flake8-type-checking.quote-annotations`
///
/// ## References
/// - [PEP 563: Runtime annotation resolution and `TYPE_CHECKING`](https://peps.python.org/pep-0563/#runtime-annotation-resolution-and-type-checking)
#[derive(ViolationMetadata)]
pub struct RuntimeImportInTypeCheckingBlock {
    qualified_name: String,
    strategy: Strategy,
}

impl Violation for RuntimeImportInTypeCheckingBlock {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self {
            qualified_name,
            strategy,
        } = self;
        match strategy {
            Strategy::MoveImport => format!(
                "Move import `{qualified_name}` out of type-checking block. Import is used for more than type hinting."
            ),
            Strategy::QuoteUsages => format!(
                "Quote references to `{qualified_name}`. Import is in a type-checking block."
            ),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Self { strategy, .. } = self;
        match strategy {
            Strategy::MoveImport => Some("Move out of type-checking block".to_string()),
            Strategy::QuoteUsages => Some("Quote references".to_string()),
        }
    }
}

// FIX: duplicated with ruff_rule_flake8_type_checking::runtime_in_type_checking_block
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strategy {
    /// The import should be moved out of the type-checking block.
    ///
    /// This is required when at least one reference to the symbol is in a runtime-required context.
    /// For example, given `from foo import Bar`, `x = Bar()` would be runtime-required.
    MoveImport,
    /// All usages of the import should be wrapped in quotes.
    ///
    /// This is acceptable when all references to the symbol are in a runtime-evaluated, but not
    /// runtime-required context. For example, given `from foo import Bar`, `x: Bar` would be
    /// runtime-evaluated, but not runtime-required.
    QuoteUsages,
}
