use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::PythonVersion;

/// ## What it does
/// Identifies FastAPI routes with deprecated uses of `Depends` or similar.
///
/// ## Why is this bad?
/// The [FastAPI documentation] recommends the use of [`typing.Annotated`][typing-annotated]
/// for defining route dependencies and parameters, rather than using `Depends`,
/// `Query` or similar as a default value for a parameter. Using this approach
/// everywhere helps ensure consistency and clarity in defining dependencies
/// and parameters.
///
/// `Annotated` was added to the `typing` module in Python 3.9; however,
/// the third-party [`typing_extensions`][typing-extensions] package
/// provides a backport that can be used on older versions of Python.
///
/// ## Example
///
/// ```python
/// from fastapi import Depends, FastAPI
///
/// app = FastAPI()
///
///
/// async def common_parameters(q: str | None = None, skip: int = 0, limit: int = 100):
///     return {"q": q, "skip": skip, "limit": limit}
///
///
/// @app.get("/items/")
/// async def read_items(commons: dict = Depends(common_parameters)):
///     return commons
/// ```
///
/// Use instead:
///
/// ```python
/// from typing import Annotated
///
/// from fastapi import Depends, FastAPI
///
/// app = FastAPI()
///
///
/// async def common_parameters(q: str | None = None, skip: int = 0, limit: int = 100):
///     return {"q": q, "skip": skip, "limit": limit}
///
///
/// @app.get("/items/")
/// async def read_items(commons: Annotated[dict, Depends(common_parameters)]):
///     return commons
/// ```
///
/// ## Availability
///
/// Because this rule relies on the third-party `typing_extensions` module for Python versions
/// before 3.9, its diagnostic will not be emitted, and no fix will be offered, if
/// `typing_extensions` imports have been disabled by the [`lint.typing-extensions`] linter option.
///
/// ## Options
///
/// - `lint.typing-extensions`
///
/// [FastAPI documentation]: https://fastapi.tiangolo.com/tutorial/query-params-str-validations/?h=annotated#advantages-of-annotated
/// [typing-annotated]: https://docs.python.org/3/library/typing.html#typing.Annotated
/// [typing-extensions]: https://typing-extensions.readthedocs.io/en/stable/
#[derive(ViolationMetadata)]
pub struct FastApiNonAnnotatedDependency {
    py_version: PythonVersion,
}

impl Violation for FastApiNonAnnotatedDependency {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "FastAPI dependency without `Annotated`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        let title = if self.py_version >= PythonVersion::PY39 {
            "Replace with `typing.Annotated`"
        } else {
            "Replace with `typing_extensions.Annotated`"
        };
        Some(title.to_string())
    }
}