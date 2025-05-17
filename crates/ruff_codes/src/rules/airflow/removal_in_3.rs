use ruff_diagnostics::{Violation, FixAvailability};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::Replacement;

/// ## What it does
/// Checks for uses of deprecated Airflow functions and values.
///
/// ## Why is this bad?
/// Airflow 3.0 removed various deprecated functions, members, and other
/// values. Some have more modern replacements. Others are considered too niche
/// and not worth to be maintained in Airflow.
///
/// ## Example
/// ```python
/// from airflow.utils.dates import days_ago
///
///
/// yesterday = days_ago(today, 1)
/// ```
///
/// Use instead:
/// ```python
/// from datetime import timedelta
///
///
/// yesterday = today - timedelta(days=1)
/// ```
#[derive(ViolationMetadata)]
pub struct Airflow3Removal {
    deprecated: String,
    replacement: Replacement,
}

impl Violation for Airflow3Removal {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Airflow3Removal {
            deprecated,
            replacement,
        } = self;
        match replacement {
            Replacement::None
            | Replacement::AttrName(_)
            | Replacement::Message(_)
            | Replacement::AutoImport { module: _, name: _ }
            | Replacement::SourceModuleMoved { module: _, name: _ } => {
                format!("`{deprecated}` is removed in Airflow 3.0")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Airflow3Removal { replacement, .. } = self;
        match replacement {
            Replacement::None => None,
            Replacement::AttrName(name) => Some(format!("Use `{name}` instead")),
            Replacement::Message(message) => Some((*message).to_string()),
            Replacement::AutoImport { module, name } => {
                Some(format!("Use `{module}.{name}` instead"))
            }
            Replacement::SourceModuleMoved { module, name } => {
                Some(format!("Use `{module}.{name}` instead"))
            }
        }
    }
}