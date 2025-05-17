use ruff_diagnostics::{Violation, FixAvailability};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::Replacement;

/// ## What it does
/// Checks for uses of deprecated Airflow functions and values that still have
/// a compatibility layer.
///
/// ## Why is this bad?
/// Airflow 3.0 removed various deprecated functions, members, and other
/// values. Some have more modern replacements. Others are considered too niche
/// and not worth to be maintained in Airflow.
/// Even though these symbols still work fine on Airflow 3.0, they are expected to be removed in a future version.
/// The user is suggested to replace the original usage with the new ones.
///
/// ## Example
/// ```python
/// from airflow import Dataset
///
///
/// Dataset(uri="test://test/")
/// ```
///
/// Use instead:
/// ```python
/// from airflow.sdk import Asset
///
///
/// Asset(uri="test://test/")
/// ```
#[derive(ViolationMetadata)]
pub struct Airflow3SuggestedUpdate {
    deprecated: String,
    replacement: Replacement,
}

impl Violation for Airflow3SuggestedUpdate {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Airflow3SuggestedUpdate {
            deprecated,
            replacement,
        } = self;
        match replacement {
            Replacement::None
            | Replacement::AttrName(_)
            | Replacement::Message(_)
            | Replacement::AutoImport { module: _, name: _ }
            | Replacement::SourceModuleMoved { module: _, name: _ } => {
                format!(
                    "`{deprecated}` is removed in Airflow 3.0; \
                    It still works in Airflow 3.0 but is expected to be removed in a future version."
                )
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Airflow3SuggestedUpdate { replacement, .. } = self;
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