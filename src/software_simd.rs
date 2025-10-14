//! A __VERY__ basic std::simd replacement to compile on stable

#![allow(non_camel_case_types)]

use bytemuck::{Pod, Zeroable};
use core::{
    num::Wrapping,
    ops::{Add, AddAssign, BitXor, Deref, Shr},
};

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct SoftwareSimd<T, const N: usize>([T; N]);

pub type u64x4 = SoftwareSimd<u64, 4>;
pub type u32x8 = SoftwareSimd<u32, 8>;

unsafe impl<T, const N: usize> Pod for SoftwareSimd<T, N> where T: Pod {}
unsafe impl<T, const N: usize> Zeroable for SoftwareSimd<T, N> where T: Zeroable {}

impl<T, const N: usize> SoftwareSimd<T, N>
where
    T: Copy,
{
    pub const LEN: usize = N;

    pub fn splat(value: T) -> Self {
        Self([value; N])
    }

    pub fn from_array(array: [T; N]) -> Self {
        Self(array)
    }
    
    pub fn to_array(self) -> [T; N] {
        self.0
    }
}

impl<T, const N: usize> Deref for SoftwareSimd<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> From<[T; N]> for SoftwareSimd<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}


impl<T, const N: usize> Add for SoftwareSimd<T, N>
where
    T: Copy,
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (i, it) in self.0.iter_mut().enumerate() {
            *it = (Wrapping(*it) + Wrapping(rhs.0[i])).0;
        }

        self
    }
}
impl<T, const N: usize> AddAssign for SoftwareSimd<T, N>
where
    T: Copy,
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl<T, const N: usize> BitXor for SoftwareSimd<T, N>
where
    T: Copy + BitXor<Output = T>,
{
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        for (i, it) in self.0.iter_mut().enumerate() {
            *it = *it ^ rhs.0[i];
        }

        self
    }
}

impl<T, const N: usize> Shr<usize> for SoftwareSimd<T, N>
where
    T: Copy + Shr<usize, Output = T>,
{
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        for it in self.0.iter_mut() {
            *it = *it >> rhs;
        }

        self
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! simd_swizzle {
    ($simd:expr, $shuffle:expr) => {
        SoftwareSimd::from_array($shuffle.map(|i| $simd[i]))
    };
}
