/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod branch;
mod cop0;
mod exception;
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

    /// The 32 general purpose output registers
    out_registers: [u32; 32],

    /// The load delay register
    load_delay_register: Option<(RegisterIndex, u32)>,

    /// The high register for division remainder and multiplication result
    hi: u32,

    /// The low register for division quotient and multiplication result
    lo: u32,

    /// The 64 cop registers
    cop0_registers: [u32; 64],

    /// The program counter
    pc: u32,

    /// The branch delay program counter
    branch_delay_pc: Option<u32>,

    /// The Bus component
    bus: Bus,

    n: usize,
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
            out_registers: [0x00000000; 32],
            hi: 0x00000000,
            lo: 0x00000000,
            load_delay_register: None,
            cop0_registers: [0x00000000; 64],
            pc: 0xbfc00000,
            branch_delay_pc: None,
            bus,
            n: 0,
        }
    }

    /// Steps the next instruction
    pub(crate) fn step(&mut self) {
        if self.pc % 4 != 0 {
            panic!("unaligned pc");
        }

        let instruction = Instruction(self.bus.read_u32(self.pc), self.pc);
        self.pc += 4;
        self.n += 1;

        if self.branch_delay_pc.is_some() {
            let branch_pc = self.branch_delay_pc.take().unwrap();
            self.pc = branch_pc;
        }

        if self.load_delay_register.is_some() {
            let load_register = self.load_delay_register.take().unwrap();
            self.set_register(load_register.0, load_register.1);
        }

        self.execute(instruction);

        self.registers = self.out_registers;
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
                0b000010 => self.op_srl(instruction),
                0b000011 => self.op_sra(instruction),
                0b000100 => self.op_sllv(instruction),
                0b000111 => self.op_srav(instruction),
                0b001000 => self.op_jr(instruction),
                0b001001 => self.op_jalr(instruction),
                0b001100 => self.op_syscall(instruction),
                0b010000 => self.op_mfhi(instruction),
                0b010001 => self.op_mthi(instruction),
                0b010010 => self.op_mflo(instruction),
                0b010011 => self.op_mtlo(instruction),
                0b011010 => self.op_div(instruction),
                0b011011 => self.op_divu(instruction),
                0b100000 => self.op_add(instruction),
                0b100001 => self.op_addu(instruction),
                0b100011 => self.op_subu(instruction),
                0b100100 => self.op_and(instruction),
                0b100101 => self.op_or(instruction),
                0b100111 => self.op_nor(instruction),
                0b101010 => self.op_slt(instruction),
                0b101011 => self.op_sltu(instruction),
                _ => unimplemented!(
                    "special instruction {:#010x} with opcode {:#08b}",
                    instruction.0,
                    instruction.funct()
                ),
            },
            0b000001 => match instruction.branch_op() {
                0b00000 => self.op_bltz(instruction),
                0b00001 => self.op_bgez(instruction),
                0b10000 => self.op_bltzal(instruction),
                0b10001 => self.op_bgezal(instruction),
                _ => unimplemented!(
                    "branch instruction {:#010x} with opcode {:#07b}",
                    instruction.0,
                    instruction.branch_op()
                ),
            },
            0b000010 => self.op_j(instruction),
            0b000011 => self.op_jal(instruction),
            0b000100 => self.op_beq(instruction),
            0b000101 => self.op_bne(instruction),
            0b000110 => self.op_blez(instruction),
            0b000111 => self.op_bgtz(instruction),
            0b001000 => self.op_addi(instruction),
            0b001001 => self.op_addiu(instruction),
            0b001010 => self.op_slti(instruction),
            0b001011 => self.op_sltiu(instruction),
            0b001100 => self.op_andi(instruction),
            0b001101 => self.op_ori(instruction),
            0b001111 => self.op_lui(instruction),
            0b010000 => match instruction.cop_op() {
                0b00000 => self.op_mfc0(instruction),
                0b00100 => self.op_mtc0(instruction),
                0b10000 => match instruction.funct() {
                    0b010000 => self.op_rfe(instruction),
                    _ => unreachable!(),
                },
                _ => unimplemented!(
                    "cop0 instruction {:#010x} with opcode {:#07b}",
                    instruction.0,
                    instruction.cop_op()
                ),
            },
            0b010001 => unimplemented!(
                "cop1 instruction {:#010x} with opcode {:#07b}",
                instruction.0,
                instruction.cop_op()
            ),
            0b010010 => unimplemented!(
                "cop2 instruction {:#010x} with opcode {:#07b}",
                instruction.0,
                instruction.cop_op()
            ),
            0b010011 => unimplemented!(
                "cop3 instruction {:#010x} with opcode {:#07b}",
                instruction.0,
                instruction.cop_op()
            ),
            0b100000 => self.op_lb(instruction),
            0b100001 => self.op_lh(instruction),
            0b100011 => self.op_lw(instruction),
            0b100100 => self.op_lbu(instruction),
            0b100101 => self.op_lhu(instruction),
            0b101000 => self.op_sb(instruction),
            0b101001 => self.op_sh(instruction),
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
        let address = self.pc.wrapping_add(offset);
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

        if register_index.0 != 0 {
            self.out_registers[register_index.0 as usize] = value;
        }
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
    fn set_cop0_register(&mut self, cop_register_index: CopRegisterIndex, value: u32) {
        assert!(cop_register_index.0 < 64);
        self.cop0_registers[cop_register_index.0 as usize] = value;
    }

    /// Gets a value from a cop register
    ///
    /// # Arguments:
    ///
    /// * `cop_register_index`: The cop register to be read from
    fn cop0_register(&self, cop_register_index: CopRegisterIndex) -> u32 {
        assert!(cop_register_index.0 < 64);
        self.cop0_registers[cop_register_index.0 as usize]
    }
}
