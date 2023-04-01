/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{instruction::Instruction, Cpu};

impl Cpu {
    /// Opcode SLL - Shift Word Left Logical (0b000000/0b000000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=270>
    pub(super) fn op_sll(&mut self, instruction: Instruction) {
        let rt = instruction.rt();
        let rd = instruction.rd();
        let sa = instruction.shamt();

        log::trace!("SLL {}, {}, {:#x}", rd, rt, sa);

        let result = self.register(rt) << sa;

        self.set_register(rd, result);
    }
}