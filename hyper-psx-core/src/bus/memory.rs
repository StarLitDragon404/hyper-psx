/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

/// The `Memory` trait allows for writing and reading of bytes
pub trait Memory {
    /// Allows writing bytes at a relative offset
    ///
    /// # Arguments:
    ///
    /// * `offset`: The relative address offset
    /// * `value`: The value to be written
    ///
    /// # Panics
    ///
    /// The function should panic if the given offset is out of range
    fn write_u8(&mut self, offset: u32, value: u8);

    /// Allows reading bytes from a relative offset
    ///
    /// # Arguments:
    ///
    /// * `offset`: The relative address offset
    ///
    /// # Panics
    ///
    /// The function should panic if the given offset is out of range
    fn read_u8(&self, offset: u32) -> u8;
}

impl Memory for u16 {
    fn write_u8(&mut self, offset: u32, value: u8) {
        assert!(offset < 2);
        let offset = offset * 8;
        *self = (*self & !(0xff << offset)) | ((value as u16) << offset);
    }

    fn read_u8(&self, offset: u32) -> u8 {
        assert!(offset < 2);
        let offset = offset * 8;
        ((self >> offset) & 0xff) as u8
    }
}

impl Memory for u32 {
    fn write_u8(&mut self, offset: u32, value: u8) {
        assert!(offset < 4);
        let offset = offset * 8;
        *self = (*self & !(0xff << offset)) | ((value as u32) << offset);
    }

    fn read_u8(&self, offset: u32) -> u8 {
        assert!(offset < 4);
        let offset = offset * 8;
        ((self >> offset) & 0xff) as u8
    }
}
