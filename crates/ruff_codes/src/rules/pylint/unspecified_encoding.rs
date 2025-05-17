use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `open` and related calls without an explicit `encoding`
/// argument.
///
/// ## Why is this bad?
/// Using `open` in text mode without an explicit encoding can lead to
/// non-portable code, with differing behavior across platforms. While readers
/// may assume that UTF-8 is the default encoding, in reality, the default
/// is locale-specific.
///
/// Instead, consider using the `encoding` parameter to enforce a specific
/// encoding. [PEP 597] recommends the use of `encoding="utf-8"` as a default,
/// and suggests that it may become the default in future versions of Python.
///
/// If a locale-specific encoding is intended, use `encoding="locale"`  on
/// Python 3.10 and later, or `locale.getpreferredencoding()` on earlier versions,
/// to make the encoding explicit.
///
/// ## Fix safety
/// This fix is always unsafe and may change the program's behavior. It forces
/// `encoding="utf-8"` as the default, regardless of the platform’s actual default
/// encoding, which may cause `UnicodeDecodeError` on non-UTF-8 systems.
/// ```python
/// with open("test.txt") as f:
///     print(f.read()) # before fix (on UTF-8 systems): 你好，世界！
/// with open("test.txt", encoding="utf-8") as f:
///     print(f.read()) # after fix (on Windows): UnicodeDecodeError
/// ```
///
/// ## Example
/// ```python
/// open("file.txt")
/// ```
///
/// Use instead:
/// ```python
/// open("file.txt", encoding="utf-8")
/// ```
///
/// ## References
/// - [Python documentation: `open`](https://docs.python.org/3/library/functions.html#open)
///
/// [PEP 597]: https://peps.python.org/pep-0597/
#[derive(ViolationMetadata)]
pub struct UnspecifiedEncoding {
    function_name: String,
    mode: ModeArgument,
}

impl AlwaysFixableViolation for UnspecifiedEncoding {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnspecifiedEncoding {
            function_name,
            mode,
        } = self;

        match mode {
            ModeArgument::Supported => {
                format!("`{function_name}` in text mode without explicit `encoding` argument")
            }
            ModeArgument::Unsupported => {
                format!("`{function_name}` without explicit `encoding` argument")
            }
        }
    }

    fn fix_title(&self) -> String {
        "Add explicit `encoding` argument".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModeArgument {
    /// The call supports a `mode` argument.
    Supported,
    /// The call does not support a `mode` argument.
    Unsupported,
}