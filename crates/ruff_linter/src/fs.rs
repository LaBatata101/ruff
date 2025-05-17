use std::path::Path;

use ruff_codes::types::CompiledPerFileIgnoreList;
use ruff_codes::RuleSet;

// FIX: this file is kind of a duplication with `ruff_linter_commons::fs`
/// Create a set with codes matching the pattern/code pairs.
pub(crate) fn ignores_from_path(path: &Path, ignore_list: &CompiledPerFileIgnoreList) -> RuleSet {
    ignore_list
        .iter_matches(path, "Adding per-file ignores")
        .flatten()
        .collect()
}
