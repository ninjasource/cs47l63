// ******************************
// 4.10 Clocking and Sample Rates
// ******************************

// 4.10.4 Miscellaneous Clock Controls - Sample Rate

pub mod sample_rate {
    use crate::spec::{
        CS47L63_SAMPLE_RATE1, CS47L63_SAMPLE_RATE2, CS47L63_SAMPLE_RATE3, CS47L63_SAMPLE_RATE4,
    };

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Select {
        None = 0x00,
        _12kHz = 0x01,
        _24kHz = 0x02,
        _48kHz = 0x03,
        _96kHz = 0x04,
        _192kHz = 0x05,
        _11p025KHz = 0x09,
        _22p05kHz = 0x0A,
        _44p1kHz = 0x0B,
        _88p2kHz = 0x0C,
        _176p4kHz = 0x0D,
        _8kHz = 0x11,
        _16kHz = 0x12,
        _32kHz = 0x13,
    }

    pub enum Num {
        _1,
        _2,
        _3,
        _4,
    }

    pub struct SampleRate {
        pub num: Num,
        pub select: Select,
    }

    impl SampleRate {
        pub const fn serialize(&self) -> [u32; 2] {
            let reg = match self.num {
                Num::_1 => CS47L63_SAMPLE_RATE1,
                Num::_2 => CS47L63_SAMPLE_RATE2,
                Num::_3 => CS47L63_SAMPLE_RATE3,
                Num::_4 => CS47L63_SAMPLE_RATE4,
            };

            [reg, self.select as u32]
        }
    }
}

// 4.10.4 Miscellaneous Clock Controls - Clock

pub mod clock {
    use crate::spec::{
        CS47L63_ASYNC_CLOCK1, CS47L63_ASYNC_CLOCK2, CS47L63_SYSTEM_CLOCK1, CS47L63_SYSTEM_CLOCK2,
    };

    /// Clock Fraction
    /// Applies to SYSTEM_CLOCK1 only
    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Fraction {
        MultipleOf6M144Hz = 0,
        MultipleOf5M6448Hz = 1,
    }

    /// Clock Frequency
    /// Applies to SYSTEM_CLOCK1, SYSTEM_CLOCK2, ASYNC_CLOCK1 and ASYNC_CLOCK2
    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Freqency {
        _6p144MHz = 0b000,  // (5.6448 MHz at 44.1kHz)
        _12p288MHz = 0b001, // (11.2896 MHz at 44.1kHz)
        _24p576MHz = 0b010, // (22.5792 MHz at 44.1kHz)
        _49p152MHz = 0b011, // (45.1584 MHz at 44.1kHz)
        _98p304MHz = 0b100, // (90.3168 MHz at 44.1kHz)
    }

    /// Clock Enabled
    /// Applies to SYSTEM_CLOCK1, SYSTEM_CLOCK2, ASYNC_CLOCK1 and ASYNC_CLOCK2
    #[derive(Debug, Clone, Copy)]
    pub enum Source {
        Mclk1 = 0x00,
        Mclk2 = 0x01,
        FLL1x2_90to100MHz = 0x04,
        FLL2x2_90to100MHz = 0x05,
        Asp1Bclk = 0x08,
        Asp2Bclk = 0x09,
        FLL1_45to50MHz = 0x0C,
        FLL2_45to50MHz = 0x0D,
    }

    pub const fn system_clock_2(freq: Freqency, src: Source) -> [u32; 2] {
        [CS47L63_SYSTEM_CLOCK2, (freq as u32) << 8 | src as u32]
    }

    pub struct SystemClock1 {
        pub frac: Fraction,
        pub freq: Freqency,
        pub enabled: bool,
        pub src: Source,
    }

    impl SystemClock1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_SYSTEM_CLOCK1,
                (self.frac as u32) << 15
                    | (self.freq as u32) << 8
                    | (self.enabled as u32) << 6
                    | self.src as u32,
            ]
        }
    }

    pub struct AsyncClock1 {
        pub freq: Freqency,
        pub enabled: bool,
        pub src: Source,
    }

    impl AsyncClock1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_ASYNC_CLOCK1,
                (self.freq as u32) << 8 | (self.enabled as u32) << 6 | self.src as u32,
            ]
        }
    }

    pub struct AsyncClock2 {
        pub freq: Freqency,
        pub src: Source,
    }

    impl AsyncClock2 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_ASYNC_CLOCK2,
                (self.freq as u32) << 8 | self.src as u32,
            ]
        }
    }
}

// 4.10.7 Frequency-Locked Loop

pub mod fll {
    use crate::registers::utils::truncate_to;
    use crate::spec::{
        CS47L63_FLL1_CONTROL1, CS47L63_FLL1_CONTROL2, CS47L63_FLL1_CONTROL3,
        CS47L63_FLL1_GPIO_CLOCK, CS47L63_FLL2_GPIO_CLOCK,
    };

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum ReferenceClockDivider {
        _1 = 0b00,
        _2 = 0b01,
        _4 = 0b10,
        _8 = 0b11,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum ReferenceClockSource {
        MCLK1 = 0x0,
        MCLK2 = 0x1,
        InternalOscillator = 0x2,
        NoInput = 0x3,
        ASP1BCLK = 0x8,
        ASP2BCLK = 0x9,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DetectorGainSigned {
        _1 = 0b0000,
        _0p5 = 0b0001,
        _0p25 = 0b0010,
        _0p125 = 0b0011,
        _2PowMinus4 = 0b0100,
        _2PowMinus5 = 0b0101,
        _2PowMinus6 = 0b0110,
        _2PowMinus7 = 0b0111,
        _256 = 0b1000,
        _128 = 0b1001,
        _64 = 0b1010,
        _32 = 0b1011,
        _16 = 0b1100,
        _8 = 0b1101,
        _4 = 0b1110,
        _2 = 0b1111,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DetectorGainUnsigned {
        _1 = 0b0000,
        _0p5 = 0b0001,
        _0p25 = 0b0010,
        _0p125 = 0b0011,
        _2PowMinus4 = 0b0100,
        _2PowMinus5 = 0b0101,
        _2PowMinus6 = 0b0110,
        _2PowMinus7 = 0b0111,
        _2PowMinus8 = 0b1000,
        _2PowMinus9 = 0b1001,
        _2PowMinus10 = 0b1010,
        _2PowMinus11 = 0b1011,
        _2PowMinus12 = 0b1100,
        _2PowMinus13 = 0b1101,
        _2PowMinus14 = 0b1110,
        Disabled = 0b1111,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum FractionalMode {
        Reserved = 0b00,
        IntegerMode = 0b01,
        Reserved1 = 0b10,
        FractionalMode = 0b11,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum GpioClockSource {
        Fll = 0b00,
        Reserved = 0b01,
        Reserved1 = 0b10,
        Disabled = 0b11,
    }

    pub struct Fll1Control1 {
        pub control_update: bool,
        pub hold: bool,
        pub enabled: bool,
    }

    impl Fll1Control1 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_FLL1_CONTROL1,
                (self.control_update as u32) << 2 | (self.hold as u32) << 1 | self.enabled as u32,
            ]
        }
    }

    /// fll1 control2
    /// lock_detect_threshold (range 0-15)
    /// multiplier range 0-1023
    pub struct Fll1Control2 {
        pub lock_detect_threshold: u32,
        pub lock_detect: bool,
        pub phase_detect: bool,
        pub ref_detect: bool,
        pub divider: ReferenceClockDivider,
        pub source: ReferenceClockSource,
        pub multiplier: u32,
    }

    impl Fll1Control2 {
        pub const fn serialize(&self) -> [u32; 2] {
            let lock_detect_threshold = truncate_to(self.lock_detect_threshold, 4);
            let multiplier = truncate_to(self.multiplier, 10);
            [
                CS47L63_FLL1_CONTROL2,
                lock_detect_threshold << 28
                    | (self.lock_detect as u32) << 27
                    | (self.phase_detect as u32) << 22
                    | (self.ref_detect as u32) << 21
                    | (self.divider as u32) << 16
                    | (self.source as u32) << 12
                    | multiplier,
            ]
        }
    }

    pub struct Fll1Control3 {
        pub lambda: u16,
        pub theta: u16,
    }

    impl Fll1Control3 {
        pub const fn serialize(&self) -> [u32; 2] {
            [
                CS47L63_FLL1_CONTROL3,
                (self.lambda as u32) << 16 | self.theta as u32,
            ]
        }
    }

    /// fll1_control4
    /// clock_feedback_ratio (range 0-1023)
    pub const fn fll1_control4(
        phase_detector_gain_fine: DetectorGainSigned,
        phase_detector_gain_coarse: DetectorGainSigned,
        freq_detector_gain_fine: DetectorGainUnsigned,
        freq_detector_gain_coarse: DetectorGainSigned,
        fractional_mode: FractionalMode,
        clock_feedback_ratio: u32,
    ) -> [u32; 2] {
        let clock_feedback_ratio = truncate_to(clock_feedback_ratio, 10);
        [
            CS47L63_FLL1_CONTROL3,
            (phase_detector_gain_fine as u32) << 28
                | (phase_detector_gain_coarse as u32) << 24
                | (freq_detector_gain_fine as u32) << 20
                | (freq_detector_gain_coarse as u32) << 16
                | 1 << 14
                | (fractional_mode as u32) << 12
                | clock_feedback_ratio,
        ]
    }

    /// fll gpio clock
    /// gpio clock divider (range 2-127)
    pub struct FllGpioClock {
        pub num: Num,
        pub source: GpioClockSource,
        pub divider: u32,
        pub enabled: bool,
    }

    impl FllGpioClock {
        pub const fn serialize(&self) -> [u32; 2] {
            let reg = match self.num {
                Num::_1 => CS47L63_FLL1_GPIO_CLOCK,
                Num::_2 => CS47L63_FLL2_GPIO_CLOCK,
            };

            [
                reg,
                (self.source as u32) << 10 | self.divider << 1 | self.enabled as u32,
            ]
        }
    }

    pub enum Num {
        _1,
        _2,
    }
}
