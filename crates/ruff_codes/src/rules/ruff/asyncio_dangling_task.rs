use std::fmt;

use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `asyncio.create_task` and `asyncio.ensure_future` calls
/// that do not store a reference to the returned result.
///
/// ## Why is this bad?
/// Per the `asyncio` documentation, the event loop only retains a weak
/// reference to tasks. If the task returned by `asyncio.create_task` and
/// `asyncio.ensure_future` is not stored in a variable, or a collection,
/// or otherwise referenced, it may be garbage collected at any time. This
/// can lead to unexpected and inconsistent behavior, as your tasks may or
/// may not run to completion.
///
/// ## Example
/// ```python
/// import asyncio
///
/// for i in range(10):
///     # This creates a weak reference to the task, which may be garbage
///     # collected at any time.
///     asyncio.create_task(some_coro(param=i))
/// ```
///
/// Use instead:
/// ```python
/// import asyncio
///
/// background_tasks = set()
///
/// for i in range(10):
///     task = asyncio.create_task(some_coro(param=i))
///
///     # Add task to the set. This creates a strong reference.
///     background_tasks.add(task)
///
///     # To prevent keeping references to finished tasks forever,
///     # make each task remove its own reference from the set after
///     # completion:
///     task.add_done_callback(background_tasks.discard)
/// ```
///
/// ## References
/// - [_The Heisenbug lurking in your async code_](https://textual.textualize.io/blog/2023/02/11/the-heisenbug-lurking-in-your-async-code/)
/// - [The Python Standard Library](https://docs.python.org/3/library/asyncio-task.html#asyncio.create_task)
#[derive(ViolationMetadata)]
pub struct AsyncioDanglingTask {
    expr: String,
    method: Method,
}

impl Violation for AsyncioDanglingTask {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AsyncioDanglingTask { expr, method } = self;
        format!("Store a reference to the return value of `{expr}.{method}`")
    }
}

// FIX: duplicated with ruff_rule_ruff::asyncio_dangling_task
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Method {
    CreateTask,
    EnsureFuture,
}

impl fmt::Display for Method {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::CreateTask => fmt.write_str("create_task"),
            Method::EnsureFuture => fmt.write_str("ensure_future"),
        }
    }
}