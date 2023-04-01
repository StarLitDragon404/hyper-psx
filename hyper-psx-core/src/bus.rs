/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{bios::Bios, memory::Memory};

#[derive(Clone, Debug)]
pub(crate) struct Bus {
    bios: Bios,
}

impl Bus {
    pub(crate) fn new(bios: Bios) -> Self {
        Self { bios }
    }

    pub(crate) fn read_u8(&self, address: u32) -> u8 {
        let short_address = address & 0x0fffffff;

        if short_address >= 0x0fc00000 && short_address < 0x0fc00000 + (512 * 1024) {
            let offset = short_address - 0x0fc00000;
            return self.bios.read_u8(offset);
        }

        panic!("access violation at address: {:#010x}", address);
    }

    pub(crate) fn read_u16(&self, address: u32) -> u16 {
        let byte_0 = self.read_u8(address + 0) as u16;
        let byte_1 = self.read_u8(address + 1) as u16;

        (byte_1 << 8) | (byte_0 << 0)
    }

    pub(crate) fn read_u32(&self, address: u32) -> u32 {
        let byte_0 = self.read_u8(address + 0) as u32;
        let byte_1 = self.read_u8(address + 1) as u32;
        let byte_2 = self.read_u8(address + 2) as u32;
        let byte_3 = self.read_u8(address + 3) as u32;

        (byte_3 << 24) | (byte_2 << 16) | (byte_1 << 8) | (byte_0 << 0)
    }
}
