/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::memory::Memory;

use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};
use thiserror::Error;

/// The error type of the creation process of the BIOS
#[derive(Debug, Error)]
pub enum CreationError {
    /// If the BIOS file was not found
    #[error("failed to find bios: '{0}'")]
    MissingFile(String),

    /// If the BIOS file failed to open
    #[error("failed to open bios: '{1}'")]
    OpenFailure(#[source] io::Error, String),

    /// If the BIOS file contains no metadata
    #[error("failed to fetch bios metadata: '{1}'")]
    FetchingFailure(#[source] io::Error, String),

    /// If the BIOS file failed to be read from
    #[error("failed to read bios: '{1}'")]
    ReadingFailure(#[source] io::Error, String),
}

/// The BIOS component
#[derive(Clone, Debug)]
pub(crate) struct Bios {
    /// The data vector containing the bios
    data: Vec<u8>,
}

impl Bios {
    /// Creates a BIOS Component
    ///
    /// # Arguments:
    ///
    /// * `path`: The path of the BIOS
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, CreationError> {
        let path_display = path.as_ref().display().to_string();
        if !path.as_ref().exists() {
            return Err(CreationError::MissingFile(path_display.clone()));
        }

        let buffer = Self::read_file(path)?;

        log::info!(
            "Loaded BIOS from '{}' ({} bytes)",
            path_display,
            buffer.len()
        );

        Ok(Self { data: buffer })
    }

    /// Reads a file into a vector of bytes
    ///
    /// # Arguments:
    ///
    /// * `path`: The path of the BIOS
    fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, CreationError> {
        let path_display = path.as_ref().display().to_string();
        let mut file = File::open(path)
            .map_err(|error| CreationError::OpenFailure(error, path_display.clone()))?;
        let metadata = file
            .metadata()
            .map_err(|error| CreationError::FetchingFailure(error, path_display.clone()))?;

        let mut buffer = vec![0x00; metadata.len() as usize];
        file.read(&mut buffer)
            .map_err(|error| CreationError::ReadingFailure(error, path_display))?;

        Ok(buffer)
    }
}

impl Memory for Bios {
    /// Writes a u8 to a specific address
    ///
    /// # Arguments:
    ///
    /// * `offset`: The relative address offset
    /// * `value`: The value to be written
    ///
    /// # Notes:
    ///
    /// This function shouldn't be used, because the BIOS is read-only
    fn write_u8(&mut self, offset: u32, _value: u8) {
        assert!((offset as usize) < self.data.len());
    }

    /// Reads a u8 from a specific address
    ///
    /// # Arguments:
    ///
    /// * `offset`: The relative address offset
    fn read_u8(&self, offset: u32) -> u8 {
        assert!((offset as usize) < self.data.len());

        self.data[offset as usize]
    }
}
