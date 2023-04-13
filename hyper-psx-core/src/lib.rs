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
mod utils;

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

use cgmath::Vector2;
use glfw::WindowEvent;
use std::{path::Path, time::Instant};
use thiserror::Error;

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

    /// The DMA component,
    dma: Dma,

    /// The GPU component,
    gpu: Gpu,

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

        let bus = Bus::new(bios, ram);

        let cpu = Cpu::new(bus);

        Ok(Self {
            cpu,
            dma,
            gpu,
            window,
        })
    }

    /// Runs the PSX Emulator
    pub fn run(&mut self) {
        let cpu_cycles_per_second = 33868800.0; // CPU Cyles per Second
        let frames_per_second = 60.0_f32; // Around 59.940 for NTSC;
        let cycles_per_frame = (cpu_cycles_per_second / frames_per_second).round() as u32;

        let delta_time = 1.0 / frames_per_second;

        let mut last_time = Instant::now();
        let mut accumulator = 0.0;
        while !self.window.should_close() {
            self.window.poll_events();
            self.window.handle_events(|event| {
                if let WindowEvent::Size(width, height) = *event {
                    if width == 0 || height == 0 {
                        return;
                    }

                    let size = Vector2 {
                        x: width as u32,
                        y: height as u32,
                    };

                    self.gpu.resize(size);
                };
            });

            let current_time = Instant::now();
            let mut elapsed_time = (current_time - last_time).as_secs_f32();
            if elapsed_time > 0.25 {
                elapsed_time = 0.25;
            }

            last_time = current_time;
            accumulator += elapsed_time;

            while accumulator >= delta_time {
                self.emulate_frame(cycles_per_frame);

                accumulator -= delta_time;
            }
        }
    }

    /// Emulates a frame
    ///
    /// Arguments:
    ///
    /// * `cycles_per_frame`: The amount of cycles this frame needs to do
    fn emulate_frame(&mut self, cycles_per_frame: u32) {
        for _ in 0..cycles_per_frame / 2 {
            self.cpu.step(&mut self.dma, &mut self.gpu);
        }

        self.dma.step(self.cpu.bus().ram(), &mut self.gpu);

        self.gpu.step();
        // TODO: Emulate GPU frames with VBLANK
    }
}
