//! This is the library for the [Ruff] Python linter.
//!
//! **The API is currently completely unstable**
//! and subject to change drastically.
//!
//! [Ruff]: https://github.com/astral-sh/ruff


#[cfg(feature = "clap")]
pub use ruff_codes::registry::clap_completion::RuleParser;
#[cfg(feature = "clap")]
pub use ruff_codes::rule_selector::clap_completion::RuleSelectorParser;
pub use ruff_codes::rule_selector::RuleSelector;
// pub use ruff_codes::rules::pycodestyle::rules::IOError;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod checkers;
pub mod linter;
pub mod pyproject_toml;
mod fs;


#[cfg(any(test, fuzzing))]
pub mod test;

pub const RUFF_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

// FIX: remove comment
// #[cfg(test)]
// mod tests {
//     use std::path::Path;

//     use ruff_python_ast::PySourceType;

//     use ruff_codes::Rule;
//     use ruff_linter_settings::LinterSettings;
//     use ruff_linter_commons::source_kind::SourceKind;
//     use crate::test::{print_messages, test_contents};

//     /// Test for ad-hoc debugging.
//     #[test]
//     #[ignore]
//     fn linter_quick_test() {
//         let code = r#"class Platform:
//     """ Remove sampler
//             Args:
//             Returns:
//             """
// "#;
//         let source_type = PySourceType::Python;
//         let rule = Rule::OverIndentation;

//         let source_kind = SourceKind::from_source_code(code.to_string(), source_type)
//             .expect("Source code should be valid")
//             .expect("Notebook to contain python code");

//         let (diagnostics, fixed) = test_contents(
//             &source_kind,
//             Path::new("ruff_linter/rules/quick_test"),
//             &LinterSettings::for_rule(rule),
//         );

//         assert_eq!(print_messages(&diagnostics), "");
//         assert_eq!(fixed.source_code(), code);
//     }
// }
