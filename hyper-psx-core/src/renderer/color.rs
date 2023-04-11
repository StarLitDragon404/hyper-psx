/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

/// Represents a color
#[derive(Clone, Copy, Debug)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Color {
    /// Creates a color from a u32
    pub(crate) fn from_word(word: u32) -> Self {
        let r = (word & 0xff) as u8;
        let g = ((word >> 8) & 0xff) as u8;
        let b = ((word >> 16) & 0xff) as u8;

        Self { r, g, b }
    }

    /// Returns the r value
    #[inline(always)]
    pub(crate) fn r(&self) -> u8 {
        self.r
    }

    /// Returns the g value
    #[inline(always)]
    pub(crate) fn g(&self) -> u8 {
        self.g
    }

    /// Returns the b value
    #[inline(always)]
    pub(crate) fn b(&self) -> u8 {
        self.b
    }
}
