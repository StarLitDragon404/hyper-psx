/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use super::register_index::{CopRegisterIndex, RegisterIndex};

/// An instruction wrapper
#[derive(Clone, Copy, Debug)]
pub(super) struct Instruction(pub(super) u32, pub(super) u32);

impl Instruction {
    /// Returns the 6-bit operation code (31-26)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn op(&self) -> u8 {
        ((self.0 >> 26) & 0x3f) as u8
    }

    /// Returns the 5-bit cop operation code (25-21)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn cop_op(&self) -> u8 {
        ((self.0 >> 21) & 0x1f) as u8
    }

    /// Returns the 5-bit source register specifier (25-21)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn rs(&self) -> RegisterIndex {
        RegisterIndex(((self.0 >> 21) & 0x1f) as u8)
    }

    /// Returns the 5-bit branch operation code (20-16)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn branch_op(&self) -> u8 {
        ((self.0 >> 16) & 0x1f) as u8
    }

    /// Returns the 5-bit target (source/destination) or branch condition (20-16)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn rt(&self) -> RegisterIndex {
        RegisterIndex(((self.0 >> 16) & 0x1f) as u8)
    }

    /// Returns the 16-bit immediate, branch displacement or address displacement (15-0)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn imm(&self) -> u16 {
        (self.0 & 0xffff) as u16
    }

    /// Returns the 26-bit jump target address (25-0)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn target(&self) -> u32 {
        self.0 & 0x3ffffff
    }

    /// Returns the 5-bit cop destination register specifier (15-11)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn cop_rd(&self) -> CopRegisterIndex {
        CopRegisterIndex(((self.0 >> 11) & 0x1f) as u8)
    }

    /// Returns the 5-bit destination register specifier (15-11)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn rd(&self) -> RegisterIndex {
        RegisterIndex(((self.0 >> 11) & 0x1f) as u8)
    }

    /// Returns the 5-bit shift amount (10-6)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn shamt(&self) -> u8 {
        ((self.0 >> 6) & 0x1f) as u8
    }

    /// Returns the 6-bit function field (5-0)
    ///
    /// <https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf#page=214>
    #[inline(always)]
    pub(super) fn funct(&self) -> u8 {
        (self.0 & 0x3f) as u8
    }
}
