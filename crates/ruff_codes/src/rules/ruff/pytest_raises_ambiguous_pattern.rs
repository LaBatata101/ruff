use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for non-raw literal string arguments passed to the `match` parameter
/// of `pytest.raises()` where the string contains at least one unescaped
/// regex metacharacter.
///
/// ## Why is this bad?
/// The `match` argument is implicitly converted to a regex under the hood.
/// It should be made explicit whether the string is meant to be a regex or a "plain" pattern
/// by prefixing the string with the `r` suffix, escaping the metacharacter(s)
/// in the string using backslashes, or wrapping the entire string in a call to
/// `re.escape()`.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// with pytest.raises(Exception, match="A full sentence."):
///     do_thing_that_raises()
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// with pytest.raises(Exception, match=r"A full sentence."):
///     do_thing_that_raises()
/// ```
///
/// Alternatively:
///
/// ```python
/// import pytest
/// import re
///
///
/// with pytest.raises(Exception, match=re.escape("A full sentence.")):
///     do_thing_that_raises()
/// ```
///
/// or:
///
/// ```python
/// import pytest
/// import re
///
///
/// with pytest.raises(Exception, "A full sentence\\."):
///     do_thing_that_raises()
/// ```
///
/// ## References
/// - [Python documentation: `re.escape`](https://docs.python.org/3/library/re.html#re.escape)
/// - [`pytest` documentation: `pytest.raises`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-raises)
#[derive(ViolationMetadata)]
pub struct PytestRaisesAmbiguousPattern;

impl Violation for PytestRaisesAmbiguousPattern {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Pattern passed to `match=` contains metacharacters but is neither escaped nor raw"
            .to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use a raw string or `re.escape()` to make the intention explicit".to_string())
    }
}