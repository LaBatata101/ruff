use ruff_python_ast::Expr;
use ruff_python_semantic::SemanticModel;

// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_cell_around_sorted
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnnecessaryFunction {
    List,
    Reversed,
}

impl UnnecessaryFunction {
    pub fn try_from_str(name: &str) -> Option<Self> {
        match name {
            "list" => Some(Self::List),
            "reversed" => Some(Self::Reversed),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Reversed => "reversed",
        }
    }
}

impl std::fmt::Display for UnnecessaryFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_collection_call
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Collection {
    Tuple,
    List,
    Dict,
}

impl Collection {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Dict => "dict",
            Self::List => "list",
            Self::Tuple => "tuple",
        }
    }
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_literal_dict and unnecessary_literal_within_list_call
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LiteralKind {
    Tuple,
    List,
}

impl LiteralKind {
    const fn try_from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::Tuple(_) => Some(Self::Tuple),
            Expr::List(_) => Some(Self::List),
            _ => None,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Tuple => "tuple",
            Self::List => "list",
        }
    }
}

impl std::fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}


// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_literal_set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnnecessaryLiteral {
    List,
    Tuple,
}

impl UnnecessaryLiteral {
    const fn try_from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::List(_) => Some(Self::List),
            Expr::Tuple(_) => Some(Self::Tuple),
            _ => None,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Tuple => "tuple",
            Self::List => "list",
        }
    }
}

impl std::fmt::Display for UnnecessaryLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_literal_within_dict_call
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DictKind {
    Literal,
    Comprehension,
}

impl DictKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Literal => "literal",
            Self::Comprehension => "comprehension",
        }
    }

    const fn try_from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::Dict(_) => Some(Self::Literal),
            Expr::DictComp(_) => Some(Self::Comprehension),
            _ => None,
        }
    }
}

impl std::fmt::Display for DictKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_literal_within_tuple_call
#[derive(Debug, PartialEq, Eq)]
pub enum TupleLiteralKind {
    List,
    Tuple,
    ListComp,
}

// FIX: duplicated with ruff_rule_flake8_comprehensions::unnecessary_map
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Generator,
    List,
    Set,
    Dict,
}

impl ObjectType {
    fn from(func: &Expr, semantic: &SemanticModel) -> Option<Self> {
        match semantic.resolve_builtin_symbol(func) {
            Some("map") => Some(Self::Generator),
            Some("list") => Some(Self::List),
            Some("set") => Some(Self::Set),
            Some("dict") => Some(Self::Dict),
            _ => None,
        }
    }
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObjectType::Generator => fmt.write_str("generator expression"),
            ObjectType::List => fmt.write_str("list comprehension"),
            ObjectType::Set => fmt.write_str("set comprehension"),
            ObjectType::Dict => fmt.write_str("dict comprehension"),
        }
    }
}
