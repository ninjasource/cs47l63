#![allow(dead_code)]

pub mod volume_ctrl {
    use crate::spec::{CS47L63_OUT1L_VOLUME_1, CS47L63_OUTPUT_ENABLE_1};

    // volume -64dB and up in 0.5dB steps up to +31.5dB (range 0-191)
    pub struct Out1LVolume1 {
        pub update: bool,
        pub mute: bool,
        pub volume: u8,
    }

    impl Out1LVolume1 {
        pub const REG: u32 = CS47L63_OUT1L_VOLUME_1;

        pub const fn serialize(&self) -> [u32; 2] {
            let volume_db = self.volume as u32;
            [
                Self::REG,
                (self.update as u32) << 9 | (self.mute as u32) << 8 | volume_db,
            ]
        }
    }

    impl From<u32> for Out1LVolume1 {
        fn from(value: u32) -> Self {
            let mute = (value & 0b0000_0000_0000_0000_0000_0001_0000_0000) > 0;
            let volume = (value & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u8;
            Self {
                update: false,
                mute,
                volume,
            }
        }
    }

    pub struct OutputEnable1 {
        pub enabled: bool,
    }

    impl OutputEnable1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [CS47L63_OUTPUT_ENABLE_1, (self.enabled as u32) << 1]
        }
    }
}
