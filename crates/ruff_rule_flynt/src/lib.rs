//! Rules from [flynt](https://pypi.org/project/flynt/).
mod helpers;
pub mod rules;

// FIX: remove comment
// #[cfg(test)]
// mod tests {
//     use std::path::Path;

//     use anyhow::Result;
//     use test_case::test_case;

//     use ruff_codes::Rule;
//     use crate::test::test_path;
//     use crate::{assert_messages, settings};

//     #[test_case(Rule::StaticJoinToFString, Path::new("FLY002.py"))]
//     fn rules(rule_code: Rule, path: &Path) -> Result<()> {
//         let snapshot = format!("{}_{}", rule_code.noqa_code(), path.to_string_lossy());
//         let diagnostics = test_path(
//             Path::new("flynt").join(path).as_path(),
//             &settings::LinterSettings::for_rule(rule_code),
//         )?;
//         assert_messages!(snapshot, diagnostics);
//         Ok(())
//     }
// }
