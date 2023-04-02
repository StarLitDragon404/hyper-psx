/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{
    extension::ExtensionExt,
    instruction::Instruction,
    register_index::{CopRegisterIndex, RegisterIndex},
    Cpu,
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

    /// Opcode JAL - Jump And Link (0b000011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=241>
    pub(super) fn op_jal(&mut self, instruction: Instruction) {
        let target = instruction.target();

        let address = target << 2 | (self.pc & 0xf0000000);

        log::trace!("JAL {:#x}", address);

        self.set_register(RegisterIndex(31), self.pc);
        self.branch_delay_pc = Some(address);
    }

    /// Opcode BEQ - Branch On Equal (0b000100)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=225>
    pub(super) fn op_beq(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend() << 2;

        log::trace!("BEQ {}, {}, {}", rs, rt, address_offset as i32);

        if self.register(rs) == self.register(rt) {
            self.branch(address_offset);
        }
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

    /// Opcode BGTZ - Branch On Greater Than Zero (0b000111)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=228>
    pub(super) fn op_bgtz(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend() << 2;

        log::trace!("BGTZ {}, {}", rs, address_offset as i32);

        if self.register(rs) > 0 {
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

        log::trace!("ADDI {}, {}, {}", rt, rs, value as i32);

        let Some(result) = (s as i32).checked_add(value as i32) else {
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

        log::trace!("ADDIU {}, {}, {}", rt, rs, value as i32);

        let result = self.register(rs).wrapping_add(value);

        self.set_register(rt, result);
    }

    /// Opcode ANDI - And Immediate (0b001100)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=224>
    pub(super) fn op_andi(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let imm = instruction.imm();

        let value = imm.zero_extend();

        log::trace!("ANDI {}, {}, {:#x}", rt, rs, value);

        let result = self.register(rs) & value;

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

    /// Opcode LB - Load Byte (0b100000)
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
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=244>
    pub(super) fn op_lb(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!("LB {}, {}({})", rt, address_offset as i32, base);

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to read from memory, while cache is isolated");
            return;
        }

        let result = self.bus.read_u8(address) as u32;

        self.load_delay_register = Some((rt, result));
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

        log::trace!("LW {}, {}({})", rt, address_offset as i32, base);

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to read from memory, while cache is isolated");
            return;
        }

        let result = self.bus.read_u32(address);

        self.load_delay_register = Some((rt, result));
    }

    /// Opcode SB - Store Byte (0b101000)
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
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=268>
    pub(super) fn op_sb(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!("SB {}, {}({})", rt, address_offset as i32, base);

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to write into memory, while cache is isolated");
            return;
        }

        let result = self.register(rt) as u8;

        self.bus.write_u8(address, result);
    }

    /// Opcode SH - Store Halfword (0b101001)
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
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=269>
    pub(super) fn op_sh(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!("SH {}, {}({})", rt, address_offset as i32, base);

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to write into memory, while cache is isolated");
            return;
        }

        let result = self.register(rt) as u16;

        self.bus.write_u16(address, result);
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

        log::trace!("SW {}, {}({})", rt, address_offset as i32, base);

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to write into memory, while cache is isolated");
            return;
        }

        let result = self.register(rt);

        self.bus.write_u32(address, result);
    }
}
