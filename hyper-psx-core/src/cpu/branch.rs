/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{extension::ExtensionExt, instruction::Instruction, Cpu};

impl Cpu {
    /// Opcode BLTZ - Branch On Less Than Zero (0b00000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=230>
    pub(super) fn op_bltz(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend() << 2;

        log::trace!("BLTZ {}, {}", rs, address_offset as i32);

        if (self.register(rs) as i32) < 0 {
            self.branch(address_offset);
        }
    }
}
