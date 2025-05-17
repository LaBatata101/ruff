use std::borrow::Cow;

use rustc_hash::FxHashMap;

use ruff_python_ast::helpers::format_import_from;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TrailingComma {
    Present,
    #[default]
    Absent,
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ImportFromData<'a> {
    pub module: Option<&'a str>,
    pub level: u32,
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct AliasData<'a> {
    pub name: &'a str,
    pub asname: Option<&'a str>,
}

#[derive(Debug, Default, Clone)]
pub struct ImportCommentSet<'a> {
    pub atop: Vec<Cow<'a, str>>,
    pub inline: Vec<Cow<'a, str>>,
}

#[derive(Debug, Default, Clone)]
pub struct ImportFromCommentSet<'a> {
    pub atop: Vec<Cow<'a, str>>,
    pub inline: Vec<Cow<'a, str>>,
    pub trailing: Vec<Cow<'a, str>>,
}

pub trait Importable<'a> {
    fn module_name(&self) -> Cow<'a, str>;

    fn module_base(&self) -> Cow<'a, str> {
        match self.module_name() {
            Cow::Borrowed(module_name) => Cow::Borrowed(
                module_name
                    .split('.')
                    .next()
                    .expect("module to include at least one segment"),
            ),
            Cow::Owned(module_name) => Cow::Owned(
                module_name
                    .split('.')
                    .next()
                    .expect("module to include at least one segment")
                    .to_owned(),
            ),
        }
    }
}

impl<'a> Importable<'a> for AliasData<'a> {
    fn module_name(&self) -> Cow<'a, str> {
        Cow::Borrowed(self.name)
    }
}

impl<'a> Importable<'a> for ImportFromData<'a> {
    fn module_name(&self) -> Cow<'a, str> {
        format_import_from(self.level, self.module)
    }
}

#[derive(Debug, Default)]
pub struct ImportFromStatement<'a> {
    pub comments: ImportFromCommentSet<'a>,
    pub aliases: FxHashMap<AliasData<'a>, ImportFromCommentSet<'a>>,
    pub trailing_comma: TrailingComma,
}

#[derive(Debug, Default)]
pub struct ImportBlock<'a> {
    // Set of (name, asname), used to track regular imports.
    // Ex) `import module`
    pub import: FxHashMap<AliasData<'a>, ImportCommentSet<'a>>,
    // Map from (module, level) to `AliasData`, used to track 'from' imports.
    // Ex) `from module import member`
    pub import_from: FxHashMap<ImportFromData<'a>, ImportFromStatement<'a>>,
    // Set of (module, level, name, asname), used to track re-exported 'from' imports.
    // Ex) `from module import member as member`
    pub import_from_as:
        FxHashMap<(ImportFromData<'a>, AliasData<'a>), ImportFromStatement<'a>>,
    // Map from (module, level) to `AliasData`, used to track star imports.
    // Ex) `from module import *`
    pub import_from_star: FxHashMap<ImportFromData<'a>, ImportFromStatement<'a>>,
}

type Import<'a> = (AliasData<'a>, ImportCommentSet<'a>);

type ImportFrom<'a> = (
    ImportFromData<'a>,
    ImportFromCommentSet<'a>,
    TrailingComma,
    Vec<(AliasData<'a>, ImportFromCommentSet<'a>)>,
);

#[derive(Debug)]
pub enum EitherImport<'a> {
    Import(Import<'a>),
    ImportFrom(ImportFrom<'a>),
}
