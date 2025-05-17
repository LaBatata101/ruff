use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::CmpOp;

/// ## What it does
/// Checks for membership tests against single-item containers.
///
/// ## Why is this bad?
/// Performing a membership test against a container (like a `list` or `set`)
/// with a single item is less readable and less efficient than comparing
/// against the item directly.
///
/// ## Example
/// ```python
/// 1 in [1]
/// ```
///
/// Use instead:
/// ```python
/// 1 == 1
/// ```
///
/// ## Fix safety
///
/// When the right-hand side is a string, the fix is marked as unsafe.
/// This is because `c in "a"` is true both when `c` is `"a"` and when `c` is the empty string,
/// so the fix can change the behavior of your program in these cases.
///
/// Additionally, if there are comments within the fix's range,
/// it will also be marked as unsafe.
///
/// ## References
/// - [Python documentation: Comparisons](https://docs.python.org/3/reference/expressions.html#comparisons)
/// - [Python documentation: Membership test operations](https://docs.python.org/3/reference/expressions.html#membership-test-operations)
#[derive(ViolationMetadata)]
pub struct SingleItemMembershipTest {
    membership_test: MembershipTest,
}

impl Violation for SingleItemMembershipTest {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Membership test against single-item container".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        let SingleItemMembershipTest { membership_test } = self;
        match membership_test {
            MembershipTest::In => Some("Convert to equality test".to_string()),
            MembershipTest::NotIn => Some("Convert to inequality test".to_string()),
        }
    }
}

// FIX: duplicated with ruff_rule_refurb::single_item_membership_test
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MembershipTest {
    /// Ex) `1 in [1]`
    In,
    /// Ex) `1 not in [1]`
    NotIn,
}

impl MembershipTest {
    /// Returns the replacement comparison operator for this membership test.
    fn replacement_op(self) -> CmpOp {
        match self {
            MembershipTest::In => CmpOp::Eq,
            MembershipTest::NotIn => CmpOp::NotEq,
        }
    }
}