/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bus::{memory::Memory, ram::Ram};

use std::fmt::{self, Debug, Formatter};

/// Channel id
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Id {
    #[default]
    MacroBlockIn = 0,

    MacroBlockOut = 1,

    Gpu = 2,

    Cdrom = 3,

    Spu = 4,

    Pio = 5,

    Otc = 6,
}

/// Channel transfer direction
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum TransferDirection {
    /// To main RAM
    #[default]
    ToRam = 0x0,

    // From main RAM
    FromRam = 0x1,
}

/// Channel memory step
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum MemoryAddressStep {
    /// Forwards +4
    #[default]
    Forward = 0x0,

    // Backwards -4
    Backward = 0x1,
}

/// Channel chopping
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum ChoppingMode {
    /// Normal mode
    #[default]
    Normal = 0x0,

    // Chopping mode
    Chopping = 0x1,
}

/// Channel transfer synchronisation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum SyncMode {
    /// Immediately and all at once
    #[default]
    Immediately = 0x0,

    // Sync blocks for DMA requests
    SyncBlocks = 0x1,

    /// Linked-List mode
    LinkedList = 0x2,
}

/// Channel start/busy
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Busy {
    /// Completed
    #[default]
    Completed = 0x0,

    // Busy
    Busy = 0x1,
}

/// Channel start/trigger
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Trigger {
    /// Normal
    #[default]
    Normal = 0x0,

    // Manual
    ManualStart = 0x1,
}

/// Channel (unknown) pause
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum UnknownPause {
    /// Nothing
    #[default]
    No = 0x0,

    // Paused
    Pause = 0x1,
}

/// DMA Channel
#[derive(Clone, Copy, Default)]
pub(crate) struct Channel {
    id: Id,

    // Base memory address
    base_address: u32,

    /// The size of the blocks
    block_size: u16,

    /// The amount of the blocks
    block_count: u16,

    /// The transfer direction
    transfer_direction: TransferDirection,

    /// The memory address step
    memory_address_step: MemoryAddressStep,

    /// The chopping mode
    chopping_mode: ChoppingMode,

    /// The sync mode
    sync_mode: SyncMode,

    /// The chopping DMA window size
    chopping_dma_window_size: u8,

    /// The chopping CPU window size
    chopping_cpu_window_size: u8,

    /// The busy mode
    busy: Busy,

    /// The trigger mode
    trigger: Trigger,

    /// The (unknown) pause
    unknown_pause: UnknownPause,

    /// The unknown value
    unknown: bool,
}

impl Channel {
    /// Creates a new DMA channel
    ///
    /// Arguments:
    ///
    /// * `id`: The id of the channel
    pub(super) fn new(id: Id) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Checks if the current channel is ready to transfer data by checking if
    /// it is enabled/busy and if the trigger mode is a manual start
    pub(crate) fn ready(&self) -> bool {
        if self.busy != Busy::Busy {
            return false;
        }

        if self.sync_mode != SyncMode::Immediately {
            true
        } else {
            self.trigger == Trigger::ManualStart
        }
    }

    /// Finishes off a transfer
    pub(crate) fn finish(&mut self) {
        self.busy = Busy::Completed;
        self.trigger = Trigger::Normal;

        // TODO: Trigger interrupt
    }

    /// Starts the block or linked list transfer for the DMA
    pub(crate) fn start_transfer(&mut self, ram: &mut Ram) {
        match self.sync_mode {
            SyncMode::Immediately => self.transfer_immediately(ram),
            _ => unimplemented!("transfer sync mode '{:?}'", self.sync_mode),
        }
    }

    /// Starts an immediate transfer
    fn transfer_immediately(&mut self, ram: &mut Ram) {
        let mut block_count = self.block_size;
        let mut address = self.base_address;

        let memory_address_step = match self.memory_address_step {
            MemoryAddressStep::Forward => 4,
            MemoryAddressStep::Backward => -4_i8 as u32,
        };

        let mut last_address = address;
        while block_count != 0 {
            match self.transfer_direction {
                TransferDirection::ToRam => {
                    let value = match self.id {
                        Id::Otc => {
                            if block_count == 1 {
                                // End Marker
                                0xffffff
                            } else {
                                last_address
                            }
                        }
                        _ => {
                            unimplemented!("immediate transfer from channel '{:?}' to ram", self.id)
                        }
                    };

                    let byte_0 = (value & 0xff) as u8;
                    let byte_1 = ((value >> 8) & 0xff) as u8;
                    let byte_2 = ((value >> 16) & 0xff) as u8;
                    let byte_3 = ((value >> 24) & 0xff) as u8;

                    ram.write_u8(address, byte_0);
                    ram.write_u8(address + 1, byte_1);
                    ram.write_u8(address + 2, byte_2);
                    ram.write_u8(address + 3, byte_3);
                }
                TransferDirection::FromRam => match self.id {
                    Id::Otc => unreachable!(),
                    _ => unimplemented!("immediate transfer from channel '{:?}' from ram", self.id),
                },
            }

            last_address = address;
            address = address.wrapping_add(memory_address_step);
            block_count -= 1;
        }

        self.finish();
    }
}

impl Debug for Channel {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Channel")
            .field("base_address", &format_args!("{:#010x}", self.base_address))
            .field("block_size", &format_args!("{:#06x}", self.block_size))
            .field("block_count", &self.block_count)
            .field("transfer_direction", &self.transfer_direction)
            .field("memory_address_step", &self.memory_address_step)
            .field("chopping_mode", &self.chopping_mode)
            .field("sync_mode", &self.sync_mode)
            .field(
                "chopping_dma_window_size",
                &format_args!("{:#04x}", self.chopping_dma_window_size),
            )
            .field(
                "chopping_cpu_window_size",
                &format_args!("{:#04x}", self.chopping_cpu_window_size),
            )
            .field("busy", &self.busy)
            .field("trigger", &self.trigger)
            .field("unknown_pause", &self.unknown_pause)
            .field("unknown", &self.unknown)
            .finish()
    }
}

impl Memory for Channel {
    fn write_u8(&mut self, offset: u32, value: u8) {
        match offset {
            0x00..=0x02 => {
                self.base_address.write_u8(offset, value);
            }
            0x03 => {}
            0x04..=0x05 => {
                self.block_size.write_u8(offset - 0x04, value);
            }
            0x06..=0x07 => {
                self.block_count.write_u8(offset - 0x06, value);
            }
            0x08 => {
                let transfer_direction = value & 0b00000001;
                self.transfer_direction = match transfer_direction {
                    0 => TransferDirection::ToRam,
                    1 => TransferDirection::FromRam,
                    _ => unreachable!(),
                };

                let memory_address_step = (value & 0b00000010) >> 1;
                self.memory_address_step = match memory_address_step {
                    0 => MemoryAddressStep::Forward,
                    1 => MemoryAddressStep::Backward,
                    _ => unreachable!(),
                };
            }
            0x09 => {
                let chopping_mode = value & 0b00000001;
                self.chopping_mode = match chopping_mode {
                    0 => ChoppingMode::Normal,
                    1 => ChoppingMode::Chopping,
                    _ => unreachable!(),
                };

                let sync_mode = (value & 0b00000110) >> 1;
                self.sync_mode = match sync_mode {
                    0 => SyncMode::Immediately,
                    1 => SyncMode::SyncBlocks,
                    2 => SyncMode::LinkedList,
                    _ => unreachable!(),
                };
            }
            0x0a => {
                self.chopping_dma_window_size = value & 0b00000111;
                self.chopping_cpu_window_size = (value & 0b01110000) >> 4;
            }
            0x0b => {
                let busy = value & 0b00000001;
                self.busy = match busy {
                    0 => Busy::Completed,
                    1 => Busy::Busy,
                    _ => unreachable!(),
                };

                let trigger = value & (0b00010000) >> 4;
                self.trigger = match trigger {
                    0 => Trigger::Normal,
                    1 => Trigger::ManualStart,
                    _ => unreachable!(),
                };

                let unknown_pause = (value & 0b00100000) >> 5;
                self.unknown_pause = match unknown_pause {
                    0 => UnknownPause::No,
                    1 => UnknownPause::Pause,
                    _ => unreachable!(),
                };

                self.unknown = ((value & 0b01000000) >> 6) != 0;
            }
            _ => unreachable!(
                "write to dma channel at {:#04x} with value {:#04x}",
                offset, value
            ),
        }
    }

    fn read_u8(&self, offset: u32) -> u8 {
        let mut value = 0;
        match offset {
            0x00..=0x02 => {
                value = self.base_address.read_u8(offset);
            }
            0x03 => {}
            0x04..=0x05 => {
                value = self.block_size.read_u8(offset - 0x04);
            }
            0x06..=0x07 => {
                value = self.block_count.read_u8(offset - 0x06);
            }
            0x08 => {
                value |= self.transfer_direction as u8;
                value |= (self.memory_address_step as u8) << 1;
            }
            0x09 => {
                value |= self.chopping_mode as u8;
                value |= (self.sync_mode as u8) << 1;
            }
            0x0a => {
                value |= self.chopping_dma_window_size;
                value |= self.chopping_cpu_window_size << 4;
            }
            0x0b => {
                value |= self.busy as u8;
                value |= (self.trigger as u8) << 4;
                value |= (self.unknown_pause as u8) << 5;
                value |= (self.unknown as u8) << 6;
            }
            _ => unreachable!("read from dma channel at {:#04x}", offset),
        }

        value
    }
}
