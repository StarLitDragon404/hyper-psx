/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

pub trait Memory {
    fn write_u8(&mut self, offset: u32, value: u8);

    fn read_u8(&self, offset: u32) -> u8;
}
