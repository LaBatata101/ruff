//! Shared code for [`non_pep695_type_alias`] (UP040),
//! [`non_pep695_generic_class`] (UP046), and [`non_pep695_generic_function`]
//! (UP047)


use itertools::Itertools;
use ruff_linter_commons::helpers::{expr_name_to_type_var, TypeParamKind, TypeVar, TypeVarRestriction};
use ruff_python_ast::{
    visitor::{self, Visitor},
    Expr, Stmt,
};
use ruff_python_semantic::SemanticModel;

pub use non_pep695_generic_class::*;
pub use non_pep695_generic_function::*;
pub use non_pep695_type_alias::*;
pub use private_type_parameter::*;

use ruff_linter_checkers::ast::CheckerSnapshot;

mod non_pep695_generic_class;
mod non_pep695_generic_function;
mod non_pep695_type_alias;
mod private_type_parameter;

struct TypeVarReferenceVisitor<'a> {
    vars: Vec<TypeVar<'a>>,
    semantic: &'a SemanticModel<'a>,
    /// Tracks whether any non-TypeVars have been seen to avoid replacing generic parameters when an
    /// unknown `TypeVar` is encountered.
    any_skipped: bool,
}

/// Recursively collects the names of type variable references present in an expression.
impl<'a> Visitor<'a> for TypeVarReferenceVisitor<'a> {
    fn visit_expr(&mut self, expr: &'a Expr) {
        // special case for typing.AnyStr, which is a commonly-imported type variable in the
        // standard library with the definition:
        //
        // ```python
        // AnyStr = TypeVar('AnyStr', bytes, str)
        // ```
        //
        // As of 01/2025, this line hasn't been modified in 8 years, so hopefully there won't be
        // much to keep updated here. See
        // https://github.com/python/cpython/blob/383af395af828f40d9543ee0a8fdc5cc011d43db/Lib/typing.py#L2806
        //
        // to replace AnyStr with an annotation like [AnyStr: (bytes, str)], we also have to make
        // sure that `bytes` and `str` have their builtin values and have not been shadowed
        if self.semantic.match_typing_expr(expr, "AnyStr")
            && self.semantic.has_builtin_binding("bytes")
            && self.semantic.has_builtin_binding("str")
        {
            self.vars.push(TypeVar {
                name: "AnyStr",
                restriction: Some(TypeVarRestriction::AnyStr),
                kind: TypeParamKind::TypeVar,
                default: None,
            });
            return;
        }

        match expr {
            Expr::Name(name) if name.ctx.is_load() => {
                if let Some(var) = expr_name_to_type_var(self.semantic, name) {
                    self.vars.push(var);
                } else {
                    self.any_skipped = true;
                }
            }
            _ => visitor::walk_expr(self, expr),
        }
    }
}

/// Check if the current statement is nested within another [`StmtClassDef`] or [`StmtFunctionDef`].
fn in_nested_context(checker: &CheckerSnapshot) -> bool {
    checker
        .semantic()
        .current_statements()
        .skip(1) // skip the immediate parent, we only call this within a class or function
        .any(|stmt| matches!(stmt, Stmt::ClassDef(_) | Stmt::FunctionDef(_)))
}

/// Deduplicate `vars`, returning `None` if `vars` is empty or any duplicates are found.
fn check_type_vars(vars: Vec<TypeVar<'_>>) -> Option<Vec<TypeVar<'_>>> {
    if vars.is_empty() {
        return None;
    }

    // If any type variables were not unique, just bail out here. this is a runtime error and we
    // can't predict what the user wanted. also bail out if any Python 3.13+ default values are
    // found on the type parameters
    (vars
        .iter()
        .unique_by(|tvar| tvar.name)
        .filter(|tvar| tvar.default.is_none())
        .count()
        == vars.len())
    .then_some(vars)
}