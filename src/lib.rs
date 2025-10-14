#![cfg_attr(feature = "nightly", feature(portable_simd))]

pub(crate) mod core;
pub(crate) mod rand;

#[cfg(not(feature = "nightly"))]
mod software_simd;

pub use crate::{core::ShiShuAState, rand::ShiShuARng};
