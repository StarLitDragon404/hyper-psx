/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::gpu::{
    DisplayAreaDrawing, Dither, DrawPixels, Gpu, MaskDrawing, SemiTransparency, TexturePageColors,
};

impl Gpu {
    /// GP0(28h) - Monochrome four-point polygon, opaque
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gpu-render-polygon-commands>
    pub(super) fn op_draw_monochrome_four_point_polygon_opaque(&mut self) {
        log::debug!(target: "gpu", "GP0(28h) - Monochrome four-point polygon, opaque");

        // TODO: Implemenet draw
    }

    /// GP0(E1h) - Draw Mode setting (aka "Texpage")
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp0e1h-draw-mode-setting-aka-texpage>
    pub(super) fn op_draw_mode_setting(&mut self) {
        log::debug!(target: "gpu", "GP0(E1h) - Draw Mode setting");

        let command = self.arguments[0];

        self.texture_page_x_base = (command & 0xf) as u8;
        self.texture_page_y_base_1 = ((command >> 4) & 0x1) as u8;

        let semi_transparency = ((command >> 5) & 0x3) as u8;
        self.semi_transparency = match semi_transparency {
            0 => SemiTransparency::First,
            1 => SemiTransparency::Second,
            2 => SemiTransparency::Third,
            3 => SemiTransparency::Fourth,
            _ => unreachable!(),
        };

        let texture_page_colors = ((command >> 7) & 0x3) as u8;
        self.texture_page_colors = match texture_page_colors {
            0 => TexturePageColors::Bit4,
            1 => TexturePageColors::Bit8,
            2 => TexturePageColors::Bit15,
            _ => unreachable!(),
        };

        let dither = ((command >> 9) & 0x1) as u8;
        self.dither = match dither {
            0 => Dither::Off,
            1 => Dither::Enabled,
            _ => unreachable!(),
        };

        let display_area_drawing = ((command >> 10) & 0x1) as u8;
        self.display_area_drawing = match display_area_drawing {
            0 => DisplayAreaDrawing::Prohibited,
            1 => DisplayAreaDrawing::Allowed,
            _ => unreachable!(),
        };

        self.texture_page_y_base_2 = ((command >> 11) & 0x1) as u8;
        self.texture_rectangle_x_flip = ((command >> 12) & 0x1) as u8 != 0;
        self.texture_rectangle_y_flip = ((command >> 13) & 0x1) as u8 != 0;
    }

    /// GP0(E2h) - Texture Window setting
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp0e2h-texture-window-setting>
    pub(super) fn op_texture_window_setting(&mut self) {
        log::debug!(target: "gpu", "GP0(E2h) - Texture Window setting");

        let command = self.arguments[0];

        self.texture_window_x_mask = (command & 0x1f) as u8;
        self.texture_window_y_mask = ((command >> 5) & 0x1f) as u8;
        self.texture_window_x_offset = ((command >> 10) & 0x1f) as u8;
        self.texture_window_y_offset = ((command >> 15) & 0x1f) as u8;
    }

    /// GP0(E3h) - Set Drawing Area top left (X1,Y1)
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp0e3h-set-drawing-area-top-left-x1y1>
    pub(super) fn op_set_drawing_area_top_left(&mut self) {
        log::debug!(target: "gpu", "GP0(E3h) - Set Drawing Area top left (X1,Y1)");

        let command = self.arguments[0];

        self.drawing_area_left = (command & 0x3ff) as u16;
        self.drawing_area_top = ((command >> 10) & 0x3ff) as u16;
    }

    /// GP0(E4h) - Set Drawing Area bottom right (X2,Y2)
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp0e4h-set-drawing-area-bottom-right-x2y2>
    pub(super) fn op_set_drawing_area_bottom_right(&mut self) {
        log::debug!(target: "gpu", "GP0(E4h) - Set Drawing Area bottom right (X2,Y2)");

        let command = self.arguments[0];

        self.drawing_area_right = (command & 0x3ff) as u16;
        self.drawing_area_bottom = ((command >> 10) & 0x3ff) as u16;
    }

    /// GP0(E5h) - Set Drawing Offset (X,Y)
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp0e5h-set-drawing-offset-xy>
    pub(super) fn op_set_drawing_offset(&mut self) {
        log::debug!(target: "gpu", "GP0(E5h) - Set Drawing Offset (X,Y)");

        let command = self.arguments[0];

        self.drawing_x_offset = (command & 0x7ff) as u16;
        self.drawing_y_offset = ((command >> 11) & 0x7ff) as u16;
    }

    /// GP0(E6h) - Mask Bit Setting
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp0e6h-mask-bit-setting>
    pub(super) fn op_mask_bit_setting(&mut self) {
        log::debug!(target: "gpu", "GP0(E6h) - Mask Bit Setting");

        let command = self.arguments[0];

        let mask_drawing = (command & 0x1) as u8;
        self.mask_drawing = match mask_drawing {
            0 => MaskDrawing::No,
            1 => MaskDrawing::Yes,
            _ => unreachable!(),
        };

        let draw_pixels = ((command >> 1) & 0x1) as u8;
        self.draw_pixels = match draw_pixels {
            0 => DrawPixels::Always,
            1 => DrawPixels::Unmasked,
            _ => unreachable!(),
        };
    }
}
