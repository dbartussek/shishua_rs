use wide::{u32x8, u64x4};

pub trait WideSupport {
    const LEN: usize;
}
impl WideSupport for u32x8 {
    const LEN: usize = Self::LANES as usize;
}
impl WideSupport for u64x4 {
    const LEN: usize = Self::LANES as usize;
}
