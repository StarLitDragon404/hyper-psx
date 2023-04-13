/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
    cpu::{instruction::Instruction, register::Register, Cpu},
    utils::sext::SextExt,
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

        log::debug!(
            target: "cpu",
            "{}: {:#010x}: BLTZ {}, {}",
            self.n,
            instruction.1,
            rs,
            address_offset as i32
        );

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

        log::debug!(
            target: "cpu",
            "{}: {:#010x}: BGEZ {}, {}",
            self.n,
            instruction.1,
            rs,
            address_offset as i32
        );

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

        log::debug!(
            target: "cpu",
            "{}: {:#010x}: BLTZAL {}, {}",
            self.n,
            instruction.1,
            rs,
            address_offset as i32
        );

        self.set_register(Register::Ra, self.pc + 4);

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

        log::debug!(
            target: "cpu",
            "{}: {:#010x}: BGEZ {}, {}",
            self.n,
            instruction.1,
            rs,
            address_offset as i32
        );

        self.set_register(Register::Ra, self.pc + 4);

        if s >= 0 {
            self.branch(address_offset);
        }
    }
}
