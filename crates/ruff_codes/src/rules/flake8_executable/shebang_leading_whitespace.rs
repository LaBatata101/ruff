use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for whitespace before a shebang directive.
///
/// ## Why is this bad?
/// In Python, a shebang (also known as a hashbang) is the first line of a
/// script, which specifies the interpreter that should be used to run the
/// script.
///
/// The shebang's `#!` prefix must be the first two characters of a file. The
/// presence of whitespace before the shebang will cause the shebang to be
/// ignored, which is likely a mistake.
///
/// ## Example
/// ```python
///  #!/usr/bin/env python3
/// ```
///
/// Use instead:
/// ```python
/// #!/usr/bin/env python3
/// ```
///
/// ## References
/// - [Python documentation: Executable Python Scripts](https://docs.python.org/3/tutorial/appendix.html#executable-python-scripts)
#[derive(ViolationMetadata)]
pub struct ShebangLeadingWhitespace;

impl AlwaysFixableViolation for ShebangLeadingWhitespace {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid whitespace before shebang".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove whitespace before shebang".to_string()
    }
}