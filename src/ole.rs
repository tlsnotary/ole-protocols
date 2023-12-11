//! This module implements an OLE functionality.

use mpz_share_conversion_core::fields::{p256::P256, UniformRand};
use rand::thread_rng;

#[derive(Debug, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ole() {
        let mut ole = Ole::default();
        let mut rng = thread_rng();

        let input_sender = vec![
            P256::rand(&mut rng),
            P256::rand(&mut rng),
            P256::rand(&mut rng),
        ];
        let input_receiver = vec![
            P256::rand(&mut rng),
            P256::rand(&mut rng),
            P256::rand(&mut rng),
        ];

        ole.input(Role::Sender, input_sender.clone());
        ole.input(Role::Receiver, input_receiver.clone());

        let output_sender = ole.output(Role::Sender);
        let output_receiver = ole.output(Role::Receiver);

        for (((is, ir), os), or) in input_sender
            .into_iter()
            .zip(input_receiver)
            .zip(output_sender)
            .zip(output_receiver)
        {
            assert_eq!(is * ir, os + or);
        }
    }
}
