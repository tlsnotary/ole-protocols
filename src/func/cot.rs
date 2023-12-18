//! This module implements the COT functionality (page 5) from <https://eprint.iacr.org/2015/546> without errors.

use mpz_share_conversion_core::fields::UniformRand;

use super::Role;
use crate::f2::F2;

#[derive(Debug, Default)]
pub struct Cot {
    kappa: usize,
    l: usize,
    delta: Vec<F2>,
    t: Vec<Vec<F2>>,
    q: Vec<Vec<F2>>,
}

impl Cot {
    pub fn new(kappa: usize, l: usize) -> Self {
        Self {
            kappa,
            l,
            ..Default::default()
        }
    }

    pub fn initialize_input_delta(&mut self, delta: Vec<F2>) {
        assert_eq!(delta.len(), self.kappa);

        self.delta = delta;
    }

    pub fn extend_input_x(&mut self, x: Vec<Vec<F2>>) {
        assert!(self.t.is_empty());
        assert!(self.q.is_empty());
        assert_eq!(x.len(), self.l);

        for vec in x.iter() {
            assert_eq!(vec.len(), self.kappa);
        }

        let mut rng = rand::thread_rng();

        for _ in 0..self.l {
            let inner = (0..self.kappa)
                .map(|_| F2::rand(&mut rng))
                .collect::<Vec<F2>>();
            self.t.push(inner);
        }

        for (ti, xi) in self.t.iter().zip(x.iter()) {
            let xi_times_delta = self
                .delta
                .iter()
                .zip(xi.iter())
                .map(|(&deltai, &xi)| deltai * xi)
                .collect::<Vec<F2>>();

            self.q.push(
                ti.iter()
                    .zip(xi_times_delta.iter())
                    .map(|(&tk, &xtdk)| tk + xtdk)
                    .collect::<Vec<F2>>(),
            );
        }
    }

    pub fn output(&mut self, role: Role) -> Vec<Vec<F2>> {
        let out = if role == Role::Sender {
            std::mem::take(&mut self.q)
        } else {
            std::mem::take(&mut self.t)
        };

        std::mem::take(self);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cot() {
        let mut cot = Cot::new(5, 3);
        let delta = vec![
            F2::new(true),
            F2::new(false),
            F2::new(true),
            F2::new(false),
            F2::new(true),
        ];

        let x = vec![
            vec![
                F2::new(true),
                F2::new(false),
                F2::new(true),
                F2::new(false),
                F2::new(true),
            ],
            vec![
                F2::new(false),
                F2::new(true),
                F2::new(false),
                F2::new(true),
                F2::new(true),
            ],
            vec![
                F2::new(false),
                F2::new(true),
                F2::new(true),
                F2::new(false),
                F2::new(false),
            ],
        ];

        cot.initialize_input_delta(delta.clone());
        cot.extend_input_x(x.clone());

        let q = cot.output(Role::Sender);
        let t = cot.output(Role::Receiver);

        for ((qi, ti), xi) in q.iter().zip(t.iter()).zip(x.iter()) {
            let qi_minus_ti = qi
                .iter()
                .zip(ti.iter())
                .map(|(&qij, &tij)| qij + -tij)
                .collect::<Vec<F2>>();

            let xi_times_delta = xi
                .iter()
                .zip(delta.iter())
                .map(|(&xij, &deltai)| xij * deltai)
                .collect::<Vec<F2>>();

            assert_eq!(qi_minus_ti, xi_times_delta);
        }
    }
}
