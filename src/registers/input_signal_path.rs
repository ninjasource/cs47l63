#![allow(dead_code)]

// See section 4.2 Input Signal Path

pub mod input_signal_path_enable {
    use crate::spec::CS47L63_INPUT_CONTROL;

    pub struct InputControl {
        pub in2_left_enable: bool,
        pub in2_right_enable: bool,
        pub in1_left_enable: bool,
        pub in1_right_enable: bool,
    }

    impl InputControl {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_INPUT_CONTROL,
                (self.in2_left_enable as u32) << 3
                    | (self.in2_right_enable as u32) << 2
                    | (self.in1_left_enable as u32) << 1
                    | (self.in1_right_enable as u32),
            ]
        }
    }
}

pub mod input_signal_path_config {
    use crate::spec::CS47L63_INPUT1_CONTROL1;

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum OversampleRateControl {
        Digital384kHz = 0b000,
        Digital768kHz = 0b001,
        Digital1p536MHzOrAnalogMidPower = 0b010,
        Digital2p048MHz = 0b011,
        Digital2p4576MHz = 0b100,
        Digital3p072MHzOrAnalogHighPerformance = 0b101,
        Digital6p144MHz = 0b110,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum InputPath1Mode {
        AnalogInput = 0,
        DigitalMode = 1,
    }

    pub struct Input1Control1 {
        pub oversample_rate_control: OversampleRateControl,
        pub mode: InputPath1Mode,
    }

    impl Input1Control1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_INPUT1_CONTROL1,
                (self.oversample_rate_control as u32) << 16 | 1 << 5 | self.mode as u32,
            ]
        }
    }
}

pub mod input_signal_path_control {
    use crate::spec::{
        CS47L63_IN1L_CONTROL2, CS47L63_IN1R_CONTROL2, CS47L63_IN2L_CONTROL2, CS47L63_IN2R_CONTROL2,
        CS47L63_INPUT_CONTROL3,
    };

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Reg {
        In1Left = CS47L63_IN1L_CONTROL2,
        In1Right = CS47L63_IN1R_CONTROL2,
        In2Left = CS47L63_IN2L_CONTROL2,
        In2Right = CS47L63_IN2R_CONTROL2,
    }

    /// volume -64dB and up in 0.5dB steps up to +31.5dB (range 0-191)
    pub struct InControl2 {
        pub reg: Reg,
        pub mute: bool,
        pub digital_volume: u8,
        pub analog_volume: u8,
    }

    impl InControl2 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                self.reg as u32,
                (self.mute as u32) << 28
                    | (self.digital_volume as u32) << 16
                    | self.analog_volume as u32,
            ]
        }
    }

    pub struct InputControl3 {
        pub volume_update: bool,
    }

    impl InputControl3 {
        pub const fn serialize(&self) -> [u32; 2] {
            [CS47L63_INPUT_CONTROL3, (self.volume_update as u32) << 29]
        }
    }
}
