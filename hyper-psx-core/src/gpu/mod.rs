/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod gp0;
mod gp1;

use crate::bus::memory::Memory;

/// The semi transparency mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum SemiTransparency {
    /// The first mode (B/2+F/2)
    #[default]
    First = 0,

    /// The second mode (B+F)
    Second = 1,

    /// The third mode (B-F)
    Third = 2,

    /// The fourth mode (B+F/4)
    Fourth = 3,
}

/// The texture page colors
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum TexturePageColors {
    /// 4 bits depth
    #[default]
    Bit4 = 0,

    /// 8 bits depth
    Bit8 = 1,

    /// 15 bits depth
    Bit15 = 2,
}

/// The dither mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Dither {
    /// Off mode
    #[default]
    Off = 0,

    /// Enabled mode
    Enabled = 1,
}

/// The display area drawing flag
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum DisplayAreaDrawing {
    /// Drawing to the prohibited area is allowed
    #[default]
    Prohibited = 0,

    /// Drawing to the display area is allowed
    Allowed = 1,
}

/// The mask drawing flag
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum MaskDrawing {
    /// The mask bit will not be set
    #[default]
    No = 0,

    /// The mask bit will be set
    Yes = 1,
}

/// The draw pixels flag
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum DrawPixels {
    /// It should always be drawn
    #[default]
    Always = 0,

    /// Only drawn to not masked areas
    Unmasked = 1,
}

/// The field interlace
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Interlace {
    /// Never interlace
    #[default]
    Never = 0,

    /// Always interlace
    Always = 1,
}

/// The reverse flag
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Reverse {
    /// Normal mode
    #[default]
    Normal = 0,

    /// Distorted mode
    Distorted = 1,
}

/// The horizontal resolution
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum HorizontalResolution {
    /// 256 Resolution
    #[default]
    S256 = 0,

    /// 320 Resolution
    S320 = 1,

    /// 368 Resolution
    S368 = 2,

    /// 512 Resolution
    S512 = 3,

    /// 640 Resolution
    S640 = 4,
}

/// The vertical resolution
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum VerticalResolution {
    /// 240 Resolution
    #[default]
    S240 = 0,

    /// 480 Resolution
    S480 = 1,
}

/// The video mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum VideoMode {
    /// 60Hz
    #[default]
    Hz60 = 0,

    /// 50Hz
    Hz50 = 1,
}

/// The color depth
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum ColorDepth {
    /// 15 bits
    #[default]
    Bit15 = 0,

    /// 24 bits
    Bit24 = 1,
}

/// The vertical interlace
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum VerticalInterlace {
    /// Interlaced disabled
    #[default]
    Off = 0,

    /// Interlace enabled
    On = 1,
}

/// The display enabled flag
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum DisplayEnabled {
    /// Display enabled
    #[default]
    Enabled = 0,

    /// Display disabled
    Disabled = 1,
}

/// The interrupt request flag
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum InterruptRequest {
    /// Interrupts disabled
    #[default]
    Off = 0,

    /// Interrupts enabled
    Irq = 1,
}

/// The ready flag
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Ready {
    /// Not ready
    #[default]
    No = 0,

    /// Is ready
    Ready = 1,
}

/// The DMA direction
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum DmaDirection {
    /// Disabled mode
    #[default]
    Off = 0,

    /// Fifo direction
    Fifo = 1,

    /// Cpu to Gpu mode
    CpuToGpu = 2,

    /// Gpu to Cpu mode
    GpuToCpu = 3,
}

/// The drawing mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum DrawingMode {
    /// Even lines
    #[default]
    Even = 0,

    /// Odd lines
    Odd = 1,
}

/// The GPU component
#[derive(Clone, Debug, Default)]
pub(crate) struct Gpu {
    /// The texture page x base
    texture_page_x_base: u8,

    /// The texture page y base 1
    texture_page_y_base_1: u8,

    /// The semi transparency mode
    semi_transparency: SemiTransparency,

    /// The texture page colors
    texture_page_colors: TexturePageColors,

    /// If dithering is enabled
    dither: Dither,

    /// If drawing to the display area is allowed
    display_area_drawing: DisplayAreaDrawing,

    /// If mask bit should be set while drawing
    mask_drawing: MaskDrawing,

    /// If pixels should be drawn to masked areas
    draw_pixels: DrawPixels,

    /// If field should be interlaced
    interlace: Interlace,

    /// The reverse flag
    reverse: Reverse,

    /// The texture page y base 2
    texture_page_y_base_2: u8,

    /// The horizontal resolution
    horizontal_resolution: HorizontalResolution,

    /// The vertical resolution
    vertical_resolution: VerticalResolution,

    /// The video mdoe
    video_mode: VideoMode,

    /// The display area color depth
    display_area_color_depth: ColorDepth,

    /// If it should be vertical interlace
    vertical_interlace: VerticalInterlace,

    /// If the display is enabled
    display_enabled: DisplayEnabled,

    /// If interrupts should be requested
    interrupt_request: InterruptRequest,

    /// If it is ready to receive cmd words
    ready_receive_cmd_word: Ready,

    /// If it is ready to send vram to cpu
    ready_send_vram_to_cpu: Ready,

    /// If it is ready to receive dma blocks
    ready_receive_dma_block: Ready,

    /// The DMA direction
    dma_direction: DmaDirection,

    /// The drawing mode
    drawing_mode: DrawingMode,

    /// If the texture should be flipped on the x-axis
    texture_rectangle_x_flip: bool,

    /// If the texture should be flipped on the y-axis
    texture_rectangle_y_flip: bool,

    /// The gp0 command bytes
    gp0_bytes: [u8; 3],

    /// The gp1 command bytes
    gp1_bytes: [u8; 3],
}

impl Gpu {
    /// Creates a new GPU component
    pub(crate) fn new() -> Self {
        Self {
            display_enabled: DisplayEnabled::Disabled,
            ready_receive_cmd_word: Ready::Ready,
            ready_send_vram_to_cpu: Ready::Ready,
            ready_receive_dma_block: Ready::Ready,
            ..Default::default()
        }
    }

    /// Executes a GP0 command
    ///
    /// Arguments:
    ///
    /// * `command`: The command to execute
    pub(crate) fn gp0(&mut self, command: u32) {
        let opcode = (command >> 24) as u8;

        match opcode {
            0x00 => {
                // NOP
            }
            0xe1 => self.op_draw_mode_setting(command),
            _ => unimplemented!("gp0 command {:#010x} with opcode {:#04x}", command, opcode),
        }
    }

    /// Executes a GP1 command
    ///
    /// Arguments:
    ///
    /// * `command`: The command to execute
    fn gp1(&mut self, command: u32) {
        let opcode = (command >> 24) as u8;

        match opcode {
            0x01 => self.op_reset_command_buffer(command),
            0x02 => self.op_acknowledge_gpu_interrupt(command),
            0x03 => self.op_display_enable(command),
            0x04 => self.op_dma_direction(command),
            _ => unimplemented!("gp1 command {:#010x} with opcode {:#04x}", command, opcode),
        }
    }
}

impl Memory for Gpu {
    fn write_u8(&mut self, offset: u32, value: u8) {
        match offset {
            0x00 => {
                self.gp0_bytes[0] = value;
            }
            0x01 => {
                self.gp0_bytes[1] = value;
            }
            0x02 => {
                self.gp0_bytes[2] = value;
            }
            0x03 => {
                let byte_0 = self.gp0_bytes[0] as u32;
                let byte_1 = self.gp0_bytes[1] as u32;
                let byte_2 = self.gp0_bytes[2] as u32;
                let byte_3 = value as u32;
                let command = (byte_3 << 24) | (byte_2 << 16) | (byte_1 << 8) | byte_0;

                self.gp0(command);
            }
            0x04 => {
                self.gp1_bytes[0] = value;
            }
            0x05 => {
                self.gp1_bytes[1] = value;
            }
            0x06 => {
                self.gp1_bytes[2] = value;
            }
            0x07 => {
                let byte_0 = self.gp1_bytes[0] as u32;
                let byte_1 = self.gp1_bytes[1] as u32;
                let byte_2 = self.gp1_bytes[2] as u32;
                let byte_3 = value as u32;
                let command = (byte_3 << 24) | (byte_2 << 16) | (byte_1 << 8) | byte_0;

                self.gp1(command);
            }
            _ => unreachable!("write to gpu at {:#04x} with value {:#04x}", offset, value),
        }
    }

    fn read_u8(&self, offset: u32) -> u8 {
        match offset {
            0x00..=0x03 => {
                // TODO: Implement GPUREAD regsiter
                0x00
            }
            0x04 => {
                let mut value = 0;
                value |= self.texture_page_x_base;
                value |= self.texture_page_y_base_1 << 4;
                value |= (self.semi_transparency as u8) << 5;
                value |= (self.texture_page_colors as u8 & 0b00000001) << 7;
                value
            }
            0x05 => {
                let mut value = 0;
                value |= (self.texture_page_colors as u8 & 0b00000010) >> 1;
                value |= (self.dither as u8) << 1;
                value |= (self.display_area_drawing as u8) << 2;
                value |= (self.mask_drawing as u8) << 3;
                value |= (self.draw_pixels as u8) << 4;
                value |= (self.interlace as u8) << 5;
                value |= (self.reverse as u8) << 6;
                value |= self.texture_page_y_base_2 << 7;
                value
            }
            0x06 => {
                let mut value = 0;
                value |= match self.horizontal_resolution {
                    HorizontalResolution::S256 => 0b00000000,
                    HorizontalResolution::S320 => 0b00000010,
                    HorizontalResolution::S368 => 0b00000001,
                    HorizontalResolution::S512 => 0b00000100,
                    HorizontalResolution::S640 => 0b00000110,
                };
                value |= (self.vertical_resolution as u8) << 3;
                value |= (self.video_mode as u8) << 4;
                value |= (self.display_area_color_depth as u8) << 5;
                value |= (self.vertical_interlace as u8) << 6;
                value |= (self.display_enabled as u8) << 7;
                value
            }
            0x07 => {
                let mut value = 0;
                value |= self.interrupt_request as u8;
                value |= match self.dma_direction {
                    DmaDirection::Off => 0,
                    DmaDirection::Fifo => 1,
                    DmaDirection::CpuToGpu => self.ready_receive_dma_block as u8,
                    DmaDirection::GpuToCpu => self.ready_send_vram_to_cpu as u8,
                } << 1;
                value |= (self.ready_receive_cmd_word as u8) << 2;
                value |= (self.ready_send_vram_to_cpu as u8) << 3;
                value |= (self.ready_receive_dma_block as u8) << 4;
                value |= (self.dma_direction as u8) << 5;
                value |= (self.drawing_mode as u8) << 7;
                value
            }
            _ => unreachable!("read from gpu at {:#04x}", offset,),
        }
    }
}
