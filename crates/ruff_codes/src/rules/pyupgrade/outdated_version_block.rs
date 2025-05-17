use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for conditional blocks gated on `sys.version_info` comparisons
/// that are outdated for the minimum supported Python version.
///
/// ## Why is this bad?
/// In Python, code can be conditionally executed based on the active
/// Python version by comparing against the `sys.version_info` tuple.
///
/// If a code block is only executed for Python versions older than the
/// minimum supported version, it should be removed.
///
/// ## Example
/// ```python
/// import sys
///
/// if sys.version_info < (3, 0):
///     print("py2")
/// else:
///     print("py3")
/// ```
///
/// Use instead:
/// ```python
/// print("py3")
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## Fix safety
/// This rule's fix is marked as unsafe because it will remove all code,
/// comments, and annotations within unreachable version blocks.
///
/// ## References
/// - [Python documentation: `sys.version_info`](https://docs.python.org/3/library/sys.html#sys.version_info)
#[derive(ViolationMetadata)]
pub struct OutdatedVersionBlock {
    reason: Reason,
}

impl Violation for OutdatedVersionBlock {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.reason {
            Reason::AlwaysFalse | Reason::AlwaysTrue => {
                "Version block is outdated for minimum Python version".to_string()
            }
            Reason::Invalid => "Version specifier is invalid".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        match self.reason {
            Reason::AlwaysFalse | Reason::AlwaysTrue => {
                Some("Remove outdated version block".to_string())
            }
            Reason::Invalid => None,
        }
    }
}

// FIX: duplicated with ruff_rule_pyupgrade::outdated_version_block
#[derive(Debug, PartialEq, Eq)]
enum Reason {
    AlwaysTrue,
    AlwaysFalse,
    Invalid,
}