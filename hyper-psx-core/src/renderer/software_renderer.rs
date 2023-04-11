/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::renderer::{color::Color, position::Position, window::Window, Renderer};

use cgmath::{Vector2, Vector3};
use pixels::{Pixels, SurfaceTexture};
use thiserror::Error;

/// The error type for the window
#[derive(Debug, Error)]
pub enum CreationError {
    /// If the Pixles buffer failed to create
    #[error("failed to create pixels buffer")]
    PixelsCreationFailure(#[from] pixels::Error),
}

/// The software renderer
#[derive(Debug)]
pub(crate) struct SoftwareRenderer {
    /// The pixels to write to
    pixels: Pixels,

    /// Current window width
    width: u32,

    /// Current window height
    height: u32,
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
                SurfaceTexture::new(window_size.0, window_size.1, window.internal());
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
            width: window.size().0,
            height: window.size().1,
        })
    }

    /// Calculates the barycentric of 1 point inside of 3 points
    ///
    /// Arguments:
    ///
    /// * `positions`: The 3 vertices of a triangle
    /// * `point`: The point inside the triangle
    fn barycentric(positions: [Position; 3], point: Vector2<f32>) -> Vector3<f32> {
        let u = Vector3::new(
            positions[2].x() as f32 - positions[0].x() as f32,
            positions[1].x() as f32 - positions[0].x() as f32,
            positions[0].x() as f32 - point.x,
        )
        .cross(Vector3::new(
            positions[2].y() as f32 - positions[0].y() as f32,
            positions[1].y() as f32 - positions[0].y() as f32,
            positions[0].y() as f32 - point.y,
        ));

        if u.z.abs() < 1.0 {
            Vector3::new(-1.0, -1.0, -1.0)
        } else {
            Vector3::new(1.0 - ((u.x + u.y) / u.z), u.y / u.z, u.x / u.z)
        }
    }
}

impl Renderer for SoftwareRenderer {
    fn render(&mut self) {
        self.pixels.render().unwrap();
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).unwrap();
        self.width = width;
        self.height = height;
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
        let clamp = Vector2::new((self.width - 1) as f32, (self.height - 1) as f32);

        let mut bbox_min = Vector2::new(f32::MAX, f32::MAX);
        let mut bbox_max = Vector2::new(f32::MIN, f32::MIN);

        for point in positions {
            bbox_min.x = 0.0f32.max(bbox_min.x.min(point.x() as f32));
            bbox_max.x = clamp.x.min(bbox_max.x.max(point.x() as f32));

            bbox_min.y = 0.0f32.max(bbox_min.y.min(point.y() as f32));
            bbox_max.y = clamp.y.min(bbox_max.y.max(point.y() as f32));
        }

        for x in (bbox_min.x as i32)..=(bbox_max.x as i32) {
            for y in (bbox_min.y as i32)..=(bbox_max.y as i32) {
                let point = Vector2::new(x as f32, y as f32);

                let bary_centric_screen = Self::barycentric(positions, point);
                if bary_centric_screen.x < 0.0
                    || bary_centric_screen.y < 0.0
                    || bary_centric_screen.z < 0.0
                {
                    continue;
                }

                let buffer = self.pixels.frame_mut();
                buffer[((y as u32 * self.width + x as u32) * 4) as usize] = colors[0].r();
                buffer[(((y as u32 * self.width + x as u32) * 4) + 1) as usize] = colors[0].g();
                buffer[(((y as u32 * self.width + x as u32) * 4) + 2) as usize] = colors[0].b();
            }
        }
    }
}
