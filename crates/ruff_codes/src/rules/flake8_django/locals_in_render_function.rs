use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of `locals()` in `render` functions.
///
/// ## Why is this bad?
/// Using `locals()` can expose internal variables or other unintentional
/// data to the rendered template.
///
/// ## Example
/// ```python
/// from django.shortcuts import render
///
///
/// def index(request):
///     posts = Post.objects.all()
///     return render(request, "app/index.html", locals())
/// ```
///
/// Use instead:
/// ```python
/// from django.shortcuts import render
///
///
/// def index(request):
///     posts = Post.objects.all()
///     context = {"posts": posts}
///     return render(request, "app/index.html", context)
/// ```
#[derive(ViolationMetadata)]
pub struct DjangoLocalsInRenderFunction;

impl Violation for DjangoLocalsInRenderFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid passing `locals()` as context to a `render` function".to_string()
    }
}