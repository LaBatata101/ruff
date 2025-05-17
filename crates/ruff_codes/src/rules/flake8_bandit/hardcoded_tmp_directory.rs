use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of hardcoded temporary file or directory paths.
///
/// ## Why is this bad?
/// The use of hardcoded paths for temporary files can be insecure. If an
/// attacker discovers the location of a hardcoded path, they can replace the
/// contents of the file or directory with a malicious payload.
///
/// Other programs may also read or write contents to these hardcoded paths,
/// causing unexpected behavior.
///
/// ## Example
/// ```python
/// with open("/tmp/foo.txt", "w") as file:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// import tempfile
///
/// with tempfile.NamedTemporaryFile() as file:
///     ...
/// ```
///
/// ## Options
/// - `lint.flake8-bandit.hardcoded-tmp-directory`
/// - `lint.flake8-bandit.hardcoded-tmp-directory-extend`
///
/// ## References
/// - [Common Weakness Enumeration: CWE-377](https://cwe.mitre.org/data/definitions/377.html)
/// - [Common Weakness Enumeration: CWE-379](https://cwe.mitre.org/data/definitions/379.html)
/// - [Python documentation: `tempfile`](https://docs.python.org/3/library/tempfile.html)
#[derive(ViolationMetadata)]
pub struct HardcodedTempFile {
    string: String,
}

impl Violation for HardcodedTempFile {
    #[derive_message_formats]
    fn message(&self) -> String {
        let HardcodedTempFile { string } = self;
        format!(
            "Probable insecure usage of temporary file or directory: \"{}\"",
            string.escape_debug()
        )
    }
}