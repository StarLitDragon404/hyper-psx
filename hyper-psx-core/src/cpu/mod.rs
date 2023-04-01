/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod instruction;
mod instructions;

use crate::{bus::Bus, cpu::instruction::Instruction};

/// The CPU component
#[derive(Clone, Debug)]
pub(crate) struct Cpu {
    /// The 32 general purpose registers
    registers: [u32; 32],

    /// The program counter
    pc: u32,

    /// The Bus component
    bus: Bus,
}

impl Cpu {
    /// Creates a CPU Component
    ///
    /// # Arguments:
    ///
    /// * `bus`: The Bus component
    pub(crate) fn new(bus: Bus) -> Self {
        Self {
            registers: [0x00000000; 32],
            pc: 0xbfc00000,
            bus,
        }
    }

    /// Steps the next instruction
    pub(crate) fn step(&mut self) {
        let instruction = Instruction(self.bus.read_u32(self.pc));
        self.pc += 4;

        self.execute(instruction);
    }

    /// Executes an instruction
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The instruction to be executed
    fn execute(&mut self, instruction: Instruction) {
        match instruction.op() {
            0b001111 => self.op_lui(instruction),
            _ => unimplemented!(
                "instruction {:#010x} with opcode {:#08b}",
                instruction.0,
                instruction.op()
            ),
        }
    }

    /// Sets a register to a value
    ///
    /// # Arguments:
    ///
    /// * `register_index`: The register to be set
    /// * `value`: The value for the regsiter
    fn set_register(&mut self, register_index: u8, value: u32) {
        self.registers[register_index as usize] = value;
    }
}
