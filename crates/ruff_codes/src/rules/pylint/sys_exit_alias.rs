use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the `exit()` and `quit()`.
///
/// ## Why is this bad?
/// `exit` and `quit` come from the `site` module, which is typically imported
/// automatically during startup. However, it is not _guaranteed_ to be
/// imported, and so using these functions may result in a `NameError` at
/// runtime. Generally, these constants are intended to be used in an interactive
/// interpreter, and not in programs.
///
/// Prefer `sys.exit()`, as the `sys` module is guaranteed to exist in all
/// contexts.
///
/// ## Fix safety
/// This fix is always unsafe. When replacing `exit` or `quit` with `sys.exit`,
/// the behavior can change in the following ways:
///
/// 1. If the code runs in an environment where the `site` module is not imported
///    (e.g., with `python -S`), the original code would raise a `NameError`, while
///    the fixed code would execute normally.
///
/// 2. `site.exit` and `sys.exit` handle tuple arguments differently. `site.exit`
///    treats tuples as regular objects and always returns exit code 1, while `sys.exit`
///    interprets tuple contents to determine the exit code: an empty tuple () results in
///    exit code 0, and a single-element tuple like (2,) uses that element's value (2) as
///    the exit code.
///
/// ## Example
/// ```python
/// if __name__ == "__main__":
///     exit()
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// if __name__ == "__main__":
///     sys.exit()
/// ```
///
/// ## References
/// - [Python documentation: Constants added by the `site` module](https://docs.python.org/3/library/constants.html#constants-added-by-the-site-module)
#[derive(ViolationMetadata)]
pub struct SysExitAlias {
    name: String,
}

impl Violation for SysExitAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let SysExitAlias { name } = self;
        format!("Use `sys.exit()` instead of `{name}`")
    }

    fn fix_title(&self) -> Option<String> {
        let SysExitAlias { name } = self;
        Some(format!("Replace `{name}` with `sys.exit()`"))
    }
}