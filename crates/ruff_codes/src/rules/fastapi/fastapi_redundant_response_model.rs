use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for FastAPI routes that use the optional `response_model` parameter
/// with the same type as the return type.
///
/// ## Why is this bad?
/// FastAPI routes automatically infer the response model type from the return
/// type, so specifying it explicitly is redundant.
///
/// The `response_model` parameter is used to override the default response
/// model type. For example, `response_model` can be used to specify that
/// a non-serializable response type should instead be serialized via an
/// alternative type.
///
/// For more information, see the [FastAPI documentation](https://fastapi.tiangolo.com/tutorial/response-model/).
///
/// ## Example
///
/// ```python
/// from fastapi import FastAPI
/// from pydantic import BaseModel
///
/// app = FastAPI()
///
///
/// class Item(BaseModel):
///     name: str
///
///
/// @app.post("/items/", response_model=Item)
/// async def create_item(item: Item) -> Item:
///     return item
/// ```
///
/// Use instead:
///
/// ```python
/// from fastapi import FastAPI
/// from pydantic import BaseModel
///
/// app = FastAPI()
///
///
/// class Item(BaseModel):
///     name: str
///
///
/// @app.post("/items/")
/// async def create_item(item: Item) -> Item:
///     return item
/// ```
#[derive(ViolationMetadata)]
pub struct FastApiRedundantResponseModel;

impl AlwaysFixableViolation for FastApiRedundantResponseModel {
    #[derive_message_formats]
    fn message(&self) -> String {
        "FastAPI route with redundant `response_model` argument".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove argument".to_string()
    }
}