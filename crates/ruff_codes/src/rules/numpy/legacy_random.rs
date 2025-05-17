use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of legacy `np.random` function calls.
///
/// ## Why is this bad?
/// According to the NumPy documentation's [Legacy Random Generation]:
///
/// > The `RandomState` provides access to legacy generators... This class
/// > should only be used if it is essential to have randoms that are
/// > identical to what would have been produced by previous versions of
/// > NumPy.
///
/// The members exposed directly on the `random` module are convenience
/// functions that alias to methods on a global singleton `RandomState`
/// instance. NumPy recommends using a dedicated `Generator` instance
/// rather than the random variate generation methods exposed directly on
/// the `random` module, as the new `Generator` is both faster and has
/// better statistical properties.
///
/// See the documentation on [Random Sampling] and [NEP 19] for further
/// details.
///
/// ## Example
/// ```python
/// import numpy as np
///
/// np.random.seed(1337)
/// np.random.normal()
/// ```
///
/// Use instead:
/// ```python
/// rng = np.random.default_rng(1337)
/// rng.normal()
/// ```
///
/// [Legacy Random Generation]: https://numpy.org/doc/stable/reference/random/legacy.html#legacy
/// [Random Sampling]: https://numpy.org/doc/stable/reference/random/index.html#random-quick-start
/// [NEP 19]: https://numpy.org/neps/nep-0019-rng-policy.html
#[derive(ViolationMetadata)]
pub struct NumpyLegacyRandom {
    method_name: String,
}

impl Violation for NumpyLegacyRandom {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NumpyLegacyRandom { method_name } = self;
        format!("Replace legacy `np.random.{method_name}` call with `np.random.Generator`")
    }
}