use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::Expr;

/// ## What it does
/// Checks for unnecessary dict, list, and set comprehension.
///
/// ## Why is this bad?
/// It's unnecessary to use a dict/list/set comprehension to build a data structure if the
/// elements are unchanged. Wrap the iterable with `dict()`, `list()`, or `set()` instead.
///
/// ## Example
/// ```python
/// {a: b for a, b in iterable}
/// [x for x in iterable]
/// {x for x in iterable}
/// ```
///
/// Use instead:
/// ```python
/// dict(iterable)
/// list(iterable)
/// set(iterable)
/// ```
///
/// ## Known problems
///
/// This rule may produce false positives for dictionary comprehensions that iterate over a mapping.
/// The dict constructor behaves differently depending on if it receives a sequence (e.g., a
/// list) or a mapping (e.g., a dict). When a comprehension iterates over the keys of a mapping,
/// replacing it with a `dict()` constructor call will give a different result.
///
/// For example:
///
/// ```pycon
/// >>> d1 = {(1, 2): 3, (4, 5): 6}
/// >>> {x: y for x, y in d1}  # Iterates over the keys of a mapping
/// {1: 2, 4: 5}
/// >>> dict(d1)               # Ruff's incorrect suggested fix
/// (1, 2): 3, (4, 5): 6}
/// >>> dict(d1.keys())        # Correct fix
/// {1: 2, 4: 5}
/// ```
///
/// When the comprehension iterates over a sequence, Ruff's suggested fix is correct. However, Ruff
/// cannot consistently infer if the iterable type is a sequence or a mapping and cannot suggest
/// the correct fix for mappings.
///
/// ## Fix safety
/// Due to the known problem with dictionary comprehensions, this fix is marked as unsafe.
///
/// Additionally, this fix may drop comments when rewriting the comprehension.
#[derive(ViolationMetadata)]
pub struct UnnecessaryComprehension {
    kind: ComprehensionKind,
}

impl AlwaysFixableViolation for UnnecessaryComprehension {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryComprehension { kind } = self;
        format!("Unnecessary {kind} comprehension (rewrite using `{kind}()`)")
    }

    fn fix_title(&self) -> String {
        let UnnecessaryComprehension { kind } = self;
        format!("Rewrite using `{kind}()`")
    }
}

// FIX: this is duplicated with ruff_rule_flake8_comprehensions::unnecessary_comprehension
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ComprehensionKind {
    List,
    Set,
    Dict,
}

impl ComprehensionKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Dict => "dict",
            Self::Set => "set",
        }
    }

    const fn try_from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::ListComp(_) => Some(Self::List),
            Expr::DictComp(_) => Some(Self::Dict),
            Expr::SetComp(_) => Some(Self::Set),
            _ => None,
        }
    }
}

impl std::fmt::Display for ComprehensionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}