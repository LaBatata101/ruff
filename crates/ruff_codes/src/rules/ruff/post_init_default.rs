use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__post_init__` dataclass methods with parameter defaults.
///
/// ## Why is this bad?
/// Adding a default value to a parameter in a `__post_init__` method has no
/// impact on whether the parameter will have a default value in the dataclass's
/// generated `__init__` method. To create an init-only dataclass parameter with
/// a default value, you should use an `InitVar` field in the dataclass's class
/// body and give that `InitVar` field a default value.
///
/// As the [documentation] states:
///
/// > Init-only fields are added as parameters to the generated `__init__()`
/// > method, and are passed to the optional `__post_init__()` method. They are
/// > not otherwise used by dataclasses.
///
/// ## Example
/// ```python
/// from dataclasses import InitVar, dataclass
///
///
/// @dataclass
/// class Foo:
///     bar: InitVar[int] = 0
///
///     def __post_init__(self, bar: int = 1, baz: int = 2) -> None:
///         print(bar, baz)
///
///
/// foo = Foo()  # Prints '0 2'.
/// ```
///
/// Use instead:
/// ```python
/// from dataclasses import InitVar, dataclass
///
///
/// @dataclass
/// class Foo:
///     bar: InitVar[int] = 1
///     baz: InitVar[int] = 2
///
///     def __post_init__(self, bar: int, baz: int) -> None:
///         print(bar, baz)
///
///
/// foo = Foo()  # Prints '1 2'.
/// ```
///
/// ## Fix safety
///
/// This fix is always marked as unsafe because, although switching to `InitVar` is usually correct,
/// it is incorrect when the parameter is not intended to be part of the public API or when the value
/// is meant to be shared across all instances.
///
/// ## References
/// - [Python documentation: Post-init processing](https://docs.python.org/3/library/dataclasses.html#post-init-processing)
/// - [Python documentation: Init-only variables](https://docs.python.org/3/library/dataclasses.html#init-only-variables)
///
/// [documentation]: https://docs.python.org/3/library/dataclasses.html#init-only-variables
#[derive(ViolationMetadata)]
pub struct PostInitDefault;

impl Violation for PostInitDefault {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`__post_init__` method with argument defaults".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use `dataclasses.InitVar` instead".to_string())
    }
}