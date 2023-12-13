use super::pascal_tri;
use crate::ole::{Ole, Role};
use mpz_share_conversion_core::fields::{compute_product_repeated, gf2_128::Gf2_128, UniformRand};
use rand::thread_rng;

#[derive(Debug)]
pub struct Verifier {
    block_num: usize,
    h2: Gf2_128,
    r2: Gf2_128,
    bi: Vec<Gf2_128>,
    d: Option<Gf2_128>,
    hi: Vec<Gf2_128>,
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
            d: None,
            hi: vec![],
        }
    }

    pub fn preprocess_ole_input(&self, ole: &mut Ole<Gf2_128>) {
        let mut r2_powers = vec![Gf2_128::new(1)];

        compute_product_repeated(&mut r2_powers, self.r2, self.block_num);
        ole.input(Role::Receiver, r2_powers)
    }

    pub fn preprocess_ole_output(&mut self, ole: &mut Ole<Gf2_128>) {
        self.bi = ole.output(Role::Receiver);
    }

    pub fn handshake_a_open_d(&self) -> Gf2_128 {
        self.h2 + -self.r2
    }

    pub fn handshake_a_set_d(&mut self, d: Gf2_128) {
        self.d = Some(d);
    }

    pub fn handshake_a_set_hi(&mut self) {
        let mut di = vec![Gf2_128::new(1)];
        compute_product_repeated(&mut di, self.d.unwrap(), self.block_num);

        let pascal_tri = pascal_tri::<Gf2_128>(self.block_num);

        for k in 0..self.block_num {
            for el in pascal_tri[k].iter() {
                self.hi.push(*el * di[k] * self.bi[self.block_num - k]);
            }
        }
    }

    pub fn handshake_output_ghash(&self, blocks: &[Gf2_128]) -> Gf2_128 {
        let mut res = Gf2_128::new(0);

        for (i, block) in blocks.iter().enumerate() {
            res = res + *block * self.hi[i];
        }
        res
    }
}
