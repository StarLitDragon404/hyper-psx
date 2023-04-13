/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

/// A trait for zero-extending an unsigned integer
pub(crate) trait ZextExt {
    /// Type of the bigger unsigned integer
    type Target;

    /// Casts the unsigned integer to a bigger size and fills the higher bits with 0
    fn zero_extend(self) -> Self::Target;
}

impl ZextExt for u8 {
    type Target = u16;

    fn zero_extend(self) -> Self::Target {
        self as Self::Target
    }
}

impl ZextExt for u16 {
    type Target = u32;

    fn zero_extend(self) -> Self::Target {
        self as Self::Target
    }
}

impl ZextExt for u32 {
    type Target = u32;

    fn zero_extend(self) -> Self::Target {
        self as Self::Target
    }
}
