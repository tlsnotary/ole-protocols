//! This module is a testing ground for the E2F protocol (page 33) from <https://eprint.iacr.org/2023/964>

mod prover;
mod verifier;

use crate::ole::Ole;
use mpz_share_conversion_core::fields::p256::P256;
pub use prover::Prover;
pub use verifier::Verifier;

/// Returns the x-coordinate shares of the sum of the two EC points
pub fn e2f(
    prover_point: (P256, P256),
    prover: &mut Prover,
    verifier_point: (P256, P256),
    verifier: &mut Verifier,
) -> (P256, P256) {
    let mut ole = Ole::default();

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
    prover.handshake5_input_ec(prover_point);
    verifier.handshake5_input_ec(verifier_point);

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
    let varepsilon2 = varespilon2_share_prover + varespilon2_share_verifier;

    prover.handshake6_set_eta(omega, varepsilon2);
    verifier.handshake6_set_eta(omega, varepsilon2);

    let varepsilon3_share_prover = prover.handshake7_varepsilon3_share_open();
    let varepsilon3_share_verifier = verifier.handshake7_varepsilon3_share_open();
    let varepsilon3 = varepsilon3_share_prover + varepsilon3_share_verifier;

    prover.handshake7_set_z1(varepsilon3);
    verifier.handshake7_set_z2(varepsilon3);

    // Output
    let z1 = prover.handshake8_z1_open();
    let z2 = verifier.handshake8_z2_open();

    (z1, z2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpz_share_conversion_core::Field;
    use p256::{elliptic_curve::sec1::ToEncodedPoint, EncodedPoint, NonZeroScalar, PublicKey};
    use rand::thread_rng;

    #[test]
    fn test_e2f() {
        let mut rng = thread_rng();
        let prover_scalar = NonZeroScalar::random(&mut rng);
        let verifier_scalar = NonZeroScalar::random(&mut rng);

        let prover_ec = point_to_p256(scalar_to_encoded_point(prover_scalar));
        let mut prover = Prover::default();

        let verifier_ec = point_to_p256(scalar_to_encoded_point(verifier_scalar));
        let mut verifier = Verifier::default();

        let (z1, z2) = e2f(prover_ec, &mut prover, verifier_ec, &mut verifier);

        let x_ec_expected = add_ec_points(prover_ec, verifier_ec);
        assert_eq!(z1 + z2, x_ec_expected.0);
    }

    #[test]
    fn test_sharing_sums() {
        let mut rng = thread_rng();
        let prover_scalar = NonZeroScalar::random(&mut rng);
        let verifier_scalar = NonZeroScalar::random(&mut rng);

        let prover_ec = point_to_p256(scalar_to_encoded_point(prover_scalar));
        let mut prover = Prover::default();

        let verifier_ec = point_to_p256(scalar_to_encoded_point(verifier_scalar));
        let mut verifier = Verifier::default();

        let _ = e2f(prover_ec, &mut prover, verifier_ec, &mut verifier);

        // Assertions

        // c
        let c = prover.a1.unwrap() * prover.b1.unwrap()
            + prover.a1_b2_share.unwrap()
            + prover.a2_b1_share.unwrap()
            + verifier.a2_b1_share.unwrap()
            + verifier.a1_b2_share.unwrap()
            + verifier.a2.unwrap() * verifier.b2.unwrap();
        assert_eq!(prover.c1.unwrap() + verifier.c2.unwrap(), c);

        // c_prime
        let c_prime = prover.a1.unwrap() * prover.b1_prime.unwrap()
            + prover.a1_b2_prime_share.unwrap()
            + prover.a2_b1_prime_share.unwrap()
            + verifier.a2_b1_prime_share.unwrap()
            + verifier.a1_b2_prime_share.unwrap()
            + verifier.a2.unwrap() * verifier.b2_prime.unwrap();
        assert_eq!(
            prover.c1_prime.unwrap() + verifier.c2_prime.unwrap(),
            c_prime
        );

        // r_squared
        let r_squared = prover.r1.unwrap() * prover.r1.unwrap()
            + P256::new(2).unwrap() * (prover.r1_r2_share.unwrap() + verifier.r1_r2_share.unwrap())
            + verifier.r2.unwrap() * verifier.r2.unwrap();
        assert_eq!(
            prover.r_squared_share.unwrap() + verifier.r_squared_share.unwrap(),
            r_squared
        );

        // omega
        let b = prover.b1.unwrap() + verifier.b2.unwrap();
        let varepsilon1 = -prover.ec_point.unwrap().0 + verifier.ec_point.unwrap().0 + -b;
        let omega = varepsilon1 * (prover.a1.unwrap() + verifier.a2.unwrap())
            + prover.c1.unwrap()
            + verifier.c2.unwrap();
        assert_eq!(
            prover.omega_share.unwrap() + verifier.omega_share.unwrap(),
            omega
        );

        // eta
        let b_prime = prover.b1_prime.unwrap() + verifier.b2_prime.unwrap();
        let varepsilon2 = -prover.ec_point.unwrap().1 + verifier.ec_point.unwrap().1 + -b_prime;
        let eta = (prover.omega_share.unwrap() + verifier.omega_share.unwrap()).inverse()
            * (varepsilon2 * (prover.a1.unwrap() + verifier.a2.unwrap())
                + prover.c1_prime.unwrap()
                + verifier.c2_prime.unwrap());
        assert_eq!(prover.eta_share.unwrap() + verifier.eta_share.unwrap(), eta);

        // z
        let varepsilon3 = prover.eta_share.unwrap()
            + verifier.eta_share.unwrap()
            + -prover.r1.unwrap()
            + -verifier.r2.unwrap();
        let z = varepsilon3 * varepsilon3
            + P256::new(2).unwrap() * varepsilon3 * (prover.r1.unwrap() + verifier.r2.unwrap())
            + prover.r_squared_share.unwrap()
            + verifier.r_squared_share.unwrap()
            + -prover.ec_point.unwrap().0
            + -verifier.ec_point.unwrap().0;
        assert_eq!(prover.z1.unwrap() + verifier.z2.unwrap(), z);
    }

    #[test]
    fn test_add_ec_points() {
        let mut rng = thread_rng();
        let scalar1 = p256::NonZeroScalar::random(&mut rng);
        let scalar2 = p256::NonZeroScalar::random(&mut rng);

        let pk1 = PublicKey::from_secret_scalar(&scalar1);
        let pk2 = PublicKey::from_secret_scalar(&scalar2);
        let pr1 = pk1.to_projective();
        let pr2 = pk2.to_projective();

        let ec_added_expected = point_to_p256((pr1 + pr2).to_affine().to_encoded_point(false));

        let ec1 = pr1.to_affine().to_encoded_point(false);
        let ec2 = pr2.to_affine().to_encoded_point(false);
        let ec_added = add_ec_points(point_to_p256(ec1), point_to_p256(ec2));

        assert_eq!(ec_added, ec_added_expected);
    }

    fn add_ec_points((x1, y1): (P256, P256), (x2, y2): (P256, P256)) -> (P256, P256) {
        let nominator = y2 + -y1;
        let denominator = x2 + -x1;

        let fraction = nominator * denominator.inverse();
        let squared = fraction * fraction;

        let x_r = squared + -x1 + -x2;
        let y_r = fraction * (x1 + -x_r) + -y1;

        (x_r, y_r)
    }

    fn scalar_to_encoded_point(scalar: NonZeroScalar) -> EncodedPoint {
        PublicKey::from_secret_scalar(&scalar).to_encoded_point(false)
    }

    fn point_to_p256(point: EncodedPoint) -> (P256, P256) {
        let mut x: [u8; 32] = (*point.x().unwrap()).into();
        let mut y: [u8; 32] = (*point.y().unwrap()).into();

        // reverse to little endian
        x.reverse();
        y.reverse();

        let x = P256::try_from(x).unwrap();
        let y = P256::try_from(y).unwrap();

        (x, y)
    }
}
