/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

pub(crate) mod channel;

use crate::{
    bus::{memory::Memory, ram::Ram},
    dma::channel::{Channel, Id},
    gpu::Gpu,
};

/// Direct Memory Access Component
#[derive(Clone, Debug)]
pub(crate) struct Dma {
    // TODO: Replace registers with individual fields
    /// DPCR - Control register
    control: u32,

    /// DICR - Interrupt register
    interrupt: u32,

    /// DMA0-DMA6 - Channels
    channels: [Channel; 7],
}

impl Dma {
    /// Creates a DMA Component
    pub(crate) fn new() -> Self {
        let channels = [
            Channel::new(Id::MacroBlockIn),
            Channel::new(Id::MacroBlockOut),
            Channel::new(Id::Gpu),
            Channel::new(Id::Cdrom),
            Channel::new(Id::Spu),
            Channel::new(Id::Pio),
            Channel::new(Id::Otc),
        ];

        Self {
            control: 0x07654321,
            interrupt: 0,
            channels,
        }
    }

    /// Executes 1 cycle
    ///
    /// Arguments:
    ///
    /// * `ram`: The RAM component
    /// * `gpu`: The GPU component
    pub(crate) fn step(&mut self, ram: &mut Ram, gpu: &mut Gpu) {
        for channel in &mut self.channels {
            channel.step(ram, gpu);
        }
    }

    /// Gives the channel id based on the offset
    ///
    /// # Arguments:
    ///
    /// * `offset`: The memory offset
    #[inline(always)]
    pub(crate) fn channel_id(offset: u32) -> u8 {
        ((offset >> 4) & 0xf) as u8
    }
}

impl Memory for Dma {
    fn write_u8(&mut self, offset: u32, value: u8) {
        match offset {
            0x00..=0x0c
            | 0x10..=0x1c
            | 0x20..=0x2c
            | 0x30..=0x3c
            | 0x40..=0x4c
            | 0x50..=0x5c
            | 0x60..=0x6c => {
                let channel_id = Self::channel_id(offset);
                let channel_offset = offset - (channel_id as u32 * 0x10);
                let channel = &mut self.channels[channel_id as usize];

                channel.write_u8(channel_offset, value);
            }
            0x70..=0x73 => {
                self.control.write_u8(offset - 0x70, value);
            }
            0x74..=0x77 => {
                self.interrupt.write_u8(offset - 0x74, value);
            }
            _ => unreachable!("write to dma at {:#04x} with value {:#04x}", offset, value),
        }
    }

    fn read_u8(&self, offset: u32) -> u8 {
        match offset {
            0x00..=0x0c
            | 0x10..=0x1c
            | 0x20..=0x2c
            | 0x30..=0x3c
            | 0x40..=0x4c
            | 0x50..=0x5c
            | 0x60..=0x6c => {
                let channel_id = Self::channel_id(offset);
                let channel_offset = offset - (channel_id as u32 * 0x10);
                self.channels[channel_id as usize].read_u8(channel_offset)
            }
            0x70..=0x73 => self.control.read_u8(offset - 0x70),
            0x74..=0x77 => self.interrupt.read_u8(offset - 0x74),
            _ => unreachable!("read from dma at {:#04x}", offset,),
        }
    }
}
