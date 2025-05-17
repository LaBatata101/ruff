use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `metaclass=abc.ABCMeta` to define abstract base classes
/// (ABCs).
///
/// ## Why is this bad?
///
/// Instead of `class C(metaclass=abc.ABCMeta): ...`, use `class C(ABC): ...`
/// to define an abstract base class. Inheriting from the `ABC` wrapper class
/// is semantically identical to setting `metaclass=abc.ABCMeta`, but more
/// succinct.
///
/// ## Example
/// ```python
/// class C(metaclass=ABCMeta):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// class C(ABC):
///     pass
/// ```
///
/// ## References
/// - [Python documentation: `abc.ABC`](https://docs.python.org/3/library/abc.html#abc.ABC)
/// - [Python documentation: `abc.ABCMeta`](https://docs.python.org/3/library/abc.html#abc.ABCMeta)
#[derive(ViolationMetadata)]
pub struct MetaClassABCMeta;

impl AlwaysFixableViolation for MetaClassABCMeta {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of `metaclass=abc.ABCMeta` to define abstract base class".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `abc.ABC`".to_string()
    }
}