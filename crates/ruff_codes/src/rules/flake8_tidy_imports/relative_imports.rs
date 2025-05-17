use std::fmt::{Display, Formatter};

use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, CacheKey, ViolationMetadata};
use serde::{Deserialize, Serialize};

/// ## What it does
/// Checks for relative imports.
///
/// ## Why is this bad?
/// Absolute imports, or relative imports from siblings, are recommended by [PEP 8]:
///
/// > Absolute imports are recommended, as they are usually more readable and tend to be better behaved...
/// > ```python
/// > import mypkg.sibling
/// > from mypkg import sibling
/// > from mypkg.sibling import example
/// > ```
/// > However, explicit relative imports are an acceptable alternative to absolute imports,
/// > especially when dealing with complex package layouts where using absolute imports would be
/// > unnecessarily verbose:
/// > ```python
/// > from . import sibling
/// > from .sibling import example
/// > ```
///
/// ## Example
/// ```python
/// from .. import foo
/// ```
///
/// Use instead:
/// ```python
/// from mypkg import foo
/// ```
///
/// ## Options
/// - `lint.flake8-tidy-imports.ban-relative-imports`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#imports
#[derive(ViolationMetadata)]
pub struct RelativeImports {
    strictness: Strictness,
}

impl Violation for RelativeImports {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.strictness {
            Strictness::Parents => {
                "Prefer absolute imports over relative imports from parent modules".to_string()
            }
            Strictness::All => "Prefer absolute imports over relative imports".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let RelativeImports { strictness } = self;
        Some(match strictness {
            Strictness::Parents => {
                "Replace relative imports from parent modules with absolute imports".to_string()
            }
            Strictness::All => "Replace relative imports with absolute imports".to_string(),
        })
    }
}

// FIX: duplicated with ruff_rule_flake8_tidy_imports::relative_imports
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, CacheKey, Default)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Strictness {
    /// Ban imports that extend into the parent module or beyond.
    #[default]
    Parents,
    /// Ban all relative imports.
    All,
}

impl Display for Strictness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parents => write!(f, "\"parents\""),
            Self::All => write!(f, "\"all\""),
        }
    }
}