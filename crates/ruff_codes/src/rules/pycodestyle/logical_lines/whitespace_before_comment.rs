use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks if inline comments are separated by at least two spaces.
///
/// ## Why is this bad?
/// An inline comment is a comment on the same line as a statement.
///
/// Per [PEP 8], inline comments should be separated by at least two spaces from
/// the preceding statement.
///
/// ## Example
/// ```python
/// x = x + 1 # Increment x
/// ```
///
/// Use instead:
/// ```python
/// x = x + 1  # Increment x
/// x = x + 1    # Increment x
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#comments
#[derive(ViolationMetadata)]
pub struct TooFewSpacesBeforeInlineComment;

impl AlwaysFixableViolation for TooFewSpacesBeforeInlineComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Insert at least two spaces before an inline comment".to_string()
    }

    fn fix_title(&self) -> String {
        "Insert spaces".to_string()
    }
}

/// ## What it does
/// Checks if one space is used after inline comments.
///
/// ## Why is this bad?
/// An inline comment is a comment on the same line as a statement.
///
/// Per [PEP 8], inline comments should start with a # and a single space.
///
/// ## Example
/// ```python
/// x = x + 1  #Increment x
/// x = x + 1  #  Increment x
/// x = x + 1  # \xa0Increment x
/// ```
///
/// Use instead:
/// ```python
/// x = x + 1  # Increment x
/// x = x + 1    # Increment x
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#comments
#[derive(ViolationMetadata)]
pub struct NoSpaceAfterInlineComment;

impl AlwaysFixableViolation for NoSpaceAfterInlineComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Inline comment should start with `# `".to_string()
    }

    fn fix_title(&self) -> String {
        "Format space".to_string()
    }
}

/// ## What it does
/// Checks for block comments that lack a single space after the leading `#` character.
///
/// ## Why is this bad?
/// Per [PEP 8], "Block comments generally consist of one or more paragraphs built
/// out of complete sentences, with each sentence ending in a period."
///
/// Block comments should start with a `#` followed by a single space.
///
/// Shebangs (lines starting with `#!`, at the top of a file) are exempt from this
/// rule.
///
/// ## Example
/// ```python
/// #Block comment
/// ```
///
/// Use instead:
/// ```python
/// # Block comment
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#comments
#[derive(ViolationMetadata)]
pub struct NoSpaceAfterBlockComment;

impl AlwaysFixableViolation for NoSpaceAfterBlockComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Block comment should start with `# `".to_string()
    }

    fn fix_title(&self) -> String {
        "Format space".to_string()
    }
}

/// ## What it does
/// Checks for block comments that start with multiple leading `#` characters.
///
/// ## Why is this bad?
/// Per [PEP 8], "Block comments generally consist of one or more paragraphs built
/// out of complete sentences, with each sentence ending in a period."
///
/// Each line of a block comment should start with a `#` followed by a single space.
///
/// Shebangs (lines starting with `#!`, at the top of a file) are exempt from this
/// rule.
///
/// ## Example
/// ```python
/// ### Block comment
/// ```
///
/// Use instead:
/// ```python
/// # Block comment
/// ```
///
/// Alternatively, this rule makes an exception for comments that consist
/// solely of `#` characters, as in:
///
/// ```python
/// ##############
/// # Block header
/// ##############
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#comments
#[derive(ViolationMetadata)]
pub struct MultipleLeadingHashesForBlockComment;

impl AlwaysFixableViolation for MultipleLeadingHashesForBlockComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Too many leading `#` before block comment".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove leading `#`".to_string()
    }
}
