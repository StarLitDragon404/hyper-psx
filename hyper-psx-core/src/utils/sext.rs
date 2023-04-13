/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

/// A trait for sign-extending an unsigned integer
pub(crate) trait SextExt {
    /// Type of the bigger unsigned integer
    type Target;

    /// Casts the unsigned integer to the signed counter part then to a bigger size and
    /// fills the higher bits with the sign bit
    fn sign_extend(self) -> Self::Target;
}

impl SextExt for u8 {
    type Target = u16;

    fn sign_extend(self) -> Self::Target {
        self as i8 as Self::Target
    }
}

impl SextExt for u16 {
    type Target = u32;

    fn sign_extend(self) -> Self::Target {
        self as i16 as Self::Target
    }
}

impl SextExt for u32 {
    type Target = u32;

    fn sign_extend(self) -> Self::Target {
        self as i32 as Self::Target
    }
}
