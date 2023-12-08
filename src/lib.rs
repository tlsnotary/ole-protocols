//! This crate is a testing ground for the E2F protocol (page 33) from <https://eprint.iacr.org/2023/964>

use mpz_share_conversion_core::fields::{p256::P256, UniformRand};
use rand::thread_rng;

mod prover;
mod verifier;

pub use prover::Prover;
pub use verifier::Verifier;

pub struct Ole {
    input_sender: Vec<P256>,
    input_receiver: Vec<P256>,
    output: Vec<P256>,
}

impl Ole {
    pub fn input(&mut self, role: Role, input: Vec<P256>) {
        if role == Role::Sender {
            self.input_sender = input;
        } else {
            self.input_receiver = input;
        }
    }

    pub fn output(&mut self, role: Role) -> Vec<P256> {
        assert!(self.input_sender.len() == self.input_receiver.len());

        if !self.output.is_empty() {
            return std::mem::take(&mut self.output);
        }

        let mut rng = thread_rng();
        let mut output = vec![];
        let mut output_cached = vec![];

        for (s, r) in self.input_sender.iter().zip(self.input_receiver.iter()) {
            let s_out = P256::rand(&mut rng);
            let r_out = *s * *r + -s_out;

            if role == Role::Sender {
                output.push(s_out);
                output_cached.push(r_out);
            } else {
                output.push(r_out);
                output_cached.push(s_out);
            }
        }
        self.input_sender.clear();
        self.input_receiver.clear();

        self.output = output_cached;
        output
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Sender,
    Receiver,
}
