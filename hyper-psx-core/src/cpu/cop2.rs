/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::cpu::{extension::ExtensionExt, instruction::Instruction, Cpu};

impl Cpu {
    /// Opcode LWC2 - Load Word From Coprocessor (0b110010)
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
    /// * Coprocessor unusable exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=250>
    pub(super) fn op_lwc2(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let _t = self.register(rt);
        let address_offset = offset.sign_extend();
        let _address = self.register(base).wrapping_add(address_offset);

        log::debug!(
            target: "cpu",
            "{}: {:#010x}: LWC2 {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        todo!();
    }

    /// Opcode SWC2 - Store Word From Coprocessor (0b111010)
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
    /// * Coprocessor unusable exception
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=283>
    pub(super) fn op_swc2(&mut self, instruction: Instruction) {
        let base = instruction.rs();
        let rt = instruction.rt();
        let offset = instruction.imm();

        let _t = self.register(rt);
        let address_offset = offset.sign_extend();
        let _address = self.register(base).wrapping_add(address_offset);

        log::debug!(
            target: "cpu",
            "{}: {:#010x}: SWC2 {}, {}({})",
            self.n,
            instruction.1,
            rt,
            address_offset as i32,
            base
        );

        todo!();
    }
}
