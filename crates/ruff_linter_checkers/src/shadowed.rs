use itertools::Itertools;
use ruff_python_semantic::Binding;
use ruff_python_stdlib::{builtins::is_python_builtin, keyword::is_keyword};

use crate::ast::CheckerSnapshot;

/// Enumeration of various ways in which a binding can shadow other variables
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ShadowedKind {
    /// The variable shadows a global, nonlocal or local symbol
    Some,
    /// The variable shadows a builtin symbol
    BuiltIn,
    /// The variable shadows a keyword
    Keyword,
    /// The variable does not shadow any other symbols
    None,
}

impl ShadowedKind {
    /// Determines the kind of shadowing or conflict for the proposed new name of a given [`Binding`].
    ///
    /// This function is useful for checking whether or not the `target` of a [`Renamer::rename`]
    /// will shadow another binding.
    pub fn new(binding: &Binding, new_name: &str, checker: &CheckerSnapshot) -> ShadowedKind {
        // Check the kind in order of precedence
        if is_keyword(new_name) {
            return ShadowedKind::Keyword;
        }

        if is_python_builtin(
            new_name,
            checker.target_version().minor,
            checker.source_type.is_ipynb(),
        ) {
            return ShadowedKind::BuiltIn;
        }

        let semantic = checker.semantic();

        if !semantic.is_available_in_scope(new_name, binding.scope) {
            return ShadowedKind::Some;
        }

        if binding
            .references()
            .map(|reference_id| semantic.reference(reference_id).scope_id())
            .dedup()
            .any(|scope| !semantic.is_available_in_scope(new_name, scope))
        {
            return ShadowedKind::Some;
        }

        // Default to no shadowing
        ShadowedKind::None
    }

    /// Returns `true` if `self` shadows any global, nonlocal, or local symbol, keyword, or builtin.
    pub const fn shadows_any(self) -> bool {
        matches!(
            self,
            ShadowedKind::Some | ShadowedKind::BuiltIn | ShadowedKind::Keyword
        )
    }
}
