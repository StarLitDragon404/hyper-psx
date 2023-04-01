/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

/// The `ExtensionExt` trait allows for extending an integer
pub(super) trait ExtensionExt {
    type Target;

    /// Extends the integer with zeros infront
    fn zero_extend(self) -> Self::Target;

    /// Extends the integer with a sign infront
    fn sign_extend(self) -> Self::Target;
}

impl ExtensionExt for u16 {
    type Target = u32;

    fn zero_extend(self) -> Self::Target {
        self as Self::Target
    }

    fn sign_extend(self) -> Self::Target {
        self as i16 as Self::Target
    }
}
