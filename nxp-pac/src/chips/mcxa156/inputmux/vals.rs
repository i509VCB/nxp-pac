#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0TrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM0_OUT_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM0_OUT_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_OUT_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM1_OUT_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_OUT_TRIG0 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM0_SM2_OUT_TRIG1 input is selected."]
    Val23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "WUU."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    Val34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    Val36 = 0x24,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val37 = 0x25,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val38 = 0x26,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val39 = 0x27,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH0 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH1 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH2 input is selected."]
    Val47 = 0x2f,
    #[doc = "FlexIO CH3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val49 = 0x31,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Adc0TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Adc0TrigTrigin {
        Adc0TrigTrigin::from_bits(val)
    }
}
impl From<Adc0TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Adc0TrigTrigin) -> u8 {
        Adc0TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1TrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM0_OUT_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM0_OUT_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_OUT_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM1_OUT_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_OUT_TRIG0 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM0_SM2_OUT_TRIG1 input is selected."]
    Val23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "WUU."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    Val34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    Val36 = 0x24,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val37 = 0x25,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val38 = 0x26,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val39 = 0x27,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH0 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH1 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH2 input is selected."]
    Val47 = 0x2f,
    #[doc = "FlexIO CH3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val49 = 0x31,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Adc1TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Adc1TrigTrigin {
        Adc1TrigTrigin::from_bits(val)
    }
}
impl From<Adc1TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Adc1TrigTrigin) -> u8 {
        Adc1TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aoi0InputInp {
    _RESERVED_0 = 0x0,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val1 = 0x01,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val2 = 0x02,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val3 = 0x03,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val4 = 0x04,
    #[doc = "CMP0_OUT input is selected."]
    Val5 = 0x05,
    #[doc = "CMP1_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT0."]
    Val12 = 0x0c,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val14 = 0x0e,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val15 = 0x0f,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val16 = 0x10,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val17 = 0x11,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val18 = 0x12,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val19 = 0x13,
    #[doc = "LPTMR0 input is selected."]
    Val20 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val22 = 0x16,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val23 = 0x17,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val24 = 0x18,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val25 = 0x19,
    #[doc = "QDC0_POS_MATCH input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM0_MUX_TRIG0 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "TRIG_IN0 input is selected."]
    Val35 = 0x23,
    #[doc = "TRIG_IN1 input is selected."]
    Val36 = 0x24,
    #[doc = "TRIG_IN2 input is selected."]
    Val37 = 0x25,
    #[doc = "TRIG_IN3 input is selected."]
    Val38 = 0x26,
    #[doc = "TRIG_IN4 input is selected."]
    Val39 = 0x27,
    #[doc = "TRIG_IN5 input is selected."]
    Val40 = 0x28,
    #[doc = "TRIG_IN6 input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN7 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN8 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN9 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN10 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN11 input is selected."]
    Val46 = 0x2e,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val47 = 0x2f,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val48 = 0x30,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val49 = 0x31,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val51 = 0x33,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val52 = 0x34,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val53 = 0x35,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val54 = 0x36,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val55 = 0x37,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val56 = 0x38,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val57 = 0x39,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val58 = 0x3a,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val59 = 0x3b,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val60 = 0x3c,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val61 = 0x3d,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val62 = 0x3e,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val63 = 0x3f,
    #[doc = "FlexIO CH0 input is selected."]
    Val64 = 0x40,
    #[doc = "FlexIO CH1 input is selected."]
    Val65 = 0x41,
    #[doc = "FlexIO CH2 input is selected."]
    Val66 = 0x42,
    #[doc = "FlexIO CH3 input is selected."]
    Val67 = 0x43,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val68 = 0x44,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val69 = 0x45,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val70 = 0x46,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val71 = 0x47,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val72 = 0x48,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val73 = 0x49,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val74 = 0x4a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val75 = 0x4b,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val76 = 0x4c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val77 = 0x4d,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val78 = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Aoi0InputInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aoi0InputInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aoi0InputInp {
    #[inline(always)]
    fn from(val: u8) -> Aoi0InputInp {
        Aoi0InputInp::from_bits(val)
    }
}
impl From<Aoi0InputInp> for u8 {
    #[inline(always)]
    fn from(val: Aoi0InputInp) -> u8 {
        Aoi0InputInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aoi1InputInp {
    _RESERVED_0 = 0x0,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val1 = 0x01,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val2 = 0x02,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val3 = 0x03,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val4 = 0x04,
    #[doc = "CMP0_OUT input is selected."]
    Val5 = 0x05,
    #[doc = "CMP1_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT0."]
    Val12 = 0x0c,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val14 = 0x0e,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val15 = 0x0f,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val16 = 0x10,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val17 = 0x11,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val18 = 0x12,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val19 = 0x13,
    #[doc = "LPTMR0 input is selected."]
    Val20 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val22 = 0x16,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val23 = 0x17,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val24 = 0x18,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val25 = 0x19,
    #[doc = "QDC0_POS_MATCH input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM0_MUX_TRIG0 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "TRIG_IN0 input is selected."]
    Val35 = 0x23,
    #[doc = "TRIG_IN1 input is selected."]
    Val36 = 0x24,
    #[doc = "TRIG_IN2 input is selected."]
    Val37 = 0x25,
    #[doc = "TRIG_IN3 input is selected."]
    Val38 = 0x26,
    #[doc = "TRIG_IN4 input is selected."]
    Val39 = 0x27,
    #[doc = "TRIG_IN5 input is selected."]
    Val40 = 0x28,
    #[doc = "TRIG_IN6 input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN7 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN8 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN9 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN10 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN11 input is selected."]
    Val46 = 0x2e,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val47 = 0x2f,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val48 = 0x30,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val49 = 0x31,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val51 = 0x33,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val52 = 0x34,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val53 = 0x35,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val54 = 0x36,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val55 = 0x37,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val56 = 0x38,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val57 = 0x39,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val58 = 0x3a,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val59 = 0x3b,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val60 = 0x3c,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val61 = 0x3d,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val62 = 0x3e,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val63 = 0x3f,
    #[doc = "FlexIO CH0 input is selected."]
    Val64 = 0x40,
    #[doc = "FlexIO CH1 input is selected."]
    Val65 = 0x41,
    #[doc = "FlexIO CH2 input is selected."]
    Val66 = 0x42,
    #[doc = "FlexIO CH3 input is selected."]
    Val67 = 0x43,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val68 = 0x44,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val69 = 0x45,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val70 = 0x46,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val71 = 0x47,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val72 = 0x48,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val73 = 0x49,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val74 = 0x4a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val75 = 0x4b,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val76 = 0x4c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val77 = 0x4d,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val78 = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Aoi1InputInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aoi1InputInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aoi1InputInp {
    #[inline(always)]
    fn from(val: u8) -> Aoi1InputInp {
        Aoi1InputInp::from_bits(val)
    }
}
impl From<Aoi1InputInp> for u8 {
    #[inline(always)]
    fn from(val: Aoi1InputInp) -> u8 {
        Aoi1InputInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0TrigTrigin {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP1_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer1_MAT0."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "LPTMR0 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "QDC0_POS_MATCH0."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val22 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    #[doc = "CTimer3_MAT0."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT1."]
    Val40 = 0x28,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val47 = 0x2f,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val48 = 0x30,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val49 = 0x31,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Cmp0TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Cmp0TrigTrigin {
        Cmp0TrigTrigin::from_bits(val)
    }
}
impl From<Cmp0TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Cmp0TrigTrigin) -> u8 {
        Cmp0TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1TrigTrigin {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer1_MAT0."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "LPTMR0 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "QDC0_POS_MATCH0."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val22 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    #[doc = "CTimer3_MAT0."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT1."]
    Val40 = 0x28,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val47 = 0x2f,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val48 = 0x30,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val49 = 0x31,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Cmp1TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Cmp1TrigTrigin {
        Cmp1TrigTrigin::from_bits(val)
    }
}
impl From<Cmp1TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Cmp1TrigTrigin) -> u8 {
        Cmp1TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer0capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0capInp {
        Ctimer0capInp::from_bits(val)
    }
}
impl From<Ctimer0capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0capInp) -> u8 {
        Ctimer0capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer1capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1capInp {
        Ctimer1capInp::from_bits(val)
    }
}
impl From<Ctimer1capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1capInp) -> u8 {
        Ctimer1capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer2capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2capInp {
        Ctimer2capInp::from_bits(val)
    }
}
impl From<Ctimer2capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2capInp) -> u8 {
        Ctimer2capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer3capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3capInp {
        Ctimer3capInp::from_bits(val)
    }
}
impl From<Ctimer3capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3capInp) -> u8 {
        Ctimer3capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer4capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4capInp {
        Ctimer4capInp::from_bits(val)
    }
}
impl From<Ctimer4capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4capInp) -> u8 {
        Ctimer4capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0TrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "WUU input is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    Val34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    Val36 = 0x24,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val37 = 0x25,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val38 = 0x26,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val39 = 0x27,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Dac0TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Dac0TrigTrigin {
        Dac0TrigTrigin::from_bits(val)
    }
}
impl From<Dac0TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Dac0TrigTrigin) -> u8 {
        Dac0TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtTrig0Inp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "LPUART0 input is selected."]
    Val9 = 0x09,
    #[doc = "LPUART1 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPUART2 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPUART3 input is selected."]
    Val12 = 0x0c,
    #[doc = "LPUART4 input is selected."]
    Val13 = 0x0d,
    #[doc = "AOI1_OUT0 input is selected."]
    Val14 = 0x0e,
    #[doc = "AOI1_OUT1 input is selected."]
    Val15 = 0x0f,
    #[doc = "AOI1_OUT2 input is selected."]
    Val16 = 0x10,
    #[doc = "AOI1_OUT3 input is selected."]
    Val17 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl ExtTrig0Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtTrig0Inp {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtTrig0Inp {
    #[inline(always)]
    fn from(val: u8) -> ExtTrig0Inp {
        ExtTrig0Inp::from_bits(val)
    }
}
impl From<ExtTrig0Inp> for u8 {
    #[inline(always)]
    fn from(val: ExtTrig0Inp) -> u8 {
        ExtTrig0Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0FaultTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0FaultTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0FaultTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0FaultTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0FaultTrigin {
        FlexPwm0FaultTrigin::from_bits(val)
    }
}
impl From<FlexPwm0FaultTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0FaultTrigin) -> u8 {
        FlexPwm0FaultTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0ForceTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0ForceTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0ForceTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0ForceTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0ForceTrigin {
        FlexPwm0ForceTrigin::from_bits(val)
    }
}
impl From<FlexPwm0ForceTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0ForceTrigin) -> u8 {
        FlexPwm0ForceTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0Sm0Exta0Trigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0Sm0Exta0Trigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0Sm0Exta0Trigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0Sm0Exta0Trigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0Sm0Exta0Trigin {
        FlexPwm0Sm0Exta0Trigin::from_bits(val)
    }
}
impl From<FlexPwm0Sm0Exta0Trigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0Sm0Exta0Trigin) -> u8 {
        FlexPwm0Sm0Exta0Trigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0Sm0ExtsyncTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0Sm0ExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0Sm0ExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0Sm0ExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0Sm0ExtsyncTrigin {
        FlexPwm0Sm0ExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm0Sm0ExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0Sm0ExtsyncTrigin) -> u8 {
        FlexPwm0Sm0ExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0Sm1ExtaTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0Sm1ExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0Sm1ExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0Sm1ExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0Sm1ExtaTrigin {
        FlexPwm0Sm1ExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm0Sm1ExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0Sm1ExtaTrigin) -> u8 {
        FlexPwm0Sm1ExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0Sm1ExtsyncTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0Sm1ExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0Sm1ExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0Sm1ExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0Sm1ExtsyncTrigin {
        FlexPwm0Sm1ExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm0Sm1ExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0Sm1ExtsyncTrigin) -> u8 {
        FlexPwm0Sm1ExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0Sm2ExtaTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0Sm2ExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0Sm2ExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0Sm2ExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0Sm2ExtaTrigin {
        FlexPwm0Sm2ExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm0Sm2ExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0Sm2ExtaTrigin) -> u8 {
        FlexPwm0Sm2ExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0Sm2ExtsyncTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0Sm2ExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0Sm2ExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0Sm2ExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0Sm2ExtsyncTrigin {
        FlexPwm0Sm2ExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm0Sm2ExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0Sm2ExtsyncTrigin) -> u8 {
        FlexPwm0Sm2ExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1FaultTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1FaultTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1FaultTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1FaultTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1FaultTrigin {
        FlexPwm1FaultTrigin::from_bits(val)
    }
}
impl From<FlexPwm1FaultTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1FaultTrigin) -> u8 {
        FlexPwm1FaultTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1ForceTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1ForceTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1ForceTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1ForceTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1ForceTrigin {
        FlexPwm1ForceTrigin::from_bits(val)
    }
}
impl From<FlexPwm1ForceTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1ForceTrigin) -> u8 {
        FlexPwm1ForceTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1Sm0Exta0Trigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1Sm0Exta0Trigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1Sm0Exta0Trigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1Sm0Exta0Trigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1Sm0Exta0Trigin {
        FlexPwm1Sm0Exta0Trigin::from_bits(val)
    }
}
impl From<FlexPwm1Sm0Exta0Trigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1Sm0Exta0Trigin) -> u8 {
        FlexPwm1Sm0Exta0Trigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1Sm0ExtsyncTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1Sm0ExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1Sm0ExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1Sm0ExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1Sm0ExtsyncTrigin {
        FlexPwm1Sm0ExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm1Sm0ExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1Sm0ExtsyncTrigin) -> u8 {
        FlexPwm1Sm0ExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1Sm1ExtaTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1Sm1ExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1Sm1ExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1Sm1ExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1Sm1ExtaTrigin {
        FlexPwm1Sm1ExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm1Sm1ExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1Sm1ExtaTrigin) -> u8 {
        FlexPwm1Sm1ExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1Sm1ExtsyncTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1Sm1ExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1Sm1ExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1Sm1ExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1Sm1ExtsyncTrigin {
        FlexPwm1Sm1ExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm1Sm1ExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1Sm1ExtsyncTrigin) -> u8 {
        FlexPwm1Sm1ExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1Sm2ExtaTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1Sm2ExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1Sm2ExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1Sm2ExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1Sm2ExtaTrigin {
        FlexPwm1Sm2ExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm1Sm2ExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1Sm2ExtaTrigin) -> u8 {
        FlexPwm1Sm2ExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1Sm2ExtsyncTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1Sm2ExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1Sm2ExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1Sm2ExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1Sm2ExtsyncTrigin {
        FlexPwm1Sm2ExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm1Sm2ExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1Sm2ExtsyncTrigin) -> u8 {
        FlexPwm1Sm2ExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "AOI0_OUT0 input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT1 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT2 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val5 = 0x05,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val6 = 0x06,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val7 = 0x07,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val8 = 0x08,
    #[doc = "CMP0_OUT input is selected."]
    Val9 = 0x09,
    #[doc = "CMP1_OUT input is selected."]
    Val10 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val15 = 0x0f,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val16 = 0x10,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val17 = 0x11,
    #[doc = "LPTMR0 input is selected."]
    Val18 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val22 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "WUU input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_A0_TRIG0 input is selected."]
    Val37 = 0x25,
    #[doc = "LPI2C0 Master End of Packet."]
    Val38 = 0x26,
    #[doc = "LPI2C0 Slave End of Packet."]
    Val39 = 0x27,
    #[doc = "LPI2C1 Master End of Packet."]
    Val40 = 0x28,
    #[doc = "LPI2C1 Slave End of Packet."]
    Val41 = 0x29,
    #[doc = "LPSPI0 End of Frame."]
    Val42 = 0x2a,
    #[doc = "LPSPI0 Received Data Word."]
    Val43 = 0x2b,
    #[doc = "LPSPI1 End of Frame."]
    Val44 = 0x2c,
    #[doc = "LPSPI1 Received Data Word."]
    Val45 = 0x2d,
    #[doc = "LPUART0 Received Data Word."]
    Val46 = 0x2e,
    #[doc = "LPUART0 Transmitted Data Word."]
    Val47 = 0x2f,
    #[doc = "LPUART0 Receive Line Idle."]
    Val48 = 0x30,
    #[doc = "LPUART1 Received Data Word."]
    Val49 = 0x31,
    #[doc = "LPUART1 Transmitted Data Word."]
    Val50 = 0x32,
    #[doc = "LPUART1 Receive Line Idle."]
    Val51 = 0x33,
    #[doc = "LPUART2 Received Data Word."]
    Val52 = 0x34,
    #[doc = "LPUART2 Transmitted Data Word."]
    Val53 = 0x35,
    #[doc = "LPUART2 Receive Line Idle."]
    Val54 = 0x36,
    #[doc = "LPUART3 Received Data Word."]
    Val55 = 0x37,
    #[doc = "LPUART3 Transmitted Data Word."]
    Val56 = 0x38,
    #[doc = "LPUART3 Receive Line Idle."]
    Val57 = 0x39,
    #[doc = "LPUART4 Received Data Word."]
    Val58 = 0x3a,
    #[doc = "LPUART4 Transmitted Data Word."]
    Val59 = 0x3b,
    #[doc = "LPUART4 Receive Line Idle."]
    Val60 = 0x3c,
    #[doc = "AOI1_OUT0 input is selected."]
    Val61 = 0x3d,
    #[doc = "AOI1_OUT1 input is selected."]
    Val62 = 0x3e,
    #[doc = "AOI1_OUT2 input is selected."]
    Val63 = 0x3f,
    #[doc = "AOI1_OUT3 input is selected."]
    Val64 = 0x40,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val65 = 0x41,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val66 = 0x42,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val67 = 0x43,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val68 = 0x44,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val69 = 0x45,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val70 = 0x46,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val71 = 0x47,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val72 = 0x48,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val73 = 0x49,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val74 = 0x4a,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val75 = 0x4b,
    _RESERVED_4c = 0x4c,
    #[doc = "LPI2C2 Master End of Packet."]
    Val77 = 0x4d,
    #[doc = "LPI2C2 Slave End of Packet."]
    Val78 = 0x4e,
    #[doc = "LPI2C3 Master End of Packet."]
    Val79 = 0x4f,
    #[doc = "LPI2C3 Slave End of Packet."]
    Val80 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl FlexioTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioTrigInp {
    #[inline(always)]
    fn from(val: u8) -> FlexioTrigInp {
        FlexioTrigInp::from_bits(val)
    }
}
impl From<FlexioTrigInp> for u8 {
    #[inline(always)]
    fn from(val: FlexioTrigInp) -> u8 {
        FlexioTrigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqmeasRefInp(u8);
impl FreqmeasRefInp {
    #[doc = "clk_in input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "FRO_OSC_12M input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "fro_hf_div input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "clk_16k\\[1\\] input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "SLOW_CLK input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "FREQME_CLK_IN0 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "FREQME_CLK_IN1 input is selected input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "AOI0_OUT1."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "PWM0_SM0_MUX_TRIG0."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "PWM0_SM0_MUX_TRIG1."]
    pub const Val12: Self = Self(0x0c);
    #[doc = "PWM0_SM1_MUX_TRIG0."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "PWM0_SM1_MUX_TRIG1."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "PWM0_SM2_MUX_TRIG0."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "PWM0_SM2_MUX_TRIG1."]
    pub const Val16: Self = Self(0x10);
    #[doc = "AOI1_OUT0 input is selected."]
    pub const Val32: Self = Self(0x20);
    #[doc = "AOI1_OUT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const Val39: Self = Self(0x27);
}
impl FreqmeasRefInp {
    pub const fn from_bits(val: u8) -> FreqmeasRefInp {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FreqmeasRefInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0c => f.write_str("Val12"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x20 => f.write_str("Val32"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x27 => f.write_str("Val39"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasRefInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0c => defmt::write!(f, "Val12"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x20 => defmt::write!(f, "Val32"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x27 => defmt::write!(f, "Val39"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FreqmeasRefInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasRefInp {
        FreqmeasRefInp::from_bits(val)
    }
}
impl From<FreqmeasRefInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasRefInp) -> u8 {
        FreqmeasRefInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqmeasTarInp(u8);
impl FreqmeasTarInp {
    #[doc = "clk_in input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "FRO_OSC_12M input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "fro_hf_div input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "clk_16k\\[1\\] input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "SLOW_CLK input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "FREQME_CLK_IN0 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "FREQME_CLK_IN1 input is selected input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "AOI0_OUT1."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "PWM0_SM0_MUX_TRIG0."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "PWM0_SM0_MUX_TRIG1."]
    pub const Val12: Self = Self(0x0c);
    #[doc = "PWM0_SM1_MUX_TRIG0."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "PWM0_SM1_MUX_TRIG1."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "PWM0_SM2_MUX_TRIG0."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "PWM0_SM2_MUX_TRIG1."]
    pub const Val16: Self = Self(0x10);
    #[doc = "AOI1_OUT0 input is selected."]
    pub const Val32: Self = Self(0x20);
    #[doc = "AOI1_OUT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const Val39: Self = Self(0x27);
}
impl FreqmeasTarInp {
    pub const fn from_bits(val: u8) -> FreqmeasTarInp {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FreqmeasTarInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0c => f.write_str("Val12"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x20 => f.write_str("Val32"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x27 => f.write_str("Val39"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasTarInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0c => defmt::write!(f, "Val12"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x20 => defmt::write!(f, "Val32"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x27 => defmt::write!(f, "Val39"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FreqmeasTarInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasTarInp {
        FreqmeasTarInp::from_bits(val)
    }
}
impl From<FreqmeasTarInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasTarInp) -> u8 {
        FreqmeasTarInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c0TrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpi2c0TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c0TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c0TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c0TrigInp {
        Lpi2c0TrigInp::from_bits(val)
    }
}
impl From<Lpi2c0TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c0TrigInp) -> u8 {
        Lpi2c0TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1TrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpi2c1TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1TrigInp {
        Lpi2c1TrigInp::from_bits(val)
    }
}
impl From<Lpi2c1TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1TrigInp) -> u8 {
        Lpi2c1TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2TrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpi2c2TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2TrigInp {
        Lpi2c2TrigInp::from_bits(val)
    }
}
impl From<Lpi2c2TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2TrigInp) -> u8 {
        Lpi2c2TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi0TrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpspi0TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi0TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi0TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpspi0TrigInp {
        Lpspi0TrigInp::from_bits(val)
    }
}
impl From<Lpspi0TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpspi0TrigInp) -> u8 {
        Lpspi0TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1TrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpspi1TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1TrigInp {
        Lpspi1TrigInp::from_bits(val)
    }
}
impl From<Lpspi1TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1TrigInp) -> u8 {
        Lpspi1TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart0Inp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN8 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN9 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "WUU selected."]
    Val34 = 0x22,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT1 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT2 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT3 input is selected."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val43 = 0x2b,
    #[doc = "FlexIO CH0 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH1 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH2 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH3 input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpuart0Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart0Inp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart0Inp {
    #[inline(always)]
    fn from(val: u8) -> Lpuart0Inp {
        Lpuart0Inp::from_bits(val)
    }
}
impl From<Lpuart0Inp> for u8 {
    #[inline(always)]
    fn from(val: Lpuart0Inp) -> u8 {
        Lpuart0Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1Inp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN8 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN9 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "WUU selected."]
    Val34 = 0x22,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT1 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT2 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT3 input is selected."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val43 = 0x2b,
    #[doc = "FlexIO CH0 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH1 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH2 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH3 input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpuart1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1Inp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1Inp {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1Inp {
        Lpuart1Inp::from_bits(val)
    }
}
impl From<Lpuart1Inp> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1Inp) -> u8 {
        Lpuart1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2Inp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN8 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN9 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "WUU selected."]
    Val34 = 0x22,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT1 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT2 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT3 input is selected."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val43 = 0x2b,
    #[doc = "FlexIO CH0 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH1 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH2 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH3 input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpuart2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2Inp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2Inp {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2Inp {
        Lpuart2Inp::from_bits(val)
    }
}
impl From<Lpuart2Inp> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2Inp) -> u8 {
        Lpuart2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3Inp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN8 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN9 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "WUU selected."]
    Val34 = 0x22,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT1 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT2 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT3 input is selected."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val43 = 0x2b,
    #[doc = "FlexIO CH0 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH1 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH2 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH3 input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpuart3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3Inp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3Inp {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3Inp {
        Lpuart3Inp::from_bits(val)
    }
}
impl From<Lpuart3Inp> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3Inp) -> u8 {
        Lpuart3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4Inp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN8 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN9 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "WUU selected."]
    Val34 = 0x22,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT1 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT2 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT3 input is selected."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val43 = 0x2b,
    #[doc = "FlexIO CH0 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH1 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH2 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH3 input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Lpuart4Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4Inp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4Inp {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4Inp {
        Lpuart4Inp::from_bits(val)
    }
}
impl From<Lpuart4Inp> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4Inp) -> u8 {
        Lpuart4Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opamp0TrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "WUU input is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    Val34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    Val36 = 0x24,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val37 = 0x25,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val38 = 0x26,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val39 = 0x27,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH0 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH1 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH2 input is selected."]
    Val47 = 0x2f,
    #[doc = "FlexIO CH3 input is selected."]
    Val48 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Opamp0TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opamp0TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opamp0TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Opamp0TrigInp {
        Opamp0TrigInp::from_bits(val)
    }
}
impl From<Opamp0TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Opamp0TrigInp) -> u8 {
        Opamp0TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0ExtClkTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "clk_16k\\[1\\] input is selected."]
    Val1 = 0x01,
    #[doc = "clk_in input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "EXTTRIG_IN0 input is selected."]
    Val5 = 0x05,
    #[doc = "EXTTRIG_IN7 input is selected."]
    Val6 = 0x06,
    #[doc = "AOI1_OUT0 input is selected."]
    Val7 = 0x07,
    #[doc = "AOI1_OUT1 input is selected."]
    Val8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pwm0ExtClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0ExtClkTrigin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm0ExtClkTrigin {
    #[inline(always)]
    fn from(val: u8) -> Pwm0ExtClkTrigin {
        Pwm0ExtClkTrigin::from_bits(val)
    }
}
impl From<Pwm0ExtClkTrigin> for u8 {
    #[inline(always)]
    fn from(val: Pwm0ExtClkTrigin) -> u8 {
        Pwm0ExtClkTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1ExtClkTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "clk_16k\\[1\\] input is selected."]
    Val1 = 0x01,
    #[doc = "clk_in input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "EXTTRIG_IN0 input is selected."]
    Val5 = 0x05,
    #[doc = "EXTTRIG_IN7 input is selected."]
    Val6 = 0x06,
    #[doc = "AOI1_OUT0 input is selected."]
    Val7 = 0x07,
    #[doc = "AOI1_OUT1 input is selected."]
    Val8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pwm1ExtClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1ExtClkTrigin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1ExtClkTrigin {
    #[inline(always)]
    fn from(val: u8) -> Pwm1ExtClkTrigin {
        Pwm1ExtClkTrigin::from_bits(val)
    }
}
impl From<Pwm1ExtClkTrigin> for u8 {
    #[inline(always)]
    fn from(val: Pwm1ExtClkTrigin) -> u8 {
        Pwm1ExtClkTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0HomeInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0HomeInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0HomeInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0HomeInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0HomeInp {
        Qdc0HomeInp::from_bits(val)
    }
}
impl From<Qdc0HomeInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0HomeInp) -> u8 {
        Qdc0HomeInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0Icap1Inp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0Icap1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0Icap1Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0Icap1Inp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0Icap1Inp {
        Qdc0Icap1Inp::from_bits(val)
    }
}
impl From<Qdc0Icap1Inp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0Icap1Inp) -> u8 {
        Qdc0Icap1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0Icap2Inp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0Icap2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0Icap2Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0Icap2Inp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0Icap2Inp {
        Qdc0Icap2Inp::from_bits(val)
    }
}
impl From<Qdc0Icap2Inp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0Icap2Inp) -> u8 {
        Qdc0Icap2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0Icap3Inp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0Icap3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0Icap3Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0Icap3Inp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0Icap3Inp {
        Qdc0Icap3Inp::from_bits(val)
    }
}
impl From<Qdc0Icap3Inp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0Icap3Inp) -> u8 {
        Qdc0Icap3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0IndexInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0IndexInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0IndexInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0IndexInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0IndexInp {
        Qdc0IndexInp::from_bits(val)
    }
}
impl From<Qdc0IndexInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0IndexInp) -> u8 {
        Qdc0IndexInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0PhaseaInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0PhaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0PhaseaInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0PhaseaInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0PhaseaInp {
        Qdc0PhaseaInp::from_bits(val)
    }
}
impl From<Qdc0PhaseaInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0PhaseaInp) -> u8 {
        Qdc0PhaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0PhasebInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0PhasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0PhasebInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0PhasebInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0PhasebInp {
        Qdc0PhasebInp::from_bits(val)
    }
}
impl From<Qdc0PhasebInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0PhasebInp) -> u8 {
        Qdc0PhasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0TrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0TrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0TrigInp {
        Qdc0TrigInp::from_bits(val)
    }
}
impl From<Qdc0TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0TrigInp) -> u8 {
        Qdc0TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1HomeInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1HomeInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1HomeInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1HomeInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1HomeInp {
        Qdc1HomeInp::from_bits(val)
    }
}
impl From<Qdc1HomeInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1HomeInp) -> u8 {
        Qdc1HomeInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1Icap1Inp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1Icap1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1Icap1Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1Icap1Inp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1Icap1Inp {
        Qdc1Icap1Inp::from_bits(val)
    }
}
impl From<Qdc1Icap1Inp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1Icap1Inp) -> u8 {
        Qdc1Icap1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1Icap2Inp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1Icap2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1Icap2Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1Icap2Inp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1Icap2Inp {
        Qdc1Icap2Inp::from_bits(val)
    }
}
impl From<Qdc1Icap2Inp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1Icap2Inp) -> u8 {
        Qdc1Icap2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1Icap3Inp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1Icap3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1Icap3Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1Icap3Inp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1Icap3Inp {
        Qdc1Icap3Inp::from_bits(val)
    }
}
impl From<Qdc1Icap3Inp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1Icap3Inp) -> u8 {
        Qdc1Icap3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1IndexInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1IndexInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1IndexInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1IndexInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1IndexInp {
        Qdc1IndexInp::from_bits(val)
    }
}
impl From<Qdc1IndexInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1IndexInp) -> u8 {
        Qdc1IndexInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1PhaseaInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1PhaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1PhaseaInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1PhaseaInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1PhaseaInp {
        Qdc1PhaseaInp::from_bits(val)
    }
}
impl From<Qdc1PhaseaInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1PhaseaInp) -> u8 {
        Qdc1PhaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1PhasebInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1PhasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1PhasebInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1PhasebInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1PhasebInp {
        Qdc1PhasebInp::from_bits(val)
    }
}
impl From<Qdc1PhasebInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1PhasebInp) -> u8 {
        Qdc1PhasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1TrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
    Val52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1TrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1TrigInp {
        Qdc1TrigInp::from_bits(val)
    }
}
impl From<Qdc1TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1TrigInp) -> u8 {
        Qdc1TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer0trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer0trigInp {
        Timer0trigInp::from_bits(val)
    }
}
impl From<Timer0trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer0trigInp) -> u8 {
        Timer0trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer1trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer1trigInp {
        Timer1trigInp::from_bits(val)
    }
}
impl From<Timer1trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer1trigInp) -> u8 {
        Timer1trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer2trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer2trigInp {
        Timer2trigInp::from_bits(val)
    }
}
impl From<Timer2trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer2trigInp) -> u8 {
        Timer2trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer3trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer3trigInp {
        Timer3trigInp::from_bits(val)
    }
}
impl From<Timer3trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer3trigInp) -> u8 {
        Timer3trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_IPN0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_IPN1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_IPN2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_IPN3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_IPN4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_IPN5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_IPN6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_IPN7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_IPN8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_IPN9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_IPN10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_IPN11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_IPN12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_IPN13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_IPN14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_IPN15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_IPN16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_IPN17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_IPN18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_IPN19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer4trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer4trigInp {
        Timer4trigInp::from_bits(val)
    }
}
impl From<Timer4trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer4trigInp) -> u8 {
        Timer4trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbfsTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "LPUART0 lpuart_trg_txdata input is selected."]
    Val1 = 0x01,
    #[doc = "LPUART1 lpuart_trg_txdata input is selected."]
    Val2 = 0x02,
    #[doc = "LPUART2 lpuart_trg_txdata input is selected."]
    Val3 = 0x03,
    #[doc = "LPUART3 lpuart_trg_txdata input is selected."]
    Val4 = 0x04,
    #[doc = "LPUART4 lpuart_trg_txda input is selected."]
    Val5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl UsbfsTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbfsTrigInp {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbfsTrigInp {
    #[inline(always)]
    fn from(val: u8) -> UsbfsTrigInp {
        UsbfsTrigInp::from_bits(val)
    }
}
impl From<UsbfsTrigInp> for u8 {
    #[inline(always)]
    fn from(val: UsbfsTrigInp) -> u8 {
        UsbfsTrigInp::to_bits(val)
    }
}
