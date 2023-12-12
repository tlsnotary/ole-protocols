//! This module is a testing ground for the GHASH protocol (page 36) from <https://eprint.iacr.org/2023/964>

use mpz_share_conversion_core::fields::{gf2_128::Gf2_128, UniformRand};
use rand::thread_rng;

use crate::ole::{Ole, Role};

#[derive(Debug)]
pub struct Prover {
    block_num: usize,
    h1: Gf2_128,
    r1: Gf2_128,
    ai: Vec<Gf2_128>,
}

impl Prover {
    pub fn new(block_num: usize, h1: Gf2_128) -> Self {
        let mut rng = thread_rng();
        let r1 = Gf2_128::rand(&mut rng);
        Self {
            block_num,
            h1,
            r1,
            ai: vec![],
        }
    }

    pub fn preprocess_ole_input(&self, ole: &mut Ole<Gf2_128>) {
        let mut r1_powers = vec![Gf2_128::new(1)];

        for k in 0..self.block_num {
            r1_powers.push(self.r1 * r1_powers[k]);
        }
        ole.input(Role::Sender, r1_powers)
    }

    pub fn preprocess_ole_output(&mut self, ole: &mut Ole<Gf2_128>) {
        self.ai = ole.output(Role::Sender);
    }
}

pub struct Verifier {
    block_num: usize,
    h2: Gf2_128,
    r2: Gf2_128,
    bi: Vec<Gf2_128>,
}

impl Verifier {
    pub fn new(block_num: usize, h2: Gf2_128) -> Self {
        let mut rng = thread_rng();
        let r2 = Gf2_128::rand(&mut rng);
        Self {
            block_num,
            h2,
            r2,
            bi: vec![],
        }
    }

    pub fn preprocess_ole_input(&self, ole: &mut Ole<Gf2_128>) {
        let mut r2_powers = vec![Gf2_128::new(1)];

        for k in 0..self.block_num {
            r2_powers.push(self.r2 * r2_powers[k]);
        }
        ole.input(Role::Receiver, r2_powers)
    }

    pub fn preprocess_ole_output(&mut self, ole: &mut Ole<Gf2_128>) {
        self.bi = ole.output(Role::Receiver);
    }
}
