/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to find bios: '{0}'")]
    MissingFile(String),

    #[error("failed to open bios: '{1}'")]
    OpenFailure(#[source] io::Error, String),

    #[error("failed to fetch bios metadata: '{1}'")]
    FetchingFailure(#[source] io::Error, String),

    #[error("failed to read bios: '{1}'")]
    ReadingFailure(#[source] io::Error, String),
}

#[derive(Clone, Debug)]
pub(crate) struct Bios {
    data: Vec<u8>,
}

impl Bios {
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
