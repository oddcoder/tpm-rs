// a conscious decision has been taken that any platform code pulled from external crate
// is to be rexported here
pub use rand_core;

use rand_core::{CryptoRngCore, SeedableRng};

/// Provides access to cryptographic operations.
pub trait Crypto: CryptoRngCore + SeedableRng {}

// For all types that implement every Crypto sub trait, also implement the combined trait.
impl<T> Crypto for T where T: CryptoRngCore + SeedableRng {}
