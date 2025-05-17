use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions or methods with too many nested blocks.
///
/// By default, this rule allows up to five nested blocks.
/// This can be configured using the [`lint.pylint.max-nested-blocks`] option.
///
/// ## Why is this bad?
/// Functions or methods with too many nested blocks are harder to understand
/// and maintain.
///
/// ## Options
/// - `lint.pylint.max-nested-blocks`
#[derive(ViolationMetadata)]
pub struct TooManyNestedBlocks {
    nested_blocks: usize,
    max_nested_blocks: usize,
}

impl Violation for TooManyNestedBlocks {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyNestedBlocks {
            nested_blocks,
            max_nested_blocks,
        } = self;
        format!("Too many nested blocks ({nested_blocks} > {max_nested_blocks})")
    }
}