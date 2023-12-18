//! This module implements an OLE functionality.

use super::Role;
use mpz_share_conversion_core::Field;
use rand::thread_rng;

#[derive(Debug)]
pub struct Ole<T: Field> {
    input_sender: Vec<T>,
    input_receiver: Vec<T>,
    output: Vec<T>,
}

impl<T: Field> Default for Ole<T> {
    fn default() -> Self {
        Self {
            input_sender: vec![],
            input_receiver: vec![],
            output: vec![],
        }
    }
}

impl<T: Field> Ole<T> {
    pub fn input(&mut self, role: Role, input: Vec<T>) {
        if role == Role::Sender {
            self.input_sender = input;
        } else {
            self.input_receiver = input;
        }
    }

    pub fn output(&mut self, role: Role) -> Vec<T> {
        assert!(self.input_sender.len() == self.input_receiver.len());

        if !self.output.is_empty() {
            return std::mem::take(&mut self.output);
        }

        let mut rng = thread_rng();
        let mut output = vec![];
        let mut output_cached = vec![];

        for (s, r) in self.input_sender.iter().zip(self.input_receiver.iter()) {
            let s_out = T::rand(&mut rng);
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
#[cfg(test)]
mod tests {
    use mpz_share_conversion_core::fields::{p256::P256, UniformRand};

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
