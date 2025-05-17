use ruff_diagnostics::{Violation, FixAvailability};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::ProviderReplacement;

/// ## What it does
/// Checks for uses of Airflow functions and values that have been moved to its providers
/// but still have a compatibility layer (e.g., `apache-airflow-providers-standard`).
///
/// ## Why is this bad?
/// Airflow 3.0 moved various deprecated functions, members, and other
/// values to its providers. Even though these symbols still work fine on Airflow 3.0,
/// they are expected to be removed in a future version. The user is suggested to install
/// the corresponding provider and replace the original usage with the one in the provider.
///
/// ## Example
/// ```python
/// from airflow.operators.python import PythonOperator
/// ```
///
/// Use instead:
/// ```python
/// from airflow.providers.standard.operators.python import PythonOperator
/// ```
#[derive(ViolationMetadata)]
pub struct Airflow3SuggestedToMoveToProvider {
    deprecated: String,
    replacement: ProviderReplacement,
}

impl Violation for Airflow3SuggestedToMoveToProvider {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;
    #[derive_message_formats]
    fn message(&self) -> String {
        let Airflow3SuggestedToMoveToProvider {
            deprecated,
            replacement,
        } = self;
        match replacement {
            ProviderReplacement::None => {
                format!("`{deprecated}` is removed in Airflow 3.0")
            }
            ProviderReplacement::AutoImport {
                name: _,
                module: _,
                provider,
                version: _,
            }
            | ProviderReplacement::SourceModuleMovedToProvider {
                name: _,
                module: _,
                provider,
                version: _,
            } => {
                format!(
                    "`{deprecated}` is deprecated and moved into `{provider}` provider in Airflow 3.0; \
                     It still works in Airflow 3.0 but is expected to be removed in a future version."
                )
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Airflow3SuggestedToMoveToProvider { replacement, .. } = self;
        match replacement {
            ProviderReplacement::None => None,
            ProviderReplacement::AutoImport {
                module,
                name,
                provider,
                version,
            } => {
                Some(format!("Install `apache-airflow-providers-{provider}>={version}` and use `{module}.{name}` instead."))
            },
            ProviderReplacement::SourceModuleMovedToProvider {
                module,
                name,
                provider,
                version,
            } => {
                Some(format!("Install `apache-airflow-providers-{provider}>={version}` and use `{module}.{name}` instead."))
            }
        }
    }
}