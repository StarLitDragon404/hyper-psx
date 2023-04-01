/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Clone, Copy, Debug)]
pub(super) struct Instruction(pub(super) u32);

impl Instruction {
    /// 6-bit operation code (31-26)
    #[inline(always)]
    pub(super) fn op(&self) -> u8 {
        ((self.0 >> 26) & 0x3f) as u8
    }

    /// 5-bit source register specifier (25-21)
    #[inline(always)]
    pub(super) fn rs(&self) -> u8 {
        ((self.0 >> 21) & 0x1f) as u8
    }

    /// 5-bit target (source/destination) or branch condition (20-16)
    #[inline(always)]
    pub(super) fn rt(&self) -> u8 {
        ((self.0 >> 16) & 0x1f) as u8
    }

    /// 16-bit immediate, branch displacement or address displacement (15-0)
    #[inline(always)]
    pub(super) fn imm(&self) -> u16 {
        ((self.0 >> 0) & 0xffff) as u16
    }

    /// 26-bit jump target address (25-0)
    #[inline(always)]
    pub(super) fn target(&self) -> u32 {
        ((self.0 >> 0) & 0x3ffffff) as u32
    }

    /// 5-bit destination register specifier (15-11)
    #[inline(always)]
    pub(super) fn rd(&self) -> u8 {
        ((self.0 >> 11) & 0x1f) as u8
    }

    /// 5-bit shift amount (10-6)
    #[inline(always)]
    pub(super) fn shamt(&self) -> u8 {
        ((self.0 >> 6) & 0x1f) as u8
    }

    /// 6-bit function field (5-0)
    #[inline(always)]
    pub(super) fn funct(&self) -> u8 {
        ((self.0 >> 0) & 0x3f) as u8
    }
}
