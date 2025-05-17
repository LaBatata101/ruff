use std::fmt::{Display, Formatter};

use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for incorrect function signatures on `__exit__` and `__aexit__`
/// methods.
///
/// ## Why is this bad?
/// Improperly annotated `__exit__` and `__aexit__` methods can cause
/// unexpected behavior when interacting with type checkers.
///
/// ## Example
///
/// ```pyi
/// from types import TracebackType
///
/// class Foo:
///     def __exit__(
///         self, typ: BaseException, exc: BaseException, tb: TracebackType
///     ) -> None: ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// from types import TracebackType
///
/// class Foo:
///     def __exit__(
///         self,
///         typ: type[BaseException] | None,
///         exc: BaseException | None,
///         tb: TracebackType | None,
///     ) -> None: ...
/// ```
#[derive(ViolationMetadata)]
pub struct BadExitAnnotation {
    func_kind: FuncKind,
    error_kind: ErrorKind,
}

impl Violation for BadExitAnnotation {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let method_name = self.func_kind.to_string();
        match self.error_kind {
            ErrorKind::StarArgsNotAnnotated => format!("Star-args in `{method_name}` should be annotated with `object`"),
            ErrorKind::MissingArgs => format!("If there are no star-args, `{method_name}` should have at least 3 non-keyword-only args (excluding `self`)"),
            ErrorKind::ArgsAfterFirstFourMustHaveDefault => format!("All arguments after the first four in `{method_name}` must have a default value"),
            ErrorKind::AllKwargsMustHaveDefault => format!("All keyword-only arguments in `{method_name}` must have a default value"),
            ErrorKind::FirstArgBadAnnotation => format!("The first argument in `{method_name}` should be annotated with `object` or `type[BaseException] | None`"),
            ErrorKind::SecondArgBadAnnotation => format!("The second argument in `{method_name}` should be annotated with `object` or `BaseException | None`"),
            ErrorKind::ThirdArgBadAnnotation => format!("The third argument in `{method_name}` should be annotated with `object` or `types.TracebackType | None`"),
            ErrorKind::UnrecognizedExitOverload => format!(
                "Annotations for a three-argument `{method_name}` overload (excluding `self`) \
                should either be `None, None, None` or `type[BaseException], BaseException, types.TracebackType`"
            )
        }
    }

    fn fix_title(&self) -> Option<String> {
        if matches!(self.error_kind, ErrorKind::StarArgsNotAnnotated) {
            Some("Annotate star-args with `object`".to_string())
        } else {
            None
        }
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::exit_annotations
#[derive(Debug, Copy, Clone, Eq, PartialEq, is_macro::Is)]
enum FuncKind {
    Sync,
    Async,
}

impl FuncKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Async => "__aexit__",
            Self::Sync => "__exit__",
        }
    }
}

impl Display for FuncKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::exit_annotations
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ErrorKind {
    StarArgsNotAnnotated,
    MissingArgs,
    FirstArgBadAnnotation,
    SecondArgBadAnnotation,
    ThirdArgBadAnnotation,
    ArgsAfterFirstFourMustHaveDefault,
    AllKwargsMustHaveDefault,
    UnrecognizedExitOverload,
}