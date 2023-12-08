//! The prover implementation

use crate::{Ole, Role};
use mpz_share_conversion_core::fields::{p256::P256, UniformRand};
use rand::thread_rng;

#[derive(Debug, Default)]
pub struct Prover {
    // Preprocess 1
    a1: Option<P256>,
    b1: Option<P256>,
    b1_prime: Option<P256>,
    r1: Option<P256>,

    // Preprocess 2
    a1_b2_share: Option<P256>,
    a2_b1_share: Option<P256>,
    a1_b2_prime_share: Option<P256>,
    a2_b1_prime_share: Option<P256>,
    r1_r2_share: Option<P256>,

    // Preprocess 3
    c1: Option<P256>,
    c1_prime: Option<P256>,

    // Preprocess 4
    r_squared_share: Option<P256>,

    // Handshake 5
    ec_point: Option<(P256, P256)>,
    omega_share: Option<P256>,
}

impl Prover {
    pub fn preprocess1(&mut self) {
        let mut rng = thread_rng();

        self.a1 = Some(P256::rand(&mut rng));
        self.b1 = Some(P256::rand(&mut rng));
        self.b1_prime = Some(P256::rand(&mut rng));
        self.r1 = Some(P256::rand(&mut rng));
    }

    pub fn preprocess2_ole_input(&mut self, ole: &mut Ole) {
        let a1 = self.a1.unwrap();
        let b1 = self.b1.unwrap();
        let b1_prime = self.b1_prime.unwrap();
        let r1 = self.r1.unwrap();

        ole.input(Role::Sender, vec![a1, b1, a1, b1_prime, r1]);
    }

    pub fn preprocess2_ole_output(&mut self, ole: &mut Ole) {
        let output = ole.output(Role::Sender);

        self.a1_b2_share = Some(output[0]);
        self.a2_b1_share = Some(output[1]);
        self.a1_b2_prime_share = Some(output[2]);
        self.a2_b1_prime_share = Some(output[3]);
        self.r1_r2_share = Some(output[4]);
    }

    pub fn preprocess3(&mut self) {
        let a1_b1_share = self.a1.unwrap() * self.b1.unwrap();
        let a1_b2_share = self.a1_b2_share.unwrap();
        let a2_b1_share = self.a2_b1_share.unwrap();

        self.c1 = Some(a1_b1_share + a1_b2_share + a2_b1_share);

        let a1_b1_prime_share = self.a1.unwrap() * self.b1_prime.unwrap();
        let a1_b2_prime_share = self.a1_b2_prime_share.unwrap();
        let a2_b1_prime_share = self.a2_b1_prime_share.unwrap();

        self.c1_prime = Some(a1_b1_prime_share + a1_b2_prime_share + a2_b1_prime_share);
    }

    pub fn preproces4(&mut self) {
        let r1_squared = self.r1.unwrap() * self.r1.unwrap();

        let two = P256::new(2).unwrap();
        let r1_r2_share = self.r1_r2_share.unwrap();

        self.r_squared_share = Some(r1_squared + two * r1_r2_share);
    }

    pub fn handshake5_input_ec(&mut self, ec_point: (P256, P256)) {
        self.ec_point = Some(ec_point);
    }

    pub fn handshake5_varepsilon1_share_open(&self) -> P256 {
        -self.ec_point.unwrap().0 + -self.b1.unwrap()
    }

    pub fn handshake5_set_omega(&mut self, varepsilon1: P256) {
        self.omega_share = Some(varepsilon1 * self.a1.unwrap() + self.c1.unwrap());
    }
}
