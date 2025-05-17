use itertools::Itertools;
use ruff_diagnostics::{AlwaysFixableViolation, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for over-indented sections in docstrings.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for docstrings with multiple
/// sections.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body. The convention is that all sections should use
/// consistent indentation. In each section, the header should match the
/// indentation of the docstring's opening quotes, and the body should be
/// indented one level further.
///
/// This rule is enabled when using the `numpy` and `google` conventions, and
/// disabled when using the `pep257` convention.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///         Args:
///             distance: Distance traveled.
///             time: Time spent traveling.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct OverindentedSection {
    name: String,
}

impl AlwaysFixableViolation for OverindentedSection {
    #[derive_message_formats]
    fn message(&self) -> String {
        let OverindentedSection { name } = self;
        format!("Section is over-indented (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let OverindentedSection { name } = self;
        format!("Remove over-indentation from \"{name}\"")
    }
}

/// ## What it does
/// Checks for over-indented section underlines in docstrings.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline numpy-style docstrings,
/// and helps prevent incorrect syntax in docstrings using reStructuredText.
///
/// Multiline numpy-style docstrings are typically composed of a summary line,
/// followed by a blank line, followed by a series of sections. Each section
/// has a section header and a section body, and there should be a series of
/// underline characters in the line following the header. The underline should
/// have the same indentation as the header.
///
/// This rule enforces a consistent style for multiline numpy-style docstrings
/// with sections. If your docstring uses reStructuredText, the rule also
/// helps protect against incorrect reStructuredText syntax, which would cause
/// errors if you tried to use a tool such as Sphinx to generate documentation
/// from the docstring.
///
/// This rule is enabled when using the `numpy` convention, and disabled when
/// using the `google` or `pep257` conventions.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///         ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///           -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///       ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
#[derive(ViolationMetadata)]
pub struct OverindentedSectionUnderline {
    name: String,
}

impl AlwaysFixableViolation for OverindentedSectionUnderline {
    #[derive_message_formats]
    fn message(&self) -> String {
        let OverindentedSectionUnderline { name } = self;
        format!("Section underline is over-indented (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let OverindentedSectionUnderline { name } = self;
        format!("Remove over-indentation from \"{name}\" underline")
    }
}

/// ## What it does
/// Checks for section headers in docstrings that do not begin with capital
/// letters.
///
/// ## Why is this bad?
/// For stylistic consistency, all section headers in a docstring should be
/// capitalized.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections. Each section typically has
/// a header and a body.
///
/// This rule is enabled when using the `numpy` and `google` conventions, and
/// disabled when using the `pep257` convention.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     args:
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     returns:
///         Speed as distance divided by time.
///
///     raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct NonCapitalizedSectionName {
    name: String,
}

impl AlwaysFixableViolation for NonCapitalizedSectionName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NonCapitalizedSectionName { name } = self;
        format!("Section name should be properly capitalized (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let NonCapitalizedSectionName { name } = self;
        format!("Capitalize \"{name}\"")
    }
}

/// ## What it does
/// Checks for section headers in docstrings that are followed by non-newline
/// characters.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline numpy-style docstrings.
///
/// Multiline numpy-style docstrings are typically composed of a summary line,
/// followed by a blank line, followed by a series of sections. Each section
/// has a section header and a section body. The section header should be
/// followed by a newline, rather than by some other character (like a colon).
///
/// This rule is enabled when using the `numpy` convention, and disabled
/// when using the `google` or `pep257` conventions.
///
/// ## Example
/// ```python
/// # The `Parameters`, `Returns` and `Raises` section headers are all followed
/// # by a colon in this function's docstring:
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters:
///     -----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns:
///     --------
///     float
///         Speed as distance divided by time.
///
///     Raises:
///     -------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
#[derive(ViolationMetadata)]
pub struct MissingNewLineAfterSectionName {
    name: String,
}

impl AlwaysFixableViolation for MissingNewLineAfterSectionName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MissingNewLineAfterSectionName { name } = self;
        format!("Section name should end with a newline (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let MissingNewLineAfterSectionName { name } = self;
        format!("Add newline after \"{name}\"")
    }
}

/// ## What it does
/// Checks for section headers in docstrings that are not followed by
/// underlines.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline numpy-style docstrings,
/// and helps prevent incorrect syntax in docstrings using reStructuredText.
///
/// Multiline numpy-style docstrings are typically composed of a summary line,
/// followed by a blank line, followed by a series of sections. Each section
/// has a section header and a section body, and the header should be followed
/// by a series of underline characters in the following line.
///
/// This rule enforces a consistent style for multiline numpy-style docstrings
/// with sections. If your docstring uses reStructuredText, the rule also
/// helps protect against incorrect reStructuredText syntax, which would cause
/// errors if you tried to use a tool such as Sphinx to generate documentation
/// from the docstring.
///
/// This rule is enabled when using the `numpy` convention, and disabled
/// when using the `google` or `pep257` conventions.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///
///     float
///         Speed as distance divided by time.
///
///     Raises
///
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
#[derive(ViolationMetadata)]
pub struct MissingDashedUnderlineAfterSection {
    name: String,
}

impl AlwaysFixableViolation for MissingDashedUnderlineAfterSection {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MissingDashedUnderlineAfterSection { name } = self;
        format!("Missing dashed underline after section (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let MissingDashedUnderlineAfterSection { name } = self;
        format!("Add dashed line under \"{name}\"")
    }
}

/// ## What it does
/// Checks for section underlines in docstrings that are not on the line
/// immediately following the section name.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline numpy-style docstrings,
/// and helps prevent incorrect syntax in docstrings using reStructuredText.
///
/// Multiline numpy-style docstrings are typically composed of a summary line,
/// followed by a blank line, followed by a series of sections. Each section
/// has a header and a body. There should be a series of underline characters
/// in the line immediately below the header.
///
/// This rule enforces a consistent style for multiline numpy-style docstrings
/// with sections. If your docstring uses reStructuredText, the rule also
/// helps protect against incorrect reStructuredText syntax, which would cause
/// errors if you tried to use a tool such as Sphinx to generate documentation
/// from the docstring.
///
/// This rule is enabled when using the `numpy` convention, and disabled
/// when using the `google` or `pep257` conventions.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
#[derive(ViolationMetadata)]
pub struct MissingSectionUnderlineAfterName {
    name: String,
}

impl AlwaysFixableViolation for MissingSectionUnderlineAfterName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MissingSectionUnderlineAfterName { name } = self;
        format!("Section underline should be in the line following the section's name (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let MissingSectionUnderlineAfterName { name } = self;
        format!("Add underline to \"{name}\"")
    }
}

/// ## What it does
/// Checks for section underlines in docstrings that do not match the length of
/// the corresponding section header.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline numpy-style docstrings,
/// and helps prevent incorrect syntax in docstrings using reStructuredText.
///
/// Multiline numpy-style docstrings are typically composed of a summary line,
/// followed by a blank line, followed by a series of sections. Each section
/// has a section header and a section body, and there should be a series of
/// underline characters in the line following the header. The length of the
/// underline should exactly match the length of the section header.
///
/// This rule enforces a consistent style for multiline numpy-style docstrings
/// with sections. If your docstring uses reStructuredText, the rule also
/// helps protect against incorrect reStructuredText syntax, which would cause
/// errors if you tried to use a tool such as Sphinx to generate documentation
/// from the docstring.
///
/// This rule is enabled when using the `numpy` convention, and disabled
/// when using the `google` or `pep257` conventions.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ---
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     ---
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ---
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
#[derive(ViolationMetadata)]
pub struct MismatchedSectionUnderlineLength {
    name: String,
}

impl AlwaysFixableViolation for MismatchedSectionUnderlineLength {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MismatchedSectionUnderlineLength { name } = self;
        format!("Section underline should match the length of its name (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let MismatchedSectionUnderlineLength { name } = self;
        format!("Adjust underline length to match \"{name}\"")
    }
}

/// ## What it does
/// Checks for docstring sections that are not separated by a single blank
/// line.
///
/// ## Why is this bad?
/// This rule enforces consistency in your docstrings, and helps ensure
/// compatibility with documentation tooling.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body. If a multiline numpy-style or Google-style docstring
/// consists of multiple sections, each section should be separated by a single
/// blank line.
///
/// This rule is enabled when using the `numpy` and `google` conventions, and
/// disabled when using the `pep257` convention.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Style Guide](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct NoBlankLineAfterSection {
    name: String,
}

impl AlwaysFixableViolation for NoBlankLineAfterSection {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NoBlankLineAfterSection { name } = self;
        format!("Missing blank line after section (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let NoBlankLineAfterSection { name } = self;
        format!("Add blank line after \"{name}\"")
    }
}

/// ## What it does
/// Checks for docstring sections that are not separated by a blank line.
///
/// ## Why is this bad?
/// This rule enforces consistency in numpy-style and Google-style docstrings,
/// and helps ensure compatibility with documentation tooling.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body. Sections should be separated by a single blank line.
///
/// This rule is enabled when using the `numpy` and `google` conventions, and
/// disabled when using the `pep257` convention.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
#[derive(ViolationMetadata)]
pub struct NoBlankLineBeforeSection {
    name: String,
}

impl AlwaysFixableViolation for NoBlankLineBeforeSection {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NoBlankLineBeforeSection { name } = self;
        format!("Missing blank line before section (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let NoBlankLineBeforeSection { name } = self;
        format!("Add blank line before \"{name}\"")
    }
}

/// ## What it does
/// Checks for missing blank lines after the last section of a multiline
/// docstring.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline docstrings.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, the rule is disabled when using the `google`,
/// `numpy`, and `pep257` conventions.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
#[derive(ViolationMetadata)]
pub struct MissingBlankLineAfterLastSection {
    name: String,
}

impl AlwaysFixableViolation for MissingBlankLineAfterLastSection {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MissingBlankLineAfterLastSection { name } = self;
        format!("Missing blank line after last section (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let MissingBlankLineAfterLastSection { name } = self;
        format!("Add blank line after \"{name}\"")
    }
}

/// ## What it does
/// Checks for docstrings with empty sections.
///
/// ## Why is this bad?
/// An empty section in a multiline docstring likely indicates an unfinished
/// or incomplete docstring.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body. Each section body should be non-empty; empty sections
/// should either have content added to them, or be removed entirely.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Parameters
///     ----------
///     distance : float
///         Distance traveled.
///     time : float
///         Time spent traveling.
///
///     Returns
///     -------
///     float
///         Speed as distance divided by time.
///
///     Raises
///     ------
///     FasterThanLightError
///         If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Style Guide](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct EmptyDocstringSection {
    name: String,
}

impl Violation for EmptyDocstringSection {
    #[derive_message_formats]
    fn message(&self) -> String {
        let EmptyDocstringSection { name } = self;
        format!("Section has no content (\"{name}\")")
    }
}

/// ## What it does
/// Checks for docstring section headers that do not end with a colon.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline Google-style
/// docstrings. If a multiline Google-style docstring consists of multiple
/// sections, each section header should end with a colon.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body.
///
/// This rule is enabled when using the `google` convention, and disabled when
/// using the `pep257` and `numpy` conventions.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     Returns
///         Speed as distance divided by time.
///
///     Raises
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [Google Style Guide](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct MissingSectionNameColon {
    name: String,
}

impl AlwaysFixableViolation for MissingSectionNameColon {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MissingSectionNameColon { name } = self;
        format!("Section name should end with a colon (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        let MissingSectionNameColon { name } = self;
        format!("Add colon to \"{name}\"")
    }
}

/// ## What it does
/// Checks for function docstrings that do not include documentation for all
/// parameters in the function.
///
/// ## Why is this bad?
/// This rule helps prevent you from leaving Google-style docstrings unfinished
/// or incomplete. Multiline Google-style docstrings should describe all
/// parameters for the function they are documenting.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body. Function docstrings often include a section for
/// function arguments; this rule is concerned with that section only.
/// Note that this rule only checks docstrings with an arguments (e.g. `Args`) section.
///
/// This rule is enabled when using the `google` convention, and disabled when
/// using the `pep257` and `numpy` conventions.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///         distance: Distance traveled.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
/// - `lint.pydocstyle.ignore-var-parameters`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct UndocumentedParam {
    /// The name of the function being documented.
    definition: String,
    /// The names of the undocumented parameters.
    names: Vec<String>,
}

impl Violation for UndocumentedParam {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UndocumentedParam { definition, names } = self;
        if names.len() == 1 {
            let name = &names[0];
            format!("Missing argument description in the docstring for `{definition}`: `{name}`")
        } else {
            let names = names.iter().map(|name| format!("`{name}`")).join(", ");
            format!("Missing argument descriptions in the docstring for `{definition}`: {names}")
        }
    }
}

/// ## What it does
/// Checks for docstring sections that contain blank lines between a section
/// header and a section body.
///
/// ## Why is this bad?
/// This rule enforces a consistent style for multiline docstrings.
///
/// Multiline docstrings are typically composed of a summary line, followed by
/// a blank line, followed by a series of sections, each with a section header
/// and a section body. There should be no blank lines between a section header
/// and a section body.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// Use instead:
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///         distance: Distance traveled.
///         time: Time spent traveling.
///
///     Returns:
///         Speed as distance divided by time.
///
///     Raises:
///         FasterThanLightError: If speed is greater than the speed of light.
///     """
///     try:
///         return distance / time
///     except ZeroDivisionError as exc:
///         raise FasterThanLightError from exc
/// ```
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [PEP 287 – reStructuredText Docstring Format](https://peps.python.org/pep-0287/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct BlankLinesBetweenHeaderAndContent {
    name: String,
}

impl AlwaysFixableViolation for BlankLinesBetweenHeaderAndContent {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BlankLinesBetweenHeaderAndContent { name } = self;
        format!("No blank lines allowed between a section header and its content (\"{name}\")")
    }

    fn fix_title(&self) -> String {
        "Remove blank line(s)".to_string()
    }
}
