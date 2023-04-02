/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::fmt::{self, Debug, Display, Formatter};

/// A register wrapper
#[derive(Clone, Copy)]
pub(super) struct RegisterIndex(pub(super) u8);

impl Debug for RegisterIndex {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple("RegisterIndex")
            .field(&format_args!("R{}", self.0))
            .finish()
    }
}

impl Display for RegisterIndex {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => write!(fmt, "$zero"),
            1 => write!(fmt, "$at"),
            2 | 3 => write!(fmt, "$v{}", self.0 - 2),
            4 | 5 | 6 | 7 => write!(fmt, "$a{}", self.0 - 4),
            8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => write!(fmt, "$t{}", self.0 - 8),
            16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 => write!(fmt, "$s{}", self.0 - 16),
            24 | 25 => write!(fmt, "$t{}", self.0 - 24 + 8),
            26 | 27 => write!(fmt, "$k{}", self.0 - 26),
            28 => write!(fmt, "$gp"),
            29 => write!(fmt, "$sp"),
            30 => write!(fmt, "$fp"),
            31 => write!(fmt, "$ra"),
            _ => unreachable!(),
        }
    }
}

/// A cop register wrapper
#[derive(Clone, Copy)]
pub(super) struct CopRegisterIndex(pub(super) u8);

impl Debug for CopRegisterIndex {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple("CopRegisterIndex")
            .field(&format_args!("Cop0-R{}", self.0))
            .finish()
    }
}

impl Display for CopRegisterIndex {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 | 1 | 2 => write!(fmt, "$n/a"),
            3 => write!(fmt, "$bpc"),
            4 => write!(fmt, "$n/a"),
            5 => write!(fmt, "$bda"),
            6 => write!(fmt, "$jumpdst"),
            7 => write!(fmt, "$dcic"),
            8 => write!(fmt, "$badvaddr"),
            9 => write!(fmt, "$bdam"),
            10 => write!(fmt, "$n/a"),
            11 => write!(fmt, "$bpcm"),
            12 => write!(fmt, "$sr"),
            13 => write!(fmt, "$cause"),
            14 => write!(fmt, "$epc"),
            15 => write!(fmt, "$prid"),
            16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => {
                write!(fmt, "$garbage")
            }
            32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48
            | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 => {
                write!(fmt, "$n/a")
            }
            _ => unreachable!(),
        }
    }
}
