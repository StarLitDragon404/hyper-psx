/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{instruction::Instruction, register_index::CopRegisterIndex, Cpu};

impl Cpu {
    /// Opcode MFC0 - Move From Coprocessor (0b00000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Coprocessor unusable exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=257>
    pub(super) fn op_mfc0(&mut self, instruction: Instruction) {
        let rt = instruction.rt();
        let rd = instruction.cop_rd();

        log::trace!("{}: {:#010x}: MFC0 {}, {}", self.n, instruction.1, rt, rd);

        self.set_register(rt, self.cop0_register(rd));
    }

    /// Opcode MTC0 - Move To Coprocessor (0b00100)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Coprocessor unusable exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=260>
    pub(super) fn op_mtc0(&mut self, instruction: Instruction) {
        let rt = instruction.rt();
        let rd = instruction.cop_rd();

        let t = self.register(rt);

        log::trace!("{}: {:#010x}: MTC0 {}, {}", self.n, instruction.1, rt, rd);

        self.set_cop0_register(rd, t);
    }

    /// Opcode RFE - Restore from Exception (0b10000/0b010000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Coprocessor unusable exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=325>
    pub(super) fn op_rfe(&mut self, instruction: Instruction) {
        log::trace!("{}: {:#010x}: RFE", self.n, instruction.1);

        let mut sr = self.cop0_register(CopRegisterIndex(12));
        let mode = sr & 0x3f;
        sr &= !0x3f;
        sr |= mode >> 2;

        self.set_cop0_register(CopRegisterIndex(12), sr);
    }
}
