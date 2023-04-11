/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::Debug;

use chrono::Local;
use color_eyre::Result;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::LevelFilter;
use std::{fs::OpenOptions, io};

/// Initializes the global logger
///
/// # Arguments:
///
/// * `verbosity`: The verbosity the logger should operate on
pub(crate) fn init(verbosity: usize, debug: Debug) -> Result<()> {
    let mut logger = Dispatch::new();

    let color_logger = create_color_logger(verbosity, debug);
    logger = logger.chain(color_logger);

    let file_logger = create_file_logger(verbosity, debug)?;
    logger = logger.chain(file_logger);

    logger.apply()?;

    Ok(())
}

fn create_color_logger(verbosity: usize, debug: Debug) -> Dispatch {
    let mut logger = Dispatch::new();

    let levels = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::BrightYellow)
        .info(Color::Green)
        .debug(Color::Cyan)
        .trace(Color::Magenta);

    logger = logger.format(move |out, message, record| {
        let reset = "\x1B[0m";

        let time = {
            let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
            let black = format!("\x1B[{}m", Color::BrightBlack.to_fg_str());
            format!("{black}[{current_time}]{reset}")
        };

        let level = {
            let current_level = record.level();
            let color = format!("\x1B[{}m", levels.get_color(&record.level()).to_fg_str());
            format!("{color}{current_level:<5}{reset}")
        };

        let message = format!("{reset}{message}");
        out.finish(format_args!("{time} {level} {message}",))
    });

    logger = logger.level_for("naga", LevelFilter::Off);

    logger = logger.level_for("wgpu_core", LevelFilter::Off);

    logger = logger.level_for("wgpu_hal", LevelFilter::Off);

    logger = match verbosity {
        0 => logger.level(LevelFilter::Warn),
        1 => logger.level(LevelFilter::Info),
        2 => logger.level(LevelFilter::Debug),
        3 => logger.level(LevelFilter::Trace),
        _ => logger.level(LevelFilter::Error),
    };

    logger = match debug {
        Debug::None => logger,
        Debug::Bus => logger.level_for("bus", LevelFilter::Debug),
        Debug::Cpu => logger.level_for("cpu", LevelFilter::Debug),
        Debug::Dma => logger.level_for("dma", LevelFilter::Debug),
        Debug::Gpu => logger.level_for("gpu", LevelFilter::Debug),
    };

    logger = logger.chain(io::stdout());

    logger
}

fn create_file_logger(verbosity: usize, debug: Debug) -> Result<Dispatch> {
    let mut logger = Dispatch::new();

    logger = logger.format(move |out, message, record| {
        let time = {
            let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
            format!("[{current_time}]")
        };

        let level = {
            let current_level = record.level();
            format!("{current_level:<5}")
        };

        let message = format!("{message}");
        out.finish(format_args!("{time} {level} {message}",))
    });

    logger = logger.level_for("naga", LevelFilter::Off);

    logger = logger.level_for("wgpu_core", LevelFilter::Off);

    logger = logger.level_for("wgpu_hal", LevelFilter::Off);

    logger = match verbosity {
        0 => logger.level(LevelFilter::Warn),
        1 => logger.level(LevelFilter::Info),
        2 => logger.level(LevelFilter::Debug),
        3 => logger.level(LevelFilter::Trace),
        _ => logger.level(LevelFilter::Error),
    };

    logger = match debug {
        Debug::None => logger,
        Debug::Bus => logger.level_for("bus", LevelFilter::Debug),
        Debug::Cpu => logger.level_for("cpu", LevelFilter::Debug),
        Debug::Dma => logger.level_for("dma", LevelFilter::Debug),
        Debug::Gpu => logger.level_for("gpu", LevelFilter::Debug),
    };

    logger = logger.chain(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("latest.log")?,
    );

    logger = logger.chain(fern::log_file("latest.log")?);

    Ok(logger)
}
