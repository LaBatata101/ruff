use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of `IOBase.write` in a for loop.
///
/// ## Why is this bad?
/// When writing a batch of elements, it's more idiomatic to use a single method call to
/// `IOBase.writelines`, rather than write elements one by one.
///
/// ## Example
/// ```python
/// with Path("file").open("w") as f:
///     for line in lines:
///         f.write(line)
///
/// with Path("file").open("wb") as f:
///     for line in lines:
///         f.write(line.encode())
/// ```
///
/// Use instead:
/// ```python
/// with Path("file").open("w") as f:
///     f.writelines(lines)
///
/// with Path("file").open("wb") as f:
///     f.writelines(line.encode() for line in lines)
/// ```
///
/// ## References
/// - [Python documentation: `io.IOBase.writelines`](https://docs.python.org/3/library/io.html#io.IOBase.writelines)
#[derive(ViolationMetadata)]
pub struct ForLoopWrites {
    name: String,
}

impl AlwaysFixableViolation for ForLoopWrites {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use of `{}.write` in a for loop", self.name)
    }

    fn fix_title(&self) -> String {
        format!("Replace with `{}.writelines`", self.name)
    }
}