/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::gpu::{DisplayAreaDrawing, Dither, Gpu, SemiTransparency, TexturePageColors};

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
}
