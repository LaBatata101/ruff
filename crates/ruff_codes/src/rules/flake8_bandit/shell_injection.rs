use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use super::common::Safety;

/// ## What it does
/// Check for method calls that initiate a subprocess with a shell.
///
/// ## Why is this bad?
/// Starting a subprocess with a shell can allow attackers to execute arbitrary
/// shell commands. Consider starting the process without a shell call and
/// sanitize the input to mitigate the risk of shell injection.
///
/// ## Example
/// ```python
/// import subprocess
///
/// subprocess.run("ls -l", shell=True)
/// ```
///
/// Use instead:
/// ```python
/// import subprocess
///
/// subprocess.run(["ls", "-l"])
/// ```
///
/// ## References
/// - [Python documentation: `subprocess` — Subprocess management](https://docs.python.org/3/library/subprocess.html)
/// - [Common Weakness Enumeration: CWE-78](https://cwe.mitre.org/data/definitions/78.html)
#[derive(ViolationMetadata)]
pub struct SubprocessPopenWithShellEqualsTrue {
    safety: Safety,
    is_exact: bool,
}

impl Violation for SubprocessPopenWithShellEqualsTrue {
    #[derive_message_formats]
    fn message(&self) -> String {
        match (self.safety, self.is_exact) {
            (Safety::SeemsSafe, true) => "`subprocess` call with `shell=True` seems safe, but may be changed in the future; consider rewriting without `shell`".to_string(),
            (Safety::Unknown, true) => "`subprocess` call with `shell=True` identified, security issue".to_string(),
            (Safety::SeemsSafe, false) => "`subprocess` call with truthy `shell` seems safe, but may be changed in the future; consider rewriting without `shell`".to_string(),
            (Safety::Unknown, false) => "`subprocess` call with truthy `shell` identified, security issue".to_string(),
        }
    }
}

/// ## What it does
/// Check for method calls that initiate a subprocess without a shell.
///
/// ## Why is this bad?
/// Starting a subprocess without a shell can prevent attackers from executing
/// arbitrary shell commands; however, it is still error-prone. Consider
/// validating the input.
///
/// ## Known problems
/// Prone to false positives as it is difficult to determine whether the
/// passed arguments have been validated ([#4045]).
///
/// ## Example
/// ```python
/// import subprocess
///
/// cmd = input("Enter a command: ").split()
/// subprocess.run(cmd)
/// ```
///
/// ## References
/// - [Python documentation: `subprocess` — Subprocess management](https://docs.python.org/3/library/subprocess.html)
///
/// [#4045]: https://github.com/astral-sh/ruff/issues/4045
#[derive(ViolationMetadata)]
pub struct SubprocessWithoutShellEqualsTrue;

impl Violation for SubprocessWithoutShellEqualsTrue {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`subprocess` call: check for execution of untrusted input".to_string()
    }
}

/// ## What it does
/// Checks for method calls that set the `shell` parameter to `true` or another
/// truthy value when invoking a subprocess.
///
/// ## Why is this bad?
/// Setting the `shell` parameter to `true` or another truthy value when
/// invoking a subprocess can introduce security vulnerabilities, as it allows
/// shell metacharacters and whitespace to be passed to child processes,
/// potentially leading to shell injection attacks.
///
/// It is recommended to avoid using `shell=True` unless absolutely necessary
/// and, when used, to ensure that all inputs are properly sanitized and quoted
/// to prevent such vulnerabilities.
///
/// ## Known problems
/// Prone to false positives as it is triggered on any function call with a
/// `shell=True` parameter.
///
/// ## Example
/// ```python
/// import subprocess
///
/// user_input = input("Enter a command: ")
/// subprocess.run(user_input, shell=True)
/// ```
///
/// ## References
/// - [Python documentation: Security Considerations](https://docs.python.org/3/library/subprocess.html#security-considerations)
#[derive(ViolationMetadata)]
pub struct CallWithShellEqualsTrue {
    is_exact: bool,
}

impl Violation for CallWithShellEqualsTrue {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.is_exact {
            "Function call with `shell=True` parameter identified, security issue".to_string()
        } else {
            "Function call with truthy `shell` parameter identified, security issue".to_string()
        }
    }
}

/// ## What it does
/// Checks for calls that start a process with a shell, providing guidance on
/// whether the usage is safe or not.
///
/// ## Why is this bad?
/// Starting a process with a shell can introduce security risks, such as
/// code injection vulnerabilities. It's important to be aware of whether the
/// usage of the shell is safe or not.
///
/// This rule triggers on functions like `os.system`, `popen`, etc., which
/// start processes with a shell. It evaluates whether the provided command
/// is a literal string or an expression. If the command is a literal string,
/// it's considered safe. If the command is an expression, it's considered
/// (potentially) unsafe.
///
/// ## Example
/// ```python
/// import os
///
/// # Safe usage (literal string)
/// command = "ls -l"
/// os.system(command)
///
/// # Potentially unsafe usage (expression)
/// cmd = get_user_input()
/// os.system(cmd)
/// ```
///
/// ## Note
/// The `subprocess` module provides more powerful facilities for spawning new
/// processes and retrieving their results, and using that module is preferable
/// to using `os.system` or similar functions. Consider replacing such usages
/// with `subprocess.call` or related functions.
///
/// ## References
/// - [Python documentation: `subprocess`](https://docs.python.org/3/library/subprocess.html)
#[derive(ViolationMetadata)]
pub struct StartProcessWithAShell {
    safety: Safety,
}

impl Violation for StartProcessWithAShell {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.safety {
            Safety::SeemsSafe => "Starting a process with a shell: seems safe, but may be changed in the future; consider rewriting without `shell`".to_string(),
            Safety::Unknown => "Starting a process with a shell, possible injection detected".to_string(),
        }
    }
}

/// ## What it does
/// Checks for functions that start a process without a shell.
///
/// ## Why is this bad?
/// Invoking any kind of external executable via a function call can pose
/// security risks if arbitrary variables are passed to the executable, or if
/// the input is otherwise unsanitised or unvalidated.
///
/// This rule specifically flags functions in the `os` module that spawn
/// subprocesses *without* the use of a shell. Note that these typically pose a
/// much smaller security risk than subprocesses that are started *with* a
/// shell, which are flagged by [`start-process-with-a-shell`][S605] (`S605`).
/// This gives you the option of enabling one rule while disabling the other
/// if you decide that the security risk from these functions is acceptable
/// for your use case.
///
/// ## Example
/// ```python
/// import os
///
///
/// def insecure_function(arbitrary_user_input: str):
///     os.spawnlp(os.P_NOWAIT, "/bin/mycmd", "mycmd", arbitrary_user_input)
/// ```
///
/// [S605]: https://docs.astral.sh/ruff/rules/start-process-with-a-shell
#[derive(ViolationMetadata)]
pub struct StartProcessWithNoShell;

impl Violation for StartProcessWithNoShell {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Starting a process without a shell".to_string()
    }
}

/// ## What it does
/// Checks for the starting of a process with a partial executable path.
///
/// ## Why is this bad?
/// Starting a process with a partial executable path can allow attackers to
/// execute an arbitrary executable by adjusting the `PATH` environment variable.
/// Consider using a full path to the executable instead.
///
/// ## Example
/// ```python
/// import subprocess
///
/// subprocess.Popen(["ruff", "check", "file.py"])
/// ```
///
/// Use instead:
/// ```python
/// import subprocess
///
/// subprocess.Popen(["/usr/bin/ruff", "check", "file.py"])
/// ```
///
/// ## References
/// - [Python documentation: `subprocess.Popen()`](https://docs.python.org/3/library/subprocess.html#subprocess.Popen)
/// - [Common Weakness Enumeration: CWE-426](https://cwe.mitre.org/data/definitions/426.html)
#[derive(ViolationMetadata)]
pub struct StartProcessWithPartialPath;

impl Violation for StartProcessWithPartialPath {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Starting a process with a partial executable path".to_string()
    }
}

/// ## What it does
/// Checks for possible wildcard injections in calls to `subprocess.Popen()`.
///
/// ## Why is this bad?
/// Wildcard injections can lead to unexpected behavior if unintended files are
/// matched by the wildcard. Consider using a more specific path instead.
///
/// ## Example
/// ```python
/// import subprocess
///
/// subprocess.Popen(["chmod", "777", "*.py"])
/// ```
///
/// Use instead:
/// ```python
/// import subprocess
///
/// subprocess.Popen(["chmod", "777", "main.py"])
/// ```
///
/// ## References
/// - [Common Weakness Enumeration: CWE-78](https://cwe.mitre.org/data/definitions/78.html)
#[derive(ViolationMetadata)]
pub struct UnixCommandWildcardInjection;

impl Violation for UnixCommandWildcardInjection {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Possible wildcard injection in call due to `*` usage".to_string()
    }
}
