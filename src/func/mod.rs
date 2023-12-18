//! This module implements some functionalities.

pub mod cot;
pub mod ole;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Sender,
    Receiver,
}
