use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for unrecognized `sys.platform` checks. Platform checks should be
/// simple string comparisons.
///
/// **Note**: this rule is only enabled in `.pyi` stub files.
///
/// ## Why is this bad?
/// Some `sys.platform` checks are too complex for type checkers to
/// understand, and thus result in incorrect inferences by these tools.
/// `sys.platform` checks should be simple string comparisons, like
/// `if sys.platform == "linux"`.
///
/// ## Example
/// ```pyi
/// if sys.platform.startswith("linux"):
///     # Linux specific definitions
///     ...
/// else:
///     # Posix specific definitions
///     ...
/// ```
///
/// Instead, use a simple string comparison, such as `==` or `!=`:
/// ```pyi
/// if sys.platform == "linux":
///     # Linux specific definitions
///     ...
/// else:
///     # Posix specific definitions
///     ...
/// ```
///
/// ## References
/// - [Typing documentation: Version and Platform checking](https://typing.python.org/en/latest/spec/directives.html#version-and-platform-checks)
#[derive(ViolationMetadata)]
pub struct UnrecognizedPlatformCheck;

impl Violation for UnrecognizedPlatformCheck {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unrecognized `sys.platform` check".to_string()
    }
}

/// ## What it does
/// Check for unrecognized platform names in `sys.platform` checks.
///
/// **Note**: this rule is only enabled in `.pyi` stub files.
///
/// ## Why is this bad?
/// If a `sys.platform` check compares to a platform name outside of a
/// small set of known platforms (e.g. "linux", "win32", etc.), it's likely
/// a typo or a platform name that is not recognized by type checkers.
///
/// The list of known platforms is: "linux", "win32", "cygwin", "darwin".
///
/// ## Example
/// ```pyi
/// if sys.platform == "linus": ...
/// ```
///
/// Use instead:
/// ```pyi
/// if sys.platform == "linux": ...
/// ```
///
/// ## References
/// - [Typing documentation: Version and Platform checking](https://typing.python.org/en/latest/spec/directives.html#version-and-platform-checks)
#[derive(ViolationMetadata)]
pub struct UnrecognizedPlatformName {
    platform: String,
}

impl Violation for UnrecognizedPlatformName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnrecognizedPlatformName { platform } = self;
        format!("Unrecognized platform `{platform}`")
    }
}
