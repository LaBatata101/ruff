use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for dictionary literals that associate multiple values with the
/// same key.
///
/// ## Why is this bad?
/// Dictionary keys should be unique. If a key is associated with multiple values,
/// the earlier values will be overwritten. Including multiple values for the
/// same key in a dictionary literal is likely a mistake.
///
/// ## Example
/// ```python
/// foo = {
///     "bar": 1,
///     "baz": 2,
///     "baz": 3,
/// }
/// foo["baz"]  # 3
/// ```
///
/// Use instead:
/// ```python
/// foo = {
///     "bar": 1,
///     "baz": 2,
/// }
/// foo["baz"]  # 2
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe because removing a repeated dictionary key
/// may delete comments that are attached to the removed key-value pair. This can also change
/// the program's behavior if the value expressions have side effects.
///
/// ## References
/// - [Python documentation: Dictionaries](https://docs.python.org/3/tutorial/datastructures.html#dictionaries)
#[derive(ViolationMetadata)]
pub struct MultiValueRepeatedKeyLiteral {
    name: SourceCodeSnippet,
    existing: SourceCodeSnippet,
}

impl Violation for MultiValueRepeatedKeyLiteral {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match (self.name.full_display(), self.existing.full_display()) {
            (Some(name), None) => {
                format!("Dictionary key literal `{name}` repeated")
            }
            (Some(name), Some(existing)) => {
                if name == existing {
                    format!("Dictionary key literal `{name}` repeated")
                } else {
                    format!(
                        "Dictionary key literal `{name}` repeated (`{name}` hashes to the same value as `{existing}`)"
                    )
                }
            }
            _ => "Dictionary key literal repeated".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.name.full_display() {
            Some(name) => format!("Remove repeated key literal `{name}`"),
            None => "Remove repeated key literal".to_string(),
        };
        Some(title)
    }
}

/// ## What it does
/// Checks for dictionary keys that are repeated with different values.
///
/// ## Why is this bad?
/// Dictionary keys should be unique. If a key is repeated with a different
/// value, the first values will be overwritten and the key will correspond to
/// the last value. This is likely a mistake.
///
/// ## Example
/// ```python
/// foo = {
///     bar: 1,
///     baz: 2,
///     baz: 3,
/// }
/// foo[baz]  # 3
/// ```
///
/// Use instead:
/// ```python
/// foo = {
///     bar: 1,
///     baz: 2,
/// }
/// foo[baz]  # 2
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe because removing a repeated dictionary key
/// may delete comments that are attached to the removed key-value pair. This can also change
/// the program's behavior if the value expressions have side effects.
///
/// ## References
/// - [Python documentation: Dictionaries](https://docs.python.org/3/tutorial/datastructures.html#dictionaries)
#[derive(ViolationMetadata)]
pub struct MultiValueRepeatedKeyVariable {
    name: SourceCodeSnippet,
}

impl Violation for MultiValueRepeatedKeyVariable {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(name) = self.name.full_display() {
            format!("Dictionary key `{name}` repeated")
        } else {
            "Dictionary key repeated".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.name.full_display() {
            Some(name) => format!("Remove repeated key `{name}`"),
            None => "Remove repeated key".to_string(),
        };
        Some(title)
    }
}
