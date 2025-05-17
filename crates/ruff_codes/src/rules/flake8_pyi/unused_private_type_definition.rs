use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of unused private `TypeVar`, `ParamSpec` or
/// `TypeVarTuple` declarations.
///
/// ## Why is this bad?
/// A private `TypeVar` that is defined but not used is likely a mistake. It
/// should either be used, made public, or removed to avoid confusion. A type
/// variable is considered "private" if its name starts with an underscore.
///
/// ## Example
/// ```pyi
/// import typing
/// import typing_extensions
///
/// _T = typing.TypeVar("_T")
/// _Ts = typing_extensions.TypeVarTuple("_Ts")
/// ```
///
/// ## Fix safety
/// The fix is always marked as unsafe, as it would break your code if the type
/// variable is imported by another module.
#[derive(ViolationMetadata)]
pub struct UnusedPrivateTypeVar {
    type_var_like_name: String,
    type_var_like_kind: String,
}

impl Violation for UnusedPrivateTypeVar {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedPrivateTypeVar {
            type_var_like_name,
            type_var_like_kind,
        } = self;
        format!("Private {type_var_like_kind} `{type_var_like_name}` is never used")
    }

    fn fix_title(&self) -> Option<String> {
        let UnusedPrivateTypeVar {
            type_var_like_name,
            type_var_like_kind,
        } = self;
        Some(format!(
            "Remove unused private {type_var_like_kind} `{type_var_like_name}`"
        ))
    }
}

/// ## What it does
/// Checks for the presence of unused private `typing.Protocol` definitions.
///
/// ## Why is this bad?
/// A private `typing.Protocol` that is defined but not used is likely a
/// mistake. It should either be used, made public, or removed to avoid
/// confusion.
///
/// ## Example
///
/// ```pyi
/// import typing
///
/// class _PrivateProtocol(typing.Protocol):
///     foo: int
/// ```
///
/// Use instead:
///
/// ```pyi
/// import typing
///
/// class _PrivateProtocol(typing.Protocol):
///     foo: int
///
/// def func(arg: _PrivateProtocol) -> None: ...
/// ```
#[derive(ViolationMetadata)]
pub struct UnusedPrivateProtocol {
    name: String,
}

impl Violation for UnusedPrivateProtocol {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedPrivateProtocol { name } = self;
        format!("Private protocol `{name}` is never used")
    }
}

/// ## What it does
/// Checks for the presence of unused private type aliases.
///
/// ## Why is this bad?
/// A private type alias that is defined but not used is likely a
/// mistake. It should either be used, made public, or removed to avoid
/// confusion.
///
/// ## Example
///
/// ```pyi
/// import typing
///
/// _UnusedTypeAlias: typing.TypeAlias = int
/// ```
///
/// Use instead:
///
/// ```pyi
/// import typing
///
/// _UsedTypeAlias: typing.TypeAlias = int
///
/// def func(arg: _UsedTypeAlias) -> _UsedTypeAlias: ...
/// ```
#[derive(ViolationMetadata)]
pub struct UnusedPrivateTypeAlias {
    name: String,
}

impl Violation for UnusedPrivateTypeAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedPrivateTypeAlias { name } = self;
        format!("Private TypeAlias `{name}` is never used")
    }
}

/// ## What it does
/// Checks for the presence of unused private `typing.TypedDict` definitions.
///
/// ## Why is this bad?
/// A private `typing.TypedDict` that is defined but not used is likely a
/// mistake. It should either be used, made public, or removed to avoid
/// confusion.
///
/// ## Example
///
/// ```pyi
/// import typing
///
/// class _UnusedPrivateTypedDict(typing.TypedDict):
///     foo: list[int]
/// ```
///
/// Use instead:
///
/// ```pyi
/// import typing
///
/// class _UsedPrivateTypedDict(typing.TypedDict):
///     foo: set[str]
///
/// def func(arg: _UsedPrivateTypedDict) -> _UsedPrivateTypedDict: ...
/// ```
#[derive(ViolationMetadata)]
pub struct UnusedPrivateTypedDict {
    name: String,
}

impl Violation for UnusedPrivateTypedDict {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedPrivateTypedDict { name } = self;
        format!("Private TypedDict `{name}` is never used")
    }
}
