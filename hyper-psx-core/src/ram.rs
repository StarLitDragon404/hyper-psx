/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::memory::Memory;

/// The RAM component
#[derive(Clone, Debug)]
pub(crate) struct Ram {
    /// The data vector containing the RAM
    data: Vec<u8>,
}

impl Ram {
    /// Creates a RAM Component
    pub(crate) fn new() -> Self {
        const SIZE: usize = 1024 * (2 * 1024);

        let buffer = vec![0x00; SIZE];

        Self { data: buffer }
    }
}

impl Memory for Ram {
    fn write_u8(&mut self, offset: u32, value: u8) {
        assert!((offset as usize) < self.data.len());

        self.data[offset as usize] = value;
    }

    fn read_u8(&self, offset: u32) -> u8 {
        assert!((offset as usize) < self.data.len());

        self.data[offset as usize]
    }
}
