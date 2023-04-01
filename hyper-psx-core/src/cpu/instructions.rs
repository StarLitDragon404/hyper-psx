/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{extension::ExtensionExt, instruction::Instruction, Cpu};

impl Cpu {
    /// Opcode ADDIU - Add Immediate Unsigned Word (0b001001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=221>
    pub(super) fn op_addiu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let imm = instruction.imm().sign_extend();

        log::trace!("ADDIU {}, {}, {:#x}", rt, rs, imm);

        let result = self.register(rs).wrapping_add(imm);

        self.set_register(rt, result);
    }

    /// Opcode ORI - Or Immediate (0b001101)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=267>
    pub(super) fn op_ori(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let imm = instruction.imm().zero_extend();

        log::trace!("ORI {}, {}, {:#x}", rs, rt, imm);

        let result = self.register(rs) | imm;

        self.set_register(rt, result);
    }

    /// Opcode LUI - Load Upper Immediate (0b001111)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=248>
    pub(super) fn op_lui(&mut self, instruction: Instruction) {
        let rt = instruction.rt();
        let imm = instruction.imm() as u32;

        log::trace!("LUI {}, {:#x}", rt, imm);

        let result = imm << 16;

        self.set_register(rt, result);
    }

    /// Opcode SW - Store Word (0b101011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=282>
    pub(super) fn op_sw(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm().sign_extend();

        log::trace!("SW {}, {:#x}({})", rt, offset, base);

        let address = self.register(base).wrapping_add(offset);
        let result = self.register(rt);

        self.bus.write_u32(address, result);
    }
}
