use itertools::Itertools;
use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions with `return` statements that do not have "Returns"
/// sections in their docstrings.
///
/// ## Why is this bad?
/// A missing "Returns" section is a sign of incomplete documentation.
///
/// This rule is not enforced for abstract methods or functions that only return
/// `None`. It is also ignored for "stub functions": functions where the body only
/// consists of `pass`, `...`, `raise NotImplementedError`, or similar.
///
/// ## Example
/// ```python
/// def calculate_speed(distance: float, time: float) -> float:
///     """Calculate speed as distance divided by time.
///
///     Args:
///         distance: Distance traveled.
///         time: Time spent traveling.
///     """
///     return distance / time
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
///     """
///     return distance / time
/// ```
#[derive(ViolationMetadata)]
pub struct DocstringMissingReturns;

impl Violation for DocstringMissingReturns {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`return` is not documented in docstring".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Add a \"Returns\" section to the docstring".to_string())
    }
}

/// ## What it does
/// Checks for function docstrings with unnecessary "Returns" sections.
///
/// ## Why is this bad?
/// A function without an explicit `return` statement should not have a
/// "Returns" section in its docstring.
///
/// This rule is not enforced for abstract methods. It is also ignored for
/// "stub functions": functions where the body only consists of `pass`, `...`,
/// `raise NotImplementedError`, or similar.
///
/// ## Example
/// ```python
/// def say_hello(n: int) -> None:
///     """Says hello to the user.
///
///     Args:
///         n: Number of times to say hello.
///
///     Returns:
///         Doesn't return anything.
///     """
///     for _ in range(n):
///         print("Hello!")
/// ```
///
/// Use instead:
/// ```python
/// def say_hello(n: int) -> None:
///     """Says hello to the user.
///
///     Args:
///         n: Number of times to say hello.
///     """
///     for _ in range(n):
///         print("Hello!")
/// ```
#[derive(ViolationMetadata)]
pub struct DocstringExtraneousReturns;

impl Violation for DocstringExtraneousReturns {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Docstring should not have a returns section because the function doesn't return anything"
            .to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove the \"Returns\" section".to_string())
    }
}

/// ## What it does
/// Checks for functions with `yield` statements that do not have "Yields" sections in
/// their docstrings.
///
/// ## Why is this bad?
/// A missing "Yields" section is a sign of incomplete documentation.
///
/// This rule is not enforced for abstract methods or functions that only yield `None`.
/// It is also ignored for "stub functions": functions where the body only consists
/// of `pass`, `...`, `raise NotImplementedError`, or similar.
///
/// ## Example
/// ```python
/// def count_to_n(n: int) -> int:
///     """Generate integers up to *n*.
///
///     Args:
///         n: The number at which to stop counting.
///     """
///     for i in range(1, n + 1):
///         yield i
/// ```
///
/// Use instead:
/// ```python
/// def count_to_n(n: int) -> int:
///     """Generate integers up to *n*.
///
///     Args:
///         n: The number at which to stop counting.
///
///     Yields:
///         int: The number we're at in the count.
///     """
///     for i in range(1, n + 1):
///         yield i
/// ```
#[derive(ViolationMetadata)]
pub struct DocstringMissingYields;

impl Violation for DocstringMissingYields {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`yield` is not documented in docstring".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Add a \"Yields\" section to the docstring".to_string())
    }
}

/// ## What it does
/// Checks for function docstrings with unnecessary "Yields" sections.
///
/// ## Why is this bad?
/// A function that doesn't yield anything should not have a "Yields" section
/// in its docstring.
///
/// This rule is not enforced for abstract methods. It is also ignored for
/// "stub functions": functions where the body only consists of `pass`, `...`,
/// `raise NotImplementedError`, or similar.
///
/// ## Example
/// ```python
/// def say_hello(n: int) -> None:
///     """Says hello to the user.
///
///     Args:
///         n: Number of times to say hello.
///
///     Yields:
///         Doesn't yield anything.
///     """
///     for _ in range(n):
///         print("Hello!")
/// ```
///
/// Use instead:
/// ```python
/// def say_hello(n: int) -> None:
///     """Says hello to the user.
///
///     Args:
///         n: Number of times to say hello.
///     """
///     for _ in range(n):
///         print("Hello!")
/// ```
#[derive(ViolationMetadata)]
pub struct DocstringExtraneousYields;

impl Violation for DocstringExtraneousYields {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Docstring has a \"Yields\" section but the function doesn't yield anything".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove the \"Yields\" section".to_string())
    }
}

/// ## What it does
/// Checks for function docstrings that do not document all explicitly raised
/// exceptions.
///
/// ## Why is this bad?
/// A function should document all exceptions that are directly raised in some
/// circumstances. Failing to document an exception that could be raised
/// can be misleading to users and/or a sign of incomplete documentation.
///
/// This rule is not enforced for abstract methods. It is also ignored for
/// "stub functions": functions where the body only consists of `pass`, `...`,
/// `raise NotImplementedError`, or similar.
///
/// ## Example
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
#[derive(ViolationMetadata)]
pub struct DocstringMissingException {
    id: String,
}

impl Violation for DocstringMissingException {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DocstringMissingException { id } = self;
        format!("Raised exception `{id}` missing from docstring")
    }

    fn fix_title(&self) -> Option<String> {
        let DocstringMissingException { id } = self;
        Some(format!("Add `{id}` to the docstring"))
    }
}

/// ## What it does
/// Checks for function docstrings that state that exceptions could be raised
/// even though they are not directly raised in the function body.
///
/// ## Why is this bad?
/// Some conventions prefer non-explicit exceptions be omitted from the
/// docstring.
///
/// This rule is not enforced for abstract methods. It is also ignored for
/// "stub functions": functions where the body only consists of `pass`, `...`,
/// `raise NotImplementedError`, or similar.
///
/// ## Example
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
///         ZeroDivisionError: Divided by zero.
///     """
///     return distance / time
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
///     """
///     return distance / time
/// ```
///
/// ## Known issues
/// It may often be desirable to document *all* exceptions that a function
/// could possibly raise, even those which are not explicitly raised using
/// `raise` statements in the function body.
#[derive(ViolationMetadata)]
pub struct DocstringExtraneousException {
    ids: Vec<String>,
}

impl Violation for DocstringExtraneousException {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DocstringExtraneousException { ids } = self;

        if let [id] = ids.as_slice() {
            format!("Raised exception is not explicitly raised: `{id}`")
        } else {
            format!(
                "Raised exceptions are not explicitly raised: {}",
                ids.iter().map(|id| format!("`{id}`")).join(", ")
            )
        }
    }

    fn fix_title(&self) -> Option<String> {
        let DocstringExtraneousException { ids } = self;
        Some(format!(
            "Remove {} from the docstring",
            ids.iter().map(|id| format!("`{id}`")).join(", ")
        ))
    }
}
