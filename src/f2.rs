use itybity::{BitLength, FromBitIterator, GetBit, Lsb0, Msb0};
use mpz_share_conversion_core::fields::Field;
use rand::distributions::{Distribution, Standard};
use std::ops::{Add, Mul, Neg};

/// A simple boolean field type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct F2 {
    inner: u8,
}

impl F2 {
    /// Create a new `F2` from a `bool`.
    ///
    /// `False` encodes 0 and `true` encodes 1.
    pub fn new(value: bool) -> Self {
        Self { inner: value as u8 }
    }
}

impl Field for F2 {
    const BIT_SIZE: u32 = 1;

    fn zero() -> Self {
        Self::new(false)
    }

    fn one() -> Self {
        Self::new(true)
    }

    fn two_pow(_rhs: u32) -> Self {
        unimplemented!()
    }

    fn inverse(self) -> Self {
        if self.inner == 0 {
            panic!("No inverse for 0")
        }
        Self::one()
    }

    fn to_le_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn to_be_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }
}

impl Distribution<F2> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> F2 {
        F2::new(rng.gen())
    }
}

impl Add for F2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.inner ^ rhs.inner) != 0)
    }
}

impl Mul for F2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.inner & rhs.inner != 0)
    }
}

impl Neg for F2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl BitLength for F2 {
    const BITS: usize = 1;
}

impl GetBit<Lsb0> for F2 {
    fn get_bit(&self, _index: usize) -> bool {
        unimplemented!()
    }
}

impl GetBit<Msb0> for F2 {
    fn get_bit(&self, _index: usize) -> bool {
        unimplemented!()
    }
}

impl FromBitIterator for F2 {
    fn from_lsb0_iter(_iter: impl IntoIterator<Item = bool>) -> Self {
        unimplemented!()
    }

    fn from_msb0_iter(_iter: impl IntoIterator<Item = bool>) -> Self {
        unimplemented!()
    }
}
