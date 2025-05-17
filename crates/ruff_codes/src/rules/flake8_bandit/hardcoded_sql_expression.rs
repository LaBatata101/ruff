use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for strings that resemble SQL statements involved in some form
/// string building operation.
///
/// ## Why is this bad?
/// SQL injection is a common attack vector for web applications. Directly
/// interpolating user input into SQL statements should always be avoided.
/// Instead, favor parameterized queries, in which the SQL statement is
/// provided separately from its parameters, as supported by `psycopg3`
/// and other database drivers and ORMs.
///
/// ## Example
/// ```python
/// query = "DELETE FROM foo WHERE id = '%s'" % identifier
/// ```
///
/// ## References
/// - [B608: Test for SQL injection](https://bandit.readthedocs.io/en/latest/plugins/b608_hardcoded_sql_expressions.html)
/// - [psycopg3: Server-side binding](https://www.psycopg.org/psycopg3/docs/basic/from_pg2.html#server-side-binding)
#[derive(ViolationMetadata)]
pub struct HardcodedSQLExpression;

impl Violation for HardcodedSQLExpression {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Possible SQL injection vector through string-based query construction".to_string()
    }
}