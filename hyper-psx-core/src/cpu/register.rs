/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::fmt::{self, Debug, Display, Formatter};

/// General register wrapper
#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum Register {
    /// Constant (always 0)
    Zero = 0,

    /// Assembler temporary (destroyed by some assembler pseudoinstructions!)
    At = 1,

    /// Subroutine return values, may be changed by subroutines
    V0 = 2,

    /// Subroutine return values, may be changed by subroutines
    V1 = 3,

    /// Subroutine arguments, may be changed by subroutines
    A0 = 4,

    /// Subroutine arguments, may be changed by subroutines
    A1 = 5,

    /// Subroutine arguments, may be changed by subroutines
    A2 = 6,

    /// Subroutine arguments, may be changed by subroutines
    A3 = 7,

    /// Temporaries, may be changed by subroutines
    T0 = 8,

    /// Temporaries, may be changed by subroutines
    T1 = 9,

    /// Temporaries, may be changed by subroutines
    T2 = 10,

    /// Temporaries, may be changed by subroutines
    T3 = 11,

    /// Temporaries, may be changed by subroutines
    T4 = 12,

    /// Temporaries, may be changed by subroutines
    T5 = 13,

    /// Temporaries, may be changed by subroutines
    T6 = 14,

    /// Temporaries, may be changed by subroutines
    T7 = 15,

    /// Static variables, must be saved by subs
    S0 = 16,

    /// Static variables, must be saved by subs
    S1 = 17,

    /// Static variables, must be saved by subs
    S2 = 18,

    /// Static variables, must be saved by subs
    S3 = 19,

    /// Static variables, must be saved by subs
    S4 = 20,

    /// Static variables, must be saved by subs
    S5 = 21,

    /// Static variables, must be saved by subs
    S6 = 22,

    /// Static variables, must be saved by subs
    S7 = 23,

    /// Temporaries, may be changed by subroutines
    T8 = 24,

    /// Temporaries, may be changed by subroutines
    T9 = 25,

    /// Reserved for kernel (destroyed by some IRQ handlers!)
    K0 = 26,

    /// Reserved for kernel (destroyed by some IRQ handlers!)
    K1 = 27,

    /// Global pointer (rarely used)
    Gp = 28,

    /// Stack pointer
    Sp = 29,

    /// Frame Pointer, or 9th Static variable, must be saved
    Fp = 30,

    /// Return address (used so by JAL,BLTZAL,BGEZAL opcodes)
    Ra = 31,
}

impl Debug for Register {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Zero => write!(fmt, "R0"),
            Self::At => write!(fmt, "R1"),
            Self::V0 => write!(fmt, "R2"),
            Self::V1 => write!(fmt, "R3"),
            Self::A0 => write!(fmt, "R4"),
            Self::A1 => write!(fmt, "R5"),
            Self::A2 => write!(fmt, "R6"),
            Self::A3 => write!(fmt, "R7"),
            Self::T0 => write!(fmt, "R8"),
            Self::T1 => write!(fmt, "R9"),
            Self::T2 => write!(fmt, "R10"),
            Self::T3 => write!(fmt, "R11"),
            Self::T4 => write!(fmt, "R12"),
            Self::T5 => write!(fmt, "R13"),
            Self::T6 => write!(fmt, "R14"),
            Self::T7 => write!(fmt, "R15"),
            Self::S0 => write!(fmt, "R16"),
            Self::S1 => write!(fmt, "R17"),
            Self::S2 => write!(fmt, "R18"),
            Self::S3 => write!(fmt, "R19"),
            Self::S4 => write!(fmt, "R20"),
            Self::S5 => write!(fmt, "R21"),
            Self::S6 => write!(fmt, "R22"),
            Self::S7 => write!(fmt, "R23"),
            Self::T8 => write!(fmt, "R24"),
            Self::T9 => write!(fmt, "R25"),
            Self::K0 => write!(fmt, "R26"),
            Self::K1 => write!(fmt, "R27"),
            Self::Gp => write!(fmt, "R28"),
            Self::Sp => write!(fmt, "R29"),
            Self::Fp => write!(fmt, "R30"),
            Self::Ra => write!(fmt, "R31"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Zero => write!(fmt, "$zero"),
            Self::At => write!(fmt, "$at"),
            Self::V0 => write!(fmt, "$v0"),
            Self::V1 => write!(fmt, "$v1"),
            Self::A0 => write!(fmt, "$a0"),
            Self::A1 => write!(fmt, "$a1"),
            Self::A2 => write!(fmt, "$a2"),
            Self::A3 => write!(fmt, "$a3"),
            Self::T0 => write!(fmt, "$t0"),
            Self::T1 => write!(fmt, "$t1"),
            Self::T2 => write!(fmt, "$t2"),
            Self::T3 => write!(fmt, "$t3"),
            Self::T4 => write!(fmt, "$t4"),
            Self::T5 => write!(fmt, "$t5"),
            Self::T6 => write!(fmt, "$t6"),
            Self::T7 => write!(fmt, "$t7"),
            Self::S0 => write!(fmt, "$s0"),
            Self::S1 => write!(fmt, "$s1"),
            Self::S2 => write!(fmt, "$s2"),
            Self::S3 => write!(fmt, "$s3"),
            Self::S4 => write!(fmt, "$s4"),
            Self::S5 => write!(fmt, "$s5"),
            Self::S6 => write!(fmt, "$s6"),
            Self::S7 => write!(fmt, "$s7"),
            Self::T8 => write!(fmt, "$t8"),
            Self::T9 => write!(fmt, "$t9"),
            Self::K0 => write!(fmt, "$k0"),
            Self::K1 => write!(fmt, "$k1"),
            Self::Gp => write!(fmt, "$gp"),
            Self::Sp => write!(fmt, "$sp"),
            Self::Fp => write!(fmt, "$fp"),
            Self::Ra => write!(fmt, "$ra"),
        }
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::At,
            2 => Self::V0,
            3 => Self::V1,
            4 => Self::A0,
            5 => Self::A1,
            6 => Self::A2,
            7 => Self::A3,
            8 => Self::T0,
            9 => Self::T1,
            10 => Self::T2,
            11 => Self::T3,
            12 => Self::T4,
            13 => Self::T5,
            14 => Self::T6,
            15 => Self::T7,
            16 => Self::S0,
            17 => Self::S1,
            18 => Self::S2,
            19 => Self::S3,
            20 => Self::S4,
            21 => Self::S5,
            22 => Self::S6,
            23 => Self::S7,
            24 => Self::T8,
            25 => Self::T9,
            26 => Self::K0,
            27 => Self::K1,
            28 => Self::Gp,
            29 => Self::Sp,
            30 => Self::Fp,
            31 => Self::Ra,
            _ => unreachable!("unknown general register: {}", value),
        }
    }
}

/// Cop0 register wrapper
#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum Cop0Register {
    /// Breakpoint on execute (R/W)
    Bpc = 3,
    /// Breakpoint on data access (R/W)
    Bda = 5,
    /// Randomly memorized jump address (R)
    Jumpdest = 6,
    /// Breakpoint control (R/W)
    Dcic = 7,
    /// Bad Virtual Address (R)
    Badvaddr = 8,
    /// Data Access breakpoint mask (R/W)
    Bdam = 9,
    /// Execute breakpoint mask (R/W)
    Bpcm = 11,
    /// System status register (R/W)
    Sr = 12,
    /// Describes the most recently recognised exception (R)
    Cause = 13,
    /// Return Address from Trap (R)
    Epc = 14,
    /// Processor ID (R)
    Prid = 15,
}

impl Debug for Cop0Register {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Bpc => write!(fmt, "CR3"),
            Self::Bda => write!(fmt, "CR5"),
            Self::Jumpdest => write!(fmt, "CR6"),
            Self::Dcic => write!(fmt, "CR7"),
            Self::Badvaddr => write!(fmt, "CR8"),
            Self::Bdam => write!(fmt, "CR9"),
            Self::Bpcm => write!(fmt, "CR11"),
            Self::Sr => write!(fmt, "CR12"),
            Self::Cause => write!(fmt, "CR13"),
            Self::Epc => write!(fmt, "CR14"),
            Self::Prid => write!(fmt, "CR15"),
        }
    }
}

impl Display for Cop0Register {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Bpc => write!(fmt, "$bpc"),
            Self::Bda => write!(fmt, "$bda"),
            Self::Jumpdest => write!(fmt, "$jumpdest"),
            Self::Dcic => write!(fmt, "$dcic"),
            Self::Badvaddr => write!(fmt, "$badvaddr"),
            Self::Bdam => write!(fmt, "$bdam"),
            Self::Bpcm => write!(fmt, "$bpcm"),
            Self::Sr => write!(fmt, "$sr"),
            Self::Cause => write!(fmt, "$cause"),
            Self::Epc => write!(fmt, "$epc"),
            Self::Prid => write!(fmt, "$prid"),
        }
    }
}

impl From<u8> for Cop0Register {
    fn from(value: u8) -> Self {
        match value {
            3 => Self::Bpc,
            5 => Self::Bda,
            6 => Self::Jumpdest,
            7 => Self::Dcic,
            8 => Self::Badvaddr,
            9 => Self::Bdam,
            11 => Self::Bpcm,
            12 => Self::Sr,
            13 => Self::Cause,
            14 => Self::Epc,
            15 => Self::Prid,
            _ => unreachable!("unknown cop0 register: {}", value),
        }
    }
}
