/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

//! The core for the PSX Emulator

mod bios;
mod bus;
mod cpu;
mod dma;

use crate::bios::Bios;
use crate::bus::{ram::Ram, Bus};
use crate::cpu::Cpu;
use crate::dma::Dma;

use std::path::Path;
use thiserror::Error;

/// The error type for the creation process of the PSX
#[derive(Debug, Error)]
pub enum CreationError {
    /// If the BIOS failed to load
    #[error("failed to load bios")]
    BiosFailure(#[from] bios::CreationError),
}

/// The PSX Emulator containg each component
#[derive(Clone, Debug)]
pub struct Psx {
    /// The CPU component
    cpu: Cpu,
}

impl Psx {
    /// Creates a new PSX Emulator
    ///
    /// # Arguments:
    ///
    /// * `bios_path`: The path to the BIOS
    ///
    /// # Errors
    ///
    /// This function will throw an error if the BIOS failed to load
    pub fn new<P: AsRef<Path>>(bios_path: P) -> Result<Self, CreationError> {
        let bios = Bios::new(bios_path)?;
        let ram = Ram::new();

        let dma = Dma::new();

        let bus = Bus::new(bios, ram, dma);

        let cpu = Cpu::new(bus);

        Ok(Self { cpu })
    }

    /// Runs the PSX Emulator
    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
    }
}
