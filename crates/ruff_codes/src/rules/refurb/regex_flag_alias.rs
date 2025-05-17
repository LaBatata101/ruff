use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of shorthand aliases for regular expression flags
/// (e.g., `re.I` instead of `re.IGNORECASE`).
///
/// ## Why is this bad?
/// The regular expression module provides descriptive names for each flag,
/// along with single-letter aliases. Prefer the descriptive names, as they
/// are more readable and self-documenting.
///
/// ## Example
/// ```python
/// import re
///
/// if re.match("^hello", "hello world", re.I):
///     ...
/// ```
///
/// Use instead:
/// ```python
/// import re
///
/// if re.match("^hello", "hello world", re.IGNORECASE):
///     ...
/// ```
///
#[derive(ViolationMetadata)]
pub struct RegexFlagAlias {
    flag: RegexFlag,
}

impl AlwaysFixableViolation for RegexFlagAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RegexFlagAlias { flag } = self;
        format!("Use of regular expression alias `re.{}`", flag.alias())
    }

    fn fix_title(&self) -> String {
        let RegexFlagAlias { flag } = self;
        format!("Replace with `re.{}`", flag.full_name())
    }
}

// FIX: duplicated with ruff_rule_refurb::regex_flag_alias
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegexFlag {
    Ascii,
    IgnoreCase,
    Locale,
    Multiline,
    DotAll,
    Template,
    Unicode,
    Verbose,
}

impl RegexFlag {
    #[must_use]
    const fn alias(self) -> &'static str {
        match self {
            Self::Ascii => "A",
            Self::IgnoreCase => "I",
            Self::Locale => "L",
            Self::Multiline => "M",
            Self::DotAll => "S",
            Self::Template => "T",
            Self::Unicode => "U",
            Self::Verbose => "X",
        }
    }

    #[must_use]
    const fn full_name(self) -> &'static str {
        match self {
            Self::Ascii => "ASCII",
            Self::IgnoreCase => "IGNORECASE",
            Self::Locale => "LOCALE",
            Self::Multiline => "MULTILINE",
            Self::DotAll => "DOTALL",
            Self::Template => "TEMPLATE",
            Self::Unicode => "UNICODE",
            Self::Verbose => "VERBOSE",
        }
    }
}