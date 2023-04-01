/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod logger;

use hyper_psx_core::Psx;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    logger::init(3)?;

    let _psx = Psx::new("./data/SCPH1001.BIN")?;

    Ok(())
}
