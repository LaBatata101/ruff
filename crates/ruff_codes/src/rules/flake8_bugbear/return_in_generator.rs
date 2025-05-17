use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `return {value}` statements in functions that also contain `yield`
/// or `yield from` statements.
///
/// ## Why is this bad?
/// Using `return {value}` in a generator function was syntactically invalid in
/// Python 2. In Python 3 `return {value}` _can_ be used in a generator; however,
/// the combination of `yield` and `return` can lead to confusing behavior, as
/// the `return` statement will cause the generator to raise `StopIteration`
/// with the value provided, rather than returning the value to the caller.
///
/// For example, given:
/// ```python
/// from collections.abc import Iterable
/// from pathlib import Path
///
///
/// def get_file_paths(file_types: Iterable[str] | None = None) -> Iterable[Path]:
///     dir_path = Path(".")
///     if file_types is None:
///         return dir_path.glob("*")
///
///     for file_type in file_types:
///         yield from dir_path.glob(f"*.{file_type}")
/// ```
///
/// Readers might assume that `get_file_paths()` would return an iterable of
/// `Path` objects in the directory; in reality, though, `list(get_file_paths())`
/// evaluates to `[]`, since the `return` statement causes the generator to raise
/// `StopIteration` with the value `dir_path.glob("*")`:
///
/// ```shell
/// >>> list(get_file_paths(file_types=["cfg", "toml"]))
/// [PosixPath('setup.cfg'), PosixPath('pyproject.toml')]
/// >>> list(get_file_paths())
/// []
/// ```
///
/// For intentional uses of `return` in a generator, consider suppressing this
/// diagnostic.
///
/// ## Example
/// ```python
/// from collections.abc import Iterable
/// from pathlib import Path
///
///
/// def get_file_paths(file_types: Iterable[str] | None = None) -> Iterable[Path]:
///     dir_path = Path(".")
///     if file_types is None:
///         return dir_path.glob("*")
///
///     for file_type in file_types:
///         yield from dir_path.glob(f"*.{file_type}")
/// ```
///
/// Use instead:
///
/// ```python
/// from collections.abc import Iterable
/// from pathlib import Path
///
///
/// def get_file_paths(file_types: Iterable[str] | None = None) -> Iterable[Path]:
///     dir_path = Path(".")
///     if file_types is None:
///         yield from dir_path.glob("*")
///     else:
///         for file_type in file_types:
///             yield from dir_path.glob(f"*.{file_type}")
/// ```
#[derive(ViolationMetadata)]
pub struct ReturnInGenerator;

impl Violation for ReturnInGenerator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `yield` and `return {value}` in a generator function can lead to confusing behavior"
            .to_string()
    }
}