use ruff_diagnostics::{Violation, FixAvailability};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// De-duplicates, groups, and sorts imports based on the provided `isort` settings.
///
/// ## Why is this bad?
/// Consistency is good. Use a common convention for imports to make your code
/// more readable and idiomatic.
///
/// ## Example
/// ```python
/// import pandas
/// import numpy as np
/// ```
///
/// Use instead:
/// ```python
/// import numpy as np
/// import pandas
/// ```
///
/// ## Preview
/// When [`preview`](https://docs.astral.sh/ruff/preview/) mode is enabled, Ruff applies a stricter criterion
/// for determining whether an import should be classified as first-party.
/// Specifically, for an import of the form `import foo.bar.baz`, Ruff will
/// check that `foo/bar`, relative to a [user-specified `src`](https://docs.astral.sh/ruff/settings/#src) directory, contains either
/// the directory `baz` or else a file with the name `baz.py` or `baz.pyi`.
#[derive(ViolationMetadata)]
pub struct UnsortedImports;

impl Violation for UnsortedImports {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Import block is un-sorted or un-formatted".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Organize imports".to_string())
    }
}