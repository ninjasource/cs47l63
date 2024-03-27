#![allow(dead_code)]

pub mod voltage {
    use crate::spec::{CS47L63_LDO2_CTRL1, CS47L63_MICBIAS_CTRL1, CS47L63_MICBIAS_CTRL5};

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Ldo2OutputVoltageSelect {
        _2p4V = 0x0,
        _2p47V = 0x1,
        _2p55V = 0x2,
        _2p64V = 0x3,
        _2p73V = 0x4,
        _2p84V = 0x5,
        _2p96V = 0x6,
        _3p10V = 0x7,
    }

    pub struct Ldo2Crtl1 {
        pub output_voltage_select: Ldo2OutputVoltageSelect,
        pub discharge: bool,
        pub enabled: bool,
    }

    impl Ldo2Crtl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_LDO2_CTRL1,
                (self.output_voltage_select as u32) << 5
                    | (self.discharge as u32) << 2
                    | self.enabled as u32,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum MicBias1VoltageLevel {
        _1p5V = 0x0,
        _1p6V = 0x1,
        _1p7V = 0x2,
        _1p8V = 0x3,
        _1p9V = 0x4,
        _2p0V = 0x5,
        _2p1V = 0x6,
        _2p2V = 0x7,
        _2p3V = 0x8,
        _2p4V = 0x9,
        _2p5V = 0xA,
        _2p6V = 0xB,
        _2p7V = 0xC,
        _2p8V = 0xD,
    }

    pub struct MicBiasCtrl1 {
        pub has_external_capacitor: bool,
        pub level: MicBias1VoltageLevel,
        pub fast_rate: bool,
        pub discharge: bool,
        pub bypass_mode: bool,
        pub enabled: bool,
    }

    impl MicBiasCtrl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_MICBIAS_CTRL1,
                (self.has_external_capacitor as u32) << 15
                    | (self.level as u32) << 5
                    | (self.fast_rate as u32) << 3
                    | (self.discharge as u32) << 2
                    | (self.bypass_mode as u32) << 1
                    | (self.enabled as u32),
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum MicBias1Source {
        MicBiasRegulator = 0,
        VddA = 1,
    }

    pub struct MicBiasCtrl5 {
        pub mic_bias_1c_source: MicBias1Source,
        pub mic_bias_1c_discharge: bool,
        pub mic_bias_1c_enabled: bool,
        pub mic_bias_1b_source: MicBias1Source,
        pub mic_bias_1b_discharge: bool,
        pub mic_bias_1b_enabled: bool,
        pub mic_bias_1a_source: MicBias1Source,
        pub mic_bias_1a_discharge: bool,
        pub mic_bias_1a_enabled: bool,
    }

    impl MicBiasCtrl5 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_MICBIAS_CTRL5,
                (self.mic_bias_1c_source as u32) << 10
                    | (self.mic_bias_1c_discharge as u32) << 9
                    | (self.mic_bias_1c_enabled as u32) << 8
                    | (self.mic_bias_1b_source as u32) << 6
                    | (self.mic_bias_1b_discharge as u32) << 5
                    | (self.mic_bias_1b_enabled as u32) << 4
                    | (self.mic_bias_1a_source as u32) << 2
                    | (self.mic_bias_1a_discharge as u32) << 1
                    | (self.mic_bias_1a_enabled as u32),
            ]
        }
    }
}
