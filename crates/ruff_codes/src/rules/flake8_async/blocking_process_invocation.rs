use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks that async functions do not create subprocesses with blocking methods.
///
/// ## Why is this bad?
/// Blocking an async function via a blocking call will block the entire
/// event loop, preventing it from executing other tasks while waiting for the
/// call to complete, negating the benefits of asynchronous programming.
///
/// Instead of making a blocking call, use an equivalent asynchronous library
/// or function, like [`trio.run_process()`](https://trio.readthedocs.io/en/stable/reference-io.html#trio.run_process)
/// or [`anyio.run_process()`](https://anyio.readthedocs.io/en/latest/api.html#anyio.run_process).
///
/// ## Example
/// ```python
/// async def foo():
///     os.popen(cmd)
/// ```
///
/// Use instead:
/// ```python
/// async def foo():
///     asyncio.create_subprocess_shell(cmd)
/// ```
#[derive(ViolationMetadata)]
pub struct CreateSubprocessInAsyncFunction;

impl Violation for CreateSubprocessInAsyncFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Async functions should not create subprocesses with blocking methods".to_string()
    }
}

/// ## What it does
/// Checks that async functions do not run processes with blocking methods.
///
/// ## Why is this bad?
/// Blocking an async function via a blocking call will block the entire
/// event loop, preventing it from executing other tasks while waiting for the
/// call to complete, negating the benefits of asynchronous programming.
///
/// Instead of making a blocking call, use an equivalent asynchronous library
/// or function, like [`trio.run_process()`](https://trio.readthedocs.io/en/stable/reference-io.html#trio.run_process)
/// or [`anyio.run_process()`](https://anyio.readthedocs.io/en/latest/api.html#anyio.run_process).
///
/// ## Example
/// ```python
/// async def foo():
///     subprocess.run(cmd)
/// ```
///
/// Use instead:
/// ```python
/// async def foo():
///     asyncio.create_subprocess_shell(cmd)
/// ```
#[derive(ViolationMetadata)]
pub struct RunProcessInAsyncFunction;

impl Violation for RunProcessInAsyncFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Async functions should not run processes with blocking methods".to_string()
    }
}

/// ## What it does
/// Checks that async functions do not wait on processes with blocking methods.
///
/// ## Why is this bad?
/// Blocking an async function via a blocking call will block the entire
/// event loop, preventing it from executing other tasks while waiting for the
/// call to complete, negating the benefits of asynchronous programming.
///
/// Instead of making a blocking call, use an equivalent asynchronous library
/// or function, like [`trio.to_thread.run_sync()`](https://trio.readthedocs.io/en/latest/reference-core.html#trio.to_thread.run_sync)
/// or [`anyio.to_thread.run_sync()`](https://anyio.readthedocs.io/en/latest/api.html#anyio.to_thread.run_sync).
///
/// ## Example
/// ```python
/// async def foo():
///     os.waitpid(0)
/// ```
///
/// Use instead:
/// ```python
/// def wait_for_process():
///     os.waitpid(0)
///
///
/// async def foo():
///     await asyncio.loop.run_in_executor(None, wait_for_process)
/// ```
#[derive(ViolationMetadata)]
pub struct WaitForProcessInAsyncFunction;

impl Violation for WaitForProcessInAsyncFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Async functions should not wait on processes with blocking methods".to_string()
    }
}
