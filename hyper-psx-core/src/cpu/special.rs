/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{exception::Exception, instruction::Instruction, Cpu};

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

        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: SLL {}, {}, {:#x}",
            self.n,
            instruction.1,
            rd,
            rt,
            sa
        );

        let result = t << sa;

        self.set_register(rd, result);
    }

    /// Opcode SRL - Shift Word Right Logical (0b000010)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=278>
    pub(super) fn op_srl(&mut self, instruction: Instruction) {
        let rt = instruction.rt();
        let rd = instruction.rd();
        let sa = instruction.shamt();

        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: SRL {}, {}, {:#x}",
            self.n,
            instruction.1,
            rd,
            rt,
            sa
        );

        let result = t >> sa;

        self.set_register(rd, result);
    }

    /// Opcode SRA - Shift Word Right Arithmetic (0b000011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=276>
    pub(super) fn op_sra(&mut self, instruction: Instruction) {
        let rt = instruction.rt();
        let rd = instruction.rd();
        let sa = instruction.shamt();

        let t = self.register(rt) as i32;

        log::trace!(
            "{}: {:#010x}: SRA {}, {}, {:#x}",
            self.n,
            instruction.1,
            rd,
            rt,
            sa
        );

        let result = (t >> sa) as u32;

        self.set_register(rd, result);
    }

    /// Opcode SLLV - Shift Word Left Logical Variable (0b000100)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=271>
    pub(super) fn op_sllv(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let t = self.register(rt);
        let s = self.register(rs);

        log::trace!(
            "{}: {:#010x}: SLLV {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rt,
            rs
        );

        let result = t << (s & 0x1f);

        self.set_register(rd, result);
    }

    /// Opcode SRLV - Shift Word Right Logical Variable (0b000110)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=279>
    pub(super) fn op_srlv(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let t = self.register(rt);
        let s = self.register(rs);

        log::trace!(
            "{}: {:#010x}: SRLV {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rt,
            rs
        );

        let result = t >> (s & 0x1f);

        self.set_register(rd, result);
    }

    /// Opcode SRAV - Shift Word Right Arithmetic Variable (0b000100)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=277>
    pub(super) fn op_srav(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let t = self.register(rt) as i32;
        let s = self.register(rs);

        log::trace!(
            "{}: {:#010x}: SLLV {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rt,
            rs
        );

        let result = (t >> (s & 0x1f)) as u32;

        self.set_register(rd, result);
    }

    /// Opcode JR - Jump Register (0b001000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Address error exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=243>
    pub(super) fn op_jr(&mut self, instruction: Instruction) {
        let rs = instruction.rs();

        log::trace!("{}: {:#010x}: JR {}", self.n, instruction.1, rs);

        let address = self.register(rs);

        self.branch_delay_pc = Some(address);
    }

    /// Opcode JALR - Jump And Link Register (0b001001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Address error exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=242>
    pub(super) fn op_jalr(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rd = instruction.rd();

        log::trace!("{}: {:#010x}: JALR {}", self.n, instruction.1, rs);

        let address = self.register(rs);

        self.set_register(rd, self.pc);
        self.branch_delay_pc = Some(address);
    }

    /// Opcode SYSCALL - System Call (0b001100)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * System Call exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=288>
    pub(super) fn op_syscall(&mut self, instruction: Instruction) {
        log::trace!("{}: {:#010x}: SYSCALL", self.n, instruction.1);

        self.raise_exception(instruction, Exception::Syscall);
    }

    /// Opcode BREAK - Breakpoint (0b001101)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Breakpoint exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=233>
    pub(super) fn op_break(&mut self, instruction: Instruction) {
        log::trace!("{}: {:#010x}: BREAK", self.n, instruction.1);

        self.raise_exception(instruction, Exception::Bp);
    }

    /// Opcode MFHI - Move From HI (0b010000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=258>
    pub(super) fn op_mfhi(&mut self, instruction: Instruction) {
        let rd = instruction.rd();

        log::trace!("{}: {:#010x}: MFHI {}", self.n, instruction.1, rd);

        let result = self.hi;

        self.set_register(rd, result);
    }

    /// Opcode MTHI - Move To HI (0b010001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=261>
    pub(super) fn op_mthi(&mut self, instruction: Instruction) {
        let rs = instruction.rs();

        log::trace!("{}: {:#010x}: MTHI {}", self.n, instruction.1, rs);

        let result = self.register(rs);

        self.hi = result;
    }

    /// Opcode MFLO - Move From LO (0b010010)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=259>
    pub(super) fn op_mflo(&mut self, instruction: Instruction) {
        let rd = instruction.rd();

        log::trace!("{}: {:#010x}: MFLO {}", self.n, instruction.1, rd);

        let result = self.lo;

        self.set_register(rd, result);
    }

    /// Opcode MTLO - Move To LO (0b010011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=262>
    pub(super) fn op_mtlo(&mut self, instruction: Instruction) {
        let rs = instruction.rs();

        log::trace!("{}: {:#010x}: MTLO {}", self.n, instruction.1, rs);

        let result = self.register(rs);

        self.lo = result;
    }

    /// Opcode MULT - Multiply Word (0b011001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=263>
    pub(super) fn op_mult(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();

        let s = self.register(rs) as i32 as i64;
        let t = self.register(rt) as i32 as i64;

        log::debug!("{}: {:#010x}: MULT {}, {}", self.n, instruction.1, rs, rt);

        let result = (s * t) as u64;

        self.hi = (result >> 32) as u32;
        self.lo = result as u32;
    }

    /// Opcode MULTU - Multiply Unsigned Word (0b011001)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=264>
    pub(super) fn op_multu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();

        let s = self.register(rs) as u64;
        let t = self.register(rt) as u64;

        log::trace!("{}: {:#010x}: MULTU {}, {}", self.n, instruction.1, rs, rt);

        let result = s * t;

        self.hi = (result >> 32) as u32;
        self.lo = result as u32;
    }

    /// Opcode DIV - Divide Word (0b011010)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=237>
    pub(super) fn op_div(&mut self, instruction: Instruction) {
        // TODO: Implement proper timing

        let rs = instruction.rs();
        let rt = instruction.rt();

        // The number to multiply or divide
        let s = self.register(rs) as i32;

        // The number to multiply with or to divide with
        let t = self.register(rt) as i32;

        log::trace!("{}: {:#010x}: DIV {}, {}", self.n, instruction.1, rs, rt);

        if t == 0 {
            // Division by zero
            self.hi = s as u32;
            self.lo = if s >= 0 { 0xffffffff } else { 1 };
        } else if s as u32 == 0x80000000 && t == -1 {
            // Result is greater than u32
            self.hi = 0;
            self.lo = 0x80000000;
        } else {
            self.hi = (s % t) as u32;
            self.lo = (s / t) as u32;
        }
    }

    /// Opcode DIVU - Divide Unsigned Word (0b011011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=237>
    pub(super) fn op_divu(&mut self, instruction: Instruction) {
        // TODO: Implement proper timing

        let rs = instruction.rs();
        let rt = instruction.rt();

        // The number to multiply or divide
        let s = self.register(rs);

        // The number to multiply with or to divide with
        let t = self.register(rt);

        log::trace!("{}: {:#010x}: DIVU {}, {}", self.n, instruction.1, rs, rt);

        if t == 0 {
            // Division by zero
            self.hi = s;
            self.lo = 0xffffffff;
        } else {
            self.hi = s % t;
            self.lo = s / t;
        }
    }

    /// Opcode ADD - Add Word (0b100000)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// # Exceptions:
    ///
    /// * Integer overflow exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=219>
    pub(super) fn op_add(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.register(rs) as i32;
        let t = self.register(rt) as i32;

        log::trace!(
            "{}: {:#010x}: ADD {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let Some(result) = s.checked_add(t) else {
            self.raise_exception(instruction, Exception::Ov);
            return;
        };

        let result = result as u32;

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

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: ADDU {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = s.wrapping_add(t);

        self.set_register(rd, result);
    }

    /// Opcode SUB - Subtract Word (0b100011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=280>
    pub(super) fn op_sub(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.register(rs) as i32;
        let t = self.register(rt) as i32;

        log::trace!(
            "{}: {:#010x}: SUB {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let Some(result) = s.checked_sub(t) else {
            self.raise_exception(instruction, Exception::Ov);
            return;
        };

        let result = result as u32;

        self.set_register(rd, result);
    }

    /// Opcode SUBU - Subtract Unsigned Word (0b100011)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=281>
    pub(super) fn op_subu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: SUBU {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = s.wrapping_sub(t);

        self.set_register(rd, result);
    }

    /// Opcode AND - And (0b100100)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=223>
    pub(super) fn op_and(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: AND {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = s & t;

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

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: OR {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = s | t;

        self.set_register(rd, result);
    }

    /// Opcode XOR - Exclusive Or (0b100111)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=289>
    pub(super) fn op_xor(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: XOR {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = s ^ t;

        self.set_register(rd, result);
    }

    /// Opcode NOR - Nor (0b100111)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=265>
    pub(super) fn op_nor(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: NOR {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = !(s | t);

        self.set_register(rd, result);
    }

    /// Opcode SLT - Set On Less Than (0b101010)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=272>
    pub(super) fn op_slt(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.register(rs) as i32;
        let t = self.register(rt) as i32;

        log::trace!(
            "{}: {:#010x}: SLT {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = (s < t) as u32;

        self.set_register(rd, result);
    }

    /// Opcode SLTU - Set On Less Than Unsigned (0b101011)
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

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!(
            "{}: {:#010x}: SLTU {}, {}, {}",
            self.n,
            instruction.1,
            rd,
            rs,
            rt
        );

        let result = (s < t) as u32;

        self.set_register(rd, result);
    }
}
