//! This module is a testing ground for the GHASH protocol (page 36) from <https://eprint.iacr.org/2023/964>

mod prover;
mod verifier;

use mpz_share_conversion_core::{fields::gf2_128::Gf2_128, Field};
pub use prover::Prover;
pub use verifier::Verifier;

use crate::ole::Ole;

pub fn ghash(blocks: &[Gf2_128], prover: &mut Prover, verifier: &mut Verifier) -> Gf2_128 {
    let mut ole = Ole::default();

    prover.preprocess_ole_input(&mut ole);
    verifier.preprocess_ole_input(&mut ole);

    prover.preprocess_ole_output(&mut ole);
    verifier.preprocess_ole_output(&mut ole);

    let d1 = prover.handshake_a_open_d();
    let d2 = verifier.handshake_a_open_d();
    let d = d1 + d2;

    prover.handshake_a_set_d(d);
    verifier.handshake_a_set_d(d);

    prover.handshake_a_set_hi();
    verifier.handshake_a_set_hi();

    let ghash1 = prover.handshake_output_ghash(blocks);
    let ghash2 = verifier.handshake_output_ghash(blocks);

    ghash1 + ghash2
}

fn pascal_tri<T: Field>(n: usize) -> Vec<Vec<T>> {
    let mut pascal = vec![vec![T::one()]];

    for _ in 0..n {
        let last_row = pascal.last().unwrap();
        let mut new_row = vec![T::one()];

        last_row
            .iter()
            .map_windows(|[&a, &b]| a + b)
            .for_each(|el| {
                new_row.push(el);
            });
        new_row.push(T::one());
        pascal.push(new_row);
    }
    pascal
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpz_share_conversion_core::fields::{compute_product_repeated, p256::P256, UniformRand};
    use rand::thread_rng;

    #[test]
    fn test_ghash() {
        let mut rng = thread_rng();
        let blocks: Vec<Gf2_128> = (0..10).map(|_| Gf2_128::rand(&mut rng)).collect();

        let h1: Gf2_128 = Gf2_128::rand(&mut rng);
        let h2: Gf2_128 = Gf2_128::rand(&mut rng);
        let h = h1 + h2;

        let mut prover = Prover::new(blocks.len(), h1);
        let mut verifier = Verifier::new(blocks.len(), h2);

        let ghash = ghash(&blocks, &mut prover, &mut verifier);

        let ghash_expected = {
            let mut hi = vec![h];
            compute_product_repeated(&mut hi, h, blocks.len());

            blocks
                .iter()
                .zip(hi.iter())
                .fold(Gf2_128::zero(), |acc, (&b, &h)| acc + (b * h))
        };

        assert_eq!(ghash, ghash_expected);
    }

    #[test]
    fn test_ghash_invariants() {
        let mut rng = thread_rng();
        let blocks: Vec<Gf2_128> = (0..10).map(|_| Gf2_128::rand(&mut rng)).collect();

        let h1: Gf2_128 = Gf2_128::rand(&mut rng);
        let h2: Gf2_128 = Gf2_128::rand(&mut rng);

        let mut prover = Prover::new(blocks.len(), h1);
        let mut verifier = Verifier::new(blocks.len(), h2);

        let _ = ghash(&blocks, &mut prover, &mut verifier);

        assert_eq!(prover.h1, prover.hi[1]);
        assert_eq!(verifier.h2, verifier.hi[1]);
    }

    #[test]
    fn test_pascal_tri() {
        let pascal = pascal_tri::<P256>(4);

        let expected0 = vec![P256::one()];
        let expected1 = vec![P256::one(), P256::one()];
        let expected2 = vec![P256::one(), P256::new(2).unwrap(), P256::one()];
        let expected3 = vec![
            P256::one(),
            P256::new(3).unwrap(),
            P256::new(3).unwrap(),
            P256::one(),
        ];
        let expected4 = vec![
            P256::one(),
            P256::new(4).unwrap(),
            P256::new(6).unwrap(),
            P256::new(4).unwrap(),
            P256::one(),
        ];

        assert_eq!(pascal[0], expected0);
        assert_eq!(pascal[1], expected1);
        assert_eq!(pascal[2], expected2);
        assert_eq!(pascal[3], expected3);
        assert_eq!(pascal[4], expected4);
    }
}
