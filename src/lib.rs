use itybity::ToBits;
use mpz_share_conversion_core::fields::gf2_128::Gf2_128;
use mpz_share_conversion_core::Field;
use rand::Rng;

#[test]
fn test() {
    let a: Gf2_128 = rand::thread_rng().gen();
    let b: Gf2_128 = rand::thread_rng().gen();

    let mut ts = Vec::new();
    let mut ys = Vec::new();

    for b_i in b.iter_lsb0() {
        let ((t_0, t_1), t_b) = rot(b_i);
        let u_i = t_0 + -t_1 + a;

        let y_i = if b_i { t_b + u_i } else { t_b };

        assert_eq!(y_i, if b_i { t_0 + a } else { t_0 });

        ts.push(t_0);
        ys.push(y_i);
    }

    let x = ts
        .into_iter()
        .enumerate()
        .fold(Gf2_128::zero(), |acc, (i, t_i)| {
            acc + (t_i * Gf2_128::two_pow(i as u32))
        });
    let y = -ys
        .into_iter()
        .enumerate()
        .fold(Gf2_128::zero(), |acc, (i, y_i)| {
            acc + (y_i * Gf2_128::two_pow(i as u32))
        });

    assert_eq!(x + y, a * b);
}

pub fn rot(x: bool) -> ((Gf2_128, Gf2_128), Gf2_128) {
    let t_0: Gf2_128 = rand::thread_rng().gen();
    let t_1: Gf2_128 = rand::thread_rng().gen();
    let t_x = if x { t_1 } else { t_0 };

    ((t_0, t_1), t_x)
}

pub fn get_bit(a: u64, idx: usize) -> bool {
    (a >> idx) & 1 == 1
}
