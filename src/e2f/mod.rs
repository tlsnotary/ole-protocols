//! This module is a testing ground for the E2F protocol (page 33) from <https://eprint.iacr.org/2023/964>

mod prover;
mod verifier;

pub use prover::Prover;
pub use verifier::Verifier;

#[cfg(test)]
mod tests {
    use super::{Prover, Verifier};
    use crate::ole::Ole;
    use mpz_share_conversion_core::{
        fields::{p256::P256, UniformRand},
        Field,
    };
    use rand::thread_rng;

    #[test]
    fn test_e2f() {
        // Initialize
        let mut rng = thread_rng();
        let prover_ec = (P256::rand(&mut rng), P256::rand(&mut rng));
        let verifier_ec = (P256::rand(&mut rng), P256::rand(&mut rng));

        let mut ole = Ole::default();
        let mut prover = Prover::default();
        let mut verifier = Verifier::default();

        // Preprocessing
        prover.preprocess1();
        verifier.preprocess1();

        prover.preprocess2_ole_input(&mut ole);
        verifier.preprocess2_ole_input(&mut ole);

        prover.preprocess2_ole_output(&mut ole);
        verifier.preprocess2_ole_output(&mut ole);

        prover.preprocess3();
        verifier.preprocess3();

        prover.preprocess4();
        verifier.preprocess4();

        // Handshake
        prover.handshake5_input_ec(prover_ec);
        verifier.handshake5_input_ec(verifier_ec);

        let varespilon1_share_prover = prover.handshake5_varepsilon1_share_open();
        let varespilon1_share_verifier = verifier.handshake5_varepsilon1_share_open();
        let varepsilon1 = varespilon1_share_prover + varespilon1_share_verifier;

        prover.handshake5_set_omega(varepsilon1);
        verifier.handshake5_set_omega(varepsilon1);

        let omega_share_prover = prover.handshake6_omega_share_open();
        let omega_share_verifier = verifier.handshake6_omega_share_open();
        let omega = omega_share_prover + omega_share_verifier;

        let varespilon2_share_prover = prover.handshake6_varepsilon2_share_open();
        let varespilon2_share_verifier = verifier.handshake6_varepsilon2_share_open();
        let var_epsilon2 = varespilon2_share_prover + varespilon2_share_verifier;

        prover.handshake6_set_eta(omega, var_epsilon2);
        verifier.handshake6_set_eta(omega, var_epsilon2);

        let varepsilon3_share_prover = prover.handshake7_varepsilon3_share_open();
        let varepsilon3_share_verifier = verifier.handshake7_varepsilon3_share_open();
        let varepsilon3 = varepsilon3_share_prover + varepsilon3_share_verifier;

        prover.handshake7_set_z1(varepsilon3);
        verifier.handshake7_set_z2(varepsilon3);

        // Output
        let z1 = prover.handshake8_z1_open();
        let z2 = verifier.handshake8_z2_open();

        let x_ec_combined = z1 + z2;
        let x_ec_expected = {
            let nominator = prover_ec.1 + -verifier_ec.1;
            let denominator = prover_ec.0 + -verifier_ec.0;

            let fraction = nominator * denominator.inverse();
            let squared = fraction * fraction;

            squared + -prover_ec.0 + -verifier_ec.0
        };

        assert_eq!(x_ec_combined, x_ec_expected);
    }
}
