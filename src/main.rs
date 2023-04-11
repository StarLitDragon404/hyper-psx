/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

//! A fast and reliable PSX Emulator written in pure Rust!

mod logger;

use hyper_psx_core::Psx;

use clap::{Parser, ValueEnum};
use color_eyre::Result;

/// Logger Verbosity
#[derive(Clone, Copy, ValueEnum)]
enum Verbosity {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Debugger
#[derive(Clone, Copy, ValueEnum)]
pub(crate) enum Debug {
    None,
    Bus,
    Cpu,
    Dma,
    Gpu,
}

#[derive(Parser)]
#[command(author, version)]
struct Arguments {
    /// Verbosity of the logger
    #[arg(long, value_enum, default_value_t = Verbosity::Info)]
    verbosity: Verbosity,

    /// Path to BIOS file
    #[arg(long, default_value_t = String::from("./data/SCPH1001.BIN"))]
    bios_path: String,

    /// Enable debug mode
    #[arg(long, value_enum, default_value_t = Debug::None)]
    debug: Debug,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let arguments = Arguments::parse();
    let verbosity = match arguments.verbosity {
        Verbosity::Error => 4,
        Verbosity::Warn => 0,
        Verbosity::Info => 1,
        Verbosity::Debug => 2,
        Verbosity::Trace => 3,
    };
    let debug = arguments.debug;

    logger::init(verbosity, debug)?;

    log::info!(" _     _ __   __  _____  _______  ______      _____  _______ _     _");
    log::info!(" |_____|   \\_/   |_____] |______ |_____/ ___ |_____] |______  \\___/ ");
    log::info!(" |     |    |    |       |______ |    \\_     |       ______| _/   \\_");
    log::info!("");

    let mut psx = Psx::new(arguments.bios_path)?;
    psx.run();

    Ok(())
}
