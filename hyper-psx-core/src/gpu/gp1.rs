/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::gpu::{DisplayEnabled, DmaDirection, Gpu, InterruptRequest};

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
}
