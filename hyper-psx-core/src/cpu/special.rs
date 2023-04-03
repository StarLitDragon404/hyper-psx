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

        let t = self.register(rt);

        log::trace!("SLL {}, {}, {:#x}", rd, rt, sa);

        let result = t << sa;

        self.set_register(rd, result);
    }

    /// Opcode SRA - Shift Word Right Arithmetic (0b000000)
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

        let t = self.register(rt);

        log::trace!("SRA {}, {}, {:#x}", rd, rt, sa);

        let result = ((t as i32) >> sa) as u32;

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

        log::trace!("JR {}", rs);

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

        log::trace!("JALR {}", rs);

        let address = self.register(rs);

        self.set_register(rd, self.pc);
        self.branch_delay_pc = Some(address);
    }

    /// Opcode MFLO - Move From Lo (0b010010)
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The current instruction data
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=259>
    pub(super) fn op_mflo(&mut self, instruction: Instruction) {
        let rd = instruction.rd();

        log::trace!("MFLO {}", rd);

        let result = self.lo;

        self.set_register(rd, result);
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

        log::trace!("DIV {}, {}", rs, rt);

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

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!("ADD {}, {}, {}", rd, rs, rt);

        let Some(result) = (s as i32).checked_add(t as i32) else {
            panic!("Integer overflow exception");
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

        log::trace!("ADDU {}, {}, {}", rd, rs, rt);

        let result = s.wrapping_add(t);

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

        log::trace!("SUBU {}, {}, {}", rd, rs, rt);

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

        log::trace!("AND {}, {}, {}", rd, rs, rt);

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

        log::trace!("OR {}, {}, {}", rd, rs, rt);

        let result = s | t;

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

        let s = self.register(rs);
        let t = self.register(rt);

        log::trace!("SLTU {}, {}, {}", rd, rs, rt);

        let result = (s < t) as u32;

        self.set_register(rd, result);
    }
}
