#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;

pub(crate) union UnionCast {
    pub m128: __m128,
    pub m128i: __m128i,
    pub f32x4: [f32; 4],
    pub i32x4: [i32; 4],
    pub u32x4: [u32; 4],
}
