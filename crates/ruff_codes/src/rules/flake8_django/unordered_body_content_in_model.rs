use std::fmt;

use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the order of Model's inner classes, methods, and fields as per
/// the [Django Style Guide].
///
/// ## Why is this bad?
/// The [Django Style Guide] specifies that the order of Model inner classes,
/// attributes and methods should be as follows:
///
/// 1. All database fields
/// 2. Custom manager attributes
/// 3. `class Meta`
/// 4. `def __str__()`
/// 5. `def save()`
/// 6. `def get_absolute_url()`
/// 7. Any custom methods
///
/// ## Example
/// ```python
/// from django.db import models
///
///
/// class StrBeforeFieldModel(models.Model):
///     class Meta:
///         verbose_name = "test"
///         verbose_name_plural = "tests"
///
///     def __str__(self):
///         return "foobar"
///
///     first_name = models.CharField(max_length=32)
///     last_name = models.CharField(max_length=40)
/// ```
///
/// Use instead:
/// ```python
/// from django.db import models
///
///
/// class StrBeforeFieldModel(models.Model):
///     first_name = models.CharField(max_length=32)
///     last_name = models.CharField(max_length=40)
///
///     class Meta:
///         verbose_name = "test"
///         verbose_name_plural = "tests"
///
///     def __str__(self):
///         return "foobar"
/// ```
///
/// [Django Style Guide]: https://docs.djangoproject.com/en/dev/internals/contributing/writing-code/coding-style/#model-style
#[derive(ViolationMetadata)]
pub struct DjangoUnorderedBodyContentInModel {
    element_type: ContentType,
    prev_element_type: ContentType,
}

impl Violation for DjangoUnorderedBodyContentInModel {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DjangoUnorderedBodyContentInModel {
            element_type,
            prev_element_type,
        } = self;
        format!("Order of model's inner classes, methods, and fields does not follow the Django Style Guide: {element_type} should come before {prev_element_type}")
    }
}

// FIX: duplicate with ruff_rule_flake8_django::unordered_body_content_in_model
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum ContentType {
    FieldDeclaration,
    ManagerDeclaration,
    MetaClass,
    MagicMethod,
    SaveMethod,
    GetAbsoluteUrlMethod,
    CustomMethod,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::FieldDeclaration => f.write_str("field declaration"),
            ContentType::ManagerDeclaration => f.write_str("manager declaration"),
            ContentType::MetaClass => f.write_str("`Meta` class"),
            ContentType::MagicMethod => f.write_str("Magic method"),
            ContentType::SaveMethod => f.write_str("`save` method"),
            ContentType::GetAbsoluteUrlMethod => f.write_str("`get_absolute_url` method"),
            ContentType::CustomMethod => f.write_str("custom method"),
        }
    }
}