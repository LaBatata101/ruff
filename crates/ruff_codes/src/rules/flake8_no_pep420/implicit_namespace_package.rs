use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for packages that are missing an `__init__.py` file.
///
/// ## Why is this bad?
/// Python packages are directories that contain a file named `__init__.py`.
/// The existence of this file indicates that the directory is a Python
/// package, and so it can be imported the same way a module can be
/// imported.
///
/// Directories that lack an `__init__.py` file can still be imported, but
/// they're indicative of a special kind of package, known as a "namespace
/// package" (see: [PEP 420](https://peps.python.org/pep-0420/)).
/// Namespace packages are less widely used, so a package that lacks an
/// `__init__.py` file is typically meant to be a regular package, and
/// the absence of the `__init__.py` file is probably an oversight.
///
/// ## Options
/// - `namespace-packages`
#[derive(ViolationMetadata)]
pub struct ImplicitNamespacePackage {
    filename: String,
    parent: Option<String>,
}

impl Violation for ImplicitNamespacePackage {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ImplicitNamespacePackage { filename, parent } = self;
        match parent {
            None => {
                format!("File `{filename}` is part of an implicit namespace package. Add an `__init__.py`.")
            }
            Some(parent) => {
                format!("File `{filename}` declares a package, but is nested under an implicit namespace package. Add an `__init__.py` to `{parent}`.")
            }
        }
    }
}