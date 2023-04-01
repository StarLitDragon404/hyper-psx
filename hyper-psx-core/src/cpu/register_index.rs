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
