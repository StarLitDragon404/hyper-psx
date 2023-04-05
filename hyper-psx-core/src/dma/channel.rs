/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
    bus::memory::Memory,
    dma::{base_address::BaseAddress, channel_control::ChannelControl},
};

/// DMA Channel
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Channel {
    /// MADR - DMA base address
    base_address: BaseAddress,

    /// CHCR - DMA channel control
    channel_control: ChannelControl,
}

impl Memory for Channel {
    fn write_u8(&mut self, offset: u32, value: u8) {
        match offset {
            0x00..=0x03 => self.base_address.write_u8(offset, value),
            0x08..=0x0b => self.channel_control.write_u8(offset - 0x08, value),
            _ => unreachable!(
                "write to dma channel at {:#04x} with value {:#04x}",
                offset, value
            ),
        }
    }

    fn read_u8(&self, offset: u32) -> u8 {
        match offset {
            0x00..=0x03 => self.base_address.read_u8(offset),
            0x08..=0x0b => self.channel_control.read_u8(offset - 0x08),
            _ => unreachable!("read from dma channel at {:#04x}", offset),
        }
    }
}
