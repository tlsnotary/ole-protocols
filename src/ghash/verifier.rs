use super::pascal_tri;
use crate::ole::{Ole, Role};
use mpz_share_conversion_core::{
    fields::{compute_product_repeated, gf2_128::Gf2_128, UniformRand},
    Field,
};
use rand::thread_rng;

#[derive(Debug)]
pub struct Verifier {
    pub(crate) block_num: usize,
    pub(crate) h2: Gf2_128,
    pub(crate) r2: Gf2_128,
    pub(crate) bi: Vec<Gf2_128>,
    pub(crate) d_powers: Vec<Gf2_128>,
    pub(crate) hi: Vec<Gf2_128>,
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
            d_powers: vec![],
            hi: vec![],
        }
    }

    pub fn preprocess_ole_input(&self, ole: &mut Ole<Gf2_128>) {
        let mut r2_powers = vec![Gf2_128::one()];

        compute_product_repeated(&mut r2_powers, self.r2, self.block_num);
        ole.input(Role::Receiver, r2_powers)
    }

    pub fn preprocess_ole_output(&mut self, ole: &mut Ole<Gf2_128>) {
        self.bi = ole.output(Role::Receiver);
    }

    pub fn handshake_a_open_d(&self) -> Gf2_128 {
        self.h2 + -self.bi[1]
    }

    pub fn handshake_a_set_di(&mut self, d: Gf2_128) {
        self.d_powers = vec![Gf2_128::one(), d];
        compute_product_repeated(&mut self.d_powers, d, self.block_num);
    }

    pub fn handshake_a_set_hi(&mut self) {
        let pascal_tri = pascal_tri::<Gf2_128>(self.block_num);

        for pascal_row in pascal_tri.iter().skip(1) {
            let h_pow_share = pascal_row
                .iter()
                .enumerate()
                .fold(Gf2_128::new(0), |acc, (i, &el)| {
                    acc + el * self.d_powers[pascal_row.len() - 1 - i] * self.bi[i]
                });
            self.hi.push(h_pow_share);
        }
    }

    pub fn handshake_output_ghash(&self, blocks: &[Gf2_128]) -> Gf2_128 {
        let mut res = Gf2_128::zero();

        for (i, block) in blocks.iter().enumerate() {
            res = res + *block * self.hi[i];
        }
        res
    }
}
