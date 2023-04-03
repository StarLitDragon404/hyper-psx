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

        log::trace!("{}: {:#010x}: J {:#x}", self.n, instruction.1, address);

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

        log::trace!("{}: {:#010x}: JAL {:#x}", self.n, instruction.1, address);

        self.set_register(RegisterIndex(31), self.pc + 4);
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

        let s = self.register(rs);
        let t = self.register(rt);
        let address_offset = offset.sign_extend() << 2;

        log::trace!(
            "{}: {:#010x}: BEQ {}, {}, {}",
            self.n,
            instruction.1,
            rs,
            rt,
            address_offset as i32
        );

        if s == t {
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

        let s = self.register(rs);
        let t = self.register(rt);
        let address_offset = offset.sign_extend() << 2;

        log::trace!(
            "{}: {:#010x}: BNE {}, {}, {}",
            self.n,
            instruction.1,
            rs,
            rt,
            address_offset as i32
        );

        if s != t {
            self.branch(address_offset);
        }
    }

    /// Opcode BLEZ - Branch on Less Than Or Equal To Zero (0b000110)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=229>
    pub(super) fn op_blez(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let offset = instruction.imm();

        let s = self.register(rs) as i32;
        let address_offset = offset.sign_extend() << 2;

        log::trace!(
            "{}: {:#010x}: BGTZ {}, {}",
            self.n,
            instruction.1,
            rs,
            address_offset as i32
        );

        if s <= 0 {
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

        let s = self.register(rs) as i32;
        let address_offset = offset.sign_extend() << 2;

        log::trace!(
            "{}: {:#010x}: BGTZ {}, {}",
            self.n,
            instruction.1,
            rs,
            address_offset as i32
        );

        if s > 0 {
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

        log::trace!(
            "{}: {:#010x}: ADDI {}, {}, {}",
            self.n,
            instruction.1,
            rt,
            rs,
            value as i32
        );

        let Some(result) = (s as i32).checked_add(value as i32) else {
            panic!("Integer overflow exception");
        };

        let result = result as u32;

        self.set_register(rt, result);
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

        let s = self.register(rs);
        let value = imm.sign_extend();

        log::trace!(
            "{}: {:#010x}: ADDIU {}, {}, {}",
            self.n,
            instruction.1,
            rt,
            rs,
            value as i32
        );

        let result = s.wrapping_add(value);

        self.set_register(rt, result);
    }

    /// Opcode SLTI - Set On Less Than Immediate (0b001010)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=273>
    pub(super) fn op_slti(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let imm = instruction.imm();

        let s = self.register(rs);
        let value = imm.sign_extend();

        log::trace!(
            "{}: {:#010x}: SLTI {}, {}, {}",
            self.n,
            instruction.1,
            rt,
            rs,
            value as i32
        );

        let result = ((s as i32) < value as i32) as u32;

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

        let s = self.register(rs);
        let value = imm.zero_extend();

        log::trace!(
            "{}: {:#010x}: ANDI {}, {}, {:#x}",
            self.n,
            instruction.1,
            rt,
            rs,
            value
        );

        let result = s & value;

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

        let s = self.register(rs);
        let value = imm.zero_extend();

        log::trace!(
            "{}: {:#010x}: ORI {}, {}, {:#x}",
            self.n,
            instruction.1,
            rs,
            rt,
            value
        );

        let result = s | value;

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

        log::trace!(
            "{}: {:#010x}: LUI {}, {:#x}",
            self.n,
            instruction.1,
            rt,
            value
        );

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

        log::trace!(
            "{}: {:#010x}: LB {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to read from memory, while cache is isolated");
            return;
        }

        let result = self.bus.read_u8(address).sign_extend() as u32;

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

        log::trace!(
            "{}: {:#010x}: LW {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to read from memory, while cache is isolated");
            return;
        }

        let result = self.bus.read_u32(address);

        self.load_delay_register = Some((rt, result));
    }

    /// Opcode LBU - Load Byte Unsigned (0b100100)
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
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=245>
    pub(super) fn op_lbu(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!(
            "{}: {:#010x}: LBU {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to read from memory, while cache is isolated");
            return;
        }

        let result = self.bus.read_u8(address) as u32;

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

        let t = self.register(rt);
        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!(
            "{}: {:#010x}: SB {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to write into memory, while cache is isolated");
            return;
        }

        let result = t as u8;

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

        let t = self.register(rt);
        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!(
            "{}: {:#010x}: SH {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to write into memory, while cache is isolated");
            return;
        }

        let result = t as u16;

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

        let t = self.register(rt);
        let address_offset = offset.sign_extend();
        let address = self.register(base).wrapping_add(address_offset);

        log::trace!(
            "{}: {:#010x}: SW {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        if self.cop0_register(CopRegisterIndex(12)) & 0x10000 != 0 {
            log::warn!("Tried to write into memory, while cache is isolated");
            return;
        }

        let result = t;

        self.bus.write_u32(address, result);
    }
}
