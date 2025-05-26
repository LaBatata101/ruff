use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for a shebang directive that is not at the beginning of the file.
///
/// ## Why is this bad?
/// In Python, a shebang (also known as a hashbang) is the first line of a
/// script, which specifies the interpreter that should be used to run the
/// script.
///
/// The shebang's `#!` prefix must be the first two characters of a file. If
/// the shebang is not at the beginning of the file, it will be ignored, which
/// is likely a mistake.
///
/// ## Example
/// ```python
/// foo = 1
/// #!/usr/bin/env python3
/// ```
///
/// Use instead:
/// ```python
/// #!/usr/bin/env python3
/// foo = 1
/// ```
///
/// ## References
/// - [Python documentation: Executable Python Scripts](https://docs.python.org/3/tutorial/appendix.html#executable-python-scripts)
#[derive(ViolationMetadata)]
pub struct ShebangNotFirstLine;

impl Violation for ShebangNotFirstLine {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Shebang should be at the beginning of the file".to_string()
    }
}