use ruff_diagnostics::{Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks that the task variable name matches the `task_id` value for
/// Airflow Operators.
///
/// ## Why is this bad?
/// When initializing an Airflow Operator, for consistency, the variable
/// name should match the `task_id` value. This makes it easier to
/// follow the flow of the DAG.
///
/// ## Example
/// ```python
/// from airflow.operators import PythonOperator
///
///
/// incorrect_name = PythonOperator(task_id="my_task")
/// ```
///
/// Use instead:
/// ```python
/// from airflow.operators import PythonOperator
///
///
/// my_task = PythonOperator(task_id="my_task")
/// ```
#[derive(ViolationMetadata)]
pub struct AirflowVariableNameTaskIdMismatch {
    task_id: String,
}

impl Violation for AirflowVariableNameTaskIdMismatch {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AirflowVariableNameTaskIdMismatch { task_id } = self;
        format!("Task variable name should match the `task_id`: \"{task_id}\"")
    }
}