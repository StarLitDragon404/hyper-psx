/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bus::Bus;

#[derive(Clone, Copy, Debug)]
struct Instruction(u32);

impl Instruction {
    fn op(&self) -> u8 {
        (self.0 >> 26) as u8
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Cpu {
    pc: u32,

    bus: Bus,
}

impl Cpu {
    pub(crate) fn new(bus: Bus) -> Self {
        Self {
            pc: 0xbfc00000,
            bus,
        }
    }

    pub(crate) fn step(&mut self) {
        let instruction = Instruction(self.bus.read_u32(self.pc));
        self.execute(instruction);
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.op() {
            _ => unimplemented!(
                "instruction {:#010x} with opcode {:#08b}",
                instruction.0,
                instruction.op()
            ),
        }
    }
}
