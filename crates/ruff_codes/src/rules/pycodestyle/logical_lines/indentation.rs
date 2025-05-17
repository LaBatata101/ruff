use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for indentation with a non-multiple of 4 spaces.
///
/// ## Why is this bad?
/// According to [PEP 8], 4 spaces per indentation level should be preferred.
///
/// ## Example
/// ```python
/// if True:
///    a = 1
/// ```
///
/// Use instead:
/// ```python
/// if True:
///     a = 1
/// ```
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces consistent indentation, making the rule redundant.
///
/// The rule is also incompatible with the [formatter] when using
/// `indent-width` with a value other than `4`.
///
/// ## Options
/// - `indent-width`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#indentation
/// [formatter]:https://docs.astral.sh/ruff/formatter/
#[derive(ViolationMetadata)]
pub struct IndentationWithInvalidMultiple {
    indent_width: usize,
}

impl Violation for IndentationWithInvalidMultiple {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { indent_width } = self;
        format!("Indentation is not a multiple of {indent_width}")
    }
}

/// ## What it does
/// Checks for indentation of comments with a non-multiple of 4 spaces.
///
/// ## Why is this bad?
/// According to [PEP 8], 4 spaces per indentation level should be preferred.
///
/// ## Example
/// ```python
/// if True:
///    # a = 1
/// ```
///
/// Use instead:
/// ```python
/// if True:
///     # a = 1
/// ```
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces consistent indentation, making the rule redundant.
///
/// The rule is also incompatible with the [formatter] when using
/// `indent-width` with a value other than `4`.
///
/// ## Options
/// - `indent-width`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#indentation
/// [formatter]:https://docs.astral.sh/ruff/formatter/
#[derive(ViolationMetadata)]
pub struct IndentationWithInvalidMultipleComment {
    indent_width: usize,
}

impl Violation for IndentationWithInvalidMultipleComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { indent_width } = self;
        format!("Indentation is not a multiple of {indent_width} (comment)")
    }
}

/// ## What it does
/// Checks for indented blocks that are lacking indentation.
///
/// ## Why is this bad?
/// All indented blocks should be indented; otherwise, they are not valid
/// Python syntax.
///
/// ## Example
/// ```python
/// for item in items:
/// pass
/// ```
///
/// Use instead:
/// ```python
/// for item in items:
///     pass
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#indentation
#[derive(ViolationMetadata)]
pub struct NoIndentedBlock;

impl Violation for NoIndentedBlock {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Expected an indented block".to_string()
    }
}

/// ## What it does
/// Checks for comments in a code blocks that are lacking indentation.
///
/// ## Why is this bad?
/// Comments within an indented block should themselves be indented, to
/// indicate that they are part of the block.
///
/// ## Example
/// ```python
/// for item in items:
/// # Hi
///     pass
/// ```
///
/// Use instead:
/// ```python
/// for item in items:
///     # Hi
///     pass
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#indentation
#[derive(ViolationMetadata)]
pub struct NoIndentedBlockComment;

impl Violation for NoIndentedBlockComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Expected an indented block (comment)".to_string()
    }
}

/// ## What it does
/// Checks for unexpected indentation.
///
/// ## Why is this bad?
/// Indentation outside of a code block is not valid Python syntax.
///
/// ## Example
/// ```python
/// a = 1
///     b = 2
/// ```
///
/// Use instead:
/// ```python
/// a = 1
/// b = 2
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#indentation
#[derive(ViolationMetadata)]
pub struct UnexpectedIndentation;

impl Violation for UnexpectedIndentation {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unexpected indentation".to_string()
    }
}

/// ## What it does
/// Checks for unexpected indentation of comment.
///
/// ## Why is this bad?
/// Comments should match the indentation of the containing code block.
///
/// ## Example
/// ```python
/// a = 1
///     # b = 2
/// ```
///
/// Use instead:
/// ```python
/// a = 1
/// # b = 2
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#indentation
#[derive(ViolationMetadata)]
pub struct UnexpectedIndentationComment;

impl Violation for UnexpectedIndentationComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unexpected indentation (comment)".to_string()
    }
}

/// ## What it does
/// Checks for over-indented code.
///
/// ## Why is this bad?
/// According to [PEP 8], 4 spaces per indentation level should be preferred. Increased
/// indentation can lead to inconsistent formatting, which can hurt
/// readability.
///
/// ## Example
/// ```python
/// for item in items:
///       pass
/// ```
///
/// Use instead:
/// ```python
/// for item in items:
///     pass
/// ```
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces consistent indentation, making the rule redundant.
///
/// [PEP 8]: https://peps.python.org/pep-0008/#indentation
/// [formatter]:https://docs.astral.sh/ruff/formatter/
#[derive(ViolationMetadata)]
pub struct OverIndented {
    is_comment: bool,
}

impl Violation for OverIndented {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.is_comment {
            "Over-indented (comment)".to_string()
        } else {
            "Over-indented".to_string()
        }
    }
}
