/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

pub(crate) mod color;
pub(crate) mod position;
pub(crate) mod software_renderer;
pub(crate) mod window;

use crate::renderer::{color::Color, position::Position};

pub(crate) trait Renderer {
    /// Renders the current VRAM
    fn render(&mut self);

    /// Resizes the current frame buffer
    ///
    /// Arguments:
    ///
    /// * `width`: The new frame buffer width
    /// * `height`: The new frame buffer height
    fn resize(&mut self, width: u32, height: u32);

    /// Draws a quad
    fn draw_quad(&mut self, positions: [Position; 4], colors: [Color; 4]);

    /// Draws a triangle
    fn draw_triangle(&mut self, positions: [Position; 3], colors: [Color; 3]);
}
