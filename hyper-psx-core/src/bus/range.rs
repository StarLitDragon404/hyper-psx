/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy)]
pub(super) struct Range {
    start: u32,
    length: u32,
}

impl Range {
    pub(super) const fn new(start: u32, length: u32) -> Self {
        Self { start, length }
    }

    pub(super) fn contains(&self, address: u32) -> Option<u32> {
        if address >= self.start && address < self.start + self.length {
            Some(address - self.start)
        } else {
            None
        }
    }
}

impl Display for Range {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let end = self.start + self.length;
        write!(fmt, "{:#010x}..{:#010x}", self.start, end)
    }
}

impl Debug for Range {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Range")
            .field("start", &format_args!("{:#010x}", self.start))
            .field("length", &format_args!("{:#010x}", self.length))
            .finish()
    }
}
