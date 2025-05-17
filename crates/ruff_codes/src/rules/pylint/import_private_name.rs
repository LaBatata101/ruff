use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for import statements that import a private name (a name starting
/// with an underscore `_`) from another module.
///
/// ## Why is this bad?
/// [PEP 8] states that names starting with an underscore are private. Thus,
/// they are not intended to be used outside of the module in which they are
/// defined.
///
/// Further, as private imports are not considered part of the public API, they
/// are prone to unexpected changes, especially outside of semantic versioning.
///
/// Instead, consider using the public API of the module.
///
/// This rule ignores private name imports that are exclusively used in type
/// annotations. Ideally, types would be public; however, this is not always
/// possible when using third-party libraries.
///
/// ## Known problems
/// Does not ignore private name imports from within the module that defines
/// the private name if the module is defined with [PEP 420] namespace packages
/// (i.e., directories that omit the `__init__.py` file). Namespace packages
/// must be configured via the [`namespace-packages`] setting.
///
/// ## Example
/// ```python
/// from foo import _bar
/// ```
///
/// ## Options
/// - `namespace-packages`
///
/// ## References
/// - [PEP 8: Naming Conventions](https://peps.python.org/pep-0008/#naming-conventions)
/// - [Semantic Versioning](https://semver.org/)
///
/// [PEP 8]: https://peps.python.org/pep-0008/
/// [PEP 420]: https://peps.python.org/pep-0420/
#[derive(ViolationMetadata)]
pub struct ImportPrivateName {
    name: String,
    module: Option<String>,
}

impl Violation for ImportPrivateName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ImportPrivateName { name, module } = self;
        match module {
            Some(module) => {
                format!("Private name import `{name}` from external module `{module}`")
            }
            None => format!("Private name import `{name}`"),
        }
    }
}