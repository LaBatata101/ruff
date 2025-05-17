use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `jinja2` templates that use `autoescape=False`.
///
/// ## Why is this bad?
/// `jinja2` templates that use `autoescape=False` are vulnerable to cross-site
/// scripting (XSS) attacks that allow attackers to execute arbitrary
/// JavaScript.
///
/// By default, `jinja2` sets `autoescape` to `False`, so it is important to
/// set `autoescape=True` or use the `select_autoescape` function to mitigate
/// XSS vulnerabilities.
///
/// ## Example
/// ```python
/// import jinja2
///
/// jinja2.Environment(loader=jinja2.FileSystemLoader("."))
/// ```
///
/// Use instead:
/// ```python
/// import jinja2
///
/// jinja2.Environment(loader=jinja2.FileSystemLoader("."), autoescape=True)
/// ```
///
/// ## References
/// - [Jinja documentation: API](https://jinja.palletsprojects.com/en/latest/api/#autoescaping)
/// - [Common Weakness Enumeration: CWE-94](https://cwe.mitre.org/data/definitions/94.html)
#[derive(ViolationMetadata)]
pub struct Jinja2AutoescapeFalse {
    value: bool,
}

impl Violation for Jinja2AutoescapeFalse {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.value {
            "Using jinja2 templates with `autoescape=False` is dangerous and can lead to XSS. \
                 Ensure `autoescape=True` or use the `select_autoescape` function."
                .to_string()
        } else {
            "By default, jinja2 sets `autoescape` to `False`. Consider using \
                `autoescape=True` or the `select_autoescape` function to mitigate XSS \
                vulnerabilities."
                .to_string()
        }
    }
}