use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `debug=True` in Flask.
///
/// ## Why is this bad?
/// Enabling debug mode shows an interactive debugger in the browser if an
/// error occurs, and allows running arbitrary Python code from the browser.
/// This could leak sensitive information, or allow an attacker to run
/// arbitrary code.
///
/// ## Example
/// ```python
/// import flask
///
/// app = Flask()
///
/// app.run(debug=True)
/// ```
///
/// Use instead:
/// ```python
/// import flask
///
/// app = Flask()
///
/// app.run(debug=os.environ["ENV"] == "dev")
/// ```
///
/// ## References
/// - [Flask documentation: Debug Mode](https://flask.palletsprojects.com/en/latest/quickstart/#debug-mode)
#[derive(ViolationMetadata)]
pub struct FlaskDebugTrue;

impl Violation for FlaskDebugTrue {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of `debug=True` in Flask app detected".to_string()
    }
}