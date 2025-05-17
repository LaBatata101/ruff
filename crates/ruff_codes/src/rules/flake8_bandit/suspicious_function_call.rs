use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for calls to `pickle` functions or modules that wrap them.
///
/// ## Why is this bad?
/// Deserializing untrusted data with `pickle` and other deserialization
/// modules is insecure as it can allow for the creation of arbitrary objects,
/// which can then be used to achieve arbitrary code execution and otherwise
/// unexpected behavior.
///
/// Avoid deserializing untrusted data with `pickle` and other deserialization
/// modules. Instead, consider safer formats, such as JSON.
///
/// If you must deserialize untrusted data with `pickle`, consider signing the
/// data with a secret key and verifying the signature before deserializing the
/// payload, This will prevent an attacker from injecting arbitrary objects
/// into the serialized data.
///
/// In [preview], this rule will also flag references to `pickle` functions.
///
/// ## Example
/// ```python
/// import pickle
///
/// with open("foo.pickle", "rb") as file:
///     foo = pickle.load(file)
/// ```
///
/// Use instead:
/// ```python
/// import json
///
/// with open("foo.json", "rb") as file:
///     foo = json.load(file)
/// ```
///
/// ## References
/// - [Python documentation: `pickle` — Python object serialization](https://docs.python.org/3/library/pickle.html)
/// - [Common Weakness Enumeration: CWE-502](https://cwe.mitre.org/data/definitions/502.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousPickleUsage;

impl Violation for SuspiciousPickleUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`pickle` and modules that wrap it can be unsafe when used to deserialize untrusted data, possible security issue".to_string()
    }
}

/// ## What it does
/// Checks for calls to `marshal` functions.
///
/// ## Why is this bad?
/// Deserializing untrusted data with `marshal` is insecure, as it can allow for
/// the creation of arbitrary objects, which can then be used to achieve
/// arbitrary code execution and otherwise unexpected behavior.
///
/// Avoid deserializing untrusted data with `marshal`. Instead, consider safer
/// formats, such as JSON.
///
/// If you must deserialize untrusted data with `marshal`, consider signing the
/// data with a secret key and verifying the signature before deserializing the
/// payload. This will prevent an attacker from injecting arbitrary objects
/// into the serialized data.
///
/// In [preview], this rule will also flag references to `marshal` functions.
///
/// ## Example
/// ```python
/// import marshal
///
/// with open("foo.marshal", "rb") as file:
///     foo = marshal.load(file)
/// ```
///
/// Use instead:
/// ```python
/// import json
///
/// with open("foo.json", "rb") as file:
///     foo = json.load(file)
/// ```
///
/// ## References
/// - [Python documentation: `marshal` — Internal Python object serialization](https://docs.python.org/3/library/marshal.html)
/// - [Common Weakness Enumeration: CWE-502](https://cwe.mitre.org/data/definitions/502.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousMarshalUsage;

impl Violation for SuspiciousMarshalUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Deserialization with the `marshal` module is possibly dangerous".to_string()
    }
}

/// ## What it does
/// Checks for uses of weak or broken cryptographic hash functions.
///
/// ## Why is this bad?
/// Weak or broken cryptographic hash functions may be susceptible to
/// collision attacks (where two different inputs produce the same hash) or
/// pre-image attacks (where an attacker can find an input that produces a
/// given hash). This can lead to security vulnerabilities in applications
/// that rely on these hash functions.
///
/// Avoid using weak or broken cryptographic hash functions in security
/// contexts. Instead, use a known secure hash function such as SHA-256.
///
/// In [preview], this rule will also flag references to insecure hash functions.
///
/// ## Example
/// ```python
/// from cryptography.hazmat.primitives import hashes
///
/// digest = hashes.Hash(hashes.MD5())
/// digest.update(b"Hello, world!")
/// digest.finalize()
/// ```
///
/// Use instead:
/// ```python
/// from cryptography.hazmat.primitives import hashes
///
/// digest = hashes.Hash(hashes.SHA256())
/// digest.update(b"Hello, world!")
/// digest.finalize()
/// ```
///
/// ## References
/// - [Python documentation: `hashlib` — Secure hashes and message digests](https://docs.python.org/3/library/hashlib.html)
/// - [Common Weakness Enumeration: CWE-327](https://cwe.mitre.org/data/definitions/327.html)
/// - [Common Weakness Enumeration: CWE-328](https://cwe.mitre.org/data/definitions/328.html)
/// - [Common Weakness Enumeration: CWE-916](https://cwe.mitre.org/data/definitions/916.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousInsecureHashUsage;

impl Violation for SuspiciousInsecureHashUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of insecure MD2, MD4, MD5, or SHA1 hash function".to_string()
    }
}

/// ## What it does
/// Checks for uses of weak or broken cryptographic ciphers.
///
/// ## Why is this bad?
/// Weak or broken cryptographic ciphers may be susceptible to attacks that
/// allow an attacker to decrypt ciphertext without knowing the key or
/// otherwise compromise the security of the cipher, such as forgeries.
///
/// Use strong, modern cryptographic ciphers instead of weak or broken ones.
///
/// In [preview], this rule will also flag references to insecure ciphers.
///
/// ## Example
/// ```python
/// from cryptography.hazmat.primitives.ciphers import Cipher, algorithms
///
/// algorithm = algorithms.ARC4(key)
/// cipher = Cipher(algorithm, mode=None)
/// encryptor = cipher.encryptor()
/// ```
///
/// Use instead:
/// ```python
/// from cryptography.fernet import Fernet
///
/// fernet = Fernet(key)
/// ```
///
/// ## References
/// - [Common Weakness Enumeration: CWE-327](https://cwe.mitre.org/data/definitions/327.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousInsecureCipherUsage;

impl Violation for SuspiciousInsecureCipherUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of insecure cipher, replace with a known secure cipher such as AES".to_string()
    }
}

/// ## What it does
/// Checks for uses of weak or broken cryptographic cipher modes.
///
/// ## Why is this bad?
/// Weak or broken cryptographic ciphers may be susceptible to attacks that
/// allow an attacker to decrypt ciphertext without knowing the key or
/// otherwise compromise the security of the cipher, such as forgeries.
///
/// Use strong, modern cryptographic ciphers instead of weak or broken ones.
///
/// In [preview], this rule will also flag references to insecure cipher modes.
///
/// ## Example
/// ```python
/// from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
///
/// algorithm = algorithms.ARC4(key)
/// cipher = Cipher(algorithm, mode=modes.ECB(iv))
/// encryptor = cipher.encryptor()
/// ```
///
/// Use instead:
/// ```python
/// from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
///
/// algorithm = algorithms.ARC4(key)
/// cipher = Cipher(algorithm, mode=modes.CTR(iv))
/// encryptor = cipher.encryptor()
/// ```
///
/// ## References
/// - [Common Weakness Enumeration: CWE-327](https://cwe.mitre.org/data/definitions/327.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousInsecureCipherModeUsage;

impl Violation for SuspiciousInsecureCipherModeUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of insecure block cipher mode, replace with a known secure mode such as CBC or CTR"
            .to_string()
    }
}

/// ## What it does
/// Checks for uses of `tempfile.mktemp`.
///
/// ## Why is this bad?
/// `tempfile.mktemp` returns a pathname of a file that does not exist at the
/// time the call is made; then, the caller is responsible for creating the
/// file and subsequently using it. This is insecure because another process
/// could create a file with the same name between the time the function
/// returns and the time the caller creates the file.
///
/// `tempfile.mktemp` is deprecated in favor of `tempfile.mkstemp` which
/// creates the file when it is called. Consider using `tempfile.mkstemp`
/// instead, either directly or via a context manager such as
/// `tempfile.TemporaryFile`.
///
/// In [preview], this rule will also flag references to `tempfile.mktemp`.
///
/// ## Example
/// ```python
/// import tempfile
///
/// tmp_file = tempfile.mktemp()
/// with open(tmp_file, "w") as file:
///     file.write("Hello, world!")
/// ```
///
/// Use instead:
/// ```python
/// import tempfile
///
/// with tempfile.TemporaryFile() as file:
///     file.write("Hello, world!")
/// ```
///
/// ## References
/// - [Python documentation:`mktemp`](https://docs.python.org/3/library/tempfile.html#tempfile.mktemp)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousMktempUsage;

impl Violation for SuspiciousMktempUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of insecure and deprecated function (`mktemp`)".to_string()
    }
}

/// ## What it does
/// Checks for uses of the builtin `eval()` function.
///
/// ## Why is this bad?
/// The `eval()` function is insecure as it enables arbitrary code execution.
///
/// If you need to evaluate an expression from a string, consider using
/// `ast.literal_eval()` instead, which will raise an exception if the
/// expression is not a valid Python literal.
///
/// In [preview], this rule will also flag references to `eval`.
///
/// ## Example
/// ```python
/// x = eval(input("Enter a number: "))
/// ```
///
/// Use instead:
/// ```python
/// from ast import literal_eval
///
/// x = literal_eval(input("Enter a number: "))
/// ```
///
/// ## References
/// - [Python documentation: `eval`](https://docs.python.org/3/library/functions.html#eval)
/// - [Python documentation: `literal_eval`](https://docs.python.org/3/library/ast.html#ast.literal_eval)
/// - [_Eval really is dangerous_ by Ned Batchelder](https://nedbatchelder.com/blog/201206/eval_really_is_dangerous.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousEvalUsage;

impl Violation for SuspiciousEvalUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of possibly insecure function; consider using `ast.literal_eval`".to_string()
    }
}

/// ## What it does
/// Checks for uses of calls to `django.utils.safestring.mark_safe`.
///
/// ## Why is this bad?
/// Cross-site scripting (XSS) vulnerabilities allow attackers to execute
/// arbitrary JavaScript. To guard against XSS attacks, Django templates
/// assumes that data is unsafe and automatically escapes malicious strings
/// before rending them.
///
/// `django.utils.safestring.mark_safe` marks a string as safe for use in HTML
/// templates, bypassing XSS protection. Its usage can be dangerous if the
/// contents of the string are dynamically generated, because it may allow
/// cross-site scripting attacks if the string is not properly escaped.
///
/// For dynamically generated strings, consider utilizing
/// `django.utils.html.format_html`.
///
/// In [preview], this rule will also flag references to `django.utils.safestring.mark_safe`.
///
/// ## Example
/// ```python
/// from django.utils.safestring import mark_safe
///
///
/// def render_username(username):
///     return mark_safe(f"<i>{username}</i>")  # Dangerous if username is user-provided.
/// ```
///
/// Use instead:
/// ```python
/// from django.utils.html import format_html
///
///
/// def render_username(username):
///     return django.utils.html.format_html("<i>{}</i>", username)  # username is escaped.
/// ```
///
/// ## References
/// - [Django documentation: `mark_safe`](https://docs.djangoproject.com/en/dev/ref/utils/#django.utils.safestring.mark_safe)
/// - [Django documentation: Cross Site Scripting (XSS) protection](https://docs.djangoproject.com/en/dev/topics/security/#cross-site-scripting-xss-protection)
/// - [Common Weakness Enumeration: CWE-80](https://cwe.mitre.org/data/definitions/80.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousMarkSafeUsage;

impl Violation for SuspiciousMarkSafeUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of `mark_safe` may expose cross-site scripting vulnerabilities".to_string()
    }
}

/// ## What it does
/// Checks for instances where URL open functions are used with unexpected schemes.
///
/// ## Why is this bad?
/// Some URL open functions allow the use of `file:` or custom schemes (for use
/// instead of `http:` or `https:`). An attacker may be able to use these
/// schemes to access or modify unauthorized resources, and cause unexpected
/// behavior.
///
/// To mitigate this risk, audit all uses of URL open functions and ensure that
/// only permitted schemes are used (e.g., allowing `http:` and `https:`, and
/// disallowing `file:` and `ftp:`).
///
/// In [preview], this rule will also flag references to URL open functions.
///
/// ## Example
/// ```python
/// from urllib.request import urlopen
///
/// url = input("Enter a URL: ")
///
/// with urlopen(url) as response:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// from urllib.request import urlopen
///
/// url = input("Enter a URL: ")
///
/// if not url.startswith(("http:", "https:")):
///     raise ValueError("URL must start with 'http:' or 'https:'")
///
/// with urlopen(url) as response:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `urlopen`](https://docs.python.org/3/library/urllib.request.html#urllib.request.urlopen)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousURLOpenUsage;

impl Violation for SuspiciousURLOpenUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Audit URL open for permitted schemes. Allowing use of `file:` or custom schemes is often unexpected.".to_string()
    }
}

/// ## What it does
/// Checks for uses of cryptographically weak pseudo-random number generators.
///
/// ## Why is this bad?
/// Cryptographically weak pseudo-random number generators are insecure, as they
/// are easily predictable. This can allow an attacker to guess the generated
/// numbers and compromise the security of the system.
///
/// Instead, use a cryptographically secure pseudo-random number generator
/// (such as using the [`secrets` module](https://docs.python.org/3/library/secrets.html))
/// when generating random numbers for security purposes.
///
/// In [preview], this rule will also flag references to these generators.
///
/// ## Example
/// ```python
/// import random
///
/// random.randrange(10)
/// ```
///
/// Use instead:
/// ```python
/// import secrets
///
/// secrets.randbelow(10)
/// ```
///
/// ## References
/// - [Python documentation: `random` — Generate pseudo-random numbers](https://docs.python.org/3/library/random.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousNonCryptographicRandomUsage;

impl Violation for SuspiciousNonCryptographicRandomUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Standard pseudo-random generators are not suitable for cryptographic purposes".to_string()
    }
}

/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// Consider using the `defusedxml` package when parsing untrusted XML data,
/// to protect against XML attacks.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from xml.etree.cElementTree import parse
///
/// tree = parse("untrusted.xml")  # Vulnerable to XML attacks.
/// ```
///
/// Use instead:
/// ```python
/// from defusedxml.cElementTree import parse
///
/// tree = parse("untrusted.xml")
/// ```
///
/// ## References
/// - [Python documentation: `xml` — XML processing modules](https://docs.python.org/3/library/xml.html)
/// - [PyPI: `defusedxml`](https://pypi.org/project/defusedxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLCElementTreeUsage;

impl Violation for SuspiciousXMLCElementTreeUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `xml` to parse untrusted data is known to be vulnerable to XML attacks; use `defusedxml` equivalents".to_string()
    }
}

/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// Consider using the `defusedxml` package when parsing untrusted XML data,
/// to protect against XML attacks.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from xml.etree.ElementTree import parse
///
/// tree = parse("untrusted.xml")  # Vulnerable to XML attacks.
/// ```
///
/// Use instead:
/// ```python
/// from defusedxml.ElementTree import parse
///
/// tree = parse("untrusted.xml")
/// ```
///
/// ## References
/// - [Python documentation: `xml` — XML processing modules](https://docs.python.org/3/library/xml.html)
/// - [PyPI: `defusedxml`](https://pypi.org/project/defusedxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLElementTreeUsage;

impl Violation for SuspiciousXMLElementTreeUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `xml` to parse untrusted data is known to be vulnerable to XML attacks; use `defusedxml` equivalents".to_string()
    }
}

/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// Consider using the `defusedxml` package when parsing untrusted XML data,
/// to protect against XML attacks.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from xml.sax.expatreader import create_parser
///
/// parser = create_parser()
/// ```
///
/// Use instead:
/// ```python
/// from defusedxml.sax import create_parser
///
/// parser = create_parser()
/// ```
///
/// ## References
/// - [Python documentation: `xml` — XML processing modules](https://docs.python.org/3/library/xml.html)
/// - [PyPI: `defusedxml`](https://pypi.org/project/defusedxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLExpatReaderUsage;

impl Violation for SuspiciousXMLExpatReaderUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `xml` to parse untrusted data is known to be vulnerable to XML attacks; use `defusedxml` equivalents".to_string()
    }
}

/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// Consider using the `defusedxml` package when parsing untrusted XML data,
/// to protect against XML attacks.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from xml.dom.expatbuilder import parse
///
/// parse("untrusted.xml")
/// ```
///
/// Use instead:
/// ```python
/// from defusedxml.expatbuilder import parse
///
/// tree = parse("untrusted.xml")
/// ```
///
/// ## References
/// - [Python documentation: `xml` — XML processing modules](https://docs.python.org/3/library/xml.html)
/// - [PyPI: `defusedxml`](https://pypi.org/project/defusedxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLExpatBuilderUsage;

impl Violation for SuspiciousXMLExpatBuilderUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `xml` to parse untrusted data is known to be vulnerable to XML attacks; use `defusedxml` equivalents".to_string()
    }
}

/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// Consider using the `defusedxml` package when parsing untrusted XML data,
/// to protect against XML attacks.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from xml.sax import make_parser
///
/// make_parser()
/// ```
///
/// Use instead:
/// ```python
/// from defusedxml.sax import make_parser
///
/// make_parser()
/// ```
///
/// ## References
/// - [Python documentation: `xml` — XML processing modules](https://docs.python.org/3/library/xml.html)
/// - [PyPI: `defusedxml`](https://pypi.org/project/defusedxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLSaxUsage;

impl Violation for SuspiciousXMLSaxUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `xml` to parse untrusted data is known to be vulnerable to XML attacks; use `defusedxml` equivalents".to_string()
    }
}

/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// Consider using the `defusedxml` package when parsing untrusted XML data,
/// to protect against XML attacks.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from xml.dom.minidom import parse
///
/// content = parse("untrusted.xml")
/// ```
///
/// Use instead:
/// ```python
/// from defusedxml.minidom import parse
///
/// content = parse("untrusted.xml")
/// ```
///
/// ## References
/// - [Python documentation: `xml` — XML processing modules](https://docs.python.org/3/library/xml.html)
/// - [PyPI: `defusedxml`](https://pypi.org/project/defusedxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLMiniDOMUsage;

impl Violation for SuspiciousXMLMiniDOMUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `xml` to parse untrusted data is known to be vulnerable to XML attacks; use `defusedxml` equivalents".to_string()
    }
}

/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// Consider using the `defusedxml` package when parsing untrusted XML data,
/// to protect against XML attacks.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from xml.dom.pulldom import parse
///
/// content = parse("untrusted.xml")
/// ```
///
/// Use instead:
/// ```python
/// from defusedxml.pulldom import parse
///
/// content = parse("untrusted.xml")
/// ```
///
/// ## References
/// - [Python documentation: `xml` — XML processing modules](https://docs.python.org/3/library/xml.html)
/// - [PyPI: `defusedxml`](https://pypi.org/project/defusedxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLPullDOMUsage;

impl Violation for SuspiciousXMLPullDOMUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `xml` to parse untrusted data is known to be vulnerable to XML attacks; use `defusedxml` equivalents".to_string()
    }
}

/// ## Deprecation
///
/// This rule was deprecated as the `lxml` library has been modified to address
/// known vulnerabilities and unsafe defaults. As such, the `defusedxml`
/// library is no longer necessary, `defusedxml` has [deprecated] its `lxml`
/// module.
///
/// ## What it does
/// Checks for uses of insecure XML parsers.
///
/// ## Why is this bad?
/// Many XML parsers are vulnerable to XML attacks (such as entity expansion),
/// which cause excessive memory and CPU usage by exploiting recursion. An
/// attacker could use such methods to access unauthorized resources.
///
/// In [preview], this rule will also flag references to insecure XML parsers.
///
/// ## Example
/// ```python
/// from lxml import etree
///
/// content = etree.parse("untrusted.xml")
/// ```
///
/// ## References
/// - [PyPI: `lxml`](https://pypi.org/project/lxml/)
/// - [Common Weakness Enumeration: CWE-400](https://cwe.mitre.org/data/definitions/400.html)
/// - [Common Weakness Enumeration: CWE-776](https://cwe.mitre.org/data/definitions/776.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
/// [deprecated]: https://pypi.org/project/defusedxml/0.8.0rc2/#defusedxml-lxml
#[derive(ViolationMetadata)]
pub struct SuspiciousXMLETreeUsage;

impl Violation for SuspiciousXMLETreeUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using `lxml` to parse untrusted data is known to be vulnerable to XML attacks".to_string()
    }
}

/// ## What it does
/// Checks for uses of `ssl._create_unverified_context`.
///
/// ## Why is this bad?
/// [PEP 476] enabled certificate and hostname validation by default in Python
/// standard library HTTP clients. Previously, Python did not validate
/// certificates by default, which could allow an attacker to perform a "man in
/// the middle" attack by intercepting and modifying traffic between client and
/// server.
///
/// To support legacy environments, `ssl._create_unverified_context` reverts to
/// the previous behavior that does perform verification. Otherwise, use
/// `ssl.create_default_context` to create a secure context.
///
/// In [preview], this rule will also flag references to `ssl._create_unverified_context`.
///
/// ## Example
/// ```python
/// import ssl
///
/// context = ssl._create_unverified_context()
/// ```
///
/// Use instead:
/// ```python
/// import ssl
///
/// context = ssl.create_default_context()
/// ```
///
/// ## References
/// - [PEP 476 – Enabling certificate verification by default for stdlib http clients: Opting out](https://peps.python.org/pep-0476/#opting-out)
/// - [Python documentation: `ssl` — TLS/SSL wrapper for socket objects](https://docs.python.org/3/library/ssl.html)
///
/// [PEP 476]: https://peps.python.org/pep-0476/
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousUnverifiedContextUsage;

impl Violation for SuspiciousUnverifiedContextUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Python allows using an insecure context via the `_create_unverified_context` that reverts to the previous behavior that does not validate certificates or perform hostname checks.".to_string()
    }
}

/// ## What it does
/// Checks for the use of Telnet-related functions.
///
/// ## Why is this bad?
/// Telnet is considered insecure because it does not encrypt data sent over
/// the connection and is vulnerable to numerous attacks.
///
/// Instead, consider using a more secure protocol such as SSH.
///
/// In [preview], this rule will also flag references to Telnet-related functions.
///
/// ## References
/// - [Python documentation: `telnetlib` — Telnet client](https://docs.python.org/3/library/telnetlib.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousTelnetUsage;

impl Violation for SuspiciousTelnetUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Telnet is considered insecure. Use SSH or some other encrypted protocol.".to_string()
    }
}

/// ## What it does
/// Checks for the use of FTP-related functions.
///
/// ## Why is this bad?
/// FTP is considered insecure as it does not encrypt data sent over the
/// connection and is thus vulnerable to numerous attacks.
///
/// Instead, consider using FTPS (which secures FTP using SSL/TLS) or SFTP.
///
/// In [preview], this rule will also flag references to FTP-related functions.
///
/// ## References
/// - [Python documentation: `ftplib` — FTP protocol client](https://docs.python.org/3/library/ftplib.html)
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct SuspiciousFTPLibUsage;

impl Violation for SuspiciousFTPLibUsage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "FTP-related functions are being called. FTP is considered insecure. Use SSH/SFTP/SCP or some other encrypted protocol.".to_string()
    }
}
