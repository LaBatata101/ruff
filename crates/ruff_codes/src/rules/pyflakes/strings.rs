use ruff_diagnostics::{AlwaysFixableViolation, FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::name::Name;

/// ## What it does
/// Checks for invalid `printf`-style format strings.
///
/// ## Why is this bad?
/// Conversion specifiers are required for `printf`-style format strings. These
/// specifiers must contain a `%` character followed by a conversion type.
///
/// ## Example
/// ```python
/// "Hello, %" % "world"
/// ```
///
/// Use instead:
/// ```python
/// "Hello, %s" % "world"
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatInvalidFormat {
    pub(crate) message: String,
}

impl Violation for PercentFormatInvalidFormat {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PercentFormatInvalidFormat { message } = self;
        format!("`%`-format string has invalid format string: {message}")
    }
}

/// ## What it does
/// Checks for named placeholders in `printf`-style format strings without
/// mapping-type values.
///
/// ## Why is this bad?
/// When using named placeholders in `printf`-style format strings, the values
/// must be a map type (such as a dictionary). Otherwise, the expression will
/// raise a `TypeError`.
///
/// ## Example
/// ```python
/// "%(greeting)s, %(name)s" % ("Hello", "World")
/// ```
///
/// Use instead:
/// ```python
/// "%(greeting)s, %(name)s" % {"greeting": "Hello", "name": "World"}
/// ```
///
/// Or:
/// ```python
/// "%s, %s" % ("Hello", "World")
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatExpectedMapping;

impl Violation for PercentFormatExpectedMapping {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`%`-format string expected mapping but got sequence".to_string()
    }
}

/// ## What it does
/// Checks for uses of mapping-type values in `printf`-style format strings
/// without named placeholders.
///
/// ## Why is this bad?
/// When using mapping-type values (such as `dict`) in `printf`-style format
/// strings, the keys must be named. Otherwise, the expression will raise a
/// `TypeError`.
///
/// ## Example
/// ```python
/// "%s, %s" % {"greeting": "Hello", "name": "World"}
/// ```
///
/// Use instead:
/// ```python
/// "%(greeting)s, %(name)s" % {"greeting": "Hello", "name": "World"}
/// ```
///
/// Or:
/// ```python
/// "%s, %s" % ("Hello", "World")
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatExpectedSequence;

impl Violation for PercentFormatExpectedSequence {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`%`-format string expected sequence but got mapping".to_string()
    }
}

/// ## What it does
/// Checks for unused mapping keys in `printf`-style format strings.
///
/// ## Why is this bad?
/// Unused named placeholders in `printf`-style format strings are unnecessary,
/// and likely indicative of a mistake. They should be removed.
///
/// ## Example
/// ```python
/// "Hello, %(name)s" % {"greeting": "Hello", "name": "World"}
/// ```
///
/// Use instead:
/// ```python
/// "Hello, %(name)s" % {"name": "World"}
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatExtraNamedArguments {
    missing: Vec<String>,
}

impl AlwaysFixableViolation for PercentFormatExtraNamedArguments {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PercentFormatExtraNamedArguments { missing } = self;
        let message = missing.join(", ");
        format!("`%`-format string has unused named argument(s): {message}")
    }

    fn fix_title(&self) -> String {
        let PercentFormatExtraNamedArguments { missing } = self;
        let message = missing.join(", ");
        format!("Remove extra named arguments: {message}")
    }
}

/// ## What it does
/// Checks for named placeholders in `printf`-style format strings that are not
/// present in the provided mapping.
///
/// ## Why is this bad?
/// Named placeholders that lack a corresponding value in the provided mapping
/// will raise a `KeyError`.
///
/// ## Example
/// ```python
/// "%(greeting)s, %(name)s" % {"name": "world"}
/// ```
///
/// Use instead:
/// ```python
/// "Hello, %(name)s" % {"name": "world"}
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatMissingArgument {
    missing: Vec<String>,
}

impl Violation for PercentFormatMissingArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PercentFormatMissingArgument { missing } = self;
        let message = missing.join(", ");
        format!("`%`-format string is missing argument(s) for placeholder(s): {message}")
    }
}

/// ## What it does
/// Checks for `printf`-style format strings that have mixed positional and
/// named placeholders.
///
/// ## Why is this bad?
/// Python does not support mixing positional and named placeholders in
/// `printf`-style format strings. The use of mixed placeholders will raise a
/// `TypeError` at runtime.
///
/// ## Example
/// ```python
/// "%s, %(name)s" % ("Hello", {"name": "World"})
/// ```
///
/// Use instead:
/// ```python
/// "%s, %s" % ("Hello", "World")
/// ```
///
/// Or:
/// ```python
/// "%(greeting)s, %(name)s" % {"greeting": "Hello", "name": "World"}
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatMixedPositionalAndNamed;

impl Violation for PercentFormatMixedPositionalAndNamed {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`%`-format string has mixed positional and named placeholders".to_string()
    }
}

/// ## What it does
/// Checks for `printf`-style format strings that have a mismatch between the
/// number of positional placeholders and the number of substitution values.
///
/// ## Why is this bad?
/// When a `printf`-style format string is provided with too many or too few
/// substitution values, it will raise a `TypeError` at runtime.
///
/// ## Example
/// ```python
/// "%s, %s" % ("Hello", "world", "!")
/// ```
///
/// Use instead:
/// ```python
/// "%s, %s" % ("Hello", "world")
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatPositionalCountMismatch {
    wanted: usize,
    got: usize,
}

impl Violation for PercentFormatPositionalCountMismatch {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PercentFormatPositionalCountMismatch { wanted, got } = self;
        format!("`%`-format string has {wanted} placeholder(s) but {got} substitution(s)")
    }
}

/// ## What it does
/// Checks for `printf`-style format strings that use the `*` specifier with
/// non-tuple values.
///
/// ## Why is this bad?
/// The use of the `*` specifier with non-tuple values will raise a
/// `TypeError` at runtime.
///
/// ## Example
/// ```python
/// from math import pi
///
/// "%(n).*f" % {"n": (2, pi)}
/// ```
///
/// Use instead:
/// ```python
/// from math import pi
///
/// "%.*f" % (2, pi)  # 3.14
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatStarRequiresSequence;

impl Violation for PercentFormatStarRequiresSequence {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`%`-format string `*` specifier requires sequence".to_string()
    }
}

/// ## What it does
/// Checks for `printf`-style format strings with invalid format characters.
///
/// ## Why is this bad?
/// In `printf`-style format strings, the `%` character is used to indicate
/// placeholders. If a `%` character is not followed by a valid format
/// character, it will raise a `ValueError` at runtime.
///
/// ## Example
/// ```python
/// "Hello, %S" % "world"
/// ```
///
/// Use instead:
/// ```python
/// "Hello, %s" % "world"
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#printf-style-string-formatting)
#[derive(ViolationMetadata)]
pub struct PercentFormatUnsupportedFormatCharacter {
    pub(crate) char: char,
}

impl Violation for PercentFormatUnsupportedFormatCharacter {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PercentFormatUnsupportedFormatCharacter { char } = self;
        format!("`%`-format string has unsupported format character `{char}`")
    }
}

/// ## What it does
/// Checks for `str.format` calls with invalid format strings.
///
/// ## Why is this bad?
/// Invalid format strings will raise a `ValueError`.
///
/// ## Example
/// ```python
/// "{".format(foo)
/// ```
///
/// Use instead:
/// ```python
/// "{}".format(foo)
/// ```
///
/// ## References
/// - [Python documentation: `str.format`](https://docs.python.org/3/library/stdtypes.html#str.format)
#[derive(ViolationMetadata)]
pub struct StringDotFormatInvalidFormat {
    pub(crate) message: String,
}

impl Violation for StringDotFormatInvalidFormat {
    #[derive_message_formats]
    fn message(&self) -> String {
        let StringDotFormatInvalidFormat { message } = self;
        format!("`.format` call has invalid format string: {message}")
    }
}

/// ## What it does
/// Checks for `str.format` calls with unused keyword arguments.
///
/// ## Why is this bad?
/// Unused keyword arguments are redundant, and often indicative of a mistake.
/// They should be removed.
///
/// ## Example
/// ```python
/// "Hello, {name}".format(greeting="Hello", name="World")
/// ```
///
/// Use instead:
/// ```python
/// "Hello, {name}".format(name="World")
/// ```
///
/// ## References
/// - [Python documentation: `str.format`](https://docs.python.org/3/library/stdtypes.html#str.format)
#[derive(ViolationMetadata)]
pub struct StringDotFormatExtraNamedArguments {
    missing: Vec<Name>,
}

impl Violation for StringDotFormatExtraNamedArguments {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let StringDotFormatExtraNamedArguments { missing } = self;
        let message = missing.join(", ");
        format!("`.format` call has unused named argument(s): {message}")
    }

    fn fix_title(&self) -> Option<String> {
        let StringDotFormatExtraNamedArguments { missing } = self;
        let message = missing.join(", ");
        Some(format!("Remove extra named arguments: {message}"))
    }
}

/// ## What it does
/// Checks for `str.format` calls with unused positional arguments.
///
/// ## Why is this bad?
/// Unused positional arguments are redundant, and often indicative of a mistake.
/// They should be removed.
///
/// ## Example
/// ```python
/// "Hello, {0}".format("world", "!")
/// ```
///
/// Use instead:
/// ```python
/// "Hello, {0}".format("world")
/// ```
///
/// ## References
/// - [Python documentation: `str.format`](https://docs.python.org/3/library/stdtypes.html#str.format)
#[derive(ViolationMetadata)]
pub struct StringDotFormatExtraPositionalArguments {
    missing: Vec<String>,
}

impl Violation for StringDotFormatExtraPositionalArguments {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let StringDotFormatExtraPositionalArguments { missing } = self;
        let message = missing.join(", ");
        format!("`.format` call has unused arguments at position(s): {message}")
    }

    fn fix_title(&self) -> Option<String> {
        let StringDotFormatExtraPositionalArguments { missing } = self;
        let message = missing.join(", ");
        Some(format!(
            "Remove extra positional arguments at position(s): {message}"
        ))
    }
}

/// ## What it does
/// Checks for `str.format` calls with placeholders that are missing arguments.
///
/// ## Why is this bad?
/// In `str.format` calls, omitting arguments for placeholders will raise a
/// `KeyError` at runtime.
///
/// ## Example
/// ```python
/// "{greeting}, {name}".format(name="World")
/// ```
///
/// Use instead:
/// ```python
/// "{greeting}, {name}".format(greeting="Hello", name="World")
/// ```
///
/// ## References
/// - [Python documentation: `str.format`](https://docs.python.org/3/library/stdtypes.html#str.format)
#[derive(ViolationMetadata)]
pub struct StringDotFormatMissingArguments {
    missing: Vec<String>,
}

impl Violation for StringDotFormatMissingArguments {
    #[derive_message_formats]
    fn message(&self) -> String {
        let StringDotFormatMissingArguments { missing } = self;
        let message = missing.join(", ");
        format!("`.format` call is missing argument(s) for placeholder(s): {message}")
    }
}

/// ## What it does
/// Checks for `str.format` calls that mix automatic and manual numbering.
///
/// ## Why is this bad?
/// In `str.format` calls, mixing automatic and manual numbering will raise a
/// `ValueError` at runtime.
///
/// ## Example
/// ```python
/// "{0}, {}".format("Hello", "World")
/// ```
///
/// Use instead:
/// ```python
/// "{0}, {1}".format("Hello", "World")
/// ```
///
/// Or:
/// ```python
/// "{}, {}".format("Hello", "World")
/// ```
///
/// ## References
/// - [Python documentation: `str.format`](https://docs.python.org/3/library/stdtypes.html#str.format)
#[derive(ViolationMetadata)]
pub struct StringDotFormatMixingAutomatic;

impl Violation for StringDotFormatMixingAutomatic {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`.format` string mixes automatic and manual numbering".to_string()
    }
}
