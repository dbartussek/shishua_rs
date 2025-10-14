use wide::{u32x8, u64x4};

#[doc(hidden)]
#[macro_export]
macro_rules! simd_swizzle {
    ($simd:expr, $shuffle:expr) => {
        $shuffle.map(|i| $simd.to_array()[i])
    };
}

pub trait WideSupport {
    const LEN: usize;
}
impl WideSupport for u32x8 {
    const LEN: usize = Self::LANES as usize;
}
impl WideSupport for u64x4 {
    const LEN: usize = Self::LANES as usize;
}
