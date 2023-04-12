/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

pub(crate) mod software_renderer;
pub(crate) mod window;

use cgmath::{Vector2, Vector3};

pub(crate) type Position = Vector2<i16>;
pub(crate) type Color = Vector3<u8>;

pub(crate) fn position_from_u32(word: u32) -> Position {
    let x = (word & 0xffff) as i16;
    let y = ((word >> 16) & 0xffff) as i16;

    Position { x, y }
}

pub(crate) fn color_from_u32(word: u32) -> Color {
    let r = (word & 0xff) as u8;
    let g = ((word >> 8) & 0xff) as u8;
    let b = ((word >> 16) & 0xff) as u8;

    Color { x: r, y: g, z: b }
}

pub(crate) trait Renderer {
    /// Renders the current framebuffer
    fn render(&mut self);

    /// Resizes the current framebuffer
    ///
    /// Arguments:
    ///
    /// * `size`: New framebuffer size
    fn resize(&mut self, size: Vector2<u32>);

    /// Draws a quad
    ///
    /// Arguments:
    ///
    /// * `positions`: Vertex positions
    /// * `colors`: Vertex colors
    fn draw_quad(&mut self, positions: [Position; 4], colors: [Color; 4]);

    /// Draws a triangle
    ///
    /// Arguments:
    ///
    /// * `positions`: Vertex positions
    /// * `colors`: Vertex colors
    fn draw_triangle(&mut self, positions: [Position; 3], colors: [Color; 3]);
}
