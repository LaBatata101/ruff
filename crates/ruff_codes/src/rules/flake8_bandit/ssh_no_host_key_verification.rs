use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of policies disabling SSH verification in Paramiko.
///
/// ## Why is this bad?
/// By default, Paramiko checks the identity of the remote host when establishing
/// an SSH connection. Disabling the verification might lead to the client
/// connecting to a malicious host, without the client knowing.
///
/// ## Example
/// ```python
/// from paramiko import client
///
/// ssh_client = client.SSHClient()
/// ssh_client.set_missing_host_key_policy(client.AutoAddPolicy)
/// ```
///
/// Use instead:
/// ```python
/// from paramiko import client
///
/// ssh_client = client.SSHClient()
/// ssh_client.set_missing_host_key_policy(client.RejectPolicy)
/// ```
///
/// ## References
/// - [Paramiko documentation: set_missing_host_key_policy](https://docs.paramiko.org/en/latest/api/client.html#paramiko.client.SSHClient.set_missing_host_key_policy)
#[derive(ViolationMetadata)]
pub struct SSHNoHostKeyVerification;

impl Violation for SSHNoHostKeyVerification {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Paramiko call with policy set to automatically trust the unknown host key".to_string()
    }
}