#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(portable_simd))]

//! To enable the nightly-only features, compile with the `nightly` feature flag.
//! This requires a nightly toolchain.

#[cfg(not(feature = "nightly"))]
mod scalar;
#[cfg(not(feature = "nightly"))]
pub use scalar::Taus88;

#[cfg(feature = "nightly")]
mod simd;
#[cfg(feature = "nightly")]
pub use simd::Taus88;

#[cfg(test)]
mod tests;
