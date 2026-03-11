use core::{convert::TryInto, num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize}};

use crate::Idx;

macro_rules! impl_idx_for_uint_type {
    {$t: ty} => {
        impl Idx for $t {
            fn from_usize(idx: usize) -> Self {
                idx.try_into().unwrap()
            }

            fn index(self) -> usize {
                self.try_into().unwrap()
            }
        }
    };
}

impl_idx_for_uint_type!{u8}
impl_idx_for_uint_type!{u16}
impl_idx_for_uint_type!{u32}
impl_idx_for_uint_type!{u64}
impl_idx_for_uint_type!{usize}

macro_rules! impl_idx_for_non_zero_uint_type {
    {$t: ty} => {
        impl Idx for $t {
            fn from_usize(idx: usize) -> Self {
                <$t>::new(idx.checked_add(1).unwrap().try_into().unwrap()).unwrap()
            }

            fn index(self) -> usize {
                // SAFETY: the value is non-zero so subtracting 1 can't underflow
                unsafe { self.get().unchecked_sub(1).try_into().unwrap() }
            }
        }
    };
}
impl_idx_for_non_zero_uint_type!{NonZeroU8}
impl_idx_for_non_zero_uint_type!{NonZeroU16}
impl_idx_for_non_zero_uint_type!{NonZeroU32}
impl_idx_for_non_zero_uint_type!{NonZeroU64}
impl_idx_for_non_zero_uint_type!{NonZeroUsize}
