use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `.split(os.sep)`
///
/// ## Why is this bad?
/// The `pathlib` module in the standard library should be used for path
/// manipulation. It provides a high-level API with the functionality
/// needed for common operations on `Path` objects.
///
/// ## Example
/// If not all parts of the path are needed, then the `name` and `parent`
/// attributes of the `Path` object should be used. Otherwise, the `parts`
/// attribute can be used as shown in the last example.
/// ```python
/// import os
///
/// "path/to/file_name.txt".split(os.sep)[-1]
///
/// "path/to/file_name.txt".split(os.sep)[-2]
///
/// # Iterating over the path parts
/// if any(part in blocklist for part in "my/file/path".split(os.sep)):
///     ...
/// ```
///
/// Use instead:
/// ```python
/// from pathlib import Path
///
/// Path("path/to/file_name.txt").name
///
/// Path("path/to/file_name.txt").parent.name
///
/// # Iterating over the path parts
/// if any(part in blocklist for part in Path("my/file/path").parts):
///     ...
/// ```
///
/// ## Known issues
/// While using `pathlib` can improve the readability and type safety of your code,
/// it can be less performant than working directly with strings,
/// especially on older versions of Python.
///
/// ## References
/// - [PEP 428 – The pathlib module – object-oriented filesystem paths](https://peps.python.org/pep-0428/)
/// - [Why you should be using pathlib](https://treyhunner.com/2018/12/why-you-should-be-using-pathlib/)
/// - [No really, pathlib is great](https://treyhunner.com/2019/01/no-really-pathlib-is-great/)
#[derive(ViolationMetadata)]
pub struct OsSepSplit;

impl Violation for OsSepSplit {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Replace `.split(os.sep)` with `Path.parts`".to_string()
    }
}