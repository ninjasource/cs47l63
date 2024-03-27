#![allow(dead_code)]

pub mod asp_ctrl {
    use crate::spec::{
        CS47L63_ASP1_CONTROL1, CS47L63_ASP1_CONTROL2, CS47L63_ASP1_CONTROL3,
        CS47L63_ASP1_DATA_CONTROL1, CS47L63_ASP1_DATA_CONTROL5, CS47L63_ASP1_ENABLES1,
        CS47L63_ASP2_CONTROL1, CS47L63_ASP2_CONTROL2, CS47L63_ASP2_CONTROL3,
        CS47L63_ASP2_DATA_CONTROL1, CS47L63_ASP2_DATA_CONTROL5, CS47L63_ASP2_ENABLES1,
    };

    pub enum Num {
        _1,
        _2,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum AspRate {
        SampleRate1 = 0x00,
        SampleRate2 = 0x01,
        SampleRate3 = 0x02,
        SampleRate4 = 0x03,
        AsyncSampleRate1 = 0x08,
        AsyncSampleRate2 = 0x09,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum AspBclkRate {
        _128kHz = 0x0C,
        _176p4kHz = 0x0D,
        _192kHz = 0x0E,
        _256kHz = 0x0F,
        _352p8kHz = 0x10,
        _384kHz = 0x11,
        _512kHz = 0x12,
        _705p6kHz = 0x13,
        _768kHz = 0x15,
        _1p024MHz = 0x17,
        _1p4112MHz = 0x19,
        _1p536MHz = 0x1B,
        _2p048MHz = 0x1D,
        _2p8824MHz = 0x1F,
        _3p072MHz = 0x21,
        _4p096MHz = 0x24,
        _5p6448MHz = 0x26,
        _6p144MHz = 0x28,
        _8p192MHz = 0x2F,
        _11p2896MHz = 0x31,
        _12p288MHz = 0x33,
        _22p5792MHz = 0x39,
        _24p576MHz = 0x3B,
    }

    pub struct AspControl1 {
        num: Num,
        rate: AspRate,
        bclk_freq: AspBclkRate,
    }

    impl AspControl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            let reg = match self.num {
                Num::_1 => CS47L63_ASP1_CONTROL1,
                Num::_2 => CS47L63_ASP2_CONTROL1,
            };

            [reg, (self.rate as u32) << 8 | self.bclk_freq as u32]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum AspFormat {
        Tdm1Mode = 0b000,
        Tdm0Mode = 0b001,
        I2sMode = 0b010,
        LeftJustifiedMode = 0b011,
        Tdm1p5Mode = 0b100,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum AspBclkOutputControl {
        Normal = 0,
        AlwaysEnabledInMasterMode = 1,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum AspBclkMasterSelect {
        SlaveMode = 0,
        MasterMode = 1,
    }

    /// rx_width (range 16-128)
    /// tx_width (range 16-128)
    pub struct AspControl2 {
        pub num: Num,
        pub rx_width: u8,
        pub tx_width: u8,
        pub format: AspFormat,
        pub bclk_invert: bool,
        pub bclk_frc: AspBclkOutputControl,
        pub bclk_mstr: AspBclkMasterSelect,
        pub fsync_invert: bool,
        pub fsync_frc: AspBclkOutputControl,
        pub fsync_mstr: AspBclkMasterSelect,
    }

    impl AspControl2 {
        pub const fn serialize(&self) -> [u32; 2] {
            let reg = match self.num {
                Num::_1 => CS47L63_ASP1_CONTROL2,
                Num::_2 => CS47L63_ASP2_CONTROL2,
            };

            [
                reg,
                (self.rx_width as u32) << 24
                    | (self.tx_width as u32) << 16
                    | (self.format as u32) << 8
                    | (self.bclk_invert as u32) << 6
                    | (self.bclk_frc as u32) << 5
                    | (self.bclk_mstr as u32) << 4
                    | (self.fsync_invert as u32) << 2
                    | (self.fsync_frc as u32) << 1
                    | (self.fsync_mstr as u32),
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum AspDoutTristateControl {
        Mode00 = 0b00,
        Mode01 = 0b01,
        Mode10 = 0b10,
        Mode11 = 0b11,
    }

    pub struct AspControl3 {
        pub num: Num,
        pub dout_hiz_ctrl: AspDoutTristateControl,
    }

    impl AspControl3 {
        pub const fn serialize(&self) -> [u32; 2] {
            let reg = match self.num {
                Num::_1 => CS47L63_ASP1_CONTROL3,
                Num::_2 => CS47L63_ASP2_CONTROL3,
            };

            [reg, self.dout_hiz_ctrl as u32]
        }
    }

    /// tx_data_width_bits (range 16-32)
    pub struct AspDataControl1 {
        pub num: Num,
        pub tx_data_width_bits: u8,
    }

    impl AspDataControl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            let reg = match self.num {
                Num::_1 => CS47L63_ASP1_DATA_CONTROL1,
                Num::_2 => CS47L63_ASP2_DATA_CONTROL1,
            };

            [reg, self.tx_data_width_bits as u32]
        }
    }

    /// rx_data_width_bits (range 16-32)
    pub struct AspDataControl5 {
        pub num: Num,
        pub rx_data_width_bits: u8,
    }

    impl AspDataControl5 {
        pub const fn serialize(&self) -> [u32; 2] {
            let reg = match self.num {
                Num::_1 => CS47L63_ASP1_DATA_CONTROL5,
                Num::_2 => CS47L63_ASP2_DATA_CONTROL5,
            };

            [reg, self.rx_data_width_bits as u32]
        }
    }

    pub struct Asp1Enables1 {
        pub rx8_enabled: bool,
        pub rx7_enabled: bool,
        pub rx6_enabled: bool,
        pub rx5_enabled: bool,
        pub rx4_enabled: bool,
        pub rx3_enabled: bool,
        pub rx2_enabled: bool,
        pub rx1_enabled: bool,
        pub tx8_enabled: bool,
        pub tx7_enabled: bool,
        pub tx6_enabled: bool,
        pub tx5_enabled: bool,
        pub tx4_enabled: bool,
        pub tx3_enabled: bool,
        pub tx2_enabled: bool,
        pub tx1_enabled: bool,
    }

    impl Asp1Enables1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_ASP1_ENABLES1,
                (self.rx8_enabled as u32) << 23
                    | (self.rx7_enabled as u32) << 22
                    | (self.rx6_enabled as u32) << 21
                    | (self.rx5_enabled as u32) << 20
                    | (self.rx4_enabled as u32) << 19
                    | (self.rx3_enabled as u32) << 18
                    | (self.rx2_enabled as u32) << 17
                    | (self.rx1_enabled as u32) << 16
                    | (self.tx8_enabled as u32) << 7
                    | (self.tx7_enabled as u32) << 6
                    | (self.tx6_enabled as u32) << 5
                    | (self.tx5_enabled as u32) << 4
                    | (self.tx4_enabled as u32) << 3
                    | (self.tx3_enabled as u32) << 2
                    | (self.tx2_enabled as u32) << 1
                    | (self.tx1_enabled as u32),
            ]
        }
    }

    pub struct Asp2Enables1 {
        pub rx4_enabled: bool,
        pub rx3_enabled: bool,
        pub rx2_enabled: bool,
        pub rx1_enabled: bool,
        pub tx4_enabled: bool,
        pub tx3_enabled: bool,
        pub tx2_enabled: bool,
        pub tx1_enabled: bool,
    }

    impl Asp2Enables1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_ASP2_ENABLES1,
                (self.rx4_enabled as u32) << 19
                    | (self.rx3_enabled as u32) << 18
                    | (self.rx2_enabled as u32) << 17
                    | (self.rx1_enabled as u32) << 16
                    | (self.tx4_enabled as u32) << 3
                    | (self.tx3_enabled as u32) << 2
                    | (self.tx2_enabled as u32) << 1
                    | (self.tx1_enabled as u32),
            ]
        }
    }
}
