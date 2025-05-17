use std::{fmt::{Display, Formatter}, str::FromStr};

use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Reports the following `re` and `regex` calls when
/// their first arguments are not raw strings:
///
/// - For `regex` and `re`: `compile`, `findall`, `finditer`,
///   `fullmatch`, `match`, `search`, `split`, `sub`, `subn`.
/// - `regex`-specific: `splititer`, `subf`, `subfn`, `template`.
///
/// ## Why is this bad?
/// Regular expressions should be written
/// using raw strings to avoid double escaping.
///
/// ## Example
///
/// ```python
/// re.compile("foo\\bar")
/// ```
///
/// Use instead:
///
/// ```python
/// re.compile(r"foo\bar")
/// ```
#[derive(ViolationMetadata)]
pub struct UnrawRePattern {
    module: RegexModule,
    func: String,
    kind: PatternKind,
}

impl Violation for UnrawRePattern {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { module, func, kind } = &self;
        let call = format!("`{module}.{func}()`");

        match kind {
            PatternKind::String => format!("First argument to {call} is not raw string"),
            PatternKind::Bytes => format!("First argument to {call} is not raw bytes literal"),
        }
    }

    fn fix_title(&self) -> Option<String> {
        match self.kind {
            PatternKind::String => Some("Replace with raw string".to_string()),
            PatternKind::Bytes => Some("Replace with raw bytes literal".to_string()),
        }
    }
}

// FIX: duplicated with ruff_rule_ruff::unraw_re_pattern
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RegexModule {
    Re,
    Regex,
}

impl RegexModule {
    fn is_function_taking_pattern(self, name: &str) -> bool {
        match name {
            "compile" | "findall" | "finditer" | "fullmatch" | "match" | "search" | "split"
            | "sub" | "subn" => true,
            "splititer" | "subf" | "subfn" | "template" => self == Self::Regex,
            _ => false,
        }
    }
}

impl Display for RegexModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RegexModule::Re => "re",
            RegexModule::Regex => "regex",
        })
    }
}

impl FromStr for RegexModule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "re" => Ok(Self::Re),
            "regex" => Ok(Self::Regex),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PatternKind {
    String,
    Bytes,
}