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
mod gpu;
mod renderer;

use crate::{
    bios::Bios,
    bus::{ram::Ram, Bus},
    cpu::Cpu,
    dma::Dma,
    gpu::Gpu,
    renderer::{
        software_renderer::{self, SoftwareRenderer},
        window::{self, Window},
        Renderer,
    },
};

use std::{path::Path, time::Instant};
use thiserror::Error;
use winit::event::{Event, WindowEvent};

/// The error type for the creation process of the PSX
#[derive(Debug, Error)]
pub enum CreationError {
    /// If the BIOS failed to load
    #[error("failed to load bios")]
    BiosFailure(#[from] bios::CreationError),

    /// If the Window failed to create
    #[error("failed to create window")]
    WindowFailure(#[from] window::CreationError),

    /// If the software renderer failed to create
    #[error("failed to create software renderer")]
    SoftwareRendererFailure(#[from] software_renderer::CreationError),
}

/// The PSX Emulator containg each component
#[derive(Debug)]
pub struct Psx {
    /// The CPU component
    cpu: Cpu,

    /// The window component
    window: Window,
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

        let window = Window::new()?;

        let renderer: Box<dyn Renderer> = Box::new(SoftwareRenderer::new(&window)?);
        let gpu = Gpu::new(renderer);

        let bus = Bus::new(bios, ram, dma, gpu);

        let cpu = Cpu::new(bus);

        Ok(Self { cpu, window })
    }

    /// Runs the PSX Emulator
    pub fn run(&mut self) {
        let mut last_frame = Instant::now();
        self.window.run(|event| {
            match event {
                Event::RedrawRequested(_) => {
                    last_frame = Instant::now();
                    self.cpu.render();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    if size.width == 0 || size.height == 0 {
                        return;
                    }

                    self.cpu.resize(size.width, size.height);
                }
                _ => {}
            }

            while (Instant::now() - last_frame).as_millis() <= 10 {
                self.cpu.step();
            }
        });
    }
}
