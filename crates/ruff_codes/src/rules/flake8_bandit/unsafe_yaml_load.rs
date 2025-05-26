use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the `yaml.load` function.
///
/// ## Why is this bad?
/// Running the `yaml.load` function over untrusted YAML files is insecure, as
/// `yaml.load` allows for the creation of arbitrary Python objects, which can
/// then be used to execute arbitrary code.
///
/// Instead, consider using `yaml.safe_load`, which allows for the creation of
/// simple Python objects like integers and lists, but prohibits the creation of
/// more complex objects like functions and classes.
///
/// ## Example
/// ```python
/// import yaml
///
/// yaml.load(untrusted_yaml)
/// ```
///
/// Use instead:
/// ```python
/// import yaml
///
/// yaml.safe_load(untrusted_yaml)
/// ```
///
/// ## References
/// - [PyYAML documentation: Loading YAML](https://pyyaml.org/wiki/PyYAMLDocumentation)
/// - [Common Weakness Enumeration: CWE-20](https://cwe.mitre.org/data/definitions/20.html)
#[derive(ViolationMetadata)]
pub struct UnsafeYAMLLoad {
    pub loader: Option<String>,
}

impl Violation for UnsafeYAMLLoad {
    #[derive_message_formats]
    fn message(&self) -> String {
        match &self.loader {
            Some(name) => {
                format!(
                    "Probable use of unsafe loader `{name}` with `yaml.load`. Allows \
                     instantiation of arbitrary objects. Consider `yaml.safe_load`."
                )
            }
            None => {
                "Probable use of unsafe `yaml.load`. Allows instantiation of arbitrary objects. \
                 Consider `yaml.safe_load`."
                    .to_string()
            }
        }
    }
}