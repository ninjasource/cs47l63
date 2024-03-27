#![allow(dead_code)]

use crate::spec::CS47L63_SFT_RESET;

pub struct SoftReset {
    pub reset: u8,
}

impl SoftReset {
    pub const fn new() -> Self {
        SoftReset { reset: 0x5A }
    }

    pub const fn serialize(&self) -> [u32; 2] {
        [CS47L63_SFT_RESET, (self.reset as u32) << 24]
    }
}
