/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod instruction;
mod instructions;

use crate::{bus::Bus, cpu::instruction::Instruction};

#[derive(Clone, Debug)]
pub(crate) struct Cpu {
    registers: [u32; 32],
    pc: u32,

    bus: Bus,
}

impl Cpu {
    pub(crate) fn new(bus: Bus) -> Self {
        Self {
            registers: [0x00000000; 32],
            pc: 0xbfc00000,
            bus,
        }
    }

    pub(crate) fn step(&mut self) {
        let instruction = Instruction(self.bus.read_u32(self.pc));
        self.pc += 4;

        self.execute(instruction);
    }

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

    fn set_register(&mut self, register_index: u8, value: u32) {
        self.registers[register_index as usize] = value;
    }
}
