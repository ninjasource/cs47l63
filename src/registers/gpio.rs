#![allow(dead_code)]

pub mod gpio_ctrl {
    use crate::spec::{
        CS47L63_GPIO10_CTRL1, CS47L63_GPIO11_CTRL1, CS47L63_GPIO12_CTRL1, CS47L63_GPIO1_CTRL1,
        CS47L63_GPIO2_CTRL1, CS47L63_GPIO3_CTRL1, CS47L63_GPIO4_CTRL1, CS47L63_GPIO5_CTRL1,
        CS47L63_GPIO6_CTRL1, CS47L63_GPIO7_CTRL1, CS47L63_GPIO8_CTRL1, CS47L63_GPIO9_CTRL1,
    };

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Direction {
        Output = 0,
        Input = 1,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DriveStrength {
        _4mA = 0,
        _8mA = 1,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum DebounceTime {
        _100us = 0x0,
        _1500us = 0x1,
        _3ms = 0x2,
        _6ms = 0x3,
        _12ms = 0x4,
        _24ms = 0x5,
        _48ms = 0x6,
        _96ms = 0x7,
        _192ms = 0x8,
        _384ms = 0x9,
        _768ms = 0xA,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum OutputConfig {
        Cmos = 0,
        OpenDrain = 1,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum OutputPolarity {
        NoninvertedActiveHigh = 0,
        InvertedActiveLow = 1,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum PinFunction {
        AlternateFunction = 0x000,
        ButtonDetectInputOrLogicLevelOutput = 0x001,
        DspGpio = 0x002,
        Pwm1Output = 0x080,
        Pwm2Output = 0x081,
        Alarm1Channel1Status = 0x230,
        Alarm1Channel2Status = 0x231,
        Alarm1Channel3Status = 0x232,
        Alarm1Channel4Status = 0x233,
        Timer1Status = 0x250,
        Timer2Status = 0x251,
        Timer3Status = 0x252,
        Timer4Status = 0x253,
        Spi2SlaveSelect1 = 0x608,
        Spi2SlaveSelect2 = 0x609,
        Spi2SlaveSelect3 = 0x60A,
        Spi2SlaveSelect4 = 0x60B,
    }

    /// applicable to pins 5-12 only
    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum ExtendedPinFunction {
        AlternateFunction = 0x000,
        ButtonDetectInputOrLogicLevelOutput = 0x001,
        DspGpio = 0x002,
        Irq1Output = 0x003,
        Fll1Clock = 0x010,
        Fll2Clock = 0x011,
        OscillatorClock = 0x013,
        Fll1Lock = 0x018,
        Fll2Lock = 0x01A,
        OpClkClock = 0x048,
        OpClkAsyncClock = 0x049,
        OpClkDspClock = 0x04A,
        Pwm1Output = 0x080,
        Pwm2Output = 0x081,
        InputSignalPathDetect = 0x08C,
        Asrc1In1Lock = 0x98,
        Asrc1In2Lock = 0x9A,
        Lsrc2Lock = 0x9C,
        Lsrc3Lock = 0xA0,
        OutputSignalPathStatuc = 0x1FA,
        Alarm1Channel1Status = 0x230,
        Alarm1Channel2Status = 0x231,
        Alarm1Channel3Status = 0x232,
        Alarm1Channel4Status = 0x233,
        Timer1Status = 0x250,
        Timer2Status = 0x251,
        Timer3Status = 0x252,
        Timer4Status = 0x253,
        Dsp1PowerStatus = 0x373,
        Spi2SlaveSelect1 = 0x608,
        Spi2SlaveSelect2 = 0x609,
        Spi2SlaveSelect3 = 0x60A,
        Spi2SlaveSelect4 = 0x60B,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Num {
        _1(PinFunction),
        _2(PinFunction),
        _3(PinFunction),
        _4(PinFunction),
        _5(ExtendedPinFunction),
        _6(ExtendedPinFunction),
        _7(ExtendedPinFunction),
        _8(ExtendedPinFunction),
        _9(ExtendedPinFunction),
        _10(ExtendedPinFunction),
        _11(ExtendedPinFunction),
        _12(ExtendedPinFunction),
    }

    pub struct GpioCtrl1 {
        pub num: Num,
        pub direction: Direction,
        pub pull_up_en: bool,
        pub pull_down_en: bool,
        pub drive_strength: DriveStrength,
        pub debounce_time: DebounceTime,
        pub output_level: bool,
        pub output_config: OutputConfig,
        pub debounce_en: bool,
        pub output_polarity: OutputPolarity,
    }

    impl GpioCtrl1 {
        pub const fn serialize(&self) -> [u32; 2] {
            let (reg, pin_function) = match self.num {
                Num::_1(x) => (CS47L63_GPIO1_CTRL1, x as u32),
                Num::_2(x) => (CS47L63_GPIO2_CTRL1, x as u32),
                Num::_3(x) => (CS47L63_GPIO3_CTRL1, x as u32),
                Num::_4(x) => (CS47L63_GPIO4_CTRL1, x as u32),
                Num::_5(x) => (CS47L63_GPIO5_CTRL1, x as u32),
                Num::_6(x) => (CS47L63_GPIO6_CTRL1, x as u32),
                Num::_7(x) => (CS47L63_GPIO7_CTRL1, x as u32),
                Num::_8(x) => (CS47L63_GPIO8_CTRL1, x as u32),
                Num::_9(x) => (CS47L63_GPIO9_CTRL1, x as u32),
                Num::_10(x) => (CS47L63_GPIO10_CTRL1, x as u32),
                Num::_11(x) => (CS47L63_GPIO11_CTRL1, x as u32),
                Num::_12(x) => (CS47L63_GPIO12_CTRL1, x as u32),
            };

            [
                reg,
                (self.direction as u32) << 31
                    | (self.pull_up_en as u32) << 30
                    | (self.pull_down_en as u32) << 29
                    | (self.drive_strength as u32) << 24
                    | (self.debounce_time as u32) << 16
                    | (self.output_level as u32) << 15
                    | (self.output_config as u32) << 14
                    | (self.debounce_en as u32) << 13
                    | (self.output_polarity as u32) << 12
                    | pin_function,
            ]
        }
    }
}
