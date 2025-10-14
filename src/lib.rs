#![feature(portable_simd)]

pub(crate) mod core;
pub(crate) mod rand;

pub use crate::{core::ShiShuAState, rand::ShiShuARng};
