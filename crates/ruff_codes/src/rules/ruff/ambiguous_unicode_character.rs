use std::fmt;

use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for ambiguous Unicode characters in strings.
///
/// ## Why is this bad?
/// Some Unicode characters are visually similar to ASCII characters, but have
/// different code points. For example, `GREEK CAPITAL LETTER ALPHA` (`U+0391`)
/// is visually similar, but not identical, to the ASCII character `A`.
///
/// The use of ambiguous Unicode characters can confuse readers, cause subtle
/// bugs, and even make malicious code look harmless.
///
/// In [preview], this rule will also flag Unicode characters that are
/// confusable with other, non-preferred Unicode characters. For example, the
/// spec recommends `GREEK CAPITAL LETTER OMEGA` over `OHM SIGN`.
///
/// You can omit characters from being flagged as ambiguous via the
/// [`lint.allowed-confusables`] setting.
///
/// ## Example
/// ```python
/// print("Ηello, world!")  # "Η" is the Greek eta (`U+0397`).
/// ```
///
/// Use instead:
/// ```python
/// print("Hello, world!")  # "H" is the Latin capital H (`U+0048`).
/// ```
///
/// ## Options
/// - `lint.allowed-confusables`
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct AmbiguousUnicodeCharacterString {
    confusable: char,
    representant: char,
}

impl Violation for AmbiguousUnicodeCharacterString {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AmbiguousUnicodeCharacterString {
            confusable,
            representant,
        } = self;
        format!(
            "String contains ambiguous {}. Did you mean {}?",
            NamedUnicode(*confusable),
            NamedUnicode(*representant)
        )
    }
}

/// ## What it does
/// Checks for ambiguous Unicode characters in docstrings.
///
/// ## Why is this bad?
/// Some Unicode characters are visually similar to ASCII characters, but have
/// different code points. For example, `GREEK CAPITAL LETTER ALPHA` (`U+0391`)
/// is visually similar, but not identical, to the ASCII character `A`.
///
/// The use of ambiguous Unicode characters can confuse readers, cause subtle
/// bugs, and even make malicious code look harmless.
///
/// In [preview], this rule will also flag Unicode characters that are
/// confusable with other, non-preferred Unicode characters. For example, the
/// spec recommends `GREEK CAPITAL LETTER OMEGA` over `OHM SIGN`.
///
/// You can omit characters from being flagged as ambiguous via the
/// [`lint.allowed-confusables`] setting.
///
/// ## Example
/// ```python
/// """A lovely docstring (with a `U+FF09` parenthesis）."""
/// ```
///
/// Use instead:
/// ```python
/// """A lovely docstring (with no strange parentheses)."""
/// ```
///
/// ## Options
/// - `lint.allowed-confusables`
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct AmbiguousUnicodeCharacterDocstring {
    confusable: char,
    representant: char,
}

impl Violation for AmbiguousUnicodeCharacterDocstring {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AmbiguousUnicodeCharacterDocstring {
            confusable,
            representant,
        } = self;
        format!(
            "Docstring contains ambiguous {}. Did you mean {}?",
            NamedUnicode(*confusable),
            NamedUnicode(*representant)
        )
    }
}

/// ## What it does
/// Checks for ambiguous Unicode characters in comments.
///
/// ## Why is this bad?
/// Some Unicode characters are visually similar to ASCII characters, but have
/// different code points. For example, `GREEK CAPITAL LETTER ALPHA` (`U+0391`)
/// is visually similar, but not identical, to the ASCII character `A`.
///
/// The use of ambiguous Unicode characters can confuse readers, cause subtle
/// bugs, and even make malicious code look harmless.
///
/// In [preview], this rule will also flag Unicode characters that are
/// confusable with other, non-preferred Unicode characters. For example, the
/// spec recommends `GREEK CAPITAL LETTER OMEGA` over `OHM SIGN`.
///
/// You can omit characters from being flagged as ambiguous via the
/// [`lint.allowed-confusables`] setting.
///
/// ## Example
/// ```python
/// foo()  # nоqa  # "о" is Cyrillic (`U+043E`)
/// ```
///
/// Use instead:
/// ```python
/// foo()  # noqa  # "o" is Latin (`U+006F`)
/// ```
///
/// ## Options
/// - `lint.allowed-confusables`
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct AmbiguousUnicodeCharacterComment {
    confusable: char,
    representant: char,
}

impl Violation for AmbiguousUnicodeCharacterComment {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AmbiguousUnicodeCharacterComment {
            confusable,
            representant,
        } = self;
        format!(
            "Comment contains ambiguous {}. Did you mean {}?",
            NamedUnicode(*confusable),
            NamedUnicode(*representant)
        )
    }
}

// FIX: duplicated with ruff_rule_ruff::ambiguous_unicode_character
struct NamedUnicode(char);

impl fmt::Display for NamedUnicode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let NamedUnicode(c) = self;
        if let Some(name) = unicode_names2::name(*c) {
            write!(f, "`{c}` ({name})")
        } else {
            write!(f, "`{c}`")
        }
    }
}