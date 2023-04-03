/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{
    extension::ExtensionExt, instruction::Instruction, register_index::RegisterIndex, Cpu,
};

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

        let s = self.register(rs) as i32;
        let address_offset = offset.sign_extend() << 2;

        log::trace!("BLTZ {}, {}", rs, address_offset as i32);

        if s < 0 {
            self.branch(address_offset);
        }
    }

    /// Opcode BGEZ - Branch On Greater Than Or Equal To Zero (0b00001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=226>
    pub(super) fn op_bgez(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let offset = instruction.imm();

        let s = self.register(rs) as i32;
        let address_offset = offset.sign_extend() << 2;

        log::trace!("BGEZ {}, {}", rs, address_offset as i32);

        if s >= 0 {
            self.branch(address_offset);
        }
    }

    /// Opcode BLTZAL - Branch On Less Than Zero And Link (0b10000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=231>
    pub(super) fn op_bltzal(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let offset = instruction.imm();

        let s = self.register(rs) as i32;
        let address_offset = offset.sign_extend() << 2;

        log::trace!("BLTZAL {}, {}", rs, address_offset as i32);

        self.set_register(RegisterIndex(31), self.pc);

        if s < 0 {
            self.branch(address_offset);
        }
    }

    /// Opcode BGEZAL - Branch On Greater Than Or Equal To Zero And Link (0b10001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=227>
    pub(super) fn op_bgezal(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let offset = instruction.imm();

        let s = self.register(rs) as i32;
        let address_offset = offset.sign_extend() << 2;

        log::trace!("BGEZ {}, {}", rs, address_offset as i32);

        self.set_register(RegisterIndex(31), self.pc);

        if s >= 0 {
            self.branch(address_offset);
        }
    }
}
