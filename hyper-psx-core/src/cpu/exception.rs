/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{instruction::Instruction, register_index::CopRegisterIndex, Cpu};

/// The exception types of the PSX
///
/// <https://psx-spx.consoledev.net/cpuspecifications/#cop0r13-cause-read-only-except-bit8-9-are-rw>
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub(super) enum Exception {
    // Interrupt
    Int = 0x00,

    /// Tlb modification
    Mod = 0x01,

    /// Tlb load
    Tlbl = 0x02,

    /// Tlb store
    Tlbs = 0x03,

    /// Address error, Data load or Instruction fetch
    Adel = 0x04,

    /// Address error, Data store
    Ades = 0x05,

    /// Bus error on Instruction fetch
    Ibe = 0x06,

    /// Bus error on Data load/store
    Dbe = 0x07,

    /// Generated unconditionally by syscall instruction
    Syscall = 0x08,

    /// Breakpoint - break instruction
    Bp = 0x09,

    /// Reserved instruction
    Ri = 0x0a,

    /// Coprocessor unusable
    Cpu = 0x0b,

    /// Arithmetic overflow
    Ov = 0x0c,
}

impl Cpu {
    /// Raises an exception immediately
    ///
    /// # Arguments:
    ///
    /// * `exception`: The exception to raise
    pub(super) fn raise_exception(&mut self, instruction: Instruction, exception: Exception) {
        let mut cause = self.cop0_register(CopRegisterIndex(13));

        // Set BD if in branch delay
        let bd = instruction.1 != (self.pc - 4);
        cause |= 1 << 31;

        let pc = instruction.1 - if bd { 4 } else { 0 };

        // Set EPC to PC
        self.set_cop0_register(CopRegisterIndex(14), pc);

        // Set Exception ID in CAUSE
        cause |= (exception as u32) << 2;
        self.set_cop0_register(CopRegisterIndex(13), cause);

        // Shift enable bits left in SR
        let mut sr = self.cop0_register(CopRegisterIndex(12));

        let bev = (sr & (1 << 22)) != 0;

        let mode = sr & 0x3f;
        sr &= !0x3f;
        sr |= (mode << 2) & 0x3f;
        self.set_cop0_register(CopRegisterIndex(12), sr);

        // Call the exception handler
        let handler = if bev { 0xbfc00180 } else { 0x80000080 };

        self.pc = handler;
    }
}
