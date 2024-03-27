#![allow(dead_code)]

// ****************
// 4.3 Digital Core
// ****************

// 4.3.1 Digital-Core Mixers

pub mod mixers {
    // applies to registers 0x8080 - 0x907C

    use crate::spec::*;

    // Input volume -32dB to +16dB in 1dB steps (range 0x20 - 0x50)
    // e.g. 0x00 = -32db
    // e.g. 0x20 = -32db
    // e.g. 0x21 = -31db
    // e.g. 0x38 = -8db
    // e.g. 0x40 = 0db
    // e.g. 0x50 = 16db
    // e.g. 0x51 = 16db
    pub struct InputSource {
        pub reg: ImportSourceReg,
        pub input_volume: u8,
        pub status_enabled: bool,
        pub source_select: InputSourceSelect,
    }

    impl InputSource {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                self.reg as u32,
                (self.input_volume as u32) << 17
                    | (self.status_enabled as u32) << 15
                    | self.source_select as u32,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum ImportSourceReg {
        Pwm1Input1 = CS47L63_PWM1_INPUT1,
        Pwm1Input2 = CS47L63_PWM1_INPUT2,
        Pwm1Input3 = CS47L63_PWM1_INPUT3,
        Pwm1Input4 = CS47L63_PWM1_INPUT4,
        Pwm2Input1 = CS47L63_PWM2_INPUT1,
        Pwm2Input2 = CS47L63_PWM2_INPUT2,
        Pwm2Input3 = CS47L63_PWM2_INPUT3,
        Pwm2Input4 = CS47L63_PWM2_INPUT4,
        Out1LInput1 = CS47L63_OUT1L_INPUT1,
        Out1LInput2 = CS47L63_OUT1L_INPUT2,
        Out1LInput3 = CS47L63_OUT1L_INPUT3,
        Out1LInput4 = CS47L63_OUT1L_INPUT4,
        Asp1Tx1Input1 = CS47L63_ASP1TX1_INPUT1,
        Asp1Tx1Input2 = CS47L63_ASP1TX1_INPUT2,
        Asp1Tx1Input3 = CS47L63_ASP1TX1_INPUT3,
        Asp1Tx1Input4 = CS47L63_ASP1TX1_INPUT4,
        Asp1Tx2Input1 = CS47L63_ASP1TX2_INPUT1,
        Asp1Tx2Input2 = CS47L63_ASP1TX2_INPUT2,
        Asp1Tx2Input3 = CS47L63_ASP1TX2_INPUT3,
        Asp1Tx2Input4 = CS47L63_ASP1TX2_INPUT4,
        Asp1Tx3Input1 = CS47L63_ASP1TX3_INPUT1,
        Asp1Tx3Input2 = CS47L63_ASP1TX3_INPUT2,
        Asp1Tx3Input3 = CS47L63_ASP1TX3_INPUT3,
        Asp1Tx3Input4 = CS47L63_ASP1TX3_INPUT4,
        Asp1Tx4Input1 = CS47L63_ASP1TX4_INPUT1,
        Asp1Tx4Input2 = CS47L63_ASP1TX4_INPUT2,
        Asp1Tx4Input3 = CS47L63_ASP1TX4_INPUT3,
        Asp1Tx4Input4 = CS47L63_ASP1TX4_INPUT4,
        Asp1Tx5Input1 = CS47L63_ASP1TX5_INPUT1,
        Asp1Tx5Input2 = CS47L63_ASP1TX5_INPUT2,
        Asp1Tx5Input3 = CS47L63_ASP1TX5_INPUT3,
        Asp1Tx5Input4 = CS47L63_ASP1TX5_INPUT4,
        Asp1Tx6Input1 = CS47L63_ASP1TX6_INPUT1,
        Asp1Tx6Input2 = CS47L63_ASP1TX6_INPUT2,
        Asp1Tx6Input3 = CS47L63_ASP1TX6_INPUT3,
        Asp1Tx6Input4 = CS47L63_ASP1TX6_INPUT4,
        Asp1Tx7Input1 = CS47L63_ASP1TX7_INPUT1,
        Asp1Tx7Input2 = CS47L63_ASP1TX7_INPUT2,
        Asp1Tx7Input3 = CS47L63_ASP1TX7_INPUT3,
        Asp1Tx7Input4 = CS47L63_ASP1TX7_INPUT4,
        Asp1Tx8Input1 = CS47L63_ASP1TX8_INPUT1,
        Asp1Tx8Input2 = CS47L63_ASP1TX8_INPUT2,
        Asp1Tx8Input3 = CS47L63_ASP1TX8_INPUT3,
        Asp1Tx8Input4 = CS47L63_ASP1TX8_INPUT4,
        Asp2Tx1Input1 = CS47L63_ASP2TX1_INPUT1,
        Asp2Tx1Input2 = CS47L63_ASP2TX1_INPUT2,
        Asp2Tx1Input3 = CS47L63_ASP2TX1_INPUT3,
        Asp2Tx1Input4 = CS47L63_ASP2TX1_INPUT4,
        Asp2Tx2Input1 = CS47L63_ASP2TX2_INPUT1,
        Asp2Tx2Input2 = CS47L63_ASP2TX2_INPUT2,
        Asp2Tx2Input3 = CS47L63_ASP2TX2_INPUT3,
        Asp2Tx2Input4 = CS47L63_ASP2TX2_INPUT4,
        Asp2Tx3Input1 = CS47L63_ASP2TX3_INPUT1,
        Asp2Tx3Input2 = CS47L63_ASP2TX3_INPUT2,
        Asp2Tx3Input3 = CS47L63_ASP2TX3_INPUT3,
        Asp2Tx3Input4 = CS47L63_ASP2TX3_INPUT4,
        Asp2Tx4Input1 = CS47L63_ASP2TX4_INPUT1,
        Asp2Tx4Input2 = CS47L63_ASP2TX4_INPUT2,
        Asp2Tx4Input3 = CS47L63_ASP2TX4_INPUT3,
        Asp2Tx4Input4 = CS47L63_ASP2TX4_INPUT4,
        ASrc1In1LInput1 = CS47L63_ASRC1_IN1L_INPUT1,
        ASrc1In1RInput1 = CS47L63_ASRC1_IN1R_INPUT1,
        ASrc1In2LInput1 = CS47L63_ASRC1_IN2L_INPUT1,
        ASrc1In2RInput1 = CS47L63_ASRC1_IN2R_INPUT1,
        LSrc2InLInput1 = CS47L63_LSRC2_INL_INPUT1,
        LSrc2InRInput1 = CS47L63_LSRC2_INR_INPUT1,
        LSrc3InLInput1 = CS47L63_LSRC3_INL_INPUT1,
        LSrc3InRInput1 = CS47L63_LSRC3_INR_INPUT1,
        ISrc1Int1Input1 = CS47L63_ISRC1INT1_INPUT1,
        ISrc1Int2Input1 = CS47L63_ISRC1INT2_INPUT1,
        ISrc1Int3Input1 = CS47L63_ISRC1INT3_INPUT1,
        ISrc1Int4Input1 = CS47L63_ISRC1INT4_INPUT1,
        ISrc1Dec1Input1 = CS47L63_ISRC1DEC1_INPUT1,
        ISrc1Dec2Input1 = CS47L63_ISRC1DEC2_INPUT1,
        ISrc1Dec3Input1 = CS47L63_ISRC1DEC3_INPUT1,
        ISrc1Dec4Input1 = CS47L63_ISRC1DEC4_INPUT1,
        ISrc2Int1Input1 = CS47L63_ISRC2INT1_INPUT1,
        ISrc2Int2Input1 = CS47L63_ISRC2INT2_INPUT1,
        ISrc2Dec1Input1 = CS47L63_ISRC2DEC1_INPUT1,
        ISrc2Dec2Input1 = CS47L63_ISRC2DEC2_INPUT1,
        ISrc3Int1Input1 = CS47L63_ISRC3INT1_INPUT1,
        ISrc3Int2Input1 = CS47L63_ISRC3INT2_INPUT1,
        ISrc3Dec1Input1 = CS47L63_ISRC3DEC1_INPUT1,
        ISrc3Dec2Input1 = CS47L63_ISRC3DEC2_INPUT1,
        Eq1Input1 = CS47L63_EQ1_INPUT1,
        Eq1Input2 = CS47L63_EQ1_INPUT2,
        Eq1Input3 = CS47L63_EQ1_INPUT3,
        Eq1Input4 = CS47L63_EQ1_INPUT4,
        Eq2Input1 = CS47L63_EQ2_INPUT1,
        Eq2Input2 = CS47L63_EQ2_INPUT2,
        Eq2Input3 = CS47L63_EQ2_INPUT3,
        Eq2Input4 = CS47L63_EQ2_INPUT4,
        Eq3Input1 = CS47L63_EQ3_INPUT1,
        Eq3Input2 = CS47L63_EQ3_INPUT2,
        Eq3Input3 = CS47L63_EQ3_INPUT3,
        Eq3Input4 = CS47L63_EQ3_INPUT4,
        Eq4Input1 = CS47L63_EQ4_INPUT1,
        Eq4Input2 = CS47L63_EQ4_INPUT2,
        Eq4Input3 = CS47L63_EQ4_INPUT3,
        Eq4Input4 = CS47L63_EQ4_INPUT4,
        Drc1LInput1 = CS47L63_DRC1L_INPUT1,
        Drc1LInput2 = CS47L63_DRC1L_INPUT2,
        Drc1LInput3 = CS47L63_DRC1L_INPUT3,
        Drc1LInput4 = CS47L63_DRC1L_INPUT4,
        Drc1RInput1 = CS47L63_DRC1R_INPUT1,
        Drc1RInput2 = CS47L63_DRC1R_INPUT2,
        Drc1RInput3 = CS47L63_DRC1R_INPUT3,
        Drc1RInput4 = CS47L63_DRC1R_INPUT4,
        Drc2LInput1 = CS47L63_DRC2L_INPUT1,
        Drc2LInput2 = CS47L63_DRC2L_INPUT2,
        Drc2LInput3 = CS47L63_DRC2L_INPUT3,
        Drc2LInput4 = CS47L63_DRC2L_INPUT4,
        Drc2RInput1 = CS47L63_DRC2R_INPUT1,
        Drc2RInput2 = CS47L63_DRC2R_INPUT2,
        Drc2RInput3 = CS47L63_DRC2R_INPUT3,
        Drc2RInput4 = CS47L63_DRC2R_INPUT4,
        LHpF1Input1 = CS47L63_LHPF1_INPUT1,
        LHpF1Input2 = CS47L63_LHPF1_INPUT2,
        LHpF1Input3 = CS47L63_LHPF1_INPUT3,
        LHpF1Input4 = CS47L63_LHPF1_INPUT4,
        LHpF2Input1 = CS47L63_LHPF2_INPUT1,
        LHpF2Input2 = CS47L63_LHPF2_INPUT2,
        LHpF2Input3 = CS47L63_LHPF2_INPUT3,
        LHpF2Input4 = CS47L63_LHPF2_INPUT4,
        LHpF3Input1 = CS47L63_LHPF3_INPUT1,
        LHpF3Input2 = CS47L63_LHPF3_INPUT2,
        LHpF3Input3 = CS47L63_LHPF3_INPUT3,
        LHpF3Input4 = CS47L63_LHPF3_INPUT4,
        LHpF4Input1 = CS47L63_LHPF4_INPUT1,
        LHpF4Input2 = CS47L63_LHPF4_INPUT2,
        LHpF4Input3 = CS47L63_LHPF4_INPUT3,
        LHpF4Input4 = CS47L63_LHPF4_INPUT4,
        Dsp1Rx1Input1 = CS47L63_DSP1RX1_INPUT1,
        Dsp1Rx1Input2 = CS47L63_DSP1RX1_INPUT2,
        Dsp1Rx1Input3 = CS47L63_DSP1RX1_INPUT3,
        Dsp1Rx1Input4 = CS47L63_DSP1RX1_INPUT4,
        Dsp1Rx2Input1 = CS47L63_DSP1RX2_INPUT1,
        Dsp1Rx2Input2 = CS47L63_DSP1RX2_INPUT2,
        Dsp1Rx2Input3 = CS47L63_DSP1RX2_INPUT3,
        Dsp1Rx2Input4 = CS47L63_DSP1RX2_INPUT4,
        Dsp1Rx3Input1 = CS47L63_DSP1RX3_INPUT1,
        Dsp1Rx3Input2 = CS47L63_DSP1RX3_INPUT2,
        Dsp1Rx3Input3 = CS47L63_DSP1RX3_INPUT3,
        Dsp1Rx3Input4 = CS47L63_DSP1RX3_INPUT4,
        Dsp1Rx4Input1 = CS47L63_DSP1RX4_INPUT1,
        Dsp1Rx4Input2 = CS47L63_DSP1RX4_INPUT2,
        Dsp1Rx4Input3 = CS47L63_DSP1RX4_INPUT3,
        Dsp1Rx4Input4 = CS47L63_DSP1RX4_INPUT4,
        Dsp1Rx5Input1 = CS47L63_DSP1RX5_INPUT1,
        Dsp1Rx5Input2 = CS47L63_DSP1RX5_INPUT2,
        Dsp1Rx5Input3 = CS47L63_DSP1RX5_INPUT3,
        Dsp1Rx5Input4 = CS47L63_DSP1RX5_INPUT4,
        Dsp1Rx6Input1 = CS47L63_DSP1RX6_INPUT1,
        Dsp1Rx6Input2 = CS47L63_DSP1RX6_INPUT2,
        Dsp1Rx6Input3 = CS47L63_DSP1RX6_INPUT3,
        Dsp1Rx6Input4 = CS47L63_DSP1RX6_INPUT4,
        Dsp1Rx7Input1 = CS47L63_DSP1RX7_INPUT1,
        Dsp1Rx7Input2 = CS47L63_DSP1RX7_INPUT2,
        Dsp1Rx7Input3 = CS47L63_DSP1RX7_INPUT3,
        Dsp1Rx7Input4 = CS47L63_DSP1RX7_INPUT4,
        Dsp1Rx8Input1 = CS47L63_DSP1RX8_INPUT1,
        Dsp1Rx8Input2 = CS47L63_DSP1RX8_INPUT2,
        Dsp1Rx8Input3 = CS47L63_DSP1RX8_INPUT3,
        Dsp1Rx8Input4 = CS47L63_DSP1RX8_INPUT4,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum InputSourceSelect {
        Silence = 0x000,
        ToneGenerator1 = 0x004,
        ToneGenerator2 = 0x005,
        NoiseGenerator = 0x00C,
        In1LSignalPath = 0x010,
        In1RSignalPath = 0x011,
        In2LSignalPath = 0x012,
        In2RSignalPath = 0x013,
        Asp1Rx1 = 0x020,
        Asp1Rx2 = 0x021,
        Asp1Rx3 = 0x022,
        Asp1Rx4 = 0x023,
        Asp1Rx5 = 0x024,
        Asp1Rx6 = 0x025,
        Asp1Rx7 = 0x026,
        Asp1Rx8 = 0x027,
        Asp2Rx1 = 0x030,
        Asp2Rx2 = 0x031,
        Asp2Rx3 = 0x032,
        Asp2Rx4 = 0x033,
        ASrc1In1Left = 0x088,
        ASrc1In1Right = 0x089,
        ASrc1In2Left = 0x08A,
        ASrc1In2Right = 0x08B,
        LSrc2InLeft = 0x08C,
        LSrc2InRight = 0x08D,
        LSrc3InLeft = 0x090,
        LSrc3InRight = 0x091,
        ISrc1Int1 = 0x098,
        ISrc1Int2 = 0x099,
        ISrc1Int3 = 0x09A,
        ISrc1Int4 = 0x09B,
        ISrc1Dec1 = 0x09C,
        ISrc1Dec2 = 0x09D,
        ISrc1Dec3 = 0x09E,
        ISrc1Dec4 = 0x09F,
        ISrc2Int1 = 0x0A0,
        ISrc2Int2 = 0x0A1,
        ISrc2Dec1 = 0x0A4,
        ISrc2Dec2 = 0x0A5,
        ISrc3Int1 = 0x0A8,
        ISrc3Int2 = 0x0A9,
        ISrc3Dec1 = 0x0AC,
        ISrc3Dec2 = 0x0AD,
        Eq1 = 0x0B8,
        Eq2 = 0x0B9,
        Eq3 = 0x0BA,
        Eq4 = 0x0BB,
        Drc1Left = 0x0C0,
        Drc1Right = 0x0C1,
        Drc2Left = 0x0C2,
        Drc2Right = 0x0C3,
        LHpF1 = 0x0C8,
        LHpF2 = 0x0C9,
        LHpF3 = 0x0CA,
        LHpF4 = 0x0CB,
        Dsp1Channel1 = 0x100,
        Dsp1Channel2 = 0x101,
        Dsp1Channel3 = 0x102,
        Dsp1Channel4 = 0x103,
        Dsp1Channel5 = 0x104,
        Dsp1Channel6 = 0x105,
        Dsp1Channel7 = 0x106,
        Dsp1Channel8 = 0x107,
    }

    pub struct FxStatus {
        pub lhpf1_enabled: bool, // default false
        pub lhpf2_enabled: bool,
        pub lhpf3_enabled: bool,
        pub lhpf4_enabled: bool,
        pub drc1_left_enabled: bool,
        pub drc1_right_enabled: bool,
        pub drc2_left_enabled: bool,
        pub drc2_right_enabled: bool,
        pub eq1_enabled: bool,
        pub eq2_enabled: bool,
        pub eq3_enabled: bool,
        pub eq4_enabled: bool,
    }

    impl FxStatus {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_FX_STATUS,
                (self.lhpf1_enabled as u32)
                    | (self.lhpf2_enabled as u32) << 1
                    | (self.lhpf3_enabled as u32) << 2
                    | (self.lhpf4_enabled as u32) << 3
                    | (self.drc2_left_enabled as u32) << 4
                    | (self.drc1_right_enabled as u32) << 5
                    | (self.drc1_left_enabled as u32) << 6
                    | (self.drc2_right_enabled as u32) << 7
                    | (self.eq1_enabled as u32) << 8
                    | (self.eq2_enabled as u32) << 9
                    | (self.eq3_enabled as u32) << 10
                    | (self.eq4_enabled as u32) << 11,
            ]
        }
    }
}

// 4.3.4 Five-Band Parametric Equalizer (EQ)

pub mod equalizer {

    use crate::spec::*;

    pub struct EqControl1 {
        pub eq1_enabled: bool, // default false
        pub eq2_enabled: bool,
        pub eq3_enabled: bool,
        pub eq4_enabled: bool,
    }

    impl EqControl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_EQ_CONTROL1,
                (self.eq1_enabled as u32)
                    | (self.eq2_enabled as u32) << 1
                    | (self.eq3_enabled as u32) << 2
                    | (self.eq4_enabled as u32) << 3,
            ]
        }
    }

    pub struct EqControl2 {
        pub eq1_band1_mode: Band1Mode,
        pub eq2_band1_mode: Band1Mode,
        pub eq3_band1_mode: Band1Mode,
        pub eq4_band1_mode: Band1Mode,
    }

    impl EqControl2 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_EQ_CONTROL2,
                (self.eq1_band1_mode as u32)
                    | (self.eq2_band1_mode as u32) << 1
                    | (self.eq3_band1_mode as u32) << 2
                    | (self.eq4_band1_mode as u32) << 3,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Band1Mode {
        ShelvingFilter = 0x00, // default
        PeakFilter = 0x01,
    }

    pub struct EqGain1 {
        pub reg: EqGain1Reg,
        pub band1_gain: EqBandGain,
        pub band2_gain: EqBandGain,
        pub band3_gain: EqBandGain,
        pub band4_gain: EqBandGain,
    }

    impl EqGain1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                self.reg as u32,
                (self.band1_gain as u32)
                    | (self.band2_gain as u32) << 8
                    | (self.band3_gain as u32) << 16
                    | (self.band4_gain as u32) << 24,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum EqGain1Reg {
        Eq1Gain1 = CS47L63_EQ1_GAIN1,
        Eq2Gain1 = CS47L63_EQ2_GAIN1,
        Eq3Gain1 = CS47L63_EQ3_GAIN1,
        Eq4Gain1 = CS47L63_EQ4_GAIN1,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum EqBandGain {
        Minus12db = 0x00,
        Minus11db = 0x01,
        Minus10db = 0x02,
        Minus09db = 0x03,
        Minus08db = 0x04,
        Minus07db = 0x05,
        Minus06db = 0x06,
        Minus05db = 0x07,
        Minus04db = 0x08,
        Minus03db = 0x09,
        Minus02db = 0x0A,
        Minus01db = 0x0B,
        ZeroDb = 0x0C, // default
        Plus01db = 0x0D,
        Plus02db = 0x0E,
        Plus03db = 0x0F,
        Plus04db = 0x10,
        Plus05db = 0x11,
        Plus06db = 0x12,
        Plus07db = 0x13,
        Plus08db = 0x14,
        Plus09db = 0x15,
        Plus10db = 0x16,
        Plus11db = 0x17,
        Plus12db = 0x18,
    }

    pub struct EqGain2 {
        pub reg: EqGain2Reg,
        pub band5_gain: EqBandGain,
    }

    impl EqGain2 {
        pub const fn serialize(&self) -> [u32; 2] {
            [self.reg as u32, (self.band5_gain as u32)]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum EqGain2Reg {
        Eq1Gain2 = CS47L63_EQ1_GAIN2,
        Eq2Gain2 = CS47L63_EQ2_GAIN2,
        Eq3Gain2 = CS47L63_EQ3_GAIN2,
        Eq4Gain2 = CS47L63_EQ4_GAIN2,
    }

    pub struct EqBandCoeff1 {
        pub reg: EqBandCoeff1Reg,
        pub a: u16,
        pub b: u16,
    }

    impl EqBandCoeff1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [self.reg as u32, (self.a as u32) | (self.b as u32) << 16]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum EqBandCoeff1Reg {
        Eq1Band1Coeff1 = CS47L63_EQ1_BAND1_COEFF1,
        Eq1Band2Coeff1 = CS47L63_EQ1_BAND2_COEFF1,
        Eq1Band3Coeff1 = CS47L63_EQ1_BAND3_COEFF1,
        Eq1Band4Coeff1 = CS47L63_EQ1_BAND4_COEFF1,
        Eq1Band5Coeff1 = CS47L63_EQ1_BAND5_COEFF1,
        Eq2Band1Coeff1 = CS47L63_EQ2_BAND1_COEFF1,
        Eq2Band2Coeff1 = CS47L63_EQ2_BAND2_COEFF1,
        Eq2Band3Coeff1 = CS47L63_EQ2_BAND3_COEFF1,
        Eq2Band4Coeff1 = CS47L63_EQ2_BAND4_COEFF1,
        Eq2Band5Coeff1 = CS47L63_EQ2_BAND5_COEFF1,
        Eq3Band1Coeff1 = CS47L63_EQ3_BAND1_COEFF1,
        Eq3Band2Coeff1 = CS47L63_EQ3_BAND2_COEFF1,
        Eq3Band3Coeff1 = CS47L63_EQ3_BAND3_COEFF1,
        Eq3Band4Coeff1 = CS47L63_EQ3_BAND4_COEFF1,
        Eq3Band5Coeff1 = CS47L63_EQ3_BAND5_COEFF1,
        Eq4Band1Coeff1 = CS47L63_EQ4_BAND1_COEFF1,
        Eq4Band2Coeff1 = CS47L63_EQ4_BAND2_COEFF1,
        Eq4Band3Coeff1 = CS47L63_EQ4_BAND3_COEFF1,
        Eq4Band4Coeff1 = CS47L63_EQ4_BAND4_COEFF1,
        Eq4Band5Coeff1 = CS47L63_EQ4_BAND5_COEFF1,
    }

    pub struct EqBandCoeff2 {
        pub reg: EqBandCoeff2Reg,
        pub c: u16,
    }

    impl EqBandCoeff2 {
        pub const fn serialize(&self) -> [u32; 2] {
            [self.reg as u32, (self.c as u32)]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum EqBandCoeff2Reg {
        Eq1Band1Coeff2 = CS47L63_EQ1_BAND1_COEFF2,
        Eq1Band2Coeff2 = CS47L63_EQ1_BAND2_COEFF2,
        Eq1Band3Coeff2 = CS47L63_EQ1_BAND3_COEFF2,
        Eq1Band4Coeff2 = CS47L63_EQ1_BAND4_COEFF2,
        Eq2Band1Coeff2 = CS47L63_EQ2_BAND1_COEFF2,
        Eq2Band2Coeff2 = CS47L63_EQ2_BAND2_COEFF2,
        Eq2Band3Coeff2 = CS47L63_EQ2_BAND3_COEFF2,
        Eq2Band4Coeff2 = CS47L63_EQ2_BAND4_COEFF2,
        Eq3Band1Coeff2 = CS47L63_EQ3_BAND1_COEFF2,
        Eq3Band2Coeff2 = CS47L63_EQ3_BAND2_COEFF2,
        Eq3Band3Coeff2 = CS47L63_EQ3_BAND3_COEFF2,
        Eq3Band4Coeff2 = CS47L63_EQ3_BAND4_COEFF2,
        Eq4Band1Coeff2 = CS47L63_EQ4_BAND1_COEFF2,
        Eq4Band2Coeff2 = CS47L63_EQ4_BAND2_COEFF2,
        Eq4Band3Coeff2 = CS47L63_EQ4_BAND3_COEFF2,
        Eq4Band4Coeff2 = CS47L63_EQ4_BAND4_COEFF2,
    }

    pub struct EqBandPg {
        pub reg: EqBandPgReg,
        pub pg: u16,
    }

    impl EqBandPg {
        pub const fn serialize(&self) -> [u32; 2] {
            [self.reg as u32, (self.pg as u32)]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum EqBandPgReg {
        Eq1Band1Pg = CS47L63_EQ1_BAND1_PG,
        Eq1Band2Pg = CS47L63_EQ1_BAND2_PG,
        Eq1Band3Pg = CS47L63_EQ1_BAND3_PG,
        Eq1Band4Pg = CS47L63_EQ1_BAND4_PG,
        Eq1Band5Pg = CS47L63_EQ1_BAND5_PG,
        Eq2Band1Pg = CS47L63_EQ2_BAND1_PG,
        Eq2Band2Pg = CS47L63_EQ2_BAND2_PG,
        Eq2Band3Pg = CS47L63_EQ2_BAND3_PG,
        Eq2Band4Pg = CS47L63_EQ2_BAND4_PG,
        Eq2Band5Pg = CS47L63_EQ2_BAND5_PG,
        Eq3Band1Pg = CS47L63_EQ3_BAND1_PG,
        Eq3Band2Pg = CS47L63_EQ3_BAND2_PG,
        Eq3Band3Pg = CS47L63_EQ3_BAND3_PG,
        Eq3Band4Pg = CS47L63_EQ3_BAND4_PG,
        Eq3Band5Pg = CS47L63_EQ3_BAND5_PG,
        Eq4Band1Pg = CS47L63_EQ4_BAND1_PG,
        Eq4Band2Pg = CS47L63_EQ4_BAND2_PG,
        Eq4Band3Pg = CS47L63_EQ4_BAND3_PG,
        Eq4Band4Pg = CS47L63_EQ4_BAND4_PG,
        Eq4Band5Pg = CS47L63_EQ4_BAND5_PG,
    }
}

// ***********************************************************
// 4.3.5 Dynamic Range Control (DRC also known as compression)
// ***********************************************************
//
//              output level (dB)
//
//                     ▲
//                     │
//                     │
//                     ├───────────────────────────────────────────────────────────────────┐
//                     │                                                                xxx│
//                     │                                                        xxxxxxxxx  │
//                     │                                                xxxxxxxx           │
// knee1 output level  ├───────────────────────────────────────────xxxxxx     ▲            │
//                     │                                     xxxxxx│          │            │
//                     │                                  xxxx     │          │            │
//                     │                               xxxx        │          └────────────┼──────────────  upper compressor slope (knee1)
//                     │                              xx           │                       │
//                     │                            xx             │                       │
//                     │                         xxx   <───────────┼───────────────────────┼──────────────  lower compressor slope (knee1)
//                     │                      xxxx                 │                       │
//                     │                    xxx                    │                       │
//                     ├───────────────────xx                      │                       │
//                     │                   x                       │                       │
//                     │                   x                       │                       │
//                     │                   x                       │                       │
// knee2 output level  ├──────────────────xx                       │                       │
//                     │                 xx│                       │                       │
//                     │                xx │                       │                       │
//                     │               xx  │                       │                       │
//                     │              xx   │                       │                       │
//                     │            xx     │                       │                       │
//                     │           xx      │                       │                       │
//                     │         xx        │                       │                       │
//                     │        xx  <──────┼───────────────────────┼───────────────────────┼──────────────  noise-gate slope (knee2)
//                     │      xx           │                       │                       │
//                     │    xxx            │                       │                       │
//                     │   xx              │                       │                       │
//                     │  xx               │                       │                       │
//                     │xxx                │                       │                       │
//                     ────────────────────┴───────────────────────┴───────────────────────┴─────────────>  input level (dB)
//                                 knee2 input level       knee1 input level
//

pub mod compression {

    use crate::registers::utils::truncate_to;
    use crate::spec::*;

    pub struct DrcControl1 {
        pub reg: DrcControl1Reg,
        pub left_enabled: bool,
        pub right_enabled: bool,
    }

    impl DrcControl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                self.reg as u32,
                (self.right_enabled as u32) | (self.left_enabled as u32) << 1,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DrcControl1Reg {
        Drc1Control1 = CS47L63_DRC1_CONTROL1,
        Drc2Control1 = CS47L63_DRC2_CONTROL1,
    }

    pub struct DrcControl2 {
        pub reg: DrcControl2Reg,
        pub anticlip_enabled: bool,
        pub quick_release_enabled: bool,
        pub knee2_output_enabled: bool, // noise-gate output - see Drc1Control4.knee2_output_level
        pub signal_detect_enabled: bool,
        pub signal_detect_mode: SignalDetectMode,
        pub knee2_input_enabled: bool, // noise-gate input - see Drc1Control4.knee2_input_level
        pub signal_detect_peak_threshold: SignalDetectPeakThreshold,
        pub signal_detect_rms_threshold: u8, // 0x00 to 0x1F (-30db to -76.5db in 1.5db steps)
        pub max_gain: MaxGain,
        pub min_gain: MinGain,
        pub gain_decay_rate: GainDecayRate,
        pub gain_attack_rate: GainAttackRate,
    }

    impl DrcControl2 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                self.reg as u32,
                (self.anticlip_enabled as u32) << 3
                    | (self.quick_release_enabled as u32) << 4
                    | (self.knee2_output_enabled as u32) << 5
                    | (self.signal_detect_enabled as u32) << 6
                    | (self.signal_detect_mode as u32) << 7
                    | (self.knee2_input_enabled as u32) << 8
                    | (self.signal_detect_peak_threshold as u32) << 9
                    | truncate_to(self.signal_detect_rms_threshold as u32, 5) << 11
                    | (self.max_gain as u32) << 16
                    | (self.min_gain as u32) << 18
                    | (self.gain_decay_rate as u32) << 24
                    | (self.gain_attack_rate as u32) << 28,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DrcControl2Reg {
        Drc1Control2 = CS47L63_DRC1_CONTROL2,
        Drc2Control2 = CS47L63_DRC2_CONTROL2,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum SignalDetectMode {
        PeakThreshold = 0x00,
        RmsThreshold = 0x01,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum SignalDetectPeakThreshold {
        _12db = 0b00,
        _18db = 0b01,
        _24db = 0b10,
        _30db = 0b11,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum MaxGain {
        _12db = 0b00,
        _18db = 0b01,
        _24db = 0b10,
        _36db = 0b11,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum MinGain {
        _0db = 0b000,
        Minus12db = 0b001,
        Minus18db = 0b010,
        Minus24db = 0b011,
        Minus36db = 0b100,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum GainDecayRate {
        _1_45ms = 0b0000,
        _2_9ms = 0b0001,
        _5_8ms = 0b0010,
        _11_6ms = 0b0011,
        _23_25ms = 0b0100,
        _46_5ms = 0b0101,
        _93ms = 0b0110,
        _186ms = 0b0111,
        _372ms = 0b1000,
        _743ms = 0b1001,
        _1_49s = 0b1010,
        _2_97s = 0b1011,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum GainAttackRate {
        _182us = 0b0001,
        _363us = 0b0010,
        _726us = 0b0011,
        _1_45ms = 0b0100,
        _2_9ms = 0b0101,
        _5_8ms = 0b0110,
        _11_6ms = 0b0111,
        _23_2ms = 0b1000,
        _46_4ms = 0b1001,
        _92_8ms = 0b1010,
        _185_6ms = 0b1011,
    }

    pub struct DrcControl3 {
        pub reg: DrcControl3Reg,
        pub compressor_slope_lower: CompressorSlopeLowerRegion,
        pub compressor_slope_upper: CompressorSlopeUpperRegion,
        pub quick_release_decay_rate: QuickReleaseDecayRate,
        pub quick_release_threshold: QuickReleaseThreshold,
        pub noise_gate_slope: NoiseGateSlope,
        pub noise_gate_min_gain: NoiseGateMinGain,
    }

    impl DrcControl3 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                self.reg as u32,
                (self.compressor_slope_lower as u32)
                    | (self.compressor_slope_upper as u32) << 3
                    | (self.quick_release_decay_rate as u32) << 6
                    | (self.quick_release_threshold as u32) << 8
                    | (self.noise_gate_slope as u32) << 10
                    | (self.noise_gate_min_gain as u32) << 12,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DrcControl3Reg {
        Drc1Control3 = CS47L63_DRC1_CONTROL3,
        Drc2Control3 = CS47L63_DRC2_CONTROL3,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum CompressorSlopeLowerRegion {
        _1 = 0b000, // no compression
        _1div2 = 0b001,
        _1div4 = 0b010,
        _1div8 = 0b011,
        _0 = 0b100,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum CompressorSlopeUpperRegion {
        _1 = 0b000, // no compression
        _1div2 = 0b001,
        _1div4 = 0b010,
        _1div8 = 0b011,
        _1div16 = 0b100,
        _0 = 0b101,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum QuickReleaseDecayRate {
        _0_725ms = 0b00,
        _1_45ms = 0b01,
        _5_8ms = 0b10,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum QuickReleaseThreshold {
        _12db = 0b00,
        _18db = 0b01,
        _24db = 0b10,
        _30db = 0b11,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum NoiseGateSlope {
        _1 = 0b00, // no expansion
        _2 = 0b01,
        _4 = 0b10,
        _8 = 0b11,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum NoiseGateMinGain {
        Minus36db = 0b0000,
        Minus30db = 0b0001,
        Minus24db = 0b0010,
        Minus18db = 0b0011,
        Minus12db = 0b0100,
        Minus6db = 0b0101,
        _0db = 0b0110,
        _6db = 0b0111,
        _12db = 0b1000,
        _18db = 0b1001,
        _24db = 0b1010,
        _30db = 0b1011,
        _36db = 0b1100,
    }

    pub struct DrcControl4 {
        pub reg: DrcControl4Reg,
        pub knee1_output_level: u8, // 0x00 to 0x1E (0db to -22.5db in -0.75db steps) - compressor knee
        pub knee1_input_level: u8, // 0x00 to 0x3C (0db to -45db in -0.75db steps) - compressor knee
        pub knee2_output_level: u8, // 0x00 to 0x1F (-30db to -76.5db in -1.5db steps) - noise-gate threshold see Drc1Control2.knee2_output_enabled
        pub knee2_input_level: u8, // 0x00 to 0x1F (-36db to -82.5db in -1.5db steps) - noise-gate threshold see Drc1Control2.knee2_input_enabled
    }

    impl DrcControl4 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                self.reg as u32,
                truncate_to(self.knee1_output_level as u32, 5)
                    | truncate_to(self.knee1_input_level as u32, 5) << 8
                    | truncate_to(self.knee2_output_level as u32, 5) << 16
                    | truncate_to(self.knee2_input_level as u32, 5) << 24,
            ]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DrcControl4Reg {
        Drc1Control4 = CS47L63_DRC1_CONTROL4,
        Drc2Control4 = CS47L63_DRC2_CONTROL4,
    }
}

// 4.3.11 Sample-Rate Control (not to be confused with 4.10 - Clocking and Sample-Rates)
pub mod sample_rate_control {
    use crate::spec::*;

    pub struct InputSampleRateControl {
        pub reg: InputSampleRateControlReg,
        pub sample_rate: SampleRate,
    }

    impl InputSampleRateControl {
        pub const fn serialize(&self) -> [u32; 2] {
            [self.reg as u32, (self.sample_rate as u32) << 11]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum InputSampleRateControlReg {
        InputRateControl = CS47L63_INPUT_RATE_CONTROL,
        In1LControl1 = CS47L63_IN1L_CONTROL1,
        In1RControl1 = CS47L63_IN1R_CONTROL1,
        In2LControl1 = CS47L63_IN2L_CONTROL1,
        In2RControl1 = CS47L63_IN2R_CONTROL1,
    }

    pub struct OutputControl1 {
        pub sample_rate: SampleRateInclAsync,
    }

    impl OutputControl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [CS47L63_OUTPUT_CONTROL_1, (self.sample_rate as u32) << 11]
        }
    }

    pub struct AspControl1 {
        pub reg: AspControl1Reg,
        pub sample_rate: SampleRateInclAsync,
    }

    impl AspControl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [self.reg as u32, (self.sample_rate as u32) << 8]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum AspControl1Reg {
        Asp1Control1 = CS47L63_ASP1_CONTROL1,
        Asp2Control1 = CS47L63_ASP2_CONTROL1,
    }

    pub struct FxSampleRate {
        pub sample_rate: SampleRateInclAsync,
    }

    impl FxSampleRate {
        pub const fn serialize(&self) -> [u32; 2] {
            [CS47L63_FX_SAMPLE_RATE, (self.sample_rate as u32) << 11]
        }
    }
    pub struct ToneGenerator1 {
        pub sample_rate: SampleRateInclAsync,
    }

    impl ToneGenerator1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [CS47L63_TONE_GENERATOR1, (self.sample_rate as u32) << 11]
        }
    }
    pub struct ComfortNoiseGenerator {
        pub sample_rate: SampleRateInclAsync,
    }

    impl ComfortNoiseGenerator {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_COMFORT_NOISE_GENERATOR,
                (self.sample_rate as u32) << 11,
            ]
        }
    }
    pub struct PwmDrive1 {
        pub sample_rate: SampleRateInclAsync,
    }

    impl PwmDrive1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [CS47L63_PWM_DRIVE_1, (self.sample_rate as u32) << 11]
        }
    }

    pub struct Dsp1SampleRate {
        pub reg: Dsp1SampleRateReg,
        pub sample_rate: SampleRateInclAsync,
    }

    impl Dsp1SampleRate {
        pub const fn serialize(&self) -> [u32; 2] {
            [self.reg as u32, (self.sample_rate as u32)]
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Dsp1SampleRateReg {
        Dsp1SampleRateRx1 = CS47L63_DSP1_SAMPLE_RATE_RX1,
        Dsp1SampleRateRx2 = CS47L63_DSP1_SAMPLE_RATE_RX2,
        Dsp1SampleRateRx3 = CS47L63_DSP1_SAMPLE_RATE_RX3,
        Dsp1SampleRateRx4 = CS47L63_DSP1_SAMPLE_RATE_RX4,
        Dsp1SampleRateRx5 = CS47L63_DSP1_SAMPLE_RATE_RX5,
        Dsp1SampleRateRx6 = CS47L63_DSP1_SAMPLE_RATE_RX6,
        Dsp1SampleRateRx7 = CS47L63_DSP1_SAMPLE_RATE_RX7,
        Dsp1SampleRateRx8 = CS47L63_DSP1_SAMPLE_RATE_RX8,
        Dsp1SampleRateTx1 = CS47L63_DSP1_SAMPLE_RATE_TX1,
        Dsp1SampleRateTx2 = CS47L63_DSP1_SAMPLE_RATE_TX2,
        Dsp1SampleRateTx3 = CS47L63_DSP1_SAMPLE_RATE_TX3,
        Dsp1SampleRateTx4 = CS47L63_DSP1_SAMPLE_RATE_TX4,
        Dsp1SampleRateTx5 = CS47L63_DSP1_SAMPLE_RATE_TX5,
        Dsp1SampleRateTx6 = CS47L63_DSP1_SAMPLE_RATE_TX6,
        Dsp1SampleRateTx7 = CS47L63_DSP1_SAMPLE_RATE_TX7,
        Dsp1SampleRateTx8 = CS47L63_DSP1_SAMPLE_RATE_TX8,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum SampleRate {
        SampleRate1 = 0x00,
        SampleRate2 = 0x01,
        SampleRate3 = 0x02,
        SampleRate4 = 0x03,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum SampleRateInclAsync {
        SampleRate1 = 0x00,
        SampleRate2 = 0x01,
        SampleRate3 = 0x02,
        SampleRate4 = 0x03,
        AsyncSampleRate1 = 0x08,
        AsyncSampleRate2 = 0x09,
    }
}
