//! This crate is a testing ground for the E2F protocol (page 33) from <https://eprint.iacr.org/2023/964>

mod ole;
mod prover;
mod verifier;

pub use prover::Prover;
pub use verifier::Verifier;

// In step 6 abort if w == 0
