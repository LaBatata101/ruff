use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the `xml.etree.cElementTree` module.
///
/// ## Why is this bad?
/// In Python 3.3, `xml.etree.cElementTree` was deprecated in favor of
/// `xml.etree.ElementTree`.
///
/// ## Example
/// ```python
/// from xml.etree import cElementTree
/// ```
///
/// Use instead:
/// ```python
/// from xml.etree import ElementTree
/// ```
///
/// ## References
/// - [Python documentation: `xml.etree.ElementTree`](https://docs.python.org/3/library/xml.etree.elementtree.html)
#[derive(ViolationMetadata)]
pub struct DeprecatedCElementTree;

impl AlwaysFixableViolation for DeprecatedCElementTree {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`cElementTree` is deprecated, use `ElementTree`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `ElementTree`".to_string()
    }
}