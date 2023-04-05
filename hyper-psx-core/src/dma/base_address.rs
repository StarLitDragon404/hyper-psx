/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bus::memory::Memory;

/// DMA base address
///
/// <https://psx-spx.consoledev.net/dmachannels/#1f801080hn10h-d_madr-dma-base-address-channel-06-rw>
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct BaseAddress {
    // Base memory address
    memory_address: u32,
}

impl Memory for BaseAddress {
    fn write_u8(&mut self, offset: u32, value: u8) {
        // Making sure only bits 0-23 are writeable and 24-31 are always zero
        match offset {
            0x00..=0x02 => self.memory_address.write_u8(offset, value),
            0x03 => {}
            _ => unreachable!(
                "write to dma base address at {:#04x} with value {:#04x}",
                offset, value
            ),
        }
    }

    fn read_u8(&self, offset: u32) -> u8 {
        // Making sure only bits 0-23 are readable and 24-31 are always zero
        match offset {
            0x00..=0x02 => self.memory_address.read_u8(offset),
            0x03 => 0x00,
            _ => unreachable!("read from dma base address at {:#04x}", offset),
        }
    }
}
