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

        log::trace!("ADD {}, {}, {}", rt, rs, rt);

        let Some(result) = (s as i32).checked_add(t as i32) else {
            panic!("Integer overflow exception");
        };

        self.set_register(rd, result as u32);
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

        log::trace!("SUBU {}, {}, {}", rd, rs, rt);

        let result = self.register(rt).wrapping_sub(self.register(rs));

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

        log::trace!("AND {}, {}, {}", rd, rs, rt);

        let result = self.register(rs) & self.register(rt);

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
