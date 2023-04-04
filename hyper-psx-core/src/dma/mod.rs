/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bus::memory::Memory;

/// Direct Memory Access Component
#[derive(Clone, Debug)]
pub(crate) struct Dma {
    /// DPCR - Control register
    control: u32,

    /// DICR - Interrupt register
    interrupt: u32,
}

impl Dma {
    pub(crate) fn new() -> Self {
        Self {
            control: 0x07654321,
            interrupt: 0,
        }
    }
}

impl Memory for Dma {
    fn write_u8(&mut self, offset: u32, value: u8) {
        match offset {
            0x70..=0x73 => self.control.write_u8(offset - 0x70, value),
            0x74..=0x77 => self.interrupt.write_u8(offset - 0x74, value),
            _ => todo!(
                "write to DMA at {:#010x} ({:#010x}) with value {:#04x}",
                offset,
                0x1f801080 + offset,
                value
            ),
        }
    }

    fn read_u8(&self, offset: u32) -> u8 {
        return match offset {
            0x70..=0x73 => self.control.read_u8(offset - 0x70),
            0x74..=0x77 => self.interrupt.read_u8(offset - 0x74),
            _ => todo!(
                "read from DMA at {:#010x} ({:#010x})",
                offset,
                0x1f801080 + offset
            ),
        };
    }
}
