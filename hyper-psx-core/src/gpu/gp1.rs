/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::gpu::{
    ColorDepth, DisplayEnabled, DmaDirection, Gpu, HorizontalResolution, InterruptRequest, Reverse,
    VerticalInterlace, VerticalResolution, VideoMode,
};

impl Gpu {
    /// GP1(01h) - Reset Command Buffer
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp101h-reset-command-buffer>
    pub(super) fn op_reset_command_buffer(&mut self, _command: u32) {
        // TODO: Reset FIFO
    }

    /// GP1(02h) - Acknowledge GPU Interrupt (IRQ1)
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp102h-acknowledge-gpu-interrupt-irq1>
    pub(super) fn op_acknowledge_gpu_interrupt(&mut self, _command: u32) {
        self.interrupt_request = InterruptRequest::Off;
    }

    /// GP1(03h) - Display Enable
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp103h-display-enable>
    pub(super) fn op_display_enable(&mut self, command: u32) {
        let display_enabled = (command & 0x1) as u8;
        self.display_enabled = match display_enabled {
            0 => DisplayEnabled::Enabled,
            1 => DisplayEnabled::Disabled,
            _ => unreachable!(),
        }
    }

    /// GP1(04h) - DMA Direction / Data Request
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp104h-dma-direction-data-request>
    pub(super) fn op_dma_direction(&mut self, command: u32) {
        let dma_direction = (command & 0x1) as u8;
        self.dma_direction = match dma_direction {
            0 => DmaDirection::Off,
            1 => DmaDirection::Fifo,
            2 => DmaDirection::CpuToGpu,
            3 => DmaDirection::GpuToCpu,
            _ => unreachable!(),
        };
    }

    /// GP1(05h) - Start of Display area (in VRAM)
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp105h-start-of-display-area-in-vram>
    pub(super) fn op_start_of_display_area_in_vram(&mut self, command: u32) {
        self.display_area_x_start_in_vram = (command & 0x3ff) as u16;
        self.display_area_y_start_in_vram = ((command >> 10) & 0x3ff) as u16;
    }

    /// GP1(06h) - Horizontal Display range (on Screen)
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp106h-horizontal-display-range-on-screen>
    pub(super) fn op_horizontal_display_range_on_screen(&mut self, command: u32) {
        self.display_range_horizontal_start = (command & 0xfff) as u16;
        self.display_range_horizontal_end = ((command >> 12) & 0xfff) as u16;
    }

    /// GP1(07h) - Vertical Display range (on Screen)
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp107h-vertical-display-range-on-screen>
    pub(super) fn op_vertical_display_range_on_screen(&mut self, command: u32) {
        self.display_range_vertical_start = (command & 0x3ff) as u16;
        self.display_range_vertical_end = ((command >> 10) & 0x3ff) as u16;
    }

    /// GP1(08h) - Display mode
    ///
    /// Arguments:
    ///
    /// * `command`: The command itself
    ///
    /// <https://psx-spx.consoledev.net/graphicsprocessingunitgpu/#gp108h-display-mode>
    pub(super) fn op_display_mode(&mut self, command: u32) {
        let vertical_resolution = ((command >> 2) & 0x1) as u8;
        self.vertical_resolution = match vertical_resolution {
            0 => VerticalResolution::S240,
            1 => VerticalResolution::S480,
            _ => unreachable!(),
        };

        let video_mode = ((command >> 3) & 0x1) as u8;
        self.video_mode = match video_mode {
            0 => VideoMode::Hz60,
            1 => VideoMode::Hz50,
            _ => unreachable!(),
        };

        let display_area_color_depth = ((command >> 4) & 0x1) as u8;
        self.display_area_color_depth = match display_area_color_depth {
            0 => ColorDepth::Bit15,
            1 => ColorDepth::Bit24,
            _ => unreachable!(),
        };

        let vertical_interlace = ((command >> 5) & 0x1) as u8;
        self.vertical_interlace = match vertical_interlace {
            0 => VerticalInterlace::Off,
            1 => VerticalInterlace::On,
            _ => unreachable!(),
        };

        let horizontal_resolution_1 = (command & 0x3) as u8;
        let horizontal_resolution_2 = ((command >> 6) & 0x1) as u8;
        self.horizontal_resolution = match horizontal_resolution_2 {
            0 => match horizontal_resolution_1 {
                0 => HorizontalResolution::S256,
                1 => HorizontalResolution::S320,
                2 => HorizontalResolution::S512,
                3 => HorizontalResolution::S640,
                _ => unreachable!(),
            },
            1 => HorizontalResolution::S368,
            _ => unreachable!(),
        };

        let reverse = ((command >> 7) & 0x1) as u8;
        self.reverse = match reverse {
            0 => Reverse::Normal,
            1 => Reverse::Distorted,
            _ => unreachable!(),
        };
    }
}
