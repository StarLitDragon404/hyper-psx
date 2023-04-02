/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod cop0;
mod extension;
mod instruction;
mod instructions;
mod register_index;
mod special;

use crate::{
    bus::Bus,
    cpu::{
        instruction::Instruction,
        register_index::{CopRegisterIndex, RegisterIndex},
    },
};

/// The CPU component
#[derive(Clone, Debug)]
pub(crate) struct Cpu {
    /// The 32 general purpose registers
    registers: [u32; 32],

    /// The 64 cop registers
    cop_registers: [u32; 64],

    /// The program counter
    pc: u32,

    /// The branch delay program counter
    branch_delay_pc: Option<u32>,

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
            cop_registers: [0x00000000; 64],
            pc: 0xbfc00000,
            branch_delay_pc: None,
            bus,
        }
    }

    /// Steps the next instruction
    pub(crate) fn step(&mut self) {
        let instruction = Instruction(self.bus.read_u32(self.pc));

        log::debug!("PC: {:#x}", self.pc);

        self.pc += 4;

        if self.branch_delay_pc.is_some() {
            self.pc = self.branch_delay_pc.unwrap();
            self.branch_delay_pc = None;
        }

        self.execute(instruction);
    }

    /// Executes an instruction
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The instruction to be executed
    fn execute(&mut self, instruction: Instruction) {
        match instruction.op() {
            0b000000 => match instruction.funct() {
                0b000000 => self.op_sll(instruction),
                0b100101 => self.op_or(instruction),
                _ => unimplemented!(
                    "special instruction {:#010x} with opcode {:#08b} and {:#08b}",
                    instruction.0,
                    instruction.op(),
                    instruction.funct()
                ),
            },
            0b000010 => self.op_j(instruction),
            0b000101 => self.op_bne(instruction),
            0b001000 => self.op_addi(instruction),
            0b001001 => self.op_addiu(instruction),
            0b001101 => self.op_ori(instruction),
            0b001111 => self.op_lui(instruction),
            0b010000 => match instruction.cop_op() {
                0b00100 => self.op_mtc0(instruction),
                _ => unimplemented!(
                    "cop0 instruction {:#010x} with opcode {:#08b}",
                    instruction.0,
                    instruction.cop_op()
                ),
            },
            0b010001 => unimplemented!(
                "cop1 instruction {:#010x} with opcode {:#08b}",
                instruction.0,
                instruction.cop_op()
            ),
            0b010010 => unimplemented!(
                "cop2 instruction {:#010x} with opcode {:#08b}",
                instruction.0,
                instruction.cop_op()
            ),
            0b010011 => unimplemented!(
                "cop3 instruction {:#010x} with opcode {:#08b}",
                instruction.0,
                instruction.cop_op()
            ),
            0b101011 => self.op_sw(instruction),
            _ => unimplemented!(
                "instruction {:#010x} with opcode {:#08b}",
                instruction.0,
                instruction.op()
            ),
        }
    }

    /// Branches to an offset
    ///
    /// # Arguments:
    ///
    /// * `offset`: The relative offset
    fn branch(&mut self, offset: u32) {
        let address = self.pc.wrapping_add(offset).wrapping_sub(4);
        self.branch_delay_pc = Some(address);
    }

    /// Sets a register to a value
    ///
    /// # Arguments:
    ///
    /// * `register_index`: The register to be set
    /// * `value`: The value for the regsiter
    fn set_register(&mut self, register_index: RegisterIndex, value: u32) {
        assert!(register_index.0 < 32);
        self.registers[register_index.0 as usize] = value;
    }

    /// Gets a value from a register
    ///
    /// # Arguments:
    ///
    /// * `register_index`: The register to be read from
    fn register(&self, register_index: RegisterIndex) -> u32 {
        assert!(register_index.0 < 32);
        self.registers[register_index.0 as usize]
    }

    /// Sets a cop register to a value
    ///
    /// # Arguments:
    ///
    /// * `cop_register_index`: The cop register to be set
    /// * `value`: The value for the regsiter
    fn set_cop_register(&mut self, cop_register_index: CopRegisterIndex, value: u32) {
        assert!(cop_register_index.0 < 64);
        self.cop_registers[cop_register_index.0 as usize] = value;
    }

    /// Gets a value from a cop register
    ///
    /// # Arguments:
    ///
    /// * `cop_register_index`: The cop register to be read from
    fn cop_register(&self, cop_register_index: CopRegisterIndex) -> u32 {
        assert!(cop_register_index.0 < 64);
        self.cop_registers[cop_register_index.0 as usize]
    }
}
