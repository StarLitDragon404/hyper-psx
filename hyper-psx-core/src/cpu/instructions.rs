/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{
    extension::ExtensionExt, instruction::Instruction, register_index::CopRegisterIndex, Cpu,
};

impl Cpu {
    /// Opcode J - Jump (0b000010)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=240>
    pub(super) fn op_j(&mut self, instruction: Instruction) {
        let target = instruction.target();

        let address = target << 2 | (self.pc & 0xf0000000);

        log::trace!("J {:#x}", address);

        self.branch_delay_pc = Some(address);
    }

    /// Opcode BNE - Branch On Not Equal (0b000101)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=232>
    pub(super) fn op_bne(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend() << 2;

        log::trace!("BNE {}, {}, {}", rs, rt, address_offset as i32);

        if self.register(rs) != self.register(rt) {
            self.branch(address_offset);
        }
    }

    /// Opcode ADDI - Add Immediate Word (0b001000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Integer overflow exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=220>
    pub(super) fn op_addi(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let imm = instruction.imm();

        let s = self.register(rs);
        let value = imm.sign_extend();

        log::trace!("ADDI {}, {}, {:#x}", rt, rs, value);

        let Some(result) = (s as i32).checked_add(imm as i32) else {
            panic!("Integer overflow exception");
        };

        self.set_register(rt, result as u32);
    }

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
        let imm = instruction.imm();

        let value = imm.sign_extend();

        log::trace!("ADDIU {}, {}, {:#x}", rt, rs, value);

        let result = self.register(rs).wrapping_add(value);

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
        let imm = instruction.imm();

        let value = imm.zero_extend();

        log::trace!("ORI {}, {}, {:#x}", rs, rt, value);

        let result = self.register(rs) | value;

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
        let imm = instruction.imm();

        let value = imm.zero_extend();

        log::trace!("LUI {}, {:#x}", rt, value);

        let result = value << 16;

        self.set_register(rt, result);
    }

    /// Opcode LW - Load Word (0b100011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * TLB refill exception
    /// * TLB invalid exception
    /// * Bus error exception
    /// * Address error exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=249>
    pub(super) fn op_lw(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!("LW {}, {}({})", rt, offset, base);

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to read from memory, while cache is isolated");
            return;
        }

        let result = self.bus.read_u32(address);

        self.load_delay_register = Some((rt, result));
    }

    /// Opcode SW - Store Word (0b101011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * TLB refill exception
    /// * TLB invalid exception
    /// * TLB modification exception
    /// * Bus error exception
    /// * Address error exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=282>
    pub(super) fn op_sw(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!("SW {}, {}({})", rt, offset, base);

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to write into memory, while cache is isolated");
            return;
        }

        let result = self.register(rt);

        self.bus.write_u32(address, result);
    }
}
