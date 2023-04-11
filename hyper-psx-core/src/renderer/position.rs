/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

/// Represents a position
#[derive(Clone, Copy, Debug)]
pub(crate) struct Position {
    x: i16,
    y: i16,
}

impl Position {
    /// Creates a position from a u32
    pub(crate) fn from_word(word: u32) -> Self {
        let x = (word & 0xffff) as i16;
        let y = ((word >> 16) & 0xffff) as i16;

        Self { x, y }
    }

    /// Returns the x position
    #[inline(always)]
    pub(crate) fn x(&self) -> i16 {
        self.x
    }

    /// Returns the y position
    #[inline(always)]
    pub(crate) fn y(&self) -> i16 {
        self.y
    }
}
