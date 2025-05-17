use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::Operator;

/// ## What it does
/// Checks for assignments that can be replaced with augmented assignment
/// statements.
///
/// ## Why is this bad?
/// If the right-hand side of an assignment statement consists of a binary
/// operation in which one operand is the same as the assignment target,
/// it can be rewritten as an augmented assignment. For example, `x = x + 1`
/// can be rewritten as `x += 1`.
///
/// When performing such an operation, an augmented assignment is more concise
/// and idiomatic.
///
/// ## Known problems
/// In some cases, this rule will not detect assignments in which the target
/// is on the right-hand side of a binary operation (e.g., `x = y + x`, as
/// opposed to `x = x + y`), as such operations are not commutative for
/// certain data types, like strings.
///
/// For example, `x = "prefix-" + x` is not equivalent to `x += "prefix-"`,
/// while `x = 1 + x` is equivalent to `x += 1`.
///
/// If the type of the left-hand side cannot be trivially inferred, the rule
/// will ignore the assignment.
///
/// ## Example
/// ```python
/// x = x + 1
/// ```
///
/// Use instead:
/// ```python
/// x += 1
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as augmented assignments have
/// different semantics when the target is a mutable data type, like a list or
/// dictionary.
///
/// For example, consider the following:
///
/// ```python
/// foo = [1]
/// bar = foo
/// foo = foo + [2]
/// assert (foo, bar) == ([1, 2], [1])
/// ```
///
/// If the assignment is replaced with an augmented assignment, the update
/// operation will apply to both `foo` and `bar`, as they refer to the same
/// object:
///
/// ```python
/// foo = [1]
/// bar = foo
/// foo += [2]
/// assert (foo, bar) == ([1, 2], [1, 2])
/// ```
#[derive(ViolationMetadata)]
pub struct NonAugmentedAssignment {
    operator: AugmentedOperator,
}

impl AlwaysFixableViolation for NonAugmentedAssignment {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NonAugmentedAssignment { operator } = self;
        format!("Use `{operator}` to perform an augmented assignment directly")
    }

    fn fix_title(&self) -> String {
        "Replace with augmented assignment".to_string()
    }
}

// FIX: duplicated with rufe_rule_pylint::non_augmented_assignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AugmentedOperator {
    Add,
    BitAnd,
    BitOr,
    BitXor,
    Div,
    FloorDiv,
    LShift,
    MatMult,
    Mod,
    Mult,
    Pow,
    RShift,
    Sub,
}

impl AugmentedOperator {
    /// Returns `true` if the operator is commutative.
    fn is_commutative(self) -> bool {
        matches!(
            self,
            Self::Add | Self::BitAnd | Self::BitOr | Self::BitXor | Self::Mult
        )
    }
}

impl From<Operator> for AugmentedOperator {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Add => Self::Add,
            Operator::BitAnd => Self::BitAnd,
            Operator::BitOr => Self::BitOr,
            Operator::BitXor => Self::BitXor,
            Operator::Div => Self::Div,
            Operator::FloorDiv => Self::FloorDiv,
            Operator::LShift => Self::LShift,
            Operator::MatMult => Self::MatMult,
            Operator::Mod => Self::Mod,
            Operator::Mult => Self::Mult,
            Operator::Pow => Self::Pow,
            Operator::RShift => Self::RShift,
            Operator::Sub => Self::Sub,
        }
    }
}

impl std::fmt::Display for AugmentedOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => f.write_str("+="),
            Self::BitAnd => f.write_str("&="),
            Self::BitOr => f.write_str("|="),
            Self::BitXor => f.write_str("^="),
            Self::Div => f.write_str("/="),
            Self::FloorDiv => f.write_str("//="),
            Self::LShift => f.write_str("<<="),
            Self::MatMult => f.write_str("@="),
            Self::Mod => f.write_str("%="),
            Self::Mult => f.write_str("*="),
            Self::Pow => f.write_str("**="),
            Self::RShift => f.write_str(">>="),
            Self::Sub => f.write_str("-="),
        }
    }
}