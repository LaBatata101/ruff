use std::fmt::{Display, Formatter};


// FIX: duplicated with ruff_rule_flake8_datetimez::helpers
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DatetimeModuleAntipattern {
    NoTzArgumentPassed,
    NonePassedToTzArgument,
}

// FIX: duplicated with ruff_rule_flake8_datetimez::datetime_min_max
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MinMax {
    /// `datetime.datetime.min`
    Min,
    /// `datetime.datetime.max`
    Max,
}

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MinMax::Min => write!(f, "min"),
            MinMax::Max => write!(f, "max"),
        }
    }
}
