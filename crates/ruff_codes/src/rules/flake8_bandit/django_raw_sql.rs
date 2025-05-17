use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of Django's `RawSQL` function.
///
/// ## Why is this bad?
/// Django's `RawSQL` function can be used to execute arbitrary SQL queries,
/// which can in turn lead to SQL injection vulnerabilities.
///
/// ## Example
/// ```python
/// from django.db.models.expressions import RawSQL
/// from django.contrib.auth.models import User
///
/// User.objects.annotate(val=RawSQL("%s" % input_param, []))
/// ```
///
/// ## References
/// - [Django documentation: SQL injection protection](https://docs.djangoproject.com/en/dev/topics/security/#sql-injection-protection)
/// - [Common Weakness Enumeration: CWE-89](https://cwe.mitre.org/data/definitions/89.html)
#[derive(ViolationMetadata)]
pub struct DjangoRawSql;

impl Violation for DjangoRawSql {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of `RawSQL` can lead to SQL injection vulnerabilities".to_string()
    }
}