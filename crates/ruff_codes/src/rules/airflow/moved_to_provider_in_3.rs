use ruff_diagnostics::{Violation, FixAvailability};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::ProviderReplacement;

/// ## What it does
/// Checks for uses of Airflow functions and values that have been moved to it providers.
/// (e.g., apache-airflow-providers-fab)
///
/// ## Why is this bad?
/// Airflow 3.0 moved various deprecated functions, members, and other
/// values to its providers. The user needs to install the corresponding provider and replace
/// the original usage with the one in the provider
///
/// ## Example
/// ```python
/// from airflow.auth.managers.fab.fab_auth_manage import FabAuthManager
/// ```
///
/// Use instead:
/// ```python
/// from airflow.providers.fab.auth_manager.fab_auth_manage import FabAuthManager
/// ```
#[derive(ViolationMetadata)]
pub struct Airflow3MovedToProvider {
    deprecated: String,
    replacement: ProviderReplacement,
}

impl Violation for Airflow3MovedToProvider {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Airflow3MovedToProvider {
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
                format!("`{deprecated}` is moved into `{provider}` provider in Airflow 3.0;")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Airflow3MovedToProvider { replacement, .. } = self;
        match replacement {
            ProviderReplacement::None => {None}
            ProviderReplacement::AutoImport {
                name,
                module,
                provider,
                version,
            } => {
                Some(format!("Install `apache-airflow-providers-{provider}>={version}` and use `{module}.{name}` instead."))
            } ,
            ProviderReplacement::SourceModuleMovedToProvider {
                name,
                module,
                provider,
                version,
            } => {
                Some(format!("Install `apache-airflow-providers-{provider}>={version}` and use `{module}.{name}` instead."))
            } ,
        }
    }
}