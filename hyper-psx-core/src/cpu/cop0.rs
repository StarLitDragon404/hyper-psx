/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{instruction::Instruction, Cpu};

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

        log::trace!("MFC0 {}, {}", rt, rd);

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

        log::trace!("MTC0 {}, {}", rt, rd);

        self.set_cop0_register(rd, self.register(rt));
    }
}
