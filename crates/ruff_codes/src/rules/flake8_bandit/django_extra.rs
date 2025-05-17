use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of Django's `extra` function where one or more arguments
/// passed are not literal expressions.
///
/// ## Why is this bad?
/// Django's `extra` function can be used to execute arbitrary SQL queries,
/// which can in turn lead to SQL injection vulnerabilities.
///
/// ## Example
/// ```python
/// from django.contrib.auth.models import User
///
/// # String interpolation creates a security loophole that could be used
/// # for SQL injection:
/// User.objects.all().extra(select={"test": "%secure" % "nos"})
/// ```
///
/// Use instead:
/// ```python
/// from django.contrib.auth.models import User
///
/// # SQL injection is impossible if all arguments are literal expressions:
/// User.objects.all().extra(select={"test": "secure"})
/// ```
///
/// ## References
/// - [Django documentation: SQL injection protection](https://docs.djangoproject.com/en/dev/topics/security/#sql-injection-protection)
/// - [Common Weakness Enumeration: CWE-89](https://cwe.mitre.org/data/definitions/89.html)
#[derive(ViolationMetadata)]
pub struct DjangoExtra;

impl Violation for DjangoExtra {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of Django `extra` can lead to SQL injection vulnerabilities".to_string()
    }
}