use core::array;

use core_simd::simd::prelude::*;
use core_simd::simd::{LaneCount, SupportedLaneCount};

pub trait ShiftElementsDyn {
    #[must_use]
    #[expect(dead_code)]
    fn shift_elements_left_dyn(self, count: usize) -> Self;

    #[must_use]
    fn shift_elements_right_dyn(self, count: usize) -> Self;
}

impl<const N: usize> ShiftElementsDyn for Simd<u8, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn shift_elements_left_dyn(self, count: usize) -> Self {
        let indices = Simd::from_array(array::from_fn(|x| x as u8));
        self.swizzle_dyn(indices - Simd::splat((count % N) as u8))
    }

    #[inline]
    fn shift_elements_right_dyn(self, count: usize) -> Self {
        let indices = Simd::from_array(array::from_fn(|x| x as u8));
        self.swizzle_dyn(indices + Simd::splat((count % N) as u8))
    }
}
