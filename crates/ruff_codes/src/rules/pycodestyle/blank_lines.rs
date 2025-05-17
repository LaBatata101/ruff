use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

// FIX: this is duplicated with ruff_rule_pycodestyle::blank_lines
/// Number of blank lines around top level classes and functions.
const BLANK_LINES_NESTED_LEVEL: u32 = 1;

/// ## What it does
/// Checks for missing blank lines between methods of a class.
///
/// ## Why is this bad?
/// PEP 8 recommends exactly one blank line between methods of a class.
///
/// ## Example
/// ```python
/// class MyClass(object):
///     def func1():
///         pass
///     def func2():
///         pass
/// ```
///
/// Use instead:
/// ```python
/// class MyClass(object):
///     def func1():
///         pass
///
///     def func2():
///         pass
/// ```
///
/// ## Typing stub files (`.pyi`)
/// The typing style guide recommends to not use blank lines between methods except to group
/// them. That's why this rule is not enabled in typing stub files.
///
/// ## References
/// - [PEP 8: Blank Lines](https://peps.python.org/pep-0008/#blank-lines)
/// - [Flake 8 rule](https://www.flake8rules.com/rules/E301.html)
/// - [Typing Style Guide](https://typing.python.org/en/latest/source/stubs.html#blank-lines)
#[derive(ViolationMetadata)]
pub struct BlankLineBetweenMethods;

impl AlwaysFixableViolation for BlankLineBetweenMethods {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Expected {BLANK_LINES_NESTED_LEVEL:?} blank line, found 0")
    }

    fn fix_title(&self) -> String {
        "Add missing blank line".to_string()
    }
}

/// ## What it does
/// Checks for missing blank lines between top level functions and classes.
///
/// ## Why is this bad?
/// PEP 8 recommends exactly two blank lines between top level functions and classes.
///
/// The rule respects the [`lint.isort.lines-after-imports`] setting when
/// determining the required number of blank lines between top-level `import`
/// statements and function or class definitions for compatibility with isort.
///
/// ## Example
/// ```python
/// def func1():
///     pass
/// def func2():
///     pass
/// ```
///
/// Use instead:
/// ```python
/// def func1():
///     pass
///
///
/// def func2():
///     pass
/// ```
///
/// ## Typing stub files (`.pyi`)
/// The typing style guide recommends to not use blank lines between classes and functions except to group
/// them. That's why this rule is not enabled in typing stub files.
///
/// ## Options
/// - `lint.isort.lines-after-imports`
///
/// ## References
/// - [PEP 8: Blank Lines](https://peps.python.org/pep-0008/#blank-lines)
/// - [Flake 8 rule](https://www.flake8rules.com/rules/E302.html)
/// - [Typing Style Guide](https://typing.python.org/en/latest/source/stubs.html#blank-lines)
#[derive(ViolationMetadata)]
pub struct BlankLinesTopLevel {
    actual_blank_lines: u32,
    expected_blank_lines: u32,
}

impl AlwaysFixableViolation for BlankLinesTopLevel {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BlankLinesTopLevel {
            actual_blank_lines,
            expected_blank_lines,
        } = self;

        format!("Expected {expected_blank_lines:?} blank lines, found {actual_blank_lines}")
    }

    fn fix_title(&self) -> String {
        "Add missing blank line(s)".to_string()
    }
}

/// ## What it does
/// Checks for extraneous blank lines.
///
/// ## Why is this bad?
/// PEP 8 recommends using blank lines as follows:
/// - No more than two blank lines between top-level statements.
/// - No more than one blank line between non-top-level statements.
///
/// ## Example
/// ```python
/// def func1():
///     pass
///
///
///
/// def func2():
///     pass
/// ```
///
/// Use instead:
/// ```python
/// def func1():
///     pass
///
///
/// def func2():
///     pass
/// ```
///
/// ## Typing stub files (`.pyi`)
/// The rule allows at most one blank line in typing stub files in accordance to the typing style guide recommendation.
///
/// Note: The rule respects the following `isort` settings when determining the maximum number of blank lines allowed between two statements:
///
/// * [`lint.isort.lines-after-imports`]: For top-level statements directly following an import statement.
/// * [`lint.isort.lines-between-types`]: For `import` statements directly following a `from ... import ...` statement or vice versa.
///
/// ## Options
/// - `lint.isort.lines-after-imports`
/// - `lint.isort.lines-between-types`
///
/// ## References
/// - [PEP 8: Blank Lines](https://peps.python.org/pep-0008/#blank-lines)
/// - [Flake 8 rule](https://www.flake8rules.com/rules/E303.html)
/// - [Typing Style Guide](https://typing.python.org/en/latest/source/stubs.html#blank-lines)
#[derive(ViolationMetadata)]
pub struct TooManyBlankLines {
    actual_blank_lines: u32,
}

impl AlwaysFixableViolation for TooManyBlankLines {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyBlankLines { actual_blank_lines } = self;

        format!("Too many blank lines ({actual_blank_lines})")
    }

    fn fix_title(&self) -> String {
        "Remove extraneous blank line(s)".to_string()
    }
}

/// ## What it does
/// Checks for extraneous blank line(s) after function decorators.
///
/// ## Why is this bad?
/// There should be no blank lines between a decorator and the object it is decorating.
///
/// ## Example
/// ```python
/// class User(object):
///
///     @property
///
///     def name(self):
///         pass
/// ```
///
/// Use instead:
/// ```python
/// class User(object):
///
///     @property
///     def name(self):
///         pass
/// ```
///
/// ## References
/// - [PEP 8: Blank Lines](https://peps.python.org/pep-0008/#blank-lines)
/// - [Flake 8 rule](https://www.flake8rules.com/rules/E304.html)
#[derive(ViolationMetadata)]
pub struct BlankLineAfterDecorator {
    actual_blank_lines: u32,
}

impl AlwaysFixableViolation for BlankLineAfterDecorator {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Blank lines found after function decorator ({lines})",
            lines = self.actual_blank_lines
        )
    }

    fn fix_title(&self) -> String {
        "Remove extraneous blank line(s)".to_string()
    }
}

/// ## What it does
/// Checks for missing blank lines after the end of function or class.
///
/// ## Why is this bad?
/// PEP 8 recommends using blank lines as follows:
/// - Two blank lines are expected between functions and classes
/// - One blank line is expected between methods of a class.
///
/// ## Example
/// ```python
/// class User(object):
///     pass
/// user = User()
/// ```
///
/// Use instead:
/// ```python
/// class User(object):
///     pass
///
///
/// user = User()
/// ```
///
/// ## Typing stub files (`.pyi`)
/// The typing style guide recommends to not use blank lines between statements except to group
/// them. That's why this rule is not enabled in typing stub files.
///
/// ## References
/// - [PEP 8: Blank Lines](https://peps.python.org/pep-0008/#blank-lines)
/// - [Flake 8 rule](https://www.flake8rules.com/rules/E305.html)
/// - [Typing Style Guide](https://typing.python.org/en/latest/source/stubs.html#blank-lines)
#[derive(ViolationMetadata)]
pub struct BlankLinesAfterFunctionOrClass {
    actual_blank_lines: u32,
}

impl AlwaysFixableViolation for BlankLinesAfterFunctionOrClass {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BlankLinesAfterFunctionOrClass {
            actual_blank_lines: blank_lines,
        } = self;
        format!("Expected 2 blank lines after class or function definition, found ({blank_lines})")
    }

    fn fix_title(&self) -> String {
        "Add missing blank line(s)".to_string()
    }
}

/// ## What it does
/// Checks for 1 blank line between nested function or class definitions.
///
/// ## Why is this bad?
/// PEP 8 recommends using blank lines as follows:
/// - Two blank lines are expected between functions and classes
/// - One blank line is expected between methods of a class.
///
/// ## Example
/// ```python
/// def outer():
///     def inner():
///         pass
///     def inner2():
///         pass
/// ```
///
/// Use instead:
/// ```python
/// def outer():
///     def inner():
///         pass
///
///     def inner2():
///         pass
/// ```
///
/// ## Typing stub files (`.pyi`)
/// The typing style guide recommends to not use blank lines between classes and functions except to group
/// them. That's why this rule is not enabled in typing stub files.
///
/// ## References
/// - [PEP 8: Blank Lines](https://peps.python.org/pep-0008/#blank-lines)
/// - [Flake 8 rule](https://www.flake8rules.com/rules/E306.html)
/// - [Typing Style Guide](https://typing.python.org/en/latest/source/stubs.html#blank-lines)
#[derive(ViolationMetadata)]
pub struct BlankLinesBeforeNestedDefinition;

impl AlwaysFixableViolation for BlankLinesBeforeNestedDefinition {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Expected 1 blank line before a nested definition, found 0".to_string()
    }

    fn fix_title(&self) -> String {
        "Add missing blank line".to_string()
    }
}
