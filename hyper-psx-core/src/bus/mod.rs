/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

pub(crate) mod memory;
pub(crate) mod ram;
pub(crate) mod range;

use crate::{
    bios::Bios,
    bus::{memory::Memory, ram::Ram, range::Range},
    dma::Dma,
};

/// The BUS component connecting everything
#[derive(Clone, Debug)]
pub(crate) struct Bus {
    /// The BIOS component
    bios: Bios,

    /// The RAM component
    ram: Ram,

    /// The DMA component,
    dma: Dma,
}

impl Bus {
    /// Maks for the regions to handle mirroring
    const REGION_MASKS: [u32; 8] = [
        0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, // KUSG (2GB)
        0x7fffffff, // KSEG0 (0.5GB)
        0x1fffffff, // KSEG1 (0.5GB)
        0xffffffff, 0xffffffff, // KSEG2 (1GB)
    ];

    /// RAM Range
    const RAM_RANGE: Range = Range::new(0x00000000, 0x1f000000);

    /// Expansion Region 1 Range
    const EXPANSION_REGION_1_RANGE: Range = Range::new(0x1f000000, 0x800000);

    /// Scratchpad Range
    const SCRATCHPAD_RANGE: Range = Range::new(0x1f800000, 0x400);

    /// Memory Control 1 Range
    const MEMORY_CONTROL_1_RANGE: Range = Range::new(0x1f801000, 0x24);

    /// Peripheral I/O Ports Range
    const PERIPHERAL_IO_PORTS_RANGE: Range = Range::new(0x1f801040, 0x20);

    /// Memory Control 2 Range
    const MEMORY_CONTROL_2_RANGE: Range = Range::new(0x1f801060, 0x4);

    /// Interrupt Control Range
    const INTERRUPT_CONTROL_RANGE: Range = Range::new(0x1f801070, 0x8);

    /// DMA Registers Range
    const DMA_REGISTERS_RANGE: Range = Range::new(0x1f801080, 0x80);

    /// Timers Range
    const TIMERS_RANGE: Range = Range::new(0x1f801100, 0x30);

    /// CDROM Registers Range
    const CDROM_REGISTERS_RANGE: Range = Range::new(0x1f801800, 0x4);

    /// GPU Registers Range
    const GPU_REGISTERS_RANGE: Range = Range::new(0x1f801810, 0x8);

    /// MDEC Registers Range
    const MDEC_REGISTERS_RANGE: Range = Range::new(0x1f801820, 0x8);

    /// SPU Range
    const SPU_RANGE: Range = Range::new(0x1f801c00, 0x400);

    /// Expansion Region 2 Range
    const EXPANSION_REGION_2_RANGE: Range = Range::new(0x1f802000, 0x88);

    /// Expansion Region 3 Range
    const EXPANSION_REGION_3_RANGE: Range = Range::new(0x1fa00000, 0x200000);

    /// BIOS Region Range
    const BIOS_RANGE: Range = Range::new(0x1fc00000, 0x80000);

    /// Memory Control 3 Range
    const MEMORY_CONTROL_3_RANGE: Range = Range::new(0xfffe0130, 0x4);

    /// Creates a Bus Component
    ///
    /// # Arguments:
    ///
    /// * `bios`: The BIOS component
    /// * `ram`: The RAM component
    /// * `dma`: The DMA component
    pub(crate) fn new(bios: Bios, ram: Ram, dma: Dma) -> Self {
        Self { bios, ram, dma }
    }

    /// Masks a virtual address to a phyiscal address
    ///
    /// # Arguments:
    ///
    /// * `address`: The virtual address
    fn mask_address(address: u32) -> u32 {
        let region_bits = address >> 29;
        let mask = Self::REGION_MASKS[region_bits as usize];
        address & mask
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
        let physical_adddress = Self::mask_address(address);

        if let Some(offset) = Self::RAM_RANGE.contains(physical_adddress) {
            self.ram.write_u8(offset, value);
            return;
        }

        if let Some(_offset) = Self::EXPANSION_REGION_1_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Expansion Region 1: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::SCRATCHPAD_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Scratchpad: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::MEMORY_CONTROL_1_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Memory Control 1: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::PERIPHERAL_IO_PORTS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Peripheral I/O Ports: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::MEMORY_CONTROL_2_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Memory Control 2: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::INTERRUPT_CONTROL_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Interrupt Control: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(offset) = Self::DMA_REGISTERS_RANGE.contains(physical_adddress) {
            self.dma.write_u8(offset, value);

            match offset {
                0x00..=0x0c
                | 0x10..=0x1c
                | 0x20..=0x2c
                | 0x30..=0x3c
                | 0x40..=0x4c
                | 0x50..=0x5c
                | 0x60..=0x6c => {
                    let channel_id = Dma::channel_id(offset);
                    let channel = self.dma.channel_mut(channel_id);

                    if channel.ready() {
                        channel.start_transfer(&mut self.ram);
                    }
                }
                _ => {}
            }

            return;
        }

        if let Some(_offset) = Self::TIMERS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Timers: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::CDROM_REGISTERS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to CDROM Registers: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(offset) = Self::GPU_REGISTERS_RANGE.contains(physical_adddress) {
            log::warn!(
                "Unhandled write to GPU Registers: {:#010x} ({:#x})",
                address,
                offset
            );
            return;
        }

        if let Some(_offset) = Self::MDEC_REGISTERS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to MDEC Registers: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::SPU_RANGE.contains(physical_adddress) {
            /*
            log::warn!("Unhandled write to SPU: {:#010x} ({:#x})", address, offset);
            */
            return;
        }

        if let Some(_offset) = Self::EXPANSION_REGION_2_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Expansion Region 2: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(_offset) = Self::EXPANSION_REGION_3_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Expansion Region 3: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        if let Some(offset) = Self::BIOS_RANGE.contains(physical_adddress) {
            self.bios.write_u8(offset, value);
            return;
        }

        if let Some(_offset) = Self::MEMORY_CONTROL_3_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled write to Memory Control 3: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return;
        }

        panic!(
            "access write violation at address: {:#010x} ({:#010x})",
            physical_adddress, address
        );
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

        self.write_u8(address, byte_0);
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

        self.write_u8(address, byte_0);
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
        let physical_adddress = Self::mask_address(address);

        if let Some(offset) = Self::RAM_RANGE.contains(physical_adddress) {
            return self.ram.read_u8(offset);
        }

        if let Some(_offset) = Self::EXPANSION_REGION_1_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Expansion Region 1: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0xff;
        }

        if let Some(_offset) = Self::SCRATCHPAD_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Scratchpad: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(_offset) = Self::MEMORY_CONTROL_1_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Memory Control 1: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(_offset) = Self::PERIPHERAL_IO_PORTS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Peripheral I/O Ports: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(_offset) = Self::MEMORY_CONTROL_2_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Memory Control 2: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(_offset) = Self::INTERRUPT_CONTROL_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Interrupt Control: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(offset) = Self::DMA_REGISTERS_RANGE.contains(physical_adddress) {
            return self.dma.read_u8(offset);
        }

        if let Some(_offset) = Self::TIMERS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Timers: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(_offset) = Self::CDROM_REGISTERS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from CDROM Registers: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(offset) = Self::GPU_REGISTERS_RANGE.contains(physical_adddress) {
            match offset {
                4..=7 => {
                    // Bit 28 - Ready to receive DMA Block
                    return 0x1c000000u32.read_u8(offset - 0x4);
                }
                _ => {
                    log::warn!(
                        "Unhandled read from GPU Registers: {:#010x} ({:#x})",
                        address,
                        offset
                    );
                    return 0x00;
                }
            }
        }

        if let Some(_offset) = Self::MDEC_REGISTERS_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from MDEC Registers: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(_offset) = Self::SPU_RANGE.contains(physical_adddress) {
            /*
            log::warn!("Unhandled read from SPU: {:#010x} ({:#x})", address, offset);
            */
            return 0x00;
        }

        if let Some(_offset) = Self::EXPANSION_REGION_2_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Expansion Region 2: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(_offset) = Self::EXPANSION_REGION_3_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Expansion Region 3: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        if let Some(offset) = Self::BIOS_RANGE.contains(physical_adddress) {
            return self.bios.read_u8(offset);
        }

        if let Some(_offset) = Self::MEMORY_CONTROL_3_RANGE.contains(physical_adddress) {
            /*
            log::warn!(
                "Unhandled read from Memory Control 3: {:#010x} ({:#x})",
                address,
                offset
            );
            */
            return 0x00;
        }

        panic!(
            "access read violation at address: {:#010x} ({:#010x})",
            physical_adddress, address
        );
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

        let byte_0 = self.read_u8(address) as u16;
        let byte_1 = self.read_u8(address + 1) as u16;

        (byte_1 << 8) | byte_0
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

        let byte_0 = self.read_u8(address) as u32;
        let byte_1 = self.read_u8(address + 1) as u32;
        let byte_2 = self.read_u8(address + 2) as u32;
        let byte_3 = self.read_u8(address + 3) as u32;

        (byte_3 << 24) | (byte_2 << 16) | (byte_1 << 8) | byte_0
    }
}
