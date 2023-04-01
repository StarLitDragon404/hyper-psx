/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{bios::Bios, memory::Memory};

/// The BUS component connecting everything
#[derive(Clone, Debug)]
pub(crate) struct Bus {
    /// The BIOS component
    bios: Bios,
}

impl Bus {
    /// Creates a Bus Component
    ///
    /// # Arguments:
    ///
    /// * `bios`: The loaded BIOS
    pub(crate) fn new(bios: Bios) -> Self {
        Self { bios }
    }

    /// Reads an u8 from a specific address
    ///
    /// # Arguments:
    ///
    /// * `address`: The absolute address
    ///
    /// # Panics:
    ///
    /// This functions panics if the address is not valid
    pub(crate) fn write_u8(&mut self, address: u32, value: u8) {
        let short_address = if address < 0x80000000 {
            address
        } else if address < 0xa0000000 {
            address - 0x80000000
        } else {
            address - 0xa0000000
        };

        if short_address >= 0x1f801000 && short_address < 0x1f801000 + 32 {
            let offset = short_address - 0x1f801000;
            log::warn!(
                "Unhandled write to Memory Control 1: {:#010x} ({:#x})",
                address,
                offset
            );
            return;
        }

        if short_address >= 0x1f801060 && short_address < 0x1f801060 + 16 {
            let offset = short_address - 0x1f801060;
            log::warn!(
                "Unhandled write to Memory Control byte_2: {:#010x} ({:#x})",
                address,
                offset
            );
            return;
        }

        panic!("access write violation at address: {:#010x}", address);
    }

    /// Reads an u16 from a specific address
    ///
    /// # Arguments:
    ///
    /// * `address`: The absolute address
    ///
    /// # Panics
    ///
    /// This functions panics if the address is not aligned to 16-bits
    pub(crate) fn write_u16(&mut self, address: u32, value: u16) {
        if address % 2 != 0 {
            panic!("unaligned write access at {:#010x}", address);
        }

        let byte_0 = (value & 0xff) as u8;
        let byte_1 = ((value >> 8) & 0xff) as u8;

        self.write_u8(address + 0, byte_0);
        self.write_u8(address + 1, byte_1);
    }

    /// Reads an u32 from a specific address
    ///
    /// # Arguments:
    ///
    /// * `address`: The absolute address
    ///
    /// # Panics
    ///
    /// This functions panics if the address is not aligned to 16-bits
    pub(crate) fn write_u32(&mut self, address: u32, value: u32) {
        if address % 4 != 0 {
            panic!("unaligned write access at {:#010x}", address);
        }

        let byte_0 = (value & 0xff) as u8;
        let byte_1 = ((value >> 8) & 0xff) as u8;
        let byte_2 = ((value >> 16) & 0xff) as u8;
        let byte_3 = ((value >> 24) & 0xff) as u8;

        self.write_u8(address + 0, byte_0);
        self.write_u8(address + 1, byte_1);
        self.write_u8(address + 2, byte_2);
        self.write_u8(address + 3, byte_3);
    }

    /// Reads an u8 from a specific address
    ///
    /// # Arguments:
    ///
    /// * `address`: The absolute address
    ///
    /// # Panics:
    ///
    /// This functions panics if the address is not valid
    pub(crate) fn read_u8(&self, address: u32) -> u8 {
        let short_address = if address < 0x80000000 {
            address
        } else if address < 0xa0000000 {
            address - 0x80000000
        } else {
            address - 0xa0000000
        };

        if short_address >= 0x1fc00000 && short_address < 0x1fc00000 + (512 * 1024) {
            let offset = short_address - 0x1fc00000;
            return self.bios.read_u8(offset);
        }

        panic!("access read violation at address: {:#010x}", address);
    }

    /// Reads an u16 from a specific address
    ///
    /// # Arguments:
    ///
    /// * `address`: The absolute address
    ///
    /// # Panics
    ///
    /// This functions panics if the address is not aligned to 16-bits
    pub(crate) fn read_u16(&self, address: u32) -> u16 {
        if address % 2 != 0 {
            panic!("unaligned read access at {:#010x}", address);
        }

        let byte_0 = self.read_u8(address + 0) as u16;
        let byte_1 = self.read_u8(address + 1) as u16;

        (byte_1 << 8) | (byte_0 << 0)
    }

    /// Reads an u32 from a specific address
    ///
    /// # Arguments:
    ///
    /// * `address`: The absolute address
    ///
    /// # Panics
    ///
    /// This functions panics if the address is not aligned to 32-bits
    pub(crate) fn read_u32(&self, address: u32) -> u32 {
        if address % 4 != 0 {
            panic!("unaligned read access at {:#010x}", address);
        }

        let byte_0 = self.read_u8(address + 0) as u32;
        let byte_1 = self.read_u8(address + 1) as u32;
        let byte_2 = self.read_u8(address + 2) as u32;
        let byte_3 = self.read_u8(address + 3) as u32;

        (byte_3 << 24) | (byte_2 << 16) | (byte_1 << 8) | (byte_0 << 0)
    }
}
