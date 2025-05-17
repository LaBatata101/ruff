use itertools::Itertools;
use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of deprecated imports based on the minimum supported
/// Python version.
///
/// ## Why is this bad?
/// Deprecated imports may be removed in future versions of Python, and
/// should be replaced with their new equivalents.
///
/// Note that, in some cases, it may be preferable to continue importing
/// members from `typing_extensions` even after they're added to the Python
/// standard library, as `typing_extensions` can backport bugfixes and
/// optimizations from later Python versions. This rule thus avoids flagging
/// imports from `typing_extensions` in such cases.
///
/// ## Example
/// ```python
/// from collections import Sequence
/// ```
///
/// Use instead:
/// ```python
/// from collections.abc import Sequence
/// ```
#[derive(ViolationMetadata)]
pub struct DeprecatedImport {
    deprecation: Deprecation,
}

impl Violation for DeprecatedImport {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match &self.deprecation {
            Deprecation::WithoutRename(WithoutRename {
                members, target, ..
            }) => {
                let names = members.iter().map(|name| format!("`{name}`")).join(", ");
                format!("Import from `{target}` instead: {names}")
            }
            Deprecation::WithRename(WithRename {
                module,
                member,
                target,
            }) => {
                format!("`{module}.{member}` is deprecated, use `{target}` instead")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        if let Deprecation::WithoutRename(WithoutRename { target, .. }) = &self.deprecation {
            Some(format!("Import from `{target}`"))
        } else {
            None
        }
    }
}

// FIX: duplicated with ruff_rule_pyupgrade::deprecated_import
/// An import was moved and renamed as part of a deprecation.
/// For example, `typing.AbstractSet` was moved to `collections.abc.Set`.
#[derive(Debug, PartialEq, Eq)]
struct WithRename {
    module: String,
    member: String,
    target: String,
}

// FIX: duplicated with ruff_rule_pyupgrade::deprecated_import
/// A series of imports from the same module were moved to another module,
/// but retain their original names.
#[derive(Debug, PartialEq, Eq)]
struct WithoutRename {
    target: String,
    members: Vec<String>,
    fixable: bool,
}

// FIX: duplicated with ruff_rule_pyupgrade::deprecated_import
#[derive(Debug, PartialEq, Eq)]
enum Deprecation {
    WithRename(WithRename),
    WithoutRename(WithoutRename),
}