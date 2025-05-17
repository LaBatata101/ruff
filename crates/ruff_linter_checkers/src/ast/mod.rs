//! [`Checker`] for AST-based lint rules.
//!
//! The [`Checker`] is responsible for traversing over the AST, building up the [`SemanticModel`],
//! and running any enabled [`Rule`]s at the appropriate place and time.
//!
//! The [`Checker`] is structured as a single pass over the AST that proceeds in "evaluation" order.
//! That is: the [`Checker`] typically iterates over nodes in the order in which they're evaluated
//! by the Python interpreter. This includes, e.g., deferring function body traversal until after
//! parent scopes have been fully traversed. Individual rules may also perform internal traversals
//! of the AST.
//!
//! The individual [`Visitor`] implementations within the [`Checker`] typically proceed in four
//! steps:
//!
//! 1. Binding: Bind any names introduced by the current node.
//! 2. Traversal: Recurse into the children of the current node.
//! 3. Clean-up: Perform any necessary clean-up after the current node has been fully traversed.
//! 4. Analysis: Run any relevant lint rules on the current node.
//!
//! The first three steps together compose the semantic analysis phase, while the last step
//! represents the lint-rule analysis phase. In the future, these steps may be separated into
//! distinct passes over the AST.

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use ruff_python_parser::semantic_errors::SemanticSyntaxError;
use rustc_hash::{FxHashMap, FxHashSet};

use ruff_diagnostics::{Diagnostic, Edit, IsolationLevel};
use ruff_notebook::{CellOffsets, NotebookIndex};
use ruff_python_ast::str::Quote;
use ruff_python_ast::PySourceType;
use ruff_python_ast::{self as ast, ModModule, PythonVersion};
use ruff_python_codegen::{Generator, Stylist};
use ruff_python_index::Indexer;
use ruff_python_parser::typing::{parse_type_annotation, ParsedAnnotation};
use ruff_python_parser::{ParseError, Parsed, Tokens};
use ruff_python_semantic::{
    Module, NodeId, SemanticModel, SemanticModelFlags,
};
use ruff_python_trivia::CommentRanges;
use ruff_source_file::{OneIndexed, SourceRow};
use ruff_text_size::{Ranged, TextRange, TextSize};

use ruff_codes::Rule;
use ruff_linter_commons::{package::PackageRoot, Locator};
use ruff_linter_fix::importer::{ImportRequest, Importer, ResolutionError};
use ruff_linter_noqa::{self as noqa, NoqaMapping};
use ruff_linter_settings::{flags, LinterSettings, TargetVersion};

pub mod deferred;

/// State representing whether a docstring is expected or not for the next statement.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DocstringState {
    /// The next statement is expected to be a docstring, but not necessarily so.
    ///
    /// For example, in the following code:
    ///
    /// ```python
    /// class Foo:
    ///     pass
    ///
    ///
    /// def bar(x, y):
    ///     """Docstring."""
    ///     return x +  y
    /// ```
    ///
    /// For `Foo`, the state is expected when the checker is visiting the class
    /// body but isn't going to be present. While, for `bar` function, the docstring
    /// is expected and present.
    Expected(ExpectedDocstringKind),
    Other,
}

impl Default for DocstringState {
    /// Returns the default docstring state which is to expect a module-level docstring.
    fn default() -> Self {
        Self::Expected(ExpectedDocstringKind::Module)
    }
}

impl DocstringState {
    /// Returns the docstring kind if the state is expecting a docstring.
    pub const fn expected_kind(self) -> Option<ExpectedDocstringKind> {
        match self {
            DocstringState::Expected(kind) => Some(kind),
            DocstringState::Other => None,
        }
    }
}

/// The kind of an expected docstring.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExpectedDocstringKind {
    /// A module-level docstring.
    ///
    /// For example,
    /// ```python
    /// """This is a module-level docstring."""
    ///
    /// a = 1
    /// ```
    Module,

    /// A class-level docstring.
    ///
    /// For example,
    /// ```python
    /// class Foo:
    ///     """This is the docstring for `Foo` class."""
    ///
    ///     def __init__(self) -> None:
    ///         ...
    /// ```
    Class,

    /// A function-level docstring.
    ///
    /// For example,
    /// ```python
    /// def foo():
    ///     """This is the docstring for `foo` function."""
    ///     pass
    /// ```
    Function,

    /// An attribute-level docstring.
    ///
    /// For example,
    /// ```python
    /// a = 1
    /// """This is the docstring for `a` variable."""
    ///
    ///
    /// class Foo:
    ///     b = 1
    ///     """This is the docstring for `Foo.b` class variable."""
    /// ```
    Attribute,
}

impl ExpectedDocstringKind {
    /// Returns the semantic model flag that represents the current docstring state.
    pub const fn as_flag(self) -> SemanticModelFlags {
        match self {
            ExpectedDocstringKind::Attribute => SemanticModelFlags::ATTRIBUTE_DOCSTRING,
            _ => SemanticModelFlags::PEP_257_DOCSTRING,
        }
    }
}

pub struct CheckerSnapshot<'a> {
    /// The [`Parsed`] output for the source code.
    parsed: &'a Parsed<ModModule>,
    /// An internal cache for parsed string annotations
    parsed_annotations_cache: Rc<RefCell<ParsedAnnotationsCache<'a>>>,
    /// The [`Parsed`] output for the type annotation the checker is currently in.
    parsed_type_annotation: Option<&'a ParsedAnnotation>,
    /// The [`Path`] to the file under analysis.
    path: &'a Path,
    /// The [`Path`] to the package containing the current file.
    package: Option<PackageRoot<'a>>,
    /// The module representation of the current file (e.g., `foo.bar`).
    pub module: Module<'a>,
    /// The [`PySourceType`] of the current file.
    pub source_type: PySourceType,
    /// The [`CellOffsets`] for the current file, if it's a Jupyter notebook.
    cell_offsets: Option<&'a CellOffsets>,
    /// The [`NotebookIndex`] for the current file, if it's a Jupyter notebook.
    notebook_index: Option<&'a NotebookIndex>,
    /// The [`flags::Noqa`] for the current analysis (i.e., whether to respect suppression
    /// comments).
    noqa: flags::Noqa,
    /// The [`NoqaMapping`] for the current analysis (i.e., the mapping from line number to
    /// suppression commented line number).
    noqa_line_for: &'a NoqaMapping,
    /// The [`LinterSettings`] for the current analysis, including the enabled rules.
    pub settings: &'a LinterSettings,
    /// The [`Locator`] for the current file, which enables extraction of source code from byte
    /// offsets.
    locator: &'a Locator<'a>,
    /// The [`Stylist`] for the current file, which detects the current line ending, quote, and
    /// indentation style.
    stylist: &'a Stylist<'a>,
    /// The [`Indexer`] for the current file, which contains the offsets of all comments and more.
    indexer: &'a Indexer,
    /// The [`Importer`] for the current file, which enables importing of other modules.
    importer: Rc<RefCell<Importer<'a>>>,
    /// The [`SemanticModel`], built up over the course of the AST traversal.
    semantic: Rc<RefCell<SemanticModel<'a>>>,
    /// The cumulative set of diagnostics computed across all lint rules.
    diagnostics: Rc<RefCell<Vec<Diagnostic>>>,
    /// The list of names already seen by flake8-bugbear diagnostics, to avoid duplicate violations.
    flake8_bugbear_seen: Rc<RefCell<FxHashSet<TextRange>>>,
    /// The target [`PythonVersion`] for version-dependent checks.
    target_version: TargetVersion,
    /// Errors collected by the `semantic_checker`.
    semantic_errors: Rc<RefCell<Vec<SemanticSyntaxError>>>,
}

impl<'a> CheckerSnapshot<'a> {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        parsed: &'a Parsed<ModModule>,
        settings: &'a LinterSettings,
        noqa_line_for: &'a NoqaMapping,
        noqa: flags::Noqa,
        path: &'a Path,
        package: Option<PackageRoot<'a>>,
        module: Module<'a>,
        locator: &'a Locator,
        stylist: &'a Stylist,
        indexer: &'a Indexer,
        source_type: PySourceType,
        cell_offsets: Option<&'a CellOffsets>,
        notebook_index: Option<&'a NotebookIndex>,
        target_version: TargetVersion,
        importer: Rc<RefCell<Importer<'a>>>,
        semantic: Rc<RefCell<SemanticModel<'a>>>,
        diagnostics: Rc<RefCell<Vec<Diagnostic>>>,
        semantic_errors: Rc<RefCell<Vec<SemanticSyntaxError>>>,
        flake8_bugbear_seen: Rc<RefCell<FxHashSet<TextRange>>>,
        parsed_annotations_cache: Rc<RefCell<ParsedAnnotationsCache<'a>>>,
        parsed_type_annotation: Option<&'a ParsedAnnotation>,
    ) -> CheckerSnapshot<'a> {
        Self {
            parsed,
            parsed_type_annotation,
            parsed_annotations_cache,
            settings,
            noqa_line_for,
            noqa,
            path,
            package,
            module,
            source_type,
            locator,
            stylist,
            indexer,
            importer,
            semantic,
            diagnostics,
            flake8_bugbear_seen,
            cell_offsets,
            notebook_index,
            semantic_errors,
            target_version,
        }
    }

    /// Return `true` if a [`Rule`] is disabled by a `noqa` directive.
    pub fn rule_is_ignored(&self, code: Rule, offset: TextSize) -> bool {
        // TODO(charlie): `noqa` directives are mostly enforced in `check_lines.rs`.
        // However, in rare cases, we need to check them here. For example, when
        // removing unused imports, we create a single fix that's applied to all
        // unused members on a single import. We need to preemptively omit any
        // members from the fix that will eventually be excluded by a `noqa`.
        // Unfortunately, we _do_ want to register a `Diagnostic` for each
        // eventually-ignored import, so that our `noqa` counts are accurate.
        if !self.noqa.is_enabled() {
            return false;
        }

        noqa::rule_is_ignored(
            code,
            offset,
            self.noqa_line_for,
            self.comment_ranges(),
            self.locator,
        )
    }

    /// Create a [`Generator`] to generate source code based on the current AST state.
    pub fn generator(&self) -> Generator {
        Generator::new(self.stylist.indentation(), self.stylist.line_ending())
    }

    /// Return the preferred quote for a generated `StringLiteral` node, given where we are in the
    /// AST.
    fn preferred_quote(&self) -> Quote {
        self.f_string_quote_style().unwrap_or(self.stylist.quote())
    }

    /// Return the default string flags a generated `StringLiteral` node should use, given where we
    /// are in the AST.
    pub fn default_string_flags(&self) -> ast::StringLiteralFlags {
        ast::StringLiteralFlags::empty().with_quote_style(self.preferred_quote())
    }

    /// Return the default bytestring flags a generated `ByteStringLiteral` node should use, given
    /// where we are in the AST.
    pub fn default_bytes_flags(&self) -> ast::BytesLiteralFlags {
        ast::BytesLiteralFlags::empty().with_quote_style(self.preferred_quote())
    }

    /// Return the default f-string flags a generated `FString` node should use, given where we are
    /// in the AST.
    pub fn default_fstring_flags(&self) -> ast::FStringFlags {
        ast::FStringFlags::empty().with_quote_style(self.preferred_quote())
    }

    /// Returns the appropriate quoting for f-string by reversing the one used outside of
    /// the f-string.
    ///
    /// If the current expression in the context is not an f-string, returns ``None``.
    pub fn f_string_quote_style(&self) -> Option<Quote> {
        if !self.semantic().in_f_string() {
            return None;
        }

        // Find the quote character used to start the containing f-string.
        let ast::ExprFString { value, .. } = self
            .semantic()
            .current_expressions()
            .find_map(|expr| expr.as_f_string_expr())?;
        Some(value.iter().next()?.quote_style().opposite())
    }

    /// Returns the [`SourceRow`] for the given offset.
    pub fn compute_source_row(&self, offset: TextSize) -> SourceRow {
        #[expect(deprecated)]
        let line = self.locator.compute_line_index(offset);

        if let Some(notebook_index) = self.notebook_index {
            let cell = notebook_index.cell(line).unwrap_or(OneIndexed::MIN);
            let line = notebook_index.cell_row(line).unwrap_or(OneIndexed::MIN);
            SourceRow::Notebook { cell, line }
        } else {
            SourceRow::SourceFile { line }
        }
    }

    /// Returns the [`CommentRanges`] for the parsed source code.
    pub fn comment_ranges(&self) -> &'a CommentRanges {
        self.indexer.comment_ranges()
    }

    /// Push a new [`Diagnostic`] to the collection in the [`Checker`]
    pub fn report_diagnostic(&self, diagnostic: Diagnostic) {
        let mut diagnostics = self.diagnostics.borrow_mut();
        diagnostics.push(diagnostic);
    }

    /// Extend the collection of [`Diagnostic`] objects in the [`Checker`]
    pub fn report_diagnostics<I>(&self, diagnostics: I)
    where
        I: IntoIterator<Item = Diagnostic>,
    {
        let mut checker_diagnostics = self.diagnostics.borrow_mut();
        checker_diagnostics.extend(diagnostics);
    }

    /// Adds a [`TextRange`] to the set of ranges of variable names
    /// flagged in `flake8-bugbear` violations so far.
    ///
    /// Returns whether the value was newly inserted.
    pub fn insert_flake8_bugbear_range(&self, range: TextRange) -> bool {
        let mut ranges = self.flake8_bugbear_seen.borrow_mut();
        ranges.insert(range)
    }

    /// Returns the [`Tokens`] for the parsed type annotation if the checker is in a typing context
    /// or the parsed source code.
    pub fn tokens(&self) -> &'a Tokens {
        if let Some(type_annotation) = self.parsed_type_annotation {
            type_annotation.parsed().tokens()
        } else {
            self.parsed.tokens()
        }
    }

    /// The [`Locator`] for the current file, which enables extraction of source code from byte
    /// offsets.
    pub const fn locator(&self) -> &'a Locator<'a> {
        self.locator
    }

    pub const fn source(&self) -> &'a str {
        self.locator.contents()
    }

    /// The [`Stylist`] for the current file, which detects the current line ending, quote, and
    /// indentation style.
    pub const fn stylist(&self) -> &'a Stylist<'a> {
        self.stylist
    }

    /// The [`Indexer`] for the current file, which contains the offsets of all comments and more.
    pub const fn indexer(&self) -> &'a Indexer {
        self.indexer
    }

    /// The [`Importer`] for the current file, which enables importing of other modules.
    pub fn importer(&self) -> std::cell::Ref<'_, Importer<'a>> {
        self.importer.borrow()
    }

    /// The [`SemanticModel`], built up over the course of the AST traversal.
    pub fn semantic(&self) -> std::cell::Ref<'_, SemanticModel<'a>> {
        self.semantic.borrow()
    }

    /// The [`Path`] to the file under analysis.
    pub const fn path(&self) -> &'a Path {
        self.path
    }

    /// The [`Path`] to the package containing the current file.
    pub const fn package(&self) -> Option<PackageRoot<'_>> {
        self.package
    }

    /// The [`CellOffsets`] for the current file, if it's a Jupyter notebook.
    pub const fn cell_offsets(&self) -> Option<&'a CellOffsets> {
        self.cell_offsets
    }

    /// Returns whether the given rule should be checked.
    #[inline]
    pub const fn enabled(&self, rule: Rule) -> bool {
        self.settings.rules.enabled(rule)
    }

    /// Returns whether any of the given rules should be checked.
    #[inline]
    pub const fn any_enabled(&self, rules: &[Rule]) -> bool {
        self.settings.rules.any_enabled(rules)
    }

    /// Returns the [`IsolationLevel`] to isolate fixes for a given node.
    ///
    /// The primary use-case for fix isolation is to ensure that we don't delete all statements
    /// in a given indented block, which would cause a syntax error. We therefore need to ensure
    /// that we delete at most one statement per indented block per fixer pass. Fix isolation should
    /// thus be applied whenever we delete a statement, but can otherwise be omitted.
    pub fn isolation(node_id: Option<NodeId>) -> IsolationLevel {
        node_id
            .map(|node_id| IsolationLevel::Group(node_id.into()))
            .unwrap_or_default()
    }

    /// Parse a stringified type annotation as an AST expression,
    /// e.g. `"List[str]"` in `x: "List[str]"`
    ///
    /// This method is a wrapper around [`ruff_python_parser::typing::parse_type_annotation`]
    /// that adds caching.
    pub fn parse_type_annotation(
        &self,
        annotation: &ast::ExprStringLiteral,
    ) -> Result<&'a ParsedAnnotation, &'a ParseError> {
        self.parsed_annotations_cache.borrow()
            .lookup_or_parse(annotation, self.locator.contents())
    }

    /// Apply a test to an annotation expression,
    /// abstracting over the fact that the annotation expression might be "stringized".
    ///
    /// A stringized annotation is one enclosed in string quotes:
    /// `foo: "typing.Any"` means the same thing to a type checker as `foo: typing.Any`.
    pub fn match_maybe_stringized_annotation(
        &self,
        expr: &ast::Expr,
        match_fn: impl FnOnce(&ast::Expr) -> bool,
    ) -> bool {
        if let ast::Expr::StringLiteral(string_annotation) = expr {
            let Some(parsed_annotation) = self.parse_type_annotation(string_annotation).ok() else {
                return false;
            };
            match_fn(parsed_annotation.expression())
        } else {
            match_fn(expr)
        }
    }

    /// Push `diagnostic` if the checker is not in a `@no_type_check` context.
    pub fn report_type_diagnostic(&self, diagnostic: Diagnostic) {
        if !self.semantic().in_no_type_check() {
            self.report_diagnostic(diagnostic);
        }
    }

    /// Return the [`PythonVersion`] to use for version-related lint rules.
    ///
    /// If the user did not provide a target version, this defaults to the lowest supported Python
    /// version ([`PythonVersion::default`]).
    ///
    /// Note that this method should not be used for version-related syntax errors emitted by the
    /// parser or the [`SemanticSyntaxChecker`], which should instead default to the _latest_
    /// supported Python version.
    pub fn target_version(&self) -> PythonVersion {
        self.target_version.linter_version()
    }

    /// Create a [`TypingImporter`] that will import `member` from either `typing` or
    /// `typing_extensions`.
    ///
    /// On Python <`version_added_to_typing`, `member` is imported from `typing_extensions`, while
    /// on Python >=`version_added_to_typing`, it is imported from `typing`.
    ///
    /// If the Python version is less than `version_added_to_typing` but
    /// `LinterSettings::typing_extensions` is `false`, this method returns `None`.
    pub fn typing_importer<'b>(
        &'b self,
        member: &'b str,
        version_added_to_typing: PythonVersion,
    ) -> Option<TypingImporter<'b, 'a>> {
        let source_module = if self.target_version() >= version_added_to_typing {
            "typing"
        } else if !self.settings.typing_extensions {
            return None;
        } else {
            "typing_extensions"
        };
        Some(TypingImporter {
            checker: self,
            source_module,
            member,
        })
    }
}

pub struct TypingImporter<'a, 'b> {
    checker: &'a CheckerSnapshot<'b>,
    source_module: &'static str,
    member: &'a str,
}

impl TypingImporter<'_, '_> {
    /// Create an [`Edit`] that makes the requested symbol available at `position`.
    ///
    /// See [`Importer::get_or_import_symbol`] for more details on the returned values and
    /// [`Checker::typing_importer`] for a way to construct a [`TypingImporter`].
    pub fn import(&self, position: TextSize) -> Result<(Edit, String), ResolutionError> {
        let request = ImportRequest::import_from(self.source_module, self.member);
        self.checker
            .importer
            .borrow()
            .get_or_import_symbol(&request, position, &self.checker.semantic())
    }
}

pub struct ParsedAnnotationsCache<'a> {
    arena: &'a typed_arena::Arena<Result<ParsedAnnotation, ParseError>>,
    by_offset: RefCell<FxHashMap<TextSize, Result<&'a ParsedAnnotation, &'a ParseError>>>,
}

impl<'a> ParsedAnnotationsCache<'a> {
    pub fn new(arena: &'a typed_arena::Arena<Result<ParsedAnnotation, ParseError>>) -> Self {
        Self {
            arena,
            by_offset: RefCell::default(),
        }
    }

    pub fn lookup_or_parse(
        &self,
        annotation: &ast::ExprStringLiteral,
        source: &str,
    ) -> Result<&'a ParsedAnnotation, &'a ParseError> {
        *self
            .by_offset
            .borrow_mut()
            .entry(annotation.start())
            .or_insert_with(|| {
                self.arena
                    .alloc(parse_type_annotation(annotation, source))
                    .as_ref()
            })
    }

    pub fn clear(&self) {
        self.by_offset.borrow_mut().clear();
    }
}
