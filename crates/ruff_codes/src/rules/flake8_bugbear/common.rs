use std::fmt;

// FIX: duplicated with ruff_rule_flake8_bugbear::assert_raises_exception
#[derive(Debug, PartialEq, Eq)]
pub enum ExceptionKind {
    BaseException,
    Exception,
}

impl fmt::Display for ExceptionKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExceptionKind::BaseException => fmt.write_str("BaseException"),
            ExceptionKind::Exception => fmt.write_str("Exception"),
        }
    }
}

// FIX: duplicated with ruff_rule_flake8_bugbear::re_sub_postional_args
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Method {
    Sub,
    Subn,
    Split,
}

impl fmt::Display for Method {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sub => fmt.write_str("re.sub"),
            Self::Subn => fmt.write_str("re.subn"),
            Self::Split => fmt.write_str("re.split"),
        }
    }
}

impl Method {
   pub const fn param_name(self) -> &'static str {
        match self {
            Self::Sub => "count",
            Self::Subn => "count",
            Self::Split => "maxsplit",
        }
    }

    pub const fn num_args(self) -> usize {
        match self {
            Self::Sub => 3,
            Self::Subn => 3,
            Self::Split => 2,
        }
    }
}

// FIX: duplicated with ruff_rule_flake8_bugbear::unary_prefix_increment_decrement
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum UnaryPrefixOperatorType {
    Increment,
    Decrement,
}

// FIX: duplicated with ruff_rule_flake8_bugbear::unused_loop_control_variable
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Certainty {
    Certain,
    Uncertain,
}

// FIX: duplicated with ruff_rule_flake8_bugbear::useless_comparison
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ComparisonLocationAt {
    MiddleBody,
    EndOfFunction,
}

// FIX: duplicated with ruff_rule_flake8_bugbear::useless_expression
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    Expression,
    Attribute,
}