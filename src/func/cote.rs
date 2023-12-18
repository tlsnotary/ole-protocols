//! This module implements the COTE functionality (page 5) from <https://eprint.iacr.org/2015/546>

#[derive(Debug)]
pub struct Cote {
    kappa: usize,
    l: usize,
}
