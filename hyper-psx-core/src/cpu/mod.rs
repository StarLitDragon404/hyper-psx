/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

mod branch;
mod cop0;
mod cop2;
mod exception;
mod instruction;
mod instructions;
mod register;
mod special;

use crate::{
    bus::Bus,
    cpu::{
        exception::Exception,
        instruction::Instruction,
        register::{Cop0Register, Register},
    },
    dma::Dma,
    gpu::Gpu,
};

/// The CPU component
#[derive(Debug)]
pub(crate) struct Cpu {
    /// The 32 general purpose registers
    registers: [u32; 32],

    /// The 32 general purpose output registers
    out_registers: [u32; 32],

    /// The load delay register
    load_delay_register: Option<(Register, u32)>,

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
    pub(crate) fn step(&mut self, dma: &mut Dma, gpu: &mut Gpu) {
        if self.pc % 4 != 0 {
            panic!("unaligned pc");
        }

        let instruction = Instruction(self.bus.read_u32(self.pc, dma, gpu), self.pc);
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

        self.execute(instruction, dma, gpu);

        self.registers = self.out_registers;
    }

    /// Executes an instruction
    ///
    /// # Arguments:
    ///
    /// * `instruction`: The instruction to be executed
    fn execute(&mut self, instruction: Instruction, dma: &mut Dma, gpu: &mut Gpu) {
        match instruction.op() {
            0b000000 => match instruction.funct() {
                0b000000 => self.op_sll(instruction),
                0b000010 => self.op_srl(instruction),
                0b000011 => self.op_sra(instruction),
                0b000100 => self.op_sllv(instruction),
                0b000110 => self.op_srlv(instruction),
                0b000111 => self.op_srav(instruction),
                0b001000 => self.op_jr(instruction),
                0b001001 => self.op_jalr(instruction),
                0b001100 => self.op_syscall(instruction),
                0b001101 => self.op_break(instruction),
                0b010000 => self.op_mfhi(instruction),
                0b010001 => self.op_mthi(instruction),
                0b010010 => self.op_mflo(instruction),
                0b010011 => self.op_mtlo(instruction),
                0b011000 => self.op_mult(instruction),
                0b011001 => self.op_multu(instruction),
                0b011010 => self.op_div(instruction),
                0b011011 => self.op_divu(instruction),
                0b100000 => self.op_add(instruction),
                0b100001 => self.op_addu(instruction),
                0b100010 => self.op_sub(instruction),
                0b100011 => self.op_subu(instruction),
                0b100100 => self.op_and(instruction),
                0b100101 => self.op_or(instruction),
                0b100110 => self.op_xor(instruction),
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
            0b001110 => self.op_xori(instruction),
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
            0b010001 => self.raise_exception(instruction, Exception::Cpu),
            0b010010 => {
                // GTE
                unimplemented!(
                    "cop2 instruction {:#010x} with opcode {:#07b}",
                    instruction.0,
                    instruction.cop_op()
                )
            }
            0b010011 => self.raise_exception(instruction, Exception::Cpu),
            0b100000 => self.op_lb(instruction, dma, gpu),
            0b100001 => self.op_lh(instruction, dma, gpu),
            0b100010 => self.op_lwl(instruction, dma, gpu),
            0b100011 => self.op_lw(instruction, dma, gpu),
            0b100100 => self.op_lbu(instruction, dma, gpu),
            0b100101 => self.op_lhu(instruction, dma, gpu),
            0b100110 => self.op_lwr(instruction, dma, gpu),
            0b101000 => self.op_sb(instruction, dma, gpu),
            0b101001 => self.op_sh(instruction, dma, gpu),
            0b101010 => self.op_swl(instruction, dma, gpu),
            0b101011 => self.op_sw(instruction, dma, gpu),
            0b101110 => self.op_swr(instruction, dma, gpu),
            0b110000 => self.raise_exception(instruction, Exception::Cpu),
            0b110001 => self.raise_exception(instruction, Exception::Cpu),
            0b110010 => self.op_lwc2(instruction),
            0b110011 => self.raise_exception(instruction, Exception::Cpu),
            0b111000 => self.raise_exception(instruction, Exception::Cpu),
            0b111001 => self.raise_exception(instruction, Exception::Cpu),
            0b111010 => self.op_swc2(instruction),
            0b111011 => self.raise_exception(instruction, Exception::Cpu),
            _ => {
                log::warn!(
                    "{}: {:#010x}: unimplemented instruction {:#010x} with opcode {:#08b}",
                    self.n,
                    instruction.1,
                    instruction.0,
                    instruction.op()
                );
                self.raise_exception(instruction, Exception::Ri)
            }
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
    fn set_register(&mut self, register: Register, value: u32) {
        let register_value = register as usize;

        debug_assert!(register_value < 32);

        if register_value != 0 {
            self.out_registers[register_value] = value;
        }
    }

    /// Gets a value from a register
    ///
    /// # Arguments:
    ///
    /// * `register_index`: The register to be read from
    fn register(&self, register: Register) -> u32 {
        let register_value = register as usize;

        debug_assert!(register_value < 32);

        self.registers[register_value]
    }

    /// Sets a cop register to a value
    ///
    /// # Arguments:
    ///
    /// * `cop_register_index`: The cop register to be set
    /// * `value`: The value for the regsiter
    fn set_cop0_register(&mut self, cop0_register: Cop0Register, value: u32) {
        let cop0_register_value = cop0_register as usize;

        debug_assert!(cop0_register_value < 64);

        self.cop0_registers[cop0_register_value] = value;
    }

    /// Gets a value from a cop register
    ///
    /// # Arguments:
    ///
    /// * `cop_register_index`: The cop register to be read from
    fn cop0_register(&self, cop0_register: Cop0Register) -> u32 {
        let cop0_register_value = cop0_register as usize;

        debug_assert!(cop0_register_value < 64);

        self.cop0_registers[cop0_register_value]
    }

    /// Returns the Bus
    pub(crate) fn bus(&mut self) -> &mut Bus {
        // TODO: Move bus to application
        &mut self.bus
    }
}
