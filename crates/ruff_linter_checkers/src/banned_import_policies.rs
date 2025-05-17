use ruff_linter_commons::matchers::{MatchName, MatchNameOrParent, NameMatchPolicy};
use ruff_python_ast::helpers::resolve_imported_module_path;
use ruff_python_ast::{Alias, AnyNodeRef, Stmt, StmtImport, StmtImportFrom};
use std::borrow::Cow;

use crate::ast::CheckerSnapshot;

pub enum BannedModuleImportPolicies<'a> {
    Import(&'a StmtImport),
    ImportFrom {
        module: Option<Cow<'a, str>>,
        node: &'a StmtImportFrom,
    },
    NonImport,
}

impl<'a> BannedModuleImportPolicies<'a> {
    pub fn new(stmt: &'a Stmt, checker: &CheckerSnapshot) -> Self {
        match stmt {
            Stmt::Import(import) => Self::Import(import),
            Stmt::ImportFrom(import @ StmtImportFrom { module, level, .. }) => {
                let module = resolve_imported_module_path(
                    *level,
                    module.as_deref(),
                    checker.module.qualified_name(),
                );

                Self::ImportFrom {
                    module,
                    node: import,
                }
            }
            _ => Self::NonImport,
        }
    }
}

impl<'a> IntoIterator for &'a BannedModuleImportPolicies<'a> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = BannedModuleImportPoliciesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            BannedModuleImportPolicies::Import(import) => {
                BannedModuleImportPoliciesIter::Import(import.names.iter())
            }
            BannedModuleImportPolicies::ImportFrom { module, node } => {
                BannedModuleImportPoliciesIter::ImportFrom {
                    module: module.as_deref(),
                    names: node.names.iter(),
                    import: Some(node),
                }
            }
            BannedModuleImportPolicies::NonImport => BannedModuleImportPoliciesIter::NonImport,
        }
    }
}

pub enum BannedModuleImportPoliciesIter<'a> {
    Import(std::slice::Iter<'a, Alias>),
    ImportFrom {
        module: Option<&'a str>,
        names: std::slice::Iter<'a, Alias>,
        import: Option<&'a StmtImportFrom>,
    },
    NonImport,
}

impl<'a> Iterator for BannedModuleImportPoliciesIter<'a> {
    type Item = (NameMatchPolicy<'a>, AnyNodeRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Import(names) => {
                let name = names.next()?;
                Some((
                    NameMatchPolicy::MatchNameOrParent(MatchNameOrParent { module: &name.name }),
                    name.into(),
                ))
            }
            Self::ImportFrom {
                module,
                import,
                names,
            } => {
                let module = module.as_ref()?;

                if let Some(import) = import.take() {
                    return Some((
                        NameMatchPolicy::MatchNameOrParent(MatchNameOrParent { module }),
                        import.into(),
                    ));
                }

                loop {
                    let alias = names.next()?;
                    if &alias.name == "*" {
                        continue;
                    }

                    break Some((
                        NameMatchPolicy::MatchName(MatchName {
                            module,
                            member: &alias.name,
                        }),
                        alias.into(),
                    ));
                }
            }
            Self::NonImport => None,
        }
    }
}
