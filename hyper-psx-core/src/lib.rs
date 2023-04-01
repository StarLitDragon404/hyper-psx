/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod bios;

use crate::bios::Bios;

use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to load bios")]
    BiosFailure(#[from] bios::CreationError),
}

#[derive(Clone, Debug)]
pub struct Psx {}

impl Psx {
    pub fn new<P: AsRef<Path>>(bios_path: P) -> Result<Self, CreationError> {
        let _bios = Bios::new(bios_path)?;

        Ok(Self {})
    }
}
