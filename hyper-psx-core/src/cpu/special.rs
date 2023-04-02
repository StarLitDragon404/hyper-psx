/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{instruction::Instruction, Cpu};

impl Cpu {
    /// Opcode SLL - Shift Word Left Logical (0b000000)
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

    /// Opcode ADDU - Add Unsigned Word (0b100001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=222>
    pub(super) fn op_addu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        log::trace!("ADDU {}, {}, {}", rd, rs, rt);

        let result = self.register(rt).wrapping_add(self.register(rs));

        self.set_register(rd, result);
    }

    /// Opcode OR - Or (0b100101)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=266>
    pub(super) fn op_or(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        log::trace!("OR {}, {}, {}", rd, rs, rt);

        let result = self.register(rs) | self.register(rt);

        self.set_register(rd, result);
    }

    /// Opcode SLTU - Set On Less Than Unsigned (0b100101)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=275>
    pub(super) fn op_sltu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        log::trace!("SLTU {}, {}, {}", rd, rs, rt);

        let result = self.register(rs) < self.register(rt);

        self.set_register(rd, result as u32);
    }
}
