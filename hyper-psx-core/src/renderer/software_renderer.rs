/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::renderer::{window::Window, Color, Position, Renderer};

use cgmath::{Vector2, Vector3};
use pixels::{Pixels, SurfaceTexture};
use thiserror::Error;

/// Creation error type for the software renderer
#[derive(Debug, Error)]
pub enum CreationError {
    /// If the code failed to create a pixels framebuffer
    #[error("failed to create pixels framebuffer")]
    PixelsCreationFailure(#[from] pixels::Error),
}

/// The software renderer
#[derive(Debug)]
pub(crate) struct SoftwareRenderer {
    /// The pixels framebuffer
    pixels: Pixels,

    /// The current framebuffer size
    size: Vector2<u32>,
}

impl SoftwareRenderer {
    /// Creates a new software renderer
    ///
    /// Arguments:
    ///
    /// * `window`: The corresponding window
    pub(crate) fn new(window: &Window) -> Result<Self, CreationError> {
        let mut pixels = {
            let window_size = window.size();
            let surface_texture =
                SurfaceTexture::new(window_size.x, window_size.y, window.internal());
            Pixels::new(1024, 512, surface_texture)?
        };

        for pixel in pixels.frame_mut().chunks_exact_mut(4) {
            pixel[0] = 0x00;
            pixel[1] = 0x00;
            pixel[2] = 0x00;
            pixel[3] = 0xff;
        }

        Ok(Self {
            pixels,
            size: window.size(),
        })
    }
}

impl Renderer for SoftwareRenderer {
    fn render(&mut self) {
        self.pixels.render().unwrap();
    }

    fn resize(&mut self, size: Vector2<u32>) {
        self.pixels.resize_surface(size.x, size.y).unwrap();
        self.size = size;
    }

    fn draw_quad(&mut self, positions: [Position; 4], colors: [Color; 4]) {
        self.draw_triangle(
            [positions[0], positions[2], positions[1]],
            [colors[0], colors[2], colors[1]],
        );
        self.draw_triangle(
            [positions[1], positions[2], positions[3]],
            [colors[1], colors[2], colors[3]],
        );
    }

    fn draw_triangle(&mut self, positions: [Position; 3], colors: [Color; 3]) {
        let mut bbox_min = Vector2 {
            x: f32::MAX,
            y: f32::MAX,
        };
        let mut bbox_max = Vector2 {
            x: f32::MIN,
            y: f32::MIN,
        };

        let clamp = Vector2 {
            x: (self.size.x - 1) as f32,
            y: (self.size.y - 1) as f32,
        };
        for position in positions {
            bbox_min.x = 0.0f32.max(bbox_min.x.min(position.x as f32));
            bbox_max.x = clamp.x.min(bbox_max.x.max(position.x as f32));

            bbox_min.y = 0.0f32.max(bbox_min.y.min(position.y as f32));
            bbox_max.y = clamp.y.min(bbox_max.y.max(position.y as f32));
        }

        for x in (bbox_min.x as i32)..=(bbox_max.x as i32) {
            for y in (bbox_min.y as i32)..=(bbox_max.y as i32) {
                let a = Vector2 {
                    x: positions[0].x as f32,
                    y: positions[0].y as f32,
                };

                let b = Vector2 {
                    x: positions[1].x as f32,
                    y: positions[1].y as f32,
                };

                let c = Vector2 {
                    x: positions[2].x as f32,
                    y: positions[2].y as f32,
                };

                let p = Vector2 {
                    x: x as f32,
                    y: y as f32,
                };

                let v0 = b - a;
                let v1 = c - a;
                let v2 = p - a;

                let denominator = v0.x * v1.y - v1.x * v0.y;

                let v = (v2.x * v1.y - v1.x * v2.y) / denominator;
                let w = (v0.x * v2.y - v2.x * v0.y) / denominator;
                let u = 1.0 - v - w;

                // The point lies outside of the triangle
                if v <= f32::EPSILON || w + f32::EPSILON < 0.0 || u + f32::EPSILON < 0.0 {
                    continue;
                }

                let a_color = Vector3 {
                    x: colors[0].x as f32,
                    y: colors[0].y as f32,
                    z: colors[0].z as f32,
                };

                let b_color = Vector3 {
                    x: colors[1].x as f32,
                    y: colors[1].y as f32,
                    z: colors[1].z as f32,
                };

                let c_color = Vector3 {
                    x: colors[2].x as f32,
                    y: colors[2].y as f32,
                    z: colors[2].z as f32,
                };

                let color = b_color * v + a_color * u + c_color * w;

                let index = ((y as u32 * 1024 + x as u32) * 4) as usize;
                let buffer = self.pixels.frame_mut();
                buffer[index] = color.x as u8;
                buffer[index + 1] = color.y as u8;
                buffer[index + 2] = color.z as u8;
            }
        }
    }
}
