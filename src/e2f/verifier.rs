//! The verifier implementation

use crate::func::ole::{Ole, Role};
use mpz_share_conversion_core::fields::{p256::P256, Field, UniformRand};
use rand::thread_rng;

#[derive(Debug, Default)]
pub struct Verifier {
    // Preprocess 1
    pub(crate) a2: Option<P256>,
    pub(crate) b2: Option<P256>,
    pub(crate) b2_prime: Option<P256>,
    pub(crate) r2: Option<P256>,

    // Preprocess 2
    pub(crate) a1_b2_share: Option<P256>,
    pub(crate) a2_b1_share: Option<P256>,
    pub(crate) a1_b2_prime_share: Option<P256>,
    pub(crate) a2_b1_prime_share: Option<P256>,
    pub(crate) r1_r2_share: Option<P256>,

    // Preprocess 3
    pub(crate) c2: Option<P256>,
    pub(crate) c2_prime: Option<P256>,

    // Preprocess 4
    pub(crate) r_squared_share: Option<P256>,

    // Handshake 5
    pub(crate) ec_point: Option<(P256, P256)>,
    pub(crate) omega_share: Option<P256>,

    // Handshake 6
    pub(crate) eta_share: Option<P256>,

    // Handshake 7
    pub(crate) z2: Option<P256>,
}

impl Verifier {
    pub fn preprocess1(&mut self) {
        let mut rng = thread_rng();

        self.a2 = Some(P256::rand(&mut rng));
        self.b2 = Some(P256::rand(&mut rng));
        self.b2_prime = Some(P256::rand(&mut rng));
        self.r2 = Some(P256::rand(&mut rng));
    }

    pub fn preprocess2_ole_input(&mut self, ole: &mut Ole<P256>) {
        let a2 = self.a2.unwrap();
        let b2 = self.b2.unwrap();
        let b2_prime = self.b2_prime.unwrap();
        let r2 = self.r2.unwrap();

        ole.input(Role::Receiver, vec![b2, a2, b2_prime, a2, r2]);
    }

    pub fn preprocess2_ole_output(&mut self, ole: &mut Ole<P256>) {
        let output = ole.output(Role::Receiver);

        self.a1_b2_share = Some(output[0]);
        self.a2_b1_share = Some(output[1]);
        self.a1_b2_prime_share = Some(output[2]);
        self.a2_b1_prime_share = Some(output[3]);
        self.r1_r2_share = Some(output[4]);
    }

    pub fn preprocess3(&mut self) {
        let a2_b2_share = self.a2.unwrap() * self.b2.unwrap();
        let a1_b2_share = self.a1_b2_share.unwrap();
        let a2_b1_share = self.a2_b1_share.unwrap();

        self.c2 = Some(a2_b2_share + a1_b2_share + a2_b1_share);

        let a2_b2_prime_share = self.a2.unwrap() * self.b2_prime.unwrap();
        let a1_b2_prime_share = self.a1_b2_prime_share.unwrap();
        let a2_b1_prime_share = self.a2_b1_prime_share.unwrap();

        self.c2_prime = Some(a2_b2_prime_share + a1_b2_prime_share + a2_b1_prime_share);
    }

    pub fn preprocess4(&mut self) {
        let r2_squared = self.r2.unwrap() * self.r2.unwrap();

        let two = P256::new(2).unwrap();
        let r1_r2_share = self.r1_r2_share.unwrap();

        self.r_squared_share = Some(r2_squared + two * r1_r2_share);
    }

    pub fn handshake5_input_ec(&mut self, ec_point: (P256, P256)) {
        self.ec_point = Some(ec_point);
    }

    pub fn handshake5_varepsilon1_share_open(&self) -> P256 {
        self.ec_point.unwrap().0 + -self.b2.unwrap()
    }

    pub fn handshake5_set_omega(&mut self, varepsilon1: P256) {
        self.omega_share = Some(varepsilon1 * self.a2.unwrap() + self.c2.unwrap());
    }

    pub fn handshake6_omega_share_open(&self) -> P256 {
        self.omega_share.unwrap()
    }

    pub fn handshake6_varepsilon2_share_open(&self) -> P256 {
        self.ec_point.unwrap().1 + -self.b2_prime.unwrap()
    }

    pub fn handshake6_set_eta(&mut self, omega: P256, varepsilon2: P256) {
        if omega == P256::new(0).unwrap() {
            panic!("omega is 0");
        }

        let omega_inv = omega.inverse();
        let a2 = self.a2.unwrap();
        let c2_prime = self.c2_prime.unwrap();

        self.eta_share = Some(omega_inv * (varepsilon2 * a2 + c2_prime));
    }

    pub fn handshake7_varepsilon3_share_open(&self) -> P256 {
        self.eta_share.unwrap() + -self.r2.unwrap()
    }

    pub fn handshake7_set_z2(&mut self, varepsilon3: P256) {
        let two = P256::new(2).unwrap();
        let r2 = self.r2.unwrap();
        let r_squared_share = self.r_squared_share.unwrap();
        let x2 = self.ec_point.unwrap().0;

        self.z2 = Some(two * varepsilon3 * r2 + r_squared_share + -x2);
    }

    pub fn handshake8_z2_open(&self) -> P256 {
        self.z2.unwrap()
    }
}
