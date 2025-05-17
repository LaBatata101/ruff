use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for module names that do not follow the `snake_case` naming
/// convention or are otherwise invalid.
///
/// ## Why is this bad?
/// [PEP 8] recommends the use of the `snake_case` naming convention for
/// module names:
///
/// > Modules should have short, all-lowercase names. Underscores can be used in the
/// > module name if it improves readability. Python packages should also have short,
/// > all-lowercase names, although the use of underscores is discouraged.
/// >
/// > When an extension module written in C or C++ has an accompanying Python module that
/// > provides a higher level (e.g. more object-oriented) interface, the C/C++ module has
/// > a leading underscore (e.g. `_socket`).
///
/// Further, in order for Python modules to be importable, they must be valid
/// identifiers. As such, they cannot start with a digit, or collide with hard
/// keywords, like `import` or `class`.
///
/// ## Example
/// - Instead of `example-module-name` or `example module name`, use `example_module_name`.
/// - Instead of `ExampleModule`, use `example_module`.
///
/// [PEP 8]: https://peps.python.org/pep-0008/#package-and-module-names
#[derive(ViolationMetadata)]
pub struct InvalidModuleName {
    name: String,
}

impl Violation for InvalidModuleName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let InvalidModuleName { name } = self;
        format!("Invalid module name: '{name}'")
    }
}