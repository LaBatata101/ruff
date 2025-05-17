use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Identifies FastAPI routes that declare path parameters in the route path
/// that are not included in the function signature.
///
/// ## Why is this bad?
/// Path parameters are used to extract values from the URL path.
///
/// If a path parameter is declared in the route path but not in the function
/// signature, it will not be accessible in the function body, which is likely
/// a mistake.
///
/// If a path parameter is declared in the route path, but as a positional-only
/// argument in the function signature, it will also not be accessible in the
/// function body, as FastAPI will not inject the parameter.
///
/// ## Known problems
/// If the path parameter is _not_ a valid Python identifier (e.g., `user-id`, as
/// opposed to `user_id`), FastAPI will normalize it. However, this rule simply
/// ignores such path parameters, as FastAPI's normalization behavior is undocumented.
///
/// ## Example
///
/// ```python
/// from fastapi import FastAPI
///
/// app = FastAPI()
///
///
/// @app.get("/things/{thing_id}")
/// async def read_thing(query: str): ...
/// ```
///
/// Use instead:
///
/// ```python
/// from fastapi import FastAPI
///
/// app = FastAPI()
///
///
/// @app.get("/things/{thing_id}")
/// async def read_thing(thing_id: int, query: str): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as modifying a function signature can
/// change the behavior of the code.
#[derive(ViolationMetadata)]
pub struct FastApiUnusedPathParameter {
    arg_name: String,
    function_name: String,
    is_positional: bool,
}

impl Violation for FastApiUnusedPathParameter {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self {
            arg_name,
            function_name,
            is_positional,
        } = self;
        #[expect(clippy::if_not_else)]
        if !is_positional {
            format!("Parameter `{arg_name}` appears in route path, but not in `{function_name}` signature")
        } else {
            format!(
                "Parameter `{arg_name}` appears in route path, but only as a positional-only argument in `{function_name}` signature"
            )
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Self {
            arg_name,
            is_positional,
            ..
        } = self;
        if *is_positional {
            None
        } else {
            Some(format!("Add `{arg_name}` to function signature"))
        }
    }
}