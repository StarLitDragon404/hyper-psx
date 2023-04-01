/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod bios;
mod bus;
mod cpu;
mod memory;

use crate::bios::Bios;
use crate::bus::Bus;

use cpu::Cpu;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to load bios")]
    BiosFailure(#[from] bios::CreationError),
}

#[derive(Clone, Debug)]
pub struct Psx {
    cpu: Cpu,
}

impl Psx {
    pub fn new<P: AsRef<Path>>(bios_path: P) -> Result<Self, CreationError> {
        let bios = Bios::new(bios_path)?;
        let bus = Bus::new(bios);
        let cpu = Cpu::new(bus);

        Ok(Self { cpu })
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
    }
}
