use std::fmt;

use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks duplicate characters in `str.strip` calls.
///
/// ## Why is this bad?
/// All characters in `str.strip` calls are removed from both the leading and
/// trailing ends of the string. Including duplicate characters in the call
/// is redundant and often indicative of a mistake.
///
/// In Python 3.9 and later, you can use `str.removeprefix` and
/// `str.removesuffix` to remove an exact prefix or suffix from a string,
/// respectively, which should be preferred when possible.
///
/// ## Example
/// ```python
/// # Evaluates to "foo".
/// "bar foo baz".strip("bar baz ")
/// ```
///
/// Use instead:
/// ```python
/// # Evaluates to "foo".
/// "bar foo baz".strip("abrz ")  # "foo"
/// ```
///
/// Or:
/// ```python
/// # Evaluates to "foo".
/// "bar foo baz".removeprefix("bar ").removesuffix(" baz")
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `str.strip`](https://docs.python.org/3/library/stdtypes.html?highlight=strip#str.strip)
#[derive(ViolationMetadata)]
pub struct BadStrStripCall {
    strip: StripKind,
    removal: Option<RemovalKind>,
}

impl Violation for BadStrStripCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { strip, removal } = self;
        if let Some(removal) = removal {
            format!(
                "String `{strip}` call contains duplicate characters (did you mean `{removal}`?)",
            )
        } else {
            format!("String `{strip}` call contains duplicate characters")
        }
    }
}

// FIX: duplicated with rufe_rule_pylint::bad_str_strip_call
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum StripKind {
    Strip,
    LStrip,
    RStrip,
}

impl StripKind {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "strip" => Some(Self::Strip),
            "lstrip" => Some(Self::LStrip),
            "rstrip" => Some(Self::RStrip),
            _ => None,
        }
    }
}

impl fmt::Display for StripKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let representation = match self {
            Self::Strip => "strip",
            Self::LStrip => "lstrip",
            Self::RStrip => "rstrip",
        };
        write!(f, "{representation}")
    }
}

// FIX: duplicated with rufe_rule_pylint::bad_str_strip_call
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RemovalKind {
    RemovePrefix,
    RemoveSuffix,
}

impl fmt::Display for RemovalKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let representation = match self {
            Self::RemovePrefix => "removeprefix",
            Self::RemoveSuffix => "removesuffix",
        };
        write!(f, "{representation}")
    }
}