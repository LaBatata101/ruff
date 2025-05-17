use std::fmt::Display;

use ruff_python_ast::{self as ast, name::Name, Arguments, Expr, ExprCall, ExprName, ExprSubscript, Identifier, Stmt, StmtAssign, StmtExpr, StmtFunctionDef, StmtRaise, TypeParam, TypeParamParamSpec, TypeParamTypeVar, TypeParamTypeVarTuple};
use ruff_python_semantic::{analyze::typing, SemanticModel};
use ruff_text_size::{Ranged, TextRange};

pub fn is_dunder_operator_method(method: &str) -> bool {
    matches!(
        method,
        "__lt__"
            | "__le__"
            | "__eq__"
            | "__ne__"
            | "__gt__"
            | "__ge__"
            | "__add__"
            | "__sub__"
            | "__mul__"
            | "__matmul__"
            | "__truediv__"
            | "__floordiv__"
            | "__mod__"
            | "__divmod__"
            | "__pow__"
            | "__lshift__"
            | "__rshift__"
            | "__and__"
            | "__xor__"
            | "__or__"
            | "__radd__"
            | "__rsub__"
            | "__rmul__"
            | "__rmatmul__"
            | "__rtruediv__"
            | "__rfloordiv__"
            | "__rmod__"
            | "__rdivmod__"
            | "__rpow__"
            | "__rlshift__"
            | "__rrshift__"
            | "__rand__"
            | "__rxor__"
            | "__ror__"
            | "__iadd__"
            | "__isub__"
            | "__imul__"
            | "__imatmul__"
            | "__itruediv__"
            | "__ifloordiv__"
            | "__imod__"
            | "__ipow__"
            | "__ilshift__"
            | "__irshift__"
            | "__iand__"
            | "__ixor__"
            | "__ior__"
    )
}

/// Returns `true` if a function appears to be a base class stub. In other
/// words, if it matches the following syntax:
///
/// ```text
/// variable = <string | f-string>
/// raise NotImplementedError(variable)
/// ```
///
/// See also [`is_stub`]. We're a bit more generous in what is considered a
/// stub in this rule to avoid clashing with [`EM101`].
///
/// [`is_stub`]: function_type::is_stub
/// [`EM101`]: https://docs.astral.sh/ruff/rules/raw-string-in-exception/
pub fn is_not_implemented_stub_with_variable(
    function_def: &StmtFunctionDef,
    semantic: &SemanticModel,
) -> bool {
    // Ignore doc-strings.
    let statements = match function_def.body.as_slice() {
        [Stmt::Expr(StmtExpr { value, .. }), rest @ ..] if value.is_string_literal_expr() => rest,
        _ => &function_def.body,
    };

    let [Stmt::Assign(ast::StmtAssign { targets, value, .. }), Stmt::Raise(StmtRaise {
        exc: Some(exception),
        ..
    })] = statements
    else {
        return false;
    };

    if !matches!(**value, ast::Expr::StringLiteral(_) | ast::Expr::FString(_)) {
        return false;
    }

    let ast::Expr::Call(ast::ExprCall {
        func, arguments, ..
    }) = &**exception
    else {
        return false;
    };

    if !semantic.match_builtin_expr(func, "NotImplementedError") {
        return false;
    }

    let [argument] = &*arguments.args else {
        return false;
    };

    let [target] = targets.as_slice() else {
        return false;
    };

    argument.as_name_expr().map(ast::ExprName::id) == target.as_name_expr().map(ast::ExprName::id)
}

pub fn expr_name_to_type_var<'a>(
    semantic: &'a SemanticModel,
    name: &'a ExprName,
) -> Option<TypeVar<'a>> {
    let StmtAssign { value, .. } = semantic
        .lookup_symbol(name.id.as_str())
        .and_then(|binding_id| semantic.binding(binding_id).source)
        .map(|node_id| semantic.statement(node_id))?
        .as_assign_stmt()?;

    match value.as_ref() {
        Expr::Subscript(ExprSubscript {
            value: ref subscript_value,
            ..
        }) => {
            if semantic.match_typing_expr(subscript_value, "TypeVar") {
                return Some(TypeVar {
                    name: &name.id,
                    restriction: None,
                    kind: TypeParamKind::TypeVar,
                    default: None,
                });
            }
        }
        Expr::Call(ExprCall {
            func, arguments, ..
        }) => {
            let kind = if semantic.match_typing_expr(func, "TypeVar") {
                TypeParamKind::TypeVar
            } else if semantic.match_typing_expr(func, "TypeVarTuple") {
                TypeParamKind::TypeVarTuple
            } else if semantic.match_typing_expr(func, "ParamSpec") {
                TypeParamKind::ParamSpec
            } else {
                return None;
            };

            if arguments
                .args
                .first()
                .is_some_and(Expr::is_string_literal_expr)
            {
                // TODO(brent) `default` was added in PEP 696 and Python 3.13 but can't be used in
                // generic type parameters before that
                //
                // ```python
                // T = TypeVar("T", default=Any, bound=str)
                // class slice(Generic[T]): ...
                // ```
                //
                // becomes
                //
                // ```python
                // class slice[T: str = Any]: ...
                // ```
                let default = arguments
                    .find_keyword("default")
                    .map(|default| &default.value);
                let restriction = if let Some(bound) = arguments.find_keyword("bound") {
                    Some(TypeVarRestriction::Bound(&bound.value))
                } else if arguments.args.len() > 1 {
                    Some(TypeVarRestriction::Constraint(
                        arguments.args.iter().skip(1).collect(),
                    ))
                } else {
                    None
                };

                return Some(TypeVar {
                    name: &name.id,
                    restriction,
                    kind,
                    default,
                });
            }
        }
        _ => {}
    }
    None
}

/// Search `class_bases` for a `typing.Generic` base class. Returns the `Generic` expression (if
/// any), along with its index in the class's bases tuple.
pub fn find_generic<'a>(
    class_bases: &'a Arguments,
    semantic: &SemanticModel,
) -> Option<(usize, &'a ExprSubscript)> {
    class_bases.args.iter().enumerate().find_map(|(idx, expr)| {
        expr.as_subscript_expr().and_then(|sub_expr| {
            semantic
                .match_typing_expr(&sub_expr.value, "Generic")
                .then_some((idx, sub_expr))
        })
    })
}

/// Wrapper for formatting a sequence of [`TypeVar`]s for use as a generic type parameter (e.g. `[T,
/// *Ts, **P]`). See [`DisplayTypeVar`] for further details.
pub struct DisplayTypeVars<'a> {
    pub type_vars: &'a [TypeVar<'a>],
    pub source: &'a str,
}

/// Used for displaying `type_var`. `source` is the whole file, which will be sliced to recover the
/// `TypeVarRestriction` values for generic bounds and constraints.
pub struct DisplayTypeVar<'a> {
    type_var: &'a TypeVar<'a>,
    source: &'a str,
}

impl Display for DisplayTypeVars<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nvars = self.type_vars.len();
        if nvars == 0 {
            return Ok(());
        }
        f.write_str("[")?;
        for (i, tv) in self.type_vars.iter().enumerate() {
            write!(f, "{}", tv.display(self.source))?;
            if i < nvars - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str("]")?;

        Ok(())
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypeParamKind {
    TypeVar,
    TypeVarTuple,
    ParamSpec,
}

#[derive(Debug)]
pub enum TypeVarRestriction<'a> {
    /// A type variable with a bound, e.g., `TypeVar("T", bound=int)`.
    Bound(&'a Expr),
    /// A type variable with constraints, e.g., `TypeVar("T", int, str)`.
    Constraint(Vec<&'a Expr>),
    /// `AnyStr` is a special case: the only public `TypeVar` defined in the standard library,
    /// and thus the only one that we recognise when imported from another module.
    AnyStr,
}

#[derive(Debug)]
pub struct TypeVar<'a> {
    pub name: &'a str,
    pub restriction: Option<TypeVarRestriction<'a>>,
    pub kind: TypeParamKind,
    pub default: Option<&'a Expr>,
}

impl TypeVar<'_> {
    fn display<'a>(&'a self, source: &'a str) -> DisplayTypeVar<'a> {
        DisplayTypeVar {
            type_var: self,
            source,
        }
    }
}

impl Display for DisplayTypeVar<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.type_var.kind {
            TypeParamKind::TypeVar => {}
            TypeParamKind::TypeVarTuple => f.write_str("*")?,
            TypeParamKind::ParamSpec => f.write_str("**")?,
        }
        f.write_str(self.type_var.name)?;
        if let Some(restriction) = &self.type_var.restriction {
            f.write_str(": ")?;
            match restriction {
                TypeVarRestriction::Bound(bound) => {
                    f.write_str(&self.source[bound.range()])?;
                }
                TypeVarRestriction::AnyStr => f.write_str("(bytes, str)")?,
                TypeVarRestriction::Constraint(vec) => {
                    let len = vec.len();
                    f.write_str("(")?;
                    for (i, v) in vec.iter().enumerate() {
                        f.write_str(&self.source[v.range()])?;
                        if i < len - 1 {
                            f.write_str(", ")?;
                        }
                    }
                    f.write_str(")")?;
                }
            }
        }

        Ok(())
    }
}

impl<'a> From<&'a TypeVar<'a>> for TypeParam {
    fn from(
        TypeVar {
            name,
            restriction,
            kind,
            default: _, // TODO(brent) see below
        }: &'a TypeVar<'a>,
    ) -> Self {
        match kind {
            TypeParamKind::TypeVar => {
                TypeParam::TypeVar(TypeParamTypeVar {
                    range: TextRange::default(),
                    name: Identifier::new(*name, TextRange::default()),
                    bound: match restriction {
                        Some(TypeVarRestriction::Bound(bound)) => Some(Box::new((*bound).clone())),
                        Some(TypeVarRestriction::Constraint(constraints)) => {
                            Some(Box::new(Expr::Tuple(ast::ExprTuple {
                                range: TextRange::default(),
                                elts: constraints.iter().map(|expr| (*expr).clone()).collect(),
                                ctx: ast::ExprContext::Load,
                                parenthesized: true,
                            })))
                        }
                        Some(TypeVarRestriction::AnyStr) => {
                            Some(Box::new(Expr::Tuple(ast::ExprTuple {
                                range: TextRange::default(),
                                elts: vec![
                                    Expr::Name(ExprName {
                                        range: TextRange::default(),
                                        id: Name::from("str"),
                                        ctx: ast::ExprContext::Load,
                                    }),
                                    Expr::Name(ExprName {
                                        range: TextRange::default(),
                                        id: Name::from("bytes"),
                                        ctx: ast::ExprContext::Load,
                                    }),
                                ],
                                ctx: ast::ExprContext::Load,
                                parenthesized: true,
                            })))
                        }
                        None => None,
                    },
                    // We don't handle defaults here yet. Should perhaps be a different rule since
                    // defaults are only valid in 3.13+.
                    default: None,
                })
            }
            TypeParamKind::TypeVarTuple => TypeParam::TypeVarTuple(TypeParamTypeVarTuple {
                range: TextRange::default(),
                name: Identifier::new(*name, TextRange::default()),
                default: None,
            }),
            TypeParamKind::ParamSpec => TypeParam::ParamSpec(TypeParamParamSpec {
                range: TextRange::default(),
                name: Identifier::new(*name, TextRange::default()),
                default: None,
            }),
        }
    }
}

impl<'a> From<&'a TypeParam> for TypeVar<'a> {
    fn from(param: &'a TypeParam) -> Self {
        let (kind, restriction) = match param {
            TypeParam::TypeVarTuple(_) => (TypeParamKind::TypeVarTuple, None),
            TypeParam::ParamSpec(_) => (TypeParamKind::ParamSpec, None),

            TypeParam::TypeVar(param) => {
                let restriction = match param.bound.as_deref() {
                    None => None,
                    Some(Expr::Tuple(constraints)) => Some(TypeVarRestriction::Constraint(
                        constraints.elts.iter().collect::<Vec<_>>(),
                    )),
                    Some(bound) => Some(TypeVarRestriction::Bound(bound)),
                };

                (TypeParamKind::TypeVar, restriction)
            }
        };

        Self {
            name: param.name(),
            kind,
            restriction,
            default: param.default(),
        }
    }
}

/// Returns `true` if the function is a FastAPI route.
pub fn is_fastapi_route(
    function_def: &ast::StmtFunctionDef,
    semantic: &SemanticModel,
) -> bool {
    function_def
        .decorator_list
        .iter()
        .any(|decorator| is_fastapi_route_decorator(decorator, semantic).is_some())
}

/// Returns `true` if the decorator is indicative of a FastAPI route.
pub fn is_fastapi_route_decorator<'a>(
    decorator: &'a ast::Decorator,
    semantic: &'a SemanticModel,
) -> Option<&'a ast::ExprCall> {
    let call = decorator.expression.as_call_expr()?;
    is_fastapi_route_call(call, semantic).then_some(call)
}

pub fn is_fastapi_route_call(call_expr: &ast::ExprCall, semantic: &SemanticModel) -> bool {
    let ast::Expr::Attribute(ast::ExprAttribute { attr, value, .. }) = &*call_expr.func else {
        return false;
    };

    if !matches!(
        attr.as_str(),
        "get" | "post" | "put" | "delete" | "patch" | "options" | "head" | "trace"
    ) {
        return false;
    }
    let Some(name) = value.as_name_expr() else {
        return false;
    };
    let Some(binding_id) = semantic.resolve_name(name) else {
        return false;
    };
    typing::is_fastapi_route(semantic.binding(binding_id), semantic)
}


pub fn is_pytest_raises(func: &Expr, semantic: &SemanticModel) -> bool {
    semantic
        .resolve_qualified_name(func)
        .is_some_and(|qualified_name| matches!(qualified_name.segments(), ["pytest", "raises"]))
}