/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod logger;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    logger::init(3)?;

    Ok(())
}
