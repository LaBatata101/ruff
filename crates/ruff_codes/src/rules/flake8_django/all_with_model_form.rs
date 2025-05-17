use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of `fields = "__all__"` in Django `ModelForm`
/// classes.
///
/// ## Why is this bad?
/// If a `ModelForm` includes the `fields = "__all__"` attribute, any new
/// field that is added to the model will automatically be exposed for
/// modification.
///
/// ## Example
/// ```python
/// from django.forms import ModelForm
///
///
/// class PostForm(ModelForm):
///     class Meta:
///         model = Post
///         fields = "__all__"
/// ```
///
/// Use instead:
/// ```python
/// from django.forms import ModelForm
///
///
/// class PostForm(ModelForm):
///     class Meta:
///         model = Post
///         fields = ["title", "content"]
/// ```
#[derive(ViolationMetadata)]
pub struct DjangoAllWithModelForm;

impl Violation for DjangoAllWithModelForm {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not use `__all__` with `ModelForm`, use `fields` instead".to_string()
    }
}