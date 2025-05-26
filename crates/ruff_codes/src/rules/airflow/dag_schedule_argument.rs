use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for a `DAG()` class or `@dag()` decorator without an explicit
/// `schedule` (or `schedule_interval` for Airflow 1) parameter.
///
/// ## Why is this bad?
/// The default value of the `schedule` parameter on Airflow 2 and
/// `schedule_interval` on Airflow 1 is `timedelta(days=1)`, which is almost
/// never what a user is looking for. Airflow 3 changed the default value to `None`,
/// and would break existing dags using the implicit default.
///
/// If your dag does not have an explicit `schedule` / `schedule_interval` argument,
/// Airflow 2 schedules a run for it every day (at the time determined by `start_date`).
/// Such a dag will no longer be scheduled on Airflow 3 at all, without any
/// exceptions or other messages visible to the user.
///
/// ## Example
/// ```python
/// from airflow import DAG
///
///
/// # Using the implicit default schedule.
/// dag = DAG(dag_id="my_dag")
/// ```
///
/// Use instead:
/// ```python
/// from datetime import timedelta
///
/// from airflow import DAG
///
///
/// dag = DAG(dag_id="my_dag", schedule=timedelta(days=1))
/// ```
#[derive(ViolationMetadata)]
pub struct AirflowDagNoScheduleArgument;

impl Violation for AirflowDagNoScheduleArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        "DAG should have an explicit `schedule` argument".to_string()
    }
}

