/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bus::memory::Memory;

/// The RAM component
#[derive(Clone, Debug)]
pub(crate) struct Ram {
    /// The data vector containing the RAM
    data: Box<[u8; Self::SIZE]>,
}

impl Ram {
    const SIZE: usize = 1024 * (2 * 1024);

    /// Creates a RAM Component
    pub(crate) fn new() -> Self {
        let buffer = vec![0x00; Self::SIZE]
            .into_boxed_slice()
            .try_into()
            .unwrap();

        Self { data: buffer }
    }
}

impl Memory for Ram {
    fn write_u8(&mut self, offset: u32, value: u8) {
        debug_assert!((offset as usize) < self.data.len());

        self.data[offset as usize] = value;
    }

    fn read_u8(&self, offset: u32) -> u8 {
        debug_assert!((offset as usize) < self.data.len());

        self.data[offset as usize]
    }
}
