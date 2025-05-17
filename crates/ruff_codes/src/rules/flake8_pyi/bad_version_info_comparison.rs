use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of comparators other than `<` and `>=` for
/// `sys.version_info` checks. All other comparators, such
/// as `>`, `<=`, and `==`, are banned.
///
/// ## Why is this bad?
/// Comparing `sys.version_info` with `==` or `<=` has unexpected behavior
/// and can lead to bugs.
///
/// For example, `sys.version_info > (3, 8, 1)` will resolve to `True` if your
/// Python version is 3.8.1; meanwhile, `sys.version_info <= (3, 8)` will _not_
/// resolve to `True` if your Python version is 3.8.10:
///
/// ```python
/// >>> import sys
/// >>> print(sys.version_info)
/// sys.version_info(major=3, minor=8, micro=10, releaselevel='final', serial=0)
/// >>> print(sys.version_info > (3, 8))
/// True
/// >>> print(sys.version_info == (3, 8))
/// False
/// >>> print(sys.version_info <= (3, 8))
/// False
/// >>> print(sys.version_info in (3, 8))
/// False
/// ```
///
/// ## Example
/// ```py
/// import sys
///
/// if sys.version_info > (3, 8): ...
/// ```
///
/// Use instead:
/// ```py
/// import sys
///
/// if sys.version_info >= (3, 9): ...
/// ```
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct BadVersionInfoComparison;

impl Violation for BadVersionInfoComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `<` or `>=` for `sys.version_info` comparisons".to_string()
    }
}

/// ## What it does
/// Checks for code that branches on `sys.version_info` comparisons where
/// branches corresponding to older Python versions come before branches
/// corresponding to newer Python versions.
///
/// ## Why is this bad?
/// As a convention, branches that correspond to newer Python versions should
/// come first. This makes it easier to understand the desired behavior, which
/// typically corresponds to the latest Python versions.
///
/// This rule enforces the convention by checking for `if` tests that compare
/// `sys.version_info` with `<` rather than `>=`.
///
/// By default, this rule only applies to stub files.
/// In [preview], it will also flag this anti-pattern in non-stub files.
///
/// ## Example
///
/// ```pyi
/// import sys
///
/// if sys.version_info < (3, 10):
///     def read_data(x, *, preserve_order=True): ...
///
/// else:
///     def read_data(x): ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// if sys.version_info >= (3, 10):
///     def read_data(x): ...
///
/// else:
///     def read_data(x, *, preserve_order=True): ...
/// ```
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct BadVersionInfoOrder;

impl Violation for BadVersionInfoOrder {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Put branches for newer Python versions first when branching on `sys.version_info` comparisons".to_string()
    }
}
