/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::gpu::{Gpu, InterruptRequest};

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
}
