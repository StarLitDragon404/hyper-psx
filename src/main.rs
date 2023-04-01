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

    log::info!(" _     _ __   __  _____  _______  ______      _____  _______ _     _");
    log::info!(" |_____|   \\_/   |_____] |______ |_____/ ___ |_____] |______  \\___/ ");
    log::info!(" |     |    |    |       |______ |    \\_     |       ______| _/   \\_");
    log::info!("");

    let mut psx = Psx::new("./data/SCPH1001.BIN")?;
    psx.run();

    Ok(())
}
