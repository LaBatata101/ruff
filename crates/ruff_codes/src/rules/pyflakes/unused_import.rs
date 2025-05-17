use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unused imports.
///
/// ## Why is this bad?
/// Unused imports add a performance overhead at runtime, and risk creating
/// import cycles. They also increase the cognitive load of reading the code.
///
/// If an import statement is used to check for the availability or existence
/// of a module, consider using `importlib.util.find_spec` instead.
///
/// If an import statement is used to re-export a symbol as part of a module's
/// public interface, consider using a "redundant" import alias, which
/// instructs Ruff (and other tools) to respect the re-export, and avoid
/// marking it as unused, as in:
///
/// ```python
/// from module import member as member
/// ```
///
/// Alternatively, you can use `__all__` to declare a symbol as part of the module's
/// interface, as in:
///
/// ```python
/// # __init__.py
/// import some_module
///
/// __all__ = ["some_module"]
/// ```
///
/// ## Fix safety
///
/// Fixes to remove unused imports are safe, except in `__init__.py` files.
///
/// Applying fixes to `__init__.py` files is currently in preview. The fix offered depends on the
/// type of the unused import. Ruff will suggest a safe fix to export first-party imports with
/// either a redundant alias or, if already present in the file, an `__all__` entry. If multiple
/// `__all__` declarations are present, Ruff will not offer a fix. Ruff will suggest an unsafe fix
/// to remove third-party and standard library imports -- the fix is unsafe because the module's
/// interface changes.
///
/// ## Example
///
/// ```python
/// import numpy as np  # unused import
///
///
/// def area(radius):
///     return 3.14 * radius**2
/// ```
///
/// Use instead:
///
/// ```python
/// def area(radius):
///     return 3.14 * radius**2
/// ```
///
/// To check the availability of a module, use `importlib.util.find_spec`:
///
/// ```python
/// from importlib.util import find_spec
///
/// if find_spec("numpy") is not None:
///     print("numpy is installed")
/// else:
///     print("numpy is not installed")
/// ```
///
/// ## Preview
/// When [preview](https://docs.astral.sh/ruff/preview/) is enabled,
/// the criterion for determining whether an import is first-party
/// is stricter, which could affect the suggested fix. See [this FAQ section](https://docs.astral.sh/ruff/faq/#how-does-ruff-determine-which-of-my-imports-are-first-party-third-party-etc) for more details.
///
/// ## Options
/// - `lint.ignore-init-module-imports`
/// - `lint.pyflakes.allowed-unused-imports`
///
/// ## References
/// - [Python documentation: `import`](https://docs.python.org/3/reference/simple_stmts.html#the-import-statement)
/// - [Python documentation: `importlib.util.find_spec`](https://docs.python.org/3/library/importlib.html#importlib.util.find_spec)
/// - [Typing documentation: interface conventions](https://typing.python.org/en/latest/source/libraries.html#library-interface-public-and-private-symbols)
#[derive(ViolationMetadata)]
pub struct UnusedImport {
    /// Qualified name of the import
    name: String,
    /// Unqualified name of the import
    module: String,
    /// Name of the import binding
    binding: String,
    context: UnusedImportContext,
    multiple: bool,
    ignore_init_module_imports: bool,
}

impl Violation for UnusedImport {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedImport { name, context, .. } = self;
        match context {
            UnusedImportContext::ExceptHandler => {
                format!(
                    "`{name}` imported but unused; consider using `importlib.util.find_spec` to test for availability"
                )
            }
            UnusedImportContext::DunderInitFirstParty { .. } => {
                format!(
                    "`{name}` imported but unused; consider removing, adding to `__all__`, or using a redundant alias"
                )
            }
            UnusedImportContext::Other => format!("`{name}` imported but unused"),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let UnusedImport {
            name,
            module,
            binding,
            multiple,
            ignore_init_module_imports,
            context,
        } = self;
        if *ignore_init_module_imports {
            match context {
                UnusedImportContext::DunderInitFirstParty {
                    dunder_all_count: DunderAllCount::Zero,
                    submodule_import: false,
                } => return Some(format!("Use an explicit re-export: `{module} as {module}`")),
                UnusedImportContext::DunderInitFirstParty {
                    dunder_all_count: DunderAllCount::Zero,
                    submodule_import: true,
                } => {
                    return Some(format!(
                    "Use an explicit re-export: `import {parent} as {parent}; import {binding}`",
                    parent = binding
                        .split('.')
                        .next()
                        .expect("Expected all submodule imports to contain a '.'")
                    ))
                }
                UnusedImportContext::DunderInitFirstParty {
                    dunder_all_count: DunderAllCount::One,
                    submodule_import: false,
                } => return Some(format!("Add unused import `{binding}` to __all__")),
                UnusedImportContext::DunderInitFirstParty {
                    dunder_all_count: DunderAllCount::One,
                    submodule_import: true,
                } => {
                    return Some(format!(
                        "Add `{}` to __all__",
                        binding
                            .split('.')
                            .next()
                            .expect("Expected all submodule imports to contain a '.'")
                    ))
                }
                UnusedImportContext::DunderInitFirstParty {
                    dunder_all_count: DunderAllCount::Many,
                    submodule_import: _,
                }
                | UnusedImportContext::ExceptHandler
                | UnusedImportContext::Other => {}
            }
        }
        Some(if *multiple {
            "Remove unused import".to_string()
        } else {
            format!("Remove unused import: `{name}`")
        })
    }
}

// FIX: duplicated with ruff_rule_pyflakes::unused_import
#[derive(Debug, Copy, Clone, Eq, PartialEq, is_macro::Is)]
enum UnusedImportContext {
    /// The unused import occurs inside an except handler
    ExceptHandler,
    /// The unused import is a first-party import in an `__init__.py` file
    DunderInitFirstParty {
        dunder_all_count: DunderAllCount,
        submodule_import: bool,
    },
    /// The unused import is something else
    Other,
}

// FIX: duplicated with ruff_rule_pyflakes::unused_import
/// Enumeration providing three possible answers to the question:
/// "How many `__all__` definitions are there in this file?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DunderAllCount {
    Zero,
    One,
    Many,
}