#![cfg_attr(feature = "nightly", feature(portable_simd))]
#![no_std]

pub(crate) mod core;
#[cfg(feature = "rand")]
pub(crate) mod rand;

#[cfg(not(feature = "nightly"))]
mod software_simd;

pub use crate::core::ShiShuAState;
#[cfg(feature = "rand")]
pub use crate::rand::ShiShuARng;
