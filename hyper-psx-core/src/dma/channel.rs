/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bus::memory::Memory;

/// Channel transfer direction
#[derive(Clone, Copy, Debug, Default)]
enum TransferDirection {
    /// To main RAM
    #[default]
    ToRam = 0x0,

    // From main RAM
    FromRam = 0x1,
}

/// Channel memory step
#[derive(Clone, Copy, Debug, Default)]
enum MemoryAddressStep {
    /// Forwards +4
    #[default]
    Forward = 0x0,

    // Backwards -4
    Backward = 0x1,
}

/// Channel chopping
#[derive(Clone, Copy, Debug, Default)]
enum ChoppingMode {
    /// Normal mode
    #[default]
    Normal = 0x0,

    // Chopping mode
    Chopping = 0x1,
}

/// Channel transfer synchronisation
#[derive(Clone, Copy, Debug, Default)]
enum SyncMode {
    /// Immediately and all at once
    #[default]
    Immediately = 0x0,

    // Sync blocks for DMA requests
    Blocks = 0x1,

    /// Linked-List mode
    LinkedList = 0x2,
}

/// Channel start/busy
#[derive(Clone, Copy, Debug, Default)]
enum Busy {
    /// Completed
    #[default]
    Completed = 0x0,

    // Busy
    Busy = 0x1,
}

/// Channel start/trigger
#[derive(Clone, Copy, Debug, Default)]
enum Trigger {
    /// Normal
    #[default]
    Normal = 0x0,

    // Manual
    ManualStart = 0x1,
}

/// Channel (unknown) pause
#[derive(Clone, Copy, Debug, Default)]
enum UnknownPause {
    /// Nothing
    #[default]
    No = 0x0,

    // Paused
    Pause = 0x1,
}

/// DMA Channel
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Channel {
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
                    1 => SyncMode::Blocks,
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
