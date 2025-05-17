use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for `noqa` annotations that suppress all diagnostics, as opposed to
/// targeting specific diagnostics.
///
/// ## Why is this bad?
/// Suppressing all diagnostics can hide issues in the code.
///
/// Blanket `noqa` annotations are also more difficult to interpret and
/// maintain, as the annotation does not clarify which diagnostics are intended
/// to be suppressed.
///
/// ## Example
/// ```python
/// from .base import *  # noqa
/// ```
///
/// Use instead:
/// ```python
/// from .base import *  # noqa: F403
/// ```
///
/// ## Fix safety
/// This rule will attempt to fix blanket `noqa` annotations that appear to
/// be unintentional. For example, given `# noqa F401`, the rule will suggest
/// inserting a colon, as in `# noqa: F401`.
///
/// While modifying `noqa` comments is generally safe, doing so may introduce
/// additional diagnostics.
///
/// ## References
/// - [Ruff documentation](https://docs.astral.sh/ruff/configuration/#error-suppression)
#[derive(ViolationMetadata)]
pub struct BlanketNOQA {
    missing_colon: bool,
    file_exemption: bool,
}

impl Violation for BlanketNOQA {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let BlanketNOQA {
            missing_colon,
            file_exemption,
        } = self;
        // This awkward branching is necessary to ensure that the generic message is picked up by
        // `derive_message_formats`.
        if !missing_colon && !file_exemption {
            "Use specific rule codes when using `noqa`".to_string()
        } else if *file_exemption {
            "Use specific rule codes when using `ruff: noqa`".to_string()
        } else {
            "Use a colon when specifying `noqa` rule codes".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        if self.missing_colon {
            Some("Add missing colon".to_string())
        } else {
            None
        }
    }
}