use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for first-party imports that are only used for type annotations, but
/// aren't defined in a type-checking block.
///
/// ## Why is this bad?
/// Unused imports add a performance overhead at runtime, and risk creating
/// import cycles. If an import is _only_ used in typing-only contexts, it can
/// instead be imported conditionally under an `if TYPE_CHECKING:` block to
/// minimize runtime overhead.
///
/// If [`lint.flake8-type-checking.quote-annotations`] is set to `true`,
/// annotations will be wrapped in quotes if doing so would enable the
/// corresponding import to be moved into an `if TYPE_CHECKING:` block.
///
/// If a class _requires_ that type annotations be available at runtime (as is
/// the case for Pydantic, SQLAlchemy, and other libraries), consider using
/// the [`lint.flake8-type-checking.runtime-evaluated-base-classes`] and
/// [`lint.flake8-type-checking.runtime-evaluated-decorators`] settings to mark them
/// as such.
///
/// ## Example
/// ```python
/// from __future__ import annotations
///
/// import local_module
///
///
/// def func(sized: local_module.Container) -> int:
///     return len(sized)
/// ```
///
/// Use instead:
/// ```python
/// from __future__ import annotations
///
/// from typing import TYPE_CHECKING
///
/// if TYPE_CHECKING:
///     import local_module
///
///
/// def func(sized: local_module.Container) -> int:
///     return len(sized)
/// ```
///
///
/// ## Preview
/// When [preview](https://docs.astral.sh/ruff/preview/) is enabled,
/// the criterion for determining whether an import is first-party
/// is stricter, which could affect whether this lint is triggered vs [`TC001`](https://docs.astral.sh/ruff/rules/typing-only-third-party-import/). See [this FAQ section](https://docs.astral.sh/ruff/faq/#how-does-ruff-determine-which-of-my-imports-are-first-party-third-party-etc) for more details.
///
/// ## Options
/// - `lint.flake8-type-checking.quote-annotations`
/// - `lint.flake8-type-checking.runtime-evaluated-base-classes`
/// - `lint.flake8-type-checking.runtime-evaluated-decorators`
/// - `lint.flake8-type-checking.strict`
/// - `lint.typing-modules`
///
/// ## References
/// - [PEP 563: Runtime annotation resolution and `TYPE_CHECKING`](https://peps.python.org/pep-0563/#runtime-annotation-resolution-and-type-checking)
#[derive(ViolationMetadata)]
pub struct TypingOnlyFirstPartyImport {
    qualified_name: String,
}

impl Violation for TypingOnlyFirstPartyImport {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Move application import `{}` into a type-checking block",
            self.qualified_name
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Move into type-checking block".to_string())
    }
}

/// ## What it does
/// Checks for third-party imports that are only used for type annotations, but
/// aren't defined in a type-checking block.
///
/// ## Why is this bad?
/// Unused imports add a performance overhead at runtime, and risk creating
/// import cycles. If an import is _only_ used in typing-only contexts, it can
/// instead be imported conditionally under an `if TYPE_CHECKING:` block to
/// minimize runtime overhead.
///
/// If [`lint.flake8-type-checking.quote-annotations`] is set to `true`,
/// annotations will be wrapped in quotes if doing so would enable the
/// corresponding import to be moved into an `if TYPE_CHECKING:` block.
///
/// If a class _requires_ that type annotations be available at runtime (as is
/// the case for Pydantic, SQLAlchemy, and other libraries), consider using
/// the [`lint.flake8-type-checking.runtime-evaluated-base-classes`] and
/// [`lint.flake8-type-checking.runtime-evaluated-decorators`] settings to mark them
/// as such.
///
/// ## Example
/// ```python
/// from __future__ import annotations
///
/// import pandas as pd
///
///
/// def func(df: pd.DataFrame) -> int:
///     return len(df)
/// ```
///
/// Use instead:
/// ```python
/// from __future__ import annotations
///
/// from typing import TYPE_CHECKING
///
/// if TYPE_CHECKING:
///     import pandas as pd
///
///
/// def func(df: pd.DataFrame) -> int:
///     return len(df)
/// ```
///
/// ## Preview
/// When [preview](https://docs.astral.sh/ruff/preview/) is enabled,
/// the criterion for determining whether an import is first-party
/// is stricter, which could affect whether this lint is triggered vs [`TC001`](https://docs.astral.sh/ruff/rules/typing-only-first-party-import/). See [this FAQ section](https://docs.astral.sh/ruff/faq/#how-does-ruff-determine-which-of-my-imports-are-first-party-third-party-etc) for more details.
///
/// ## Options
/// - `lint.flake8-type-checking.quote-annotations`
/// - `lint.flake8-type-checking.runtime-evaluated-base-classes`
/// - `lint.flake8-type-checking.runtime-evaluated-decorators`
/// - `lint.flake8-type-checking.strict`
/// - `lint.typing-modules`
///
/// ## References
/// - [PEP 563: Runtime annotation resolution and `TYPE_CHECKING`](https://peps.python.org/pep-0563/#runtime-annotation-resolution-and-type-checking)
#[derive(ViolationMetadata)]
pub struct TypingOnlyThirdPartyImport {
    qualified_name: String,
}

impl Violation for TypingOnlyThirdPartyImport {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Move third-party import `{}` into a type-checking block",
            self.qualified_name
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Move into type-checking block".to_string())
    }
}

/// ## What it does
/// Checks for standard library imports that are only used for type
/// annotations, but aren't defined in a type-checking block.
///
/// ## Why is this bad?
/// Unused imports add a performance overhead at runtime, and risk creating
/// import cycles. If an import is _only_ used in typing-only contexts, it can
/// instead be imported conditionally under an `if TYPE_CHECKING:` block to
/// minimize runtime overhead.
///
/// If [`lint.flake8-type-checking.quote-annotations`] is set to `true`,
/// annotations will be wrapped in quotes if doing so would enable the
/// corresponding import to be moved into an `if TYPE_CHECKING:` block.
///
/// If a class _requires_ that type annotations be available at runtime (as is
/// the case for Pydantic, SQLAlchemy, and other libraries), consider using
/// the [`lint.flake8-type-checking.runtime-evaluated-base-classes`] and
/// [`lint.flake8-type-checking.runtime-evaluated-decorators`] settings to mark them
/// as such.
///
/// ## Example
/// ```python
/// from __future__ import annotations
///
/// from pathlib import Path
///
///
/// def func(path: Path) -> str:
///     return str(path)
/// ```
///
/// Use instead:
/// ```python
/// from __future__ import annotations
///
/// from typing import TYPE_CHECKING
///
/// if TYPE_CHECKING:
///     from pathlib import Path
///
///
/// def func(path: Path) -> str:
///     return str(path)
/// ```
///
/// ## Options
/// - `lint.flake8-type-checking.quote-annotations`
/// - `lint.flake8-type-checking.runtime-evaluated-base-classes`
/// - `lint.flake8-type-checking.runtime-evaluated-decorators`
/// - `lint.flake8-type-checking.strict`
/// - `lint.typing-modules`
///
/// ## References
/// - [PEP 563: Runtime annotation resolution and `TYPE_CHECKING`](https://peps.python.org/pep-0563/#runtime-annotation-resolution-and-type-checking)
#[derive(ViolationMetadata)]
pub struct TypingOnlyStandardLibraryImport {
    qualified_name: String,
}

impl Violation for TypingOnlyStandardLibraryImport {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Move standard library import `{}` into a type-checking block",
            self.qualified_name
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Move into type-checking block".to_string())
    }
}
