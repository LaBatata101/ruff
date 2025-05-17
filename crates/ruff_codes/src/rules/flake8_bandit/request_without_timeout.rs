use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the Python `requests` or `httpx` module that omit the
/// `timeout` parameter.
///
/// ## Why is this bad?
/// The `timeout` parameter is used to set the maximum time to wait for a
/// response from the server. By omitting the `timeout` parameter, the program
/// may hang indefinitely while awaiting a response.
///
/// ## Example
/// ```python
/// import requests
///
/// requests.get("https://www.example.com/")
/// ```
///
/// Use instead:
/// ```python
/// import requests
///
/// requests.get("https://www.example.com/", timeout=10)
/// ```
///
/// ## References
/// - [Requests documentation: Timeouts](https://requests.readthedocs.io/en/latest/user/advanced/#timeouts)
/// - [httpx documentation: Timeouts](https://www.python-httpx.org/advanced/timeouts/)
#[derive(ViolationMetadata)]
pub struct RequestWithoutTimeout {
    implicit: bool,
    module: String,
}

impl Violation for RequestWithoutTimeout {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RequestWithoutTimeout { implicit, module } = self;
        if *implicit {
            format!("Probable use of `{module}` call without timeout")
        } else {
            format!("Probable use of `{module}` call with timeout set to `None`")
        }
    }
}