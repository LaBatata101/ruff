use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `bin(...)[2:]` (or `hex`, or `oct`) to convert
/// an integer into a string.
///
/// ## Why is this bad?
/// When converting an integer to a baseless binary, hexadecimal, or octal
/// string, using f-strings is more concise and readable than using the
/// `bin`, `hex`, or `oct` functions followed by a slice.
///
/// ## Example
/// ```python
/// print(bin(1337)[2:])
/// ```
///
/// Use instead:
/// ```python
/// print(f"{1337:b}")
/// ```
///
/// ## Fix safety
/// The fix is only marked as safe for integer literals, all other cases
/// are display-only, as they may change the runtime behaviour of the program
/// or introduce syntax errors.
#[derive(ViolationMetadata)]
pub struct FStringNumberFormat {
    replacement: Option<SourceCodeSnippet>,
    base: Base,
}

impl Violation for FStringNumberFormat {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let FStringNumberFormat { replacement, base } = self;
        let function_name = base.function_name();

        if let Some(display) = replacement
            .as_ref()
            .and_then(SourceCodeSnippet::full_display)
        {
            format!("Replace `{function_name}` call with `{display}`")
        } else {
            format!("Replace `{function_name}` call with f-string")
        }
    }

    fn fix_title(&self) -> Option<String> {
        if let Some(display) = self
            .replacement
            .as_ref()
            .and_then(SourceCodeSnippet::full_display)
        {
            Some(format!("Replace with `{display}`"))
        } else {
            Some("Replace with f-string".to_string())
        }
    }
}

// FIX: duplicated with ruff_rule_refurb::fstring_number_format
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Base {
    Hex,
    Bin,
    Oct,
}

impl Base {
    /// Returns the shorthand for the base.
    fn shorthand(self) -> &'static str {
        match self {
            Base::Hex => "x",
            Base::Bin => "b",
            Base::Oct => "o",
        }
    }

    /// Returns the builtin function name for the base.
    fn function_name(self) -> &'static str {
        match self {
            Base::Hex => "hex",
            Base::Bin => "bin",
            Base::Oct => "oct",
        }
    }

    /// Parses the base from a string.
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "hex" => Some(Base::Hex),
            "bin" => Some(Base::Bin),
            "oct" => Some(Base::Oct),
            _ => None,
        }
    }
}
