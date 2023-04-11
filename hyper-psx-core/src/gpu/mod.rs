/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

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
}

impl Memory for Gpu {
    fn write_u8(&mut self, offset: u32, value: u8) {
        match offset {
            0x04 => {
                self.texture_page_x_base = value & 0b00001111;
                self.texture_page_y_base_1 = (value & 0b00010000) >> 4;

                let semi_transparency = (value & 0b01100000) >> 5;
                self.semi_transparency = match semi_transparency {
                    0 => SemiTransparency::First,
                    1 => SemiTransparency::Second,
                    2 => SemiTransparency::Third,
                    3 => SemiTransparency::Fourth,
                    _ => unreachable!(),
                };

                let texture_page_colors = (value & 0b10000000) >> 7;
                self.texture_page_colors = match texture_page_colors {
                    0 => TexturePageColors::Bit4,
                    1 => TexturePageColors::Bit8,
                    _ => unreachable!(),
                }
            }
            0x05 => {
                let texture_page_colors = value & 0b00000001;
                self.texture_page_colors = match texture_page_colors {
                    0 => self.texture_page_colors,
                    1 => match self.texture_page_colors {
                        TexturePageColors::Bit4 => TexturePageColors::Bit15,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };

                let dither = (value & 0b00000010) >> 1;
                self.dither = match dither {
                    0 => Dither::Off,
                    1 => Dither::Enabled,
                    _ => unreachable!(),
                };

                let display_area_drawing = (value & 0b00000100) >> 2;
                self.display_area_drawing = match display_area_drawing {
                    0 => DisplayAreaDrawing::Prohibited,
                    1 => DisplayAreaDrawing::Allowed,
                    _ => unreachable!(),
                };

                let mask_drawing = (value & 0b00001000) >> 3;
                self.mask_drawing = match mask_drawing {
                    0 => MaskDrawing::No,
                    1 => MaskDrawing::Yes,
                    _ => unreachable!(),
                };

                let draw_pixels = (value & 0b00010000) >> 4;
                self.draw_pixels = match draw_pixels {
                    0 => DrawPixels::Always,
                    1 => DrawPixels::Unmasked,
                    _ => unreachable!(),
                };

                let interlace = (value & 0b00100000) >> 5;
                self.interlace = match interlace {
                    0 => Interlace::Never,
                    1 => Interlace::Always,
                    _ => unreachable!(),
                };

                let reverse = (value & 0b01000000) >> 6;
                self.reverse = match reverse {
                    0 => Reverse::Normal,
                    1 => Reverse::Distorted,
                    _ => unreachable!(),
                };

                self.texture_page_y_base_2 = (value & 0b10000000) >> 7;
            }
            0x06 => {
                let horizontal_resolution = value & 0b00000001;
                self.horizontal_resolution = match horizontal_resolution {
                    0 => match (value & 0b00000010) >> 1 {
                        0 => HorizontalResolution::S256,
                        1 => HorizontalResolution::S320,
                        2 => HorizontalResolution::S512,
                        3 => HorizontalResolution::S640,
                        _ => unreachable!(),
                    },
                    1 => HorizontalResolution::S368,
                    _ => unreachable!(),
                };

                let vertical_resolution =
                    ((value & 0b00001000) >> 3) == 1 && ((value & 0b01000000) >> 6) == 1;
                self.vertical_resolution = match vertical_resolution {
                    false => VerticalResolution::S240,
                    true => VerticalResolution::S480,
                };

                let video_mode = (value & 0b00010000) >> 4;
                self.video_mode = match video_mode {
                    0 => VideoMode::Hz60,
                    1 => VideoMode::Hz50,
                    _ => unreachable!(),
                };

                let display_area_color_depth = (value & 0b00100000) >> 5;
                self.display_area_color_depth = match display_area_color_depth {
                    0 => ColorDepth::Bit15,
                    1 => ColorDepth::Bit24,
                    _ => unreachable!(),
                };

                let vertical_interlace = (value & 0b01000000) >> 6;
                self.vertical_interlace = match vertical_interlace {
                    0 => VerticalInterlace::Off,
                    1 => VerticalInterlace::On,
                    _ => unreachable!(),
                };

                let display_enabled = (value & 0b10000000) >> 7;
                self.display_enabled = match display_enabled {
                    0 => DisplayEnabled::Enabled,
                    1 => DisplayEnabled::Disabled,
                    _ => unreachable!(),
                };
            }
            0x07 => {
                let interrupt_request = value & 0b00000001;
                self.interrupt_request = match interrupt_request {
                    0 => InterruptRequest::Off,
                    1 => InterruptRequest::Irq,
                    _ => unreachable!(),
                };

                let ready_receive_cmd_word = (value & 0b00000100) >> 2;
                self.ready_receive_cmd_word = match ready_receive_cmd_word {
                    0 => Ready::No,
                    1 => Ready::Ready,
                    _ => unreachable!(),
                };

                let ready_send_vram_to_cpu = (value & 0b00001000) >> 3;
                self.ready_send_vram_to_cpu = match ready_send_vram_to_cpu {
                    0 => Ready::No,
                    1 => Ready::Ready,
                    _ => unreachable!(),
                };

                let ready_receive_dma_block = (value & 0b00010000) >> 4;
                self.ready_receive_dma_block = match ready_receive_dma_block {
                    0 => Ready::No,
                    1 => Ready::Ready,
                    _ => unreachable!(),
                };

                let dma_direction = (value & 0b01100000) >> 5;
                self.dma_direction = match dma_direction {
                    0 => DmaDirection::Off,
                    1 => DmaDirection::Fifo,
                    2 => DmaDirection::CpuToGpu,
                    3 => DmaDirection::GpuToCpu,
                    _ => unreachable!(),
                };

                let drawing_mode = (value & 0b10000000) >> 7;
                self.drawing_mode = match drawing_mode {
                    0 => DrawingMode::Even,
                    1 => DrawingMode::Odd,
                    _ => unreachable!(),
                };
            }
            _ => unreachable!("write to gpu at {:#04x} with value {:#04x}", offset, value),
        }
    }

    fn read_u8(&self, offset: u32) -> u8 {
        let mut value = 0;

        match offset {
            0x04 => {
                value |= self.texture_page_x_base;
                value |= self.texture_page_y_base_1 << 4;
                value |= (self.semi_transparency as u8) << 5;
                value |= (self.texture_page_colors as u8 & 0b00000001) << 7;
            }
            0x05 => {
                value |= (self.texture_page_colors as u8 & 0b00000010) >> 1;
                value |= (self.dither as u8) << 1;
                value |= (self.display_area_drawing as u8) << 2;
                value |= (self.mask_drawing as u8) << 3;
                value |= (self.draw_pixels as u8) << 4;
                value |= (self.interlace as u8) << 5;
                value |= (self.reverse as u8) << 6;
                value |= self.texture_page_y_base_2 << 7;
            }
            0x06 => {
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
            }
            0x07 => {
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
            }
            _ => unreachable!("read from gpu at {:#04x}", offset,),
        };

        value
    }
}
