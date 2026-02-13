#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Adc0TrigTrigin(u8);
impl Adc0TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    pub const VAL0: Self = Self(0x0);
    #[doc = "PINT PIN_INT1 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "SCT0 SCT_OUT9 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CTIMER0_MAT3 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CTIMER1_MAT3 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CTIMER2_MAT3 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CTIMER3_MAT3 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CTIMER4_MAT3 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "DCDC_Burst_Done_Trig input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "CMP0_OUT input is selected"]
    pub const VAL21: Self = Self(0x15);
    #[doc = "CMP1_OUT input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "CMP2_OUT input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    pub const VAL31: Self = Self(0x1f);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    pub const VAL32: Self = Self(0x20);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    pub const VAL39: Self = Self(0x27);
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    pub const VAL40: Self = Self(0x28);
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    pub const VAL41: Self = Self(0x29);
    #[doc = "EVTG_OUT0A input is selected"]
    pub const VAL42: Self = Self(0x2a);
    #[doc = "EVTG_OUT0B input is selected"]
    pub const VAL43: Self = Self(0x2b);
    #[doc = "EVTG_OUT1A input is selected"]
    pub const VAL44: Self = Self(0x2c);
    #[doc = "EVTG_OUT1B input is selected"]
    pub const VAL45: Self = Self(0x2d);
    #[doc = "EVTG_OUT2A input is selected"]
    pub const VAL46: Self = Self(0x2e);
    #[doc = "EVTG_OUT2B input is selected"]
    pub const VAL47: Self = Self(0x2f);
    #[doc = "EVTG_OUT3A input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "EVTG_OUT3B input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPTMR0 input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPTMR1 input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "FlexIO CH0 input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "FlexIO CH1 input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "FlexIO CH2 input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "FlexIO CH3 input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "WUU input is selected"]
    pub const VAL65: Self = Self(0x41);
}
impl Adc0TrigTrigin {
    pub const fn from_bits(val: u8) -> Adc0TrigTrigin {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Adc0TrigTrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("VAL0"),
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x15 => f.write_str("VAL21"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x1f => f.write_str("VAL31"),
            0x20 => f.write_str("VAL32"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x27 => f.write_str("VAL39"),
            0x28 => f.write_str("VAL40"),
            0x29 => f.write_str("VAL41"),
            0x2a => f.write_str("VAL42"),
            0x2b => f.write_str("VAL43"),
            0x2c => f.write_str("VAL44"),
            0x2d => f.write_str("VAL45"),
            0x2e => f.write_str("VAL46"),
            0x2f => f.write_str("VAL47"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0TrigTrigin {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "VAL0"),
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x15 => defmt::write!(f, "VAL21"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x1f => defmt::write!(f, "VAL31"),
            0x20 => defmt::write!(f, "VAL32"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x27 => defmt::write!(f, "VAL39"),
            0x28 => defmt::write!(f, "VAL40"),
            0x29 => defmt::write!(f, "VAL41"),
            0x2a => defmt::write!(f, "VAL42"),
            0x2b => defmt::write!(f, "VAL43"),
            0x2c => defmt::write!(f, "VAL44"),
            0x2d => defmt::write!(f, "VAL45"),
            0x2e => defmt::write!(f, "VAL46"),
            0x2f => defmt::write!(f, "VAL47"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Adc1TrigTrigin(u8);
impl Adc1TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    pub const VAL0: Self = Self(0x0);
    #[doc = "PINT PIN_INT2 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "SCT0 SCT_OUT3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CTIMER0_MAT3 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CTIMER1_MAT3 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CTIMER2_MAT3 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CTIMER3_MAT2 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CTIMER4_MAT1 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "DCDC_Burst_Done_Trig input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "CMP0_OUT input is selected"]
    pub const VAL21: Self = Self(0x15);
    #[doc = "CMP1_OUT input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "CMP2_OUT input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    pub const VAL31: Self = Self(0x1f);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    pub const VAL32: Self = Self(0x20);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    pub const VAL39: Self = Self(0x27);
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    pub const VAL40: Self = Self(0x28);
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    pub const VAL41: Self = Self(0x29);
    #[doc = "EVTG_OUT0A input is selected"]
    pub const VAL42: Self = Self(0x2a);
    #[doc = "EVTG_OUT0B input is selected"]
    pub const VAL43: Self = Self(0x2b);
    #[doc = "EVTG_OUT1A input is selected"]
    pub const VAL44: Self = Self(0x2c);
    #[doc = "EVTG_OUT1B input is selected"]
    pub const VAL45: Self = Self(0x2d);
    #[doc = "EVTG_OUT2A input is selected"]
    pub const VAL46: Self = Self(0x2e);
    #[doc = "EVTG_OUT2B input is selected"]
    pub const VAL47: Self = Self(0x2f);
    #[doc = "EVTG_OUT3A input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "EVTG_OUT3B input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPTMR0 input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPTMR1 input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "FlexIO CH0 input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "FlexIO CH1 input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "FlexIO CH2 input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "FlexIO CH3 input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "WUU input is selected"]
    pub const VAL65: Self = Self(0x41);
}
impl Adc1TrigTrigin {
    pub const fn from_bits(val: u8) -> Adc1TrigTrigin {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Adc1TrigTrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("VAL0"),
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x15 => f.write_str("VAL21"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x1f => f.write_str("VAL31"),
            0x20 => f.write_str("VAL32"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x27 => f.write_str("VAL39"),
            0x28 => f.write_str("VAL40"),
            0x29 => f.write_str("VAL41"),
            0x2a => f.write_str("VAL42"),
            0x2b => f.write_str("VAL43"),
            0x2c => f.write_str("VAL44"),
            0x2d => f.write_str("VAL45"),
            0x2e => f.write_str("VAL46"),
            0x2f => f.write_str("VAL47"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1TrigTrigin {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "VAL0"),
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x15 => defmt::write!(f, "VAL21"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x1f => defmt::write!(f, "VAL31"),
            0x20 => defmt::write!(f, "VAL32"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x27 => defmt::write!(f, "VAL39"),
            0x28 => defmt::write!(f, "VAL40"),
            0x29 => defmt::write!(f, "VAL41"),
            0x2a => defmt::write!(f, "VAL42"),
            0x2b => defmt::write!(f, "VAL43"),
            0x2c => defmt::write!(f, "VAL44"),
            0x2d => defmt::write!(f, "VAL45"),
            0x2e => defmt::write!(f, "VAL46"),
            0x2f => defmt::write!(f, "VAL47"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
pub enum Cmp0TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT6 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT6 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER0_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL18 = 0x12,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL19 = 0x13,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL20 = 0x14,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL21 = 0x15,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL22 = 0x16,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL24 = 0x18,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL25 = 0x19,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL26 = 0x1a,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL27 = 0x1b,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL28 = 0x1c,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL29 = 0x1d,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL30 = 0x1e,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL31 = 0x1f,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL32 = 0x20,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL34 = 0x22,
    #[doc = "LPTMR0 input is selected"]
    VAL35 = 0x23,
    #[doc = "LPTMR1 input is selected"]
    VAL36 = 0x24,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT0 SCT_OUT7 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER3_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL18 = 0x12,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL19 = 0x13,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL20 = 0x14,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL21 = 0x15,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL22 = 0x16,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL24 = 0x18,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL25 = 0x19,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL26 = 0x1a,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL27 = 0x1b,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL28 = 0x1c,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL29 = 0x1d,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL30 = 0x1e,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL31 = 0x1f,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL32 = 0x20,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL34 = 0x22,
    #[doc = "LPTMR0 input is selected"]
    VAL35 = 0x23,
    #[doc = "LPTMR1 input is selected"]
    VAL36 = 0x24,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
pub enum Cmp2TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT0 SCT_OUT8 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER3_MAT2 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT2 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL18 = 0x12,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL19 = 0x13,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL20 = 0x14,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL21 = 0x15,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL22 = 0x16,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL24 = 0x18,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL25 = 0x19,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL26 = 0x1a,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL27 = 0x1b,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL28 = 0x1c,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL29 = 0x1d,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL30 = 0x1e,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL31 = 0x1f,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL32 = 0x20,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL34 = 0x22,
    #[doc = "LPTMR0 input is selected"]
    VAL35 = 0x23,
    #[doc = "LPTMR1 input is selected"]
    VAL36 = 0x24,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
impl Cmp2TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Cmp2TrigTrigin {
        Cmp2TrigTrigin::from_bits(val)
    }
}
impl From<Cmp2TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Cmp2TrigTrigin) -> u8 {
        Cmp2TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0cap0Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer0cap0Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0cap0Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0cap0Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0cap0Inp {
        Ctimer0cap0Inp::from_bits(val)
    }
}
impl From<Ctimer0cap0Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0cap0Inp) -> u8 {
        Ctimer0cap0Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0cap1Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer0cap1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0cap1Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0cap1Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0cap1Inp {
        Ctimer0cap1Inp::from_bits(val)
    }
}
impl From<Ctimer0cap1Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0cap1Inp) -> u8 {
        Ctimer0cap1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0cap2Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer0cap2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0cap2Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0cap2Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0cap2Inp {
        Ctimer0cap2Inp::from_bits(val)
    }
}
impl From<Ctimer0cap2Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0cap2Inp) -> u8 {
        Ctimer0cap2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0cap3Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer0cap3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0cap3Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0cap3Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0cap3Inp {
        Ctimer0cap3Inp::from_bits(val)
    }
}
impl From<Ctimer0cap3Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0cap3Inp) -> u8 {
        Ctimer0cap3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1cap0Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer1cap0Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1cap0Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1cap0Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1cap0Inp {
        Ctimer1cap0Inp::from_bits(val)
    }
}
impl From<Ctimer1cap0Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1cap0Inp) -> u8 {
        Ctimer1cap0Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1cap1Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer1cap1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1cap1Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1cap1Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1cap1Inp {
        Ctimer1cap1Inp::from_bits(val)
    }
}
impl From<Ctimer1cap1Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1cap1Inp) -> u8 {
        Ctimer1cap1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1cap2Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer1cap2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1cap2Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1cap2Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1cap2Inp {
        Ctimer1cap2Inp::from_bits(val)
    }
}
impl From<Ctimer1cap2Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1cap2Inp) -> u8 {
        Ctimer1cap2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1cap3Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer1cap3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1cap3Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1cap3Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1cap3Inp {
        Ctimer1cap3Inp::from_bits(val)
    }
}
impl From<Ctimer1cap3Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1cap3Inp) -> u8 {
        Ctimer1cap3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2cap0Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer2cap0Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2cap0Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2cap0Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2cap0Inp {
        Ctimer2cap0Inp::from_bits(val)
    }
}
impl From<Ctimer2cap0Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2cap0Inp) -> u8 {
        Ctimer2cap0Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2cap1Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer2cap1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2cap1Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2cap1Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2cap1Inp {
        Ctimer2cap1Inp::from_bits(val)
    }
}
impl From<Ctimer2cap1Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2cap1Inp) -> u8 {
        Ctimer2cap1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2cap2Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer2cap2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2cap2Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2cap2Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2cap2Inp {
        Ctimer2cap2Inp::from_bits(val)
    }
}
impl From<Ctimer2cap2Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2cap2Inp) -> u8 {
        Ctimer2cap2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2cap3Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer2cap3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2cap3Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2cap3Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2cap3Inp {
        Ctimer2cap3Inp::from_bits(val)
    }
}
impl From<Ctimer2cap3Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2cap3Inp) -> u8 {
        Ctimer2cap3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3cap0Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer3cap0Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3cap0Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3cap0Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3cap0Inp {
        Ctimer3cap0Inp::from_bits(val)
    }
}
impl From<Ctimer3cap0Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3cap0Inp) -> u8 {
        Ctimer3cap0Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3cap1Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer3cap1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3cap1Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3cap1Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3cap1Inp {
        Ctimer3cap1Inp::from_bits(val)
    }
}
impl From<Ctimer3cap1Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3cap1Inp) -> u8 {
        Ctimer3cap1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3cap2Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer3cap2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3cap2Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3cap2Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3cap2Inp {
        Ctimer3cap2Inp::from_bits(val)
    }
}
impl From<Ctimer3cap2Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3cap2Inp) -> u8 {
        Ctimer3cap2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3cap3Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer3cap3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3cap3Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3cap3Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3cap3Inp {
        Ctimer3cap3Inp::from_bits(val)
    }
}
impl From<Ctimer3cap3Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3cap3Inp) -> u8 {
        Ctimer3cap3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4cap0Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer4cap0Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4cap0Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4cap0Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4cap0Inp {
        Ctimer4cap0Inp::from_bits(val)
    }
}
impl From<Ctimer4cap0Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4cap0Inp) -> u8 {
        Ctimer4cap0Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4cap1Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer4cap1Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4cap1Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4cap1Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4cap1Inp {
        Ctimer4cap1Inp::from_bits(val)
    }
}
impl From<Ctimer4cap1Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4cap1Inp) -> u8 {
        Ctimer4cap1Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4cap2Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer4cap2Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4cap2Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4cap2Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4cap2Inp {
        Ctimer4cap2Inp::from_bits(val)
    }
}
impl From<Ctimer4cap2Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4cap2Inp) -> u8 {
        Ctimer4cap2Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4cap3Inp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer4cap3Inp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4cap3Inp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4cap3Inp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4cap3Inp {
        Ctimer4cap3Inp::from_bits(val)
    }
}
impl From<Ctimer4cap3Inp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4cap3Inp) -> u8 {
        Ctimer4cap3Inp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT3 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT0 SCT_OUT0 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "LPTMR0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "LPTMR1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
pub enum Dac1TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT0 SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT1 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "LPTMR0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "LPTMR1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
impl Dac1TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Dac1TrigTrigin {
        Dac1TrigTrigin::from_bits(val)
    }
}
impl From<Dac1TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Dac1TrigTrigin) -> u8 {
        Dac1TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT0 SCT_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT2 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT2 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "LPTMR0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "LPTMR1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
impl Dac2TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Dac2TrigTrigin {
        Dac2TrigTrigin::from_bits(val)
    }
}
impl From<Dac2TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Dac2TrigTrigin) -> u8 {
        Dac2TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgTrigInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER2_MAT2 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER3_MAT2 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTIMER4_MAT2 input is selected"]
    VAL11 = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_IRQ input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC1_IRQ input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL21 = 0x15,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL22 = 0x16,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL23 = 0x17,
    #[doc = "CMP0_OUT input is selected"]
    VAL24 = 0x18,
    #[doc = "CMP1_OUT input is selected"]
    VAL25 = 0x19,
    #[doc = "CMP2_OUT input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL37 = 0x25,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL38 = 0x26,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL39 = 0x27,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL40 = 0x28,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL41 = 0x29,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL42 = 0x2a,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL43 = 0x2b,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN0 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN1 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN2 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN3 input is selected"]
    VAL48 = 0x30,
    #[doc = "LPTMR0 input is selected"]
    VAL49 = 0x31,
    #[doc = "LPTMR1 input is selected"]
    VAL50 = 0x32,
    #[doc = "SINC Filter CH0 Break input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH1 Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH2 Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH3 Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH4 Break input is selected"]
    VAL55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl EvtgTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgTrigInp {
    #[inline(always)]
    fn from(val: u8) -> EvtgTrigInp {
        EvtgTrigInp::from_bits(val)
    }
}
impl From<EvtgTrigInp> for u8 {
    #[inline(always)]
    fn from(val: EvtgTrigInp) -> u8 {
        EvtgTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtTrigInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected"]
    VAL1 = 0x01,
    #[doc = "ADC0_IRQ input is selected"]
    VAL2 = 0x02,
    #[doc = "ADC1_IRQ input is selected"]
    VAL3 = 0x03,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL4 = 0x04,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL5 = 0x05,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL6 = 0x06,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL7 = 0x07,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL8 = 0x08,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL9 = 0x09,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL11 = 0x0b,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL12 = 0x0c,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL13 = 0x0d,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL14 = 0x0e,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL15 = 0x0f,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL16 = 0x10,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "LPTMR0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "LPTMR1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "SCT Out0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "SCT Out1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "SCT Out2 input is selected"]
    VAL30 = 0x1e,
    #[doc = "SCT Out3 input is selected"]
    VAL31 = 0x1f,
    #[doc = "SCT Out4 input is selected"]
    VAL32 = 0x20,
    #[doc = "SCT Out5 input is selected"]
    VAL33 = 0x21,
    #[doc = "LP_FLEXCOMM0 trigger output 3 input is selected"]
    VAL34 = 0x22,
    #[doc = "LP_FLEXCOMM1 trigger output 3 input is selected"]
    VAL35 = 0x23,
    #[doc = "LP_FLEXCOMM2 trigger output 3 input is selected"]
    VAL36 = 0x24,
    #[doc = "LP_FLEXCOMM3 trigger output 3 input is selected"]
    VAL37 = 0x25,
    #[doc = "LP_FLEXCOMM4 trigger output 3 input is selected"]
    VAL38 = 0x26,
    #[doc = "LP_FLEXCOMM5 trigger output 3 input is selected"]
    VAL39 = 0x27,
    #[doc = "LP_FLEXCOMM6 trigger output 3 input is selected"]
    VAL40 = 0x28,
    #[doc = "LP_FLEXCOMM7 trigger output 3 input is selected"]
    VAL41 = 0x29,
    #[doc = "LP_FLEXCOMM8 trigger output 3 input is selected"]
    VAL42 = 0x2a,
    #[doc = "LP_FLEXCOMM9 trigger output 3 input is selected"]
    VAL43 = 0x2b,
    #[doc = "CMP0_OUT input is selected"]
    VAL44 = 0x2c,
    #[doc = "CMP1_OUT input is selected"]
    VAL45 = 0x2d,
    #[doc = "CMP2_OUT input is selected"]
    VAL46 = 0x2e,
    #[doc = "ENET_PPS_OUT_0 input is selected"]
    VAL47 = 0x2f,
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
impl ExtTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtTrigInp {
    #[inline(always)]
    fn from(val: u8) -> ExtTrigInp {
        ExtTrigInp::from_bits(val)
    }
}
impl From<ExtTrigInp> for u8 {
    #[inline(always)]
    fn from(val: ExtTrigInp) -> u8 {
        ExtTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0ExtforceTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0ExtforceTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0ExtforceTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0ExtforceTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0ExtforceTrigin {
        FlexPwm0ExtforceTrigin::from_bits(val)
    }
}
impl From<FlexPwm0ExtforceTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0ExtforceTrigin) -> u8 {
        FlexPwm0ExtforceTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0FaultTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
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
pub enum FlexPwm0SmExtaTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0SmExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0SmExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0SmExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0SmExtaTrigin {
        FlexPwm0SmExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm0SmExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0SmExtaTrigin) -> u8 {
        FlexPwm0SmExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0SmExtsyncTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0SmExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0SmExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0SmExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0SmExtsyncTrigin {
        FlexPwm0SmExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm0SmExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0SmExtsyncTrigin) -> u8 {
        FlexPwm0SmExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1ExtforceTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1ExtforceTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1ExtforceTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1ExtforceTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1ExtforceTrigin {
        FlexPwm1ExtforceTrigin::from_bits(val)
    }
}
impl From<FlexPwm1ExtforceTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1ExtforceTrigin) -> u8 {
        FlexPwm1ExtforceTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1FaultTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
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
pub enum FlexPwm1SmExtaTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1SmExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1SmExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1SmExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1SmExtaTrigin {
        FlexPwm1SmExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm1SmExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1SmExtaTrigin) -> u8 {
        FlexPwm1SmExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1SmExtsyncTrigin {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected"]
    VAL52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected"]
    VAL53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected"]
    VAL54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected"]
    VAL55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1SmExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1SmExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1SmExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1SmExtsyncTrigin {
        FlexPwm1SmExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm1SmExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1SmExtsyncTrigin) -> u8 {
        FlexPwm1SmExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm0TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT6 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT6 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT7 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm0TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm0TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm0TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm0TrigInp {
        Flexcomm0TrigInp::from_bits(val)
    }
}
impl From<Flexcomm0TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm0TrigInp) -> u8 {
        Flexcomm0TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm1TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT6 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT6 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT7 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm1TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm1TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm1TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm1TrigInp {
        Flexcomm1TrigInp::from_bits(val)
    }
}
impl From<Flexcomm1TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm1TrigInp) -> u8 {
        Flexcomm1TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm2TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT6 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT8 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT9 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT1 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm2TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm2TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm2TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm2TrigInp {
        Flexcomm2TrigInp::from_bits(val)
    }
}
impl From<Flexcomm2TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm2TrigInp) -> u8 {
        Flexcomm2TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm3TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT8 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT9 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT1 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm3TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm3TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm3TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm3TrigInp {
        Flexcomm3TrigInp::from_bits(val)
    }
}
impl From<Flexcomm3TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm3TrigInp) -> u8 {
        Flexcomm3TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm4TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT0 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT2 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT2 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT2 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT2 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm4TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm4TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm4TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm4TrigInp {
        Flexcomm4TrigInp::from_bits(val)
    }
}
impl From<Flexcomm4TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm4TrigInp) -> u8 {
        Flexcomm4TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm5TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT0 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT2 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT2 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT2 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT2 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm5TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm5TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm5TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm5TrigInp {
        Flexcomm5TrigInp::from_bits(val)
    }
}
impl From<Flexcomm5TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm5TrigInp) -> u8 {
        Flexcomm5TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm6TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT0 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT4 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT3 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT3 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm6TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm6TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm6TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm6TrigInp {
        Flexcomm6TrigInp::from_bits(val)
    }
}
impl From<Flexcomm6TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm6TrigInp) -> u8 {
        Flexcomm6TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm7TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT0 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT4 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT3 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT3 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm7TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm7TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm7TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm7TrigInp {
        Flexcomm7TrigInp::from_bits(val)
    }
}
impl From<Flexcomm7TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm7TrigInp) -> u8 {
        Flexcomm7TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm8TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT0 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT4 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT3 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT3 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm8TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm8TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm8TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm8TrigInp {
        Flexcomm8TrigInp::from_bits(val)
    }
}
impl From<Flexcomm8TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm8TrigInp) -> u8 {
        Flexcomm8TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm9TrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT0 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT4 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPTMR1 input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "CMP0_OUT input is selected"]
    VAL15 = 0x0f,
    #[doc = "CMP1_OUT input is selected"]
    VAL16 = 0x10,
    #[doc = "CMP2_OUT input is selected"]
    VAL17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected"]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected"]
    VAL32 = 0x20,
    #[doc = "FlexIO CH4 input is selected"]
    VAL33 = 0x21,
    #[doc = "FlexIO CH5 input is selected"]
    VAL34 = 0x22,
    #[doc = "FlexIO CH6 input is selected"]
    VAL35 = 0x23,
    #[doc = "FlexIO CH7 input is selected"]
    VAL36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL41 = 0x29,
    #[doc = "WUU input is selected"]
    VAL42 = 0x2a,
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
impl Flexcomm9TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm9TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm9TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm9TrigInp {
        Flexcomm9TrigInp::from_bits(val)
    }
}
impl From<Flexcomm9TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm9TrigInp) -> u8 {
        Flexcomm9TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioTrigInp {
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT6 input is selected"]
    VAL2 = 0x02,
    #[doc = "PINT PIN_INT7 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT5 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT6 input is selected"]
    VAL5 = 0x05,
    #[doc = "SCT_OUT7 input is selected"]
    VAL6 = 0x06,
    #[doc = "SCT_OUT8 input is selected"]
    VAL7 = 0x07,
    #[doc = "SCT_OUT9 input is selected"]
    VAL8 = 0x08,
    #[doc = "T0_MAT1 input is selected"]
    VAL9 = 0x09,
    #[doc = "T1_MAT1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "T2_MAT1 input is selected"]
    VAL11 = 0x0b,
    #[doc = "T3_MAT1 input is selected"]
    VAL12 = 0x0c,
    #[doc = "T4_MAT1 input is selected"]
    VAL13 = 0x0d,
    #[doc = "LPTMR0 input is selected"]
    VAL14 = 0x0e,
    #[doc = "LPTMR1 input is selected"]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL21 = 0x15,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL22 = 0x16,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL23 = 0x17,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL25 = 0x19,
    #[doc = "CMP0_OUT input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP1_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP2_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL37 = 0x25,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL38 = 0x26,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL39 = 0x27,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL40 = 0x28,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL41 = 0x29,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL42 = 0x2a,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL43 = 0x2b,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL47 = 0x2f,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL48 = 0x30,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL49 = 0x31,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL50 = 0x32,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL51 = 0x33,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL52 = 0x34,
    #[doc = "TRIG_IN0 input is selected"]
    VAL53 = 0x35,
    #[doc = "TRIG_IN1 input is selected"]
    VAL54 = 0x36,
    #[doc = "TRIG_IN2 input is selected"]
    VAL55 = 0x37,
    #[doc = "TRIG_IN3 input is selected"]
    VAL56 = 0x38,
    #[doc = "TRIG_IN4 input is selected"]
    VAL57 = 0x39,
    #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
    VAL58 = 0x3a,
    #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
    VAL59 = 0x3b,
    #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
    VAL60 = 0x3c,
    #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
    VAL61 = 0x3d,
    #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
    VAL62 = 0x3e,
    #[doc = "LP_FLEXCOMM0 trig 0 (lpuart_trg_txword) input is selected"]
    VAL63 = 0x3f,
    #[doc = "LP_FLEXCOMM0 trig 1 (lpuart_trg_rxword) input is selected"]
    VAL64 = 0x40,
    #[doc = "LP_FLEXCOMM0 trig 2 (lpuart_trg_rxidle) input is selected"]
    VAL65 = 0x41,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL66 = 0x42,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL67 = 0x43,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL68 = 0x44,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL69 = 0x45,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL70 = 0x46,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL71 = 0x47,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL72 = 0x48,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL73 = 0x49,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL74 = 0x4a,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL75 = 0x4b,
    #[doc = "WUU input is selected"]
    VAL76 = 0x4c,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasRefInp {
    #[doc = "clk_in (output of clk_in or XTAL mux in Clockgen) input is selected"]
    VAL0 = 0x0,
    #[doc = "FRO_12M input is selected"]
    VAL1 = 0x01,
    #[doc = "FRO_144M input is selected"]
    VAL2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "OSC_32K input is selected"]
    VAL4 = 0x04,
    #[doc = "CPU/system_clk input is selected"]
    VAL5 = 0x05,
    #[doc = "FREQME_CLK_IN0 input is selected"]
    VAL6 = 0x06,
    #[doc = "FREQME_CLK_IN1 input is selected"]
    VAL7 = 0x07,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL8 = 0x08,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
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
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
impl FreqmeasRefInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasRefInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasTarInp {
    #[doc = "clk_in (output of clk_in or XTAL mux in Clockgen) input is selected"]
    VAL0 = 0x0,
    #[doc = "FRO_12M input is selected"]
    VAL1 = 0x01,
    #[doc = "FRO_144M input is selected"]
    VAL2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "OSC_32K input is selected"]
    VAL4 = 0x04,
    #[doc = "CPU/system_clk input is selected"]
    VAL5 = 0x05,
    #[doc = "FREQME_CLK_IN0 input is selected"]
    VAL6 = 0x06,
    #[doc = "FREQME_CLK_IN1 input is selected"]
    VAL7 = 0x07,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL8 = 0x08,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
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
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
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
impl FreqmeasTarInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasTarInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
pub enum OpampTrigInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected"]
    VAL1 = 0x01,
    #[doc = "PINT PIN_INT2 input is selected"]
    VAL2 = 0x02,
    #[doc = "PINT PIN_INT3 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT4 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT_OUT5 input is selected"]
    VAL5 = 0x05,
    #[doc = "SCT_OUT6 input is selected"]
    VAL6 = 0x06,
    #[doc = "SCT_OUT7 input is selected"]
    VAL7 = 0x07,
    #[doc = "SCT_OUT8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTIMER3_MAT3 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTIMER4_MAT3 input is selected"]
    VAL13 = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL21 = 0x15,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL22 = 0x16,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL37 = 0x25,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN0 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN1 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN2 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN3 input is selected"]
    VAL50 = 0x32,
    #[doc = "FlexIO CH4 input is selected"]
    VAL51 = 0x33,
    #[doc = "FlexIO CH5 input is selected"]
    VAL52 = 0x34,
    #[doc = "FlexIO CH6 input is selected"]
    VAL53 = 0x35,
    #[doc = "FlexIO CH7 input is selected"]
    VAL54 = 0x36,
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
impl OpampTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpampTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpampTrigInp {
    #[inline(always)]
    fn from(val: u8) -> OpampTrigInp {
        OpampTrigInp::from_bits(val)
    }
}
impl From<OpampTrigInp> for u8 {
    #[inline(always)]
    fn from(val: OpampTrigInp) -> u8 {
        OpampTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PintselInp {
    #[doc = "GPIO P0_0 input is selected"]
    VAL0 = 0x0,
    #[doc = "GPIO P0_1 input is selected"]
    VAL1 = 0x01,
    #[doc = "GPIO P0_2 input is selected"]
    VAL2 = 0x02,
    #[doc = "GPIO P0_3 input is selected"]
    VAL3 = 0x03,
    #[doc = "GPIO P0_4 input is selected"]
    VAL4 = 0x04,
    #[doc = "GPIO P0_5 input is selected"]
    VAL5 = 0x05,
    #[doc = "GPIO P0_6 input is selected"]
    VAL6 = 0x06,
    #[doc = "GPIO P0_7 input is selected"]
    VAL7 = 0x07,
    #[doc = "GPIO P0_8 input is selected"]
    VAL8 = 0x08,
    #[doc = "GPIO P0_9 input is selected"]
    VAL9 = 0x09,
    #[doc = "GPIO P0_10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "GPIO P0_11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "GPIO P0_12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "GPIO P0_13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "GPIO P0_14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "GPIO P0_15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "GPIO P0_16 input is selected"]
    VAL16 = 0x10,
    #[doc = "GPIO P0_17 input is selected"]
    VAL17 = 0x11,
    #[doc = "GPIO P0_18 input is selected"]
    VAL18 = 0x12,
    #[doc = "GPIO P0_19 input is selected"]
    VAL19 = 0x13,
    #[doc = "GPIO P0_20 input is selected"]
    VAL20 = 0x14,
    #[doc = "GPIO P0_21 input is selected"]
    VAL21 = 0x15,
    #[doc = "GPIO P0_22 input is selected"]
    VAL22 = 0x16,
    #[doc = "GPIO P0_23 input is selected"]
    VAL23 = 0x17,
    #[doc = "GPIO P0_24 input is selected"]
    VAL24 = 0x18,
    #[doc = "GPIO P0_25 input is selected"]
    VAL25 = 0x19,
    #[doc = "GPIO P0_26 input is selected"]
    VAL26 = 0x1a,
    #[doc = "GPIO P0_27 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO P0_28 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO P0_29 input is selected"]
    VAL29 = 0x1d,
    #[doc = "GPIO P0_30 input is selected"]
    VAL30 = 0x1e,
    #[doc = "GPIO P0_31 input is selected"]
    VAL31 = 0x1f,
    #[doc = "GPIO P1_0 input is selected"]
    VAL32 = 0x20,
    #[doc = "GPIO P1_1 input is selected"]
    VAL33 = 0x21,
    #[doc = "GPIO P1_2 input is selected"]
    VAL34 = 0x22,
    #[doc = "GPIO P1_3 input is selected"]
    VAL35 = 0x23,
    #[doc = "GPIO P1_4 input is selected"]
    VAL36 = 0x24,
    #[doc = "GPIO P1_5 input is selected"]
    VAL37 = 0x25,
    #[doc = "GPIO P1_6 input is selected"]
    VAL38 = 0x26,
    #[doc = "GPIO P1_7 input is selected"]
    VAL39 = 0x27,
    #[doc = "GPIO P1_8 input is selected"]
    VAL40 = 0x28,
    #[doc = "GPIO P1_9 input is selected"]
    VAL41 = 0x29,
    #[doc = "GPIO P1_10 input is selected"]
    VAL42 = 0x2a,
    #[doc = "GPIO P1_11 input is selected"]
    VAL43 = 0x2b,
    #[doc = "GPIO P1_12 input is selected"]
    VAL44 = 0x2c,
    #[doc = "GPIO P1_13 input is selected"]
    VAL45 = 0x2d,
    #[doc = "GPIO P1_14 input is selected"]
    VAL46 = 0x2e,
    #[doc = "GPIO P1_15 input is selected"]
    VAL47 = 0x2f,
    #[doc = "GPIO P1_16 input is selected"]
    VAL48 = 0x30,
    #[doc = "GPIO P1_17 input is selected"]
    VAL49 = 0x31,
    #[doc = "GPIO P1_18 input is selected"]
    VAL50 = 0x32,
    #[doc = "GPIO P1_19 input is selected"]
    VAL51 = 0x33,
    #[doc = "GPIO P1_20 input is selected"]
    VAL52 = 0x34,
    #[doc = "GPIO P1_21 input is selected"]
    VAL53 = 0x35,
    #[doc = "GPIO P1_22 input is selected"]
    VAL54 = 0x36,
    #[doc = "GPIO P1_23 input is selected"]
    VAL55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "GPIO P1_30 input is selected"]
    VAL62 = 0x3e,
    #[doc = "GPIO P1_31 input is selected"]
    VAL63 = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl PintselInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintselInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintselInp {
    #[inline(always)]
    fn from(val: u8) -> PintselInp {
        PintselInp::from_bits(val)
    }
}
impl From<PintselInp> for u8 {
    #[inline(always)]
    fn from(val: PintselInp) -> u8 {
        PintselInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0ExtClkTrigin {
    #[doc = "FRO16K input is selected"]
    VAL0 = 0x0,
    #[doc = "OSC_32k input is selected"]
    VAL1 = 0x01,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL2 = 0x02,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL3 = 0x03,
    #[doc = "TRIG_IN0 input is selected"]
    VAL4 = 0x04,
    #[doc = "TRIG_IN7 input is selected"]
    VAL5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pwm0ExtClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0ExtClkTrigin {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "FRO16K input is selected"]
    VAL0 = 0x0,
    #[doc = "OSC_32k input is selected"]
    VAL1 = 0x01,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL2 = 0x02,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL3 = 0x03,
    #[doc = "TRIG_IN0 input is selected"]
    VAL4 = 0x04,
    #[doc = "TRIG_IN7 input is selected"]
    VAL5 = 0x05,
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
pub enum QdcHomeInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT0 SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
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
impl QdcHomeInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcHomeInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcHomeInp {
    #[inline(always)]
    fn from(val: u8) -> QdcHomeInp {
        QdcHomeInp::from_bits(val)
    }
}
impl From<QdcHomeInp> for u8 {
    #[inline(always)]
    fn from(val: QdcHomeInp) -> u8 {
        QdcHomeInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcIndexInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
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
impl QdcIndexInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcIndexInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcIndexInp {
    #[inline(always)]
    fn from(val: u8) -> QdcIndexInp {
        QdcIndexInp::from_bits(val)
    }
}
impl From<QdcIndexInp> for u8 {
    #[inline(always)]
    fn from(val: QdcIndexInp) -> u8 {
        QdcIndexInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcPhaseaInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
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
impl QdcPhaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcPhaseaInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcPhaseaInp {
    #[inline(always)]
    fn from(val: u8) -> QdcPhaseaInp {
        QdcPhaseaInp::from_bits(val)
    }
}
impl From<QdcPhaseaInp> for u8 {
    #[inline(always)]
    fn from(val: QdcPhaseaInp) -> u8 {
        QdcPhaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcPhasebInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
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
impl QdcPhasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcPhasebInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcPhasebInp {
    #[inline(always)]
    fn from(val: u8) -> QdcPhasebInp {
        QdcPhasebInp::from_bits(val)
    }
}
impl From<QdcPhasebInp> for u8 {
    #[inline(always)]
    fn from(val: QdcPhasebInp) -> u8 {
        QdcPhasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcTrigInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT1 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected"]
    VAL11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN0 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected"]
    VAL46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected"]
    VAL47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected"]
    VAL48 = 0x30,
    #[doc = "TRIG_IN7 input is selected"]
    VAL49 = 0x31,
    #[doc = "TRIG_IN8 input is selected"]
    VAL50 = 0x32,
    #[doc = "TRIG_IN9 input is selected"]
    VAL51 = 0x33,
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
impl QdcTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcTrigInp {
    #[inline(always)]
    fn from(val: u8) -> QdcTrigInp {
        QdcTrigInp::from_bits(val)
    }
}
impl From<QdcTrigInp> for u8 {
    #[inline(always)]
    fn from(val: QdcTrigInp) -> u8 {
        QdcTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req100En0 {
    #[doc = "Disable"]
    VALUE6 = 0x0,
    #[doc = "Enable"]
    VALUE7 = 0x01,
}
impl Req100En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req100En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req100En0 {
    #[inline(always)]
    fn from(val: u8) -> Req100En0 {
        Req100En0::from_bits(val)
    }
}
impl From<Req100En0> for u8 {
    #[inline(always)]
    fn from(val: Req100En0) -> u8 {
        Req100En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req100En1 {
    #[doc = "Disable"]
    VALUE6 = 0x0,
    #[doc = "Enable"]
    VALUE7 = 0x01,
}
impl Req100En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req100En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req100En1 {
    #[inline(always)]
    fn from(val: u8) -> Req100En1 {
        Req100En1::from_bits(val)
    }
}
impl From<Req100En1> for u8 {
    #[inline(always)]
    fn from(val: Req100En1) -> u8 {
        Req100En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req101En0 {
    #[doc = "Disable"]
    VALUE8 = 0x0,
    #[doc = "Enable"]
    VALUE9 = 0x01,
}
impl Req101En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req101En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req101En0 {
    #[inline(always)]
    fn from(val: u8) -> Req101En0 {
        Req101En0::from_bits(val)
    }
}
impl From<Req101En0> for u8 {
    #[inline(always)]
    fn from(val: Req101En0) -> u8 {
        Req101En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req101En1 {
    #[doc = "Disable"]
    VALUE8 = 0x0,
    #[doc = "Enable"]
    VALUE9 = 0x01,
}
impl Req101En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req101En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req101En1 {
    #[inline(always)]
    fn from(val: u8) -> Req101En1 {
        Req101En1::from_bits(val)
    }
}
impl From<Req101En1> for u8 {
    #[inline(always)]
    fn from(val: Req101En1) -> u8 {
        Req101En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req102En0 {
    #[doc = "Disable"]
    VALUE10 = 0x0,
    #[doc = "Enable"]
    VALUE11 = 0x01,
}
impl Req102En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req102En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req102En0 {
    #[inline(always)]
    fn from(val: u8) -> Req102En0 {
        Req102En0::from_bits(val)
    }
}
impl From<Req102En0> for u8 {
    #[inline(always)]
    fn from(val: Req102En0) -> u8 {
        Req102En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req102En1 {
    #[doc = "Disable"]
    VALUE10 = 0x0,
    #[doc = "Enable"]
    VALUE11 = 0x01,
}
impl Req102En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req102En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req102En1 {
    #[inline(always)]
    fn from(val: u8) -> Req102En1 {
        Req102En1::from_bits(val)
    }
}
impl From<Req102En1> for u8 {
    #[inline(always)]
    fn from(val: Req102En1) -> u8 {
        Req102En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req103En0 {
    #[doc = "Disable"]
    VALUE12 = 0x0,
    #[doc = "Enable"]
    VALUE13 = 0x01,
}
impl Req103En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req103En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req103En0 {
    #[inline(always)]
    fn from(val: u8) -> Req103En0 {
        Req103En0::from_bits(val)
    }
}
impl From<Req103En0> for u8 {
    #[inline(always)]
    fn from(val: Req103En0) -> u8 {
        Req103En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req103En1 {
    #[doc = "Disable"]
    VALUE12 = 0x0,
    #[doc = "Enable"]
    VALUE13 = 0x01,
}
impl Req103En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req103En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req103En1 {
    #[inline(always)]
    fn from(val: u8) -> Req103En1 {
        Req103En1::from_bits(val)
    }
}
impl From<Req103En1> for u8 {
    #[inline(always)]
    fn from(val: Req103En1) -> u8 {
        Req103En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req104En0 {
    #[doc = "Disable"]
    VALUE14 = 0x0,
    #[doc = "Enable"]
    VALUE15 = 0x01,
}
impl Req104En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req104En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req104En0 {
    #[inline(always)]
    fn from(val: u8) -> Req104En0 {
        Req104En0::from_bits(val)
    }
}
impl From<Req104En0> for u8 {
    #[inline(always)]
    fn from(val: Req104En0) -> u8 {
        Req104En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req104En1 {
    #[doc = "Disable"]
    VALUE14 = 0x0,
    #[doc = "Enable"]
    VALUE15 = 0x01,
}
impl Req104En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req104En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req104En1 {
    #[inline(always)]
    fn from(val: u8) -> Req104En1 {
        Req104En1::from_bits(val)
    }
}
impl From<Req104En1> for u8 {
    #[inline(always)]
    fn from(val: Req104En1) -> u8 {
        Req104En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req105En0 {
    #[doc = "Disable"]
    VALUE16 = 0x0,
    #[doc = "Enable"]
    VALUE17 = 0x01,
}
impl Req105En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req105En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req105En0 {
    #[inline(always)]
    fn from(val: u8) -> Req105En0 {
        Req105En0::from_bits(val)
    }
}
impl From<Req105En0> for u8 {
    #[inline(always)]
    fn from(val: Req105En0) -> u8 {
        Req105En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req105En1 {
    #[doc = "Disable"]
    VALUE16 = 0x0,
    #[doc = "Enable"]
    VALUE17 = 0x01,
}
impl Req105En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req105En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req105En1 {
    #[inline(always)]
    fn from(val: u8) -> Req105En1 {
        Req105En1::from_bits(val)
    }
}
impl From<Req105En1> for u8 {
    #[inline(always)]
    fn from(val: Req105En1) -> u8 {
        Req105En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req106En0 {
    #[doc = "Disable"]
    VALUE18 = 0x0,
    #[doc = "Enable"]
    VALUE19 = 0x01,
}
impl Req106En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req106En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req106En0 {
    #[inline(always)]
    fn from(val: u8) -> Req106En0 {
        Req106En0::from_bits(val)
    }
}
impl From<Req106En0> for u8 {
    #[inline(always)]
    fn from(val: Req106En0) -> u8 {
        Req106En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req106En1 {
    #[doc = "Disable"]
    VALUE18 = 0x0,
    #[doc = "Enable"]
    VALUE19 = 0x01,
}
impl Req106En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req106En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req106En1 {
    #[inline(always)]
    fn from(val: u8) -> Req106En1 {
        Req106En1::from_bits(val)
    }
}
impl From<Req106En1> for u8 {
    #[inline(always)]
    fn from(val: Req106En1) -> u8 {
        Req106En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req107En0 {
    #[doc = "Disable"]
    VALUE20 = 0x0,
    #[doc = "Enable"]
    VALUE21 = 0x01,
}
impl Req107En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req107En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req107En0 {
    #[inline(always)]
    fn from(val: u8) -> Req107En0 {
        Req107En0::from_bits(val)
    }
}
impl From<Req107En0> for u8 {
    #[inline(always)]
    fn from(val: Req107En0) -> u8 {
        Req107En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req107En1 {
    #[doc = "Disable"]
    VALUE20 = 0x0,
    #[doc = "Enable"]
    VALUE21 = 0x01,
}
impl Req107En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req107En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req107En1 {
    #[inline(always)]
    fn from(val: u8) -> Req107En1 {
        Req107En1::from_bits(val)
    }
}
impl From<Req107En1> for u8 {
    #[inline(always)]
    fn from(val: Req107En1) -> u8 {
        Req107En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req108En0 {
    #[doc = "Disable"]
    VALUE22 = 0x0,
    #[doc = "Enable"]
    VALUE23 = 0x01,
}
impl Req108En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req108En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req108En0 {
    #[inline(always)]
    fn from(val: u8) -> Req108En0 {
        Req108En0::from_bits(val)
    }
}
impl From<Req108En0> for u8 {
    #[inline(always)]
    fn from(val: Req108En0) -> u8 {
        Req108En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req108En1 {
    #[doc = "Disable"]
    VALUE22 = 0x0,
    #[doc = "Enable"]
    VALUE23 = 0x01,
}
impl Req108En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req108En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req108En1 {
    #[inline(always)]
    fn from(val: u8) -> Req108En1 {
        Req108En1::from_bits(val)
    }
}
impl From<Req108En1> for u8 {
    #[inline(always)]
    fn from(val: Req108En1) -> u8 {
        Req108En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req109En0 {
    #[doc = "Disable"]
    VALUE24 = 0x0,
    #[doc = "Enable"]
    VALUE25 = 0x01,
}
impl Req109En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req109En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req109En0 {
    #[inline(always)]
    fn from(val: u8) -> Req109En0 {
        Req109En0::from_bits(val)
    }
}
impl From<Req109En0> for u8 {
    #[inline(always)]
    fn from(val: Req109En0) -> u8 {
        Req109En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req109En1 {
    #[doc = "Disable"]
    VALUE24 = 0x0,
    #[doc = "Enable"]
    VALUE25 = 0x01,
}
impl Req109En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req109En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req109En1 {
    #[inline(always)]
    fn from(val: u8) -> Req109En1 {
        Req109En1::from_bits(val)
    }
}
impl From<Req109En1> for u8 {
    #[inline(always)]
    fn from(val: Req109En1) -> u8 {
        Req109En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req10En0 {
    #[doc = "Disable"]
    VALUE18 = 0x0,
    #[doc = "Enable"]
    VALUE19 = 0x01,
}
impl Req10En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req10En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req10En0 {
    #[inline(always)]
    fn from(val: u8) -> Req10En0 {
        Req10En0::from_bits(val)
    }
}
impl From<Req10En0> for u8 {
    #[inline(always)]
    fn from(val: Req10En0) -> u8 {
        Req10En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req10En1 {
    #[doc = "Disable"]
    VALUE18 = 0x0,
    #[doc = "Enable"]
    VALUE19 = 0x01,
}
impl Req10En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req10En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req10En1 {
    #[inline(always)]
    fn from(val: u8) -> Req10En1 {
        Req10En1::from_bits(val)
    }
}
impl From<Req10En1> for u8 {
    #[inline(always)]
    fn from(val: Req10En1) -> u8 {
        Req10En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req110En0 {
    #[doc = "Disable"]
    VALUE26 = 0x0,
    #[doc = "Enable"]
    VALUE27 = 0x01,
}
impl Req110En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req110En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req110En0 {
    #[inline(always)]
    fn from(val: u8) -> Req110En0 {
        Req110En0::from_bits(val)
    }
}
impl From<Req110En0> for u8 {
    #[inline(always)]
    fn from(val: Req110En0) -> u8 {
        Req110En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req110En1 {
    #[doc = "Disable"]
    VALUE26 = 0x0,
    #[doc = "Enable"]
    VALUE27 = 0x01,
}
impl Req110En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req110En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req110En1 {
    #[inline(always)]
    fn from(val: u8) -> Req110En1 {
        Req110En1::from_bits(val)
    }
}
impl From<Req110En1> for u8 {
    #[inline(always)]
    fn from(val: Req110En1) -> u8 {
        Req110En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req111En0 {
    #[doc = "Disable"]
    VALUE28 = 0x0,
    #[doc = "Enable"]
    VALUE29 = 0x01,
}
impl Req111En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req111En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req111En0 {
    #[inline(always)]
    fn from(val: u8) -> Req111En0 {
        Req111En0::from_bits(val)
    }
}
impl From<Req111En0> for u8 {
    #[inline(always)]
    fn from(val: Req111En0) -> u8 {
        Req111En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req111En1 {
    #[doc = "Disable"]
    VALUE28 = 0x0,
    #[doc = "Enable"]
    VALUE29 = 0x01,
}
impl Req111En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req111En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req111En1 {
    #[inline(always)]
    fn from(val: u8) -> Req111En1 {
        Req111En1::from_bits(val)
    }
}
impl From<Req111En1> for u8 {
    #[inline(always)]
    fn from(val: Req111En1) -> u8 {
        Req111En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req112En0 {
    #[doc = "Disable"]
    VALUE30 = 0x0,
    #[doc = "Enable"]
    VALUE31 = 0x01,
}
impl Req112En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req112En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req112En0 {
    #[inline(always)]
    fn from(val: u8) -> Req112En0 {
        Req112En0::from_bits(val)
    }
}
impl From<Req112En0> for u8 {
    #[inline(always)]
    fn from(val: Req112En0) -> u8 {
        Req112En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req112En1 {
    #[doc = "Disable"]
    VALUE30 = 0x0,
    #[doc = "Enable"]
    VALUE31 = 0x01,
}
impl Req112En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req112En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req112En1 {
    #[inline(always)]
    fn from(val: u8) -> Req112En1 {
        Req112En1::from_bits(val)
    }
}
impl From<Req112En1> for u8 {
    #[inline(always)]
    fn from(val: Req112En1) -> u8 {
        Req112En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req113En0 {
    #[doc = "Disable"]
    VALUE32 = 0x0,
    #[doc = "Enable"]
    VALUE33 = 0x01,
}
impl Req113En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req113En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req113En0 {
    #[inline(always)]
    fn from(val: u8) -> Req113En0 {
        Req113En0::from_bits(val)
    }
}
impl From<Req113En0> for u8 {
    #[inline(always)]
    fn from(val: Req113En0) -> u8 {
        Req113En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req113En1 {
    #[doc = "Disable"]
    VALUE32 = 0x0,
    #[doc = "Enable"]
    VALUE33 = 0x01,
}
impl Req113En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req113En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req113En1 {
    #[inline(always)]
    fn from(val: u8) -> Req113En1 {
        Req113En1::from_bits(val)
    }
}
impl From<Req113En1> for u8 {
    #[inline(always)]
    fn from(val: Req113En1) -> u8 {
        Req113En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req114En0 {
    #[doc = "Disable"]
    VALUE34 = 0x0,
    #[doc = "Enable"]
    VALUE35 = 0x01,
}
impl Req114En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req114En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req114En0 {
    #[inline(always)]
    fn from(val: u8) -> Req114En0 {
        Req114En0::from_bits(val)
    }
}
impl From<Req114En0> for u8 {
    #[inline(always)]
    fn from(val: Req114En0) -> u8 {
        Req114En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req114En1 {
    #[doc = "Disable"]
    VALUE34 = 0x0,
    #[doc = "Enable"]
    VALUE35 = 0x01,
}
impl Req114En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req114En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req114En1 {
    #[inline(always)]
    fn from(val: u8) -> Req114En1 {
        Req114En1::from_bits(val)
    }
}
impl From<Req114En1> for u8 {
    #[inline(always)]
    fn from(val: Req114En1) -> u8 {
        Req114En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req115En0 {
    #[doc = "Disable"]
    VALUE36 = 0x0,
    #[doc = "Enable"]
    VALUE37 = 0x01,
}
impl Req115En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req115En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req115En0 {
    #[inline(always)]
    fn from(val: u8) -> Req115En0 {
        Req115En0::from_bits(val)
    }
}
impl From<Req115En0> for u8 {
    #[inline(always)]
    fn from(val: Req115En0) -> u8 {
        Req115En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req115En1 {
    #[doc = "Disable"]
    VALUE36 = 0x0,
    #[doc = "Enable"]
    VALUE37 = 0x01,
}
impl Req115En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req115En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req115En1 {
    #[inline(always)]
    fn from(val: u8) -> Req115En1 {
        Req115En1::from_bits(val)
    }
}
impl From<Req115En1> for u8 {
    #[inline(always)]
    fn from(val: Req115En1) -> u8 {
        Req115En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req116En0 {
    #[doc = "Disable"]
    VALUE38 = 0x0,
    #[doc = "Enable"]
    VALUE39 = 0x01,
}
impl Req116En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req116En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req116En0 {
    #[inline(always)]
    fn from(val: u8) -> Req116En0 {
        Req116En0::from_bits(val)
    }
}
impl From<Req116En0> for u8 {
    #[inline(always)]
    fn from(val: Req116En0) -> u8 {
        Req116En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req116En1 {
    #[doc = "Disable"]
    VALUE38 = 0x0,
    #[doc = "Enable"]
    VALUE39 = 0x01,
}
impl Req116En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req116En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req116En1 {
    #[inline(always)]
    fn from(val: u8) -> Req116En1 {
        Req116En1::from_bits(val)
    }
}
impl From<Req116En1> for u8 {
    #[inline(always)]
    fn from(val: Req116En1) -> u8 {
        Req116En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req117En0 {
    #[doc = "Disable"]
    VALUE40 = 0x0,
    #[doc = "Enable"]
    VALUE41 = 0x01,
}
impl Req117En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req117En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req117En0 {
    #[inline(always)]
    fn from(val: u8) -> Req117En0 {
        Req117En0::from_bits(val)
    }
}
impl From<Req117En0> for u8 {
    #[inline(always)]
    fn from(val: Req117En0) -> u8 {
        Req117En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req117En1 {
    #[doc = "Disable"]
    VALUE40 = 0x0,
    #[doc = "Enable"]
    VALUE41 = 0x01,
}
impl Req117En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req117En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req117En1 {
    #[inline(always)]
    fn from(val: u8) -> Req117En1 {
        Req117En1::from_bits(val)
    }
}
impl From<Req117En1> for u8 {
    #[inline(always)]
    fn from(val: Req117En1) -> u8 {
        Req117En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req118En0 {
    #[doc = "Disable"]
    VALUE42 = 0x0,
    #[doc = "Enable"]
    VALUE43 = 0x01,
}
impl Req118En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req118En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req118En0 {
    #[inline(always)]
    fn from(val: u8) -> Req118En0 {
        Req118En0::from_bits(val)
    }
}
impl From<Req118En0> for u8 {
    #[inline(always)]
    fn from(val: Req118En0) -> u8 {
        Req118En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req118En1 {
    #[doc = "Disable"]
    VALUE42 = 0x0,
    #[doc = "Enable"]
    VALUE43 = 0x01,
}
impl Req118En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req118En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req118En1 {
    #[inline(always)]
    fn from(val: u8) -> Req118En1 {
        Req118En1::from_bits(val)
    }
}
impl From<Req118En1> for u8 {
    #[inline(always)]
    fn from(val: Req118En1) -> u8 {
        Req118En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req119En0 {
    #[doc = "Disable"]
    VALUE44 = 0x0,
    #[doc = "Enable"]
    VALUE45 = 0x01,
}
impl Req119En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req119En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req119En0 {
    #[inline(always)]
    fn from(val: u8) -> Req119En0 {
        Req119En0::from_bits(val)
    }
}
impl From<Req119En0> for u8 {
    #[inline(always)]
    fn from(val: Req119En0) -> u8 {
        Req119En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req119En1 {
    #[doc = "Disable"]
    VALUE44 = 0x0,
    #[doc = "Enable"]
    VALUE45 = 0x01,
}
impl Req119En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req119En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req119En1 {
    #[inline(always)]
    fn from(val: u8) -> Req119En1 {
        Req119En1::from_bits(val)
    }
}
impl From<Req119En1> for u8 {
    #[inline(always)]
    fn from(val: Req119En1) -> u8 {
        Req119En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req11En0 {
    #[doc = "Disable"]
    VALUE20 = 0x0,
    #[doc = "Enable"]
    VALUE21 = 0x01,
}
impl Req11En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req11En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req11En0 {
    #[inline(always)]
    fn from(val: u8) -> Req11En0 {
        Req11En0::from_bits(val)
    }
}
impl From<Req11En0> for u8 {
    #[inline(always)]
    fn from(val: Req11En0) -> u8 {
        Req11En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req11En1 {
    #[doc = "Disable"]
    VALUE20 = 0x0,
    #[doc = "Enable"]
    VALUE21 = 0x01,
}
impl Req11En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req11En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req11En1 {
    #[inline(always)]
    fn from(val: u8) -> Req11En1 {
        Req11En1::from_bits(val)
    }
}
impl From<Req11En1> for u8 {
    #[inline(always)]
    fn from(val: Req11En1) -> u8 {
        Req11En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req120En0 {
    #[doc = "Disable"]
    VALUE46 = 0x0,
    #[doc = "Enable"]
    VALUE47 = 0x01,
}
impl Req120En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req120En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req120En0 {
    #[inline(always)]
    fn from(val: u8) -> Req120En0 {
        Req120En0::from_bits(val)
    }
}
impl From<Req120En0> for u8 {
    #[inline(always)]
    fn from(val: Req120En0) -> u8 {
        Req120En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req120En1 {
    #[doc = "Disable"]
    VALUE46 = 0x0,
    #[doc = "Enable"]
    VALUE47 = 0x01,
}
impl Req120En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req120En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req120En1 {
    #[inline(always)]
    fn from(val: u8) -> Req120En1 {
        Req120En1::from_bits(val)
    }
}
impl From<Req120En1> for u8 {
    #[inline(always)]
    fn from(val: Req120En1) -> u8 {
        Req120En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req121En0 {
    #[doc = "Disable"]
    VALUE48 = 0x0,
    #[doc = "Enable"]
    VALUE49 = 0x01,
}
impl Req121En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req121En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req121En0 {
    #[inline(always)]
    fn from(val: u8) -> Req121En0 {
        Req121En0::from_bits(val)
    }
}
impl From<Req121En0> for u8 {
    #[inline(always)]
    fn from(val: Req121En0) -> u8 {
        Req121En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req121En1 {
    #[doc = "Disable"]
    VALUE48 = 0x0,
    #[doc = "Enable"]
    VALUE49 = 0x01,
}
impl Req121En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req121En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req121En1 {
    #[inline(always)]
    fn from(val: u8) -> Req121En1 {
        Req121En1::from_bits(val)
    }
}
impl From<Req121En1> for u8 {
    #[inline(always)]
    fn from(val: Req121En1) -> u8 {
        Req121En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req12En0 {
    #[doc = "Disable"]
    VALUE22 = 0x0,
    #[doc = "Enable"]
    VALUE23 = 0x01,
}
impl Req12En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req12En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req12En0 {
    #[inline(always)]
    fn from(val: u8) -> Req12En0 {
        Req12En0::from_bits(val)
    }
}
impl From<Req12En0> for u8 {
    #[inline(always)]
    fn from(val: Req12En0) -> u8 {
        Req12En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req12En1 {
    #[doc = "Disable"]
    VALUE22 = 0x0,
    #[doc = "Enable"]
    VALUE23 = 0x01,
}
impl Req12En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req12En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req12En1 {
    #[inline(always)]
    fn from(val: u8) -> Req12En1 {
        Req12En1::from_bits(val)
    }
}
impl From<Req12En1> for u8 {
    #[inline(always)]
    fn from(val: Req12En1) -> u8 {
        Req12En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req13En0 {
    #[doc = "Disable"]
    VALUE24 = 0x0,
    #[doc = "Enable"]
    VALUE25 = 0x01,
}
impl Req13En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req13En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req13En0 {
    #[inline(always)]
    fn from(val: u8) -> Req13En0 {
        Req13En0::from_bits(val)
    }
}
impl From<Req13En0> for u8 {
    #[inline(always)]
    fn from(val: Req13En0) -> u8 {
        Req13En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req13En1 {
    #[doc = "Disable"]
    VALUE24 = 0x0,
    #[doc = "Enable"]
    VALUE25 = 0x01,
}
impl Req13En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req13En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req13En1 {
    #[inline(always)]
    fn from(val: u8) -> Req13En1 {
        Req13En1::from_bits(val)
    }
}
impl From<Req13En1> for u8 {
    #[inline(always)]
    fn from(val: Req13En1) -> u8 {
        Req13En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req14En0 {
    #[doc = "Disable"]
    VALUE26 = 0x0,
    #[doc = "Enable"]
    VALUE27 = 0x01,
}
impl Req14En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req14En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req14En0 {
    #[inline(always)]
    fn from(val: u8) -> Req14En0 {
        Req14En0::from_bits(val)
    }
}
impl From<Req14En0> for u8 {
    #[inline(always)]
    fn from(val: Req14En0) -> u8 {
        Req14En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req14En1 {
    #[doc = "Disable"]
    VALUE26 = 0x0,
    #[doc = "Enable"]
    VALUE27 = 0x01,
}
impl Req14En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req14En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req14En1 {
    #[inline(always)]
    fn from(val: u8) -> Req14En1 {
        Req14En1::from_bits(val)
    }
}
impl From<Req14En1> for u8 {
    #[inline(always)]
    fn from(val: Req14En1) -> u8 {
        Req14En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req15En0 {
    #[doc = "Disable"]
    VALUE28 = 0x0,
    #[doc = "Enable"]
    VALUE29 = 0x01,
}
impl Req15En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req15En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req15En0 {
    #[inline(always)]
    fn from(val: u8) -> Req15En0 {
        Req15En0::from_bits(val)
    }
}
impl From<Req15En0> for u8 {
    #[inline(always)]
    fn from(val: Req15En0) -> u8 {
        Req15En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req15En1 {
    #[doc = "Disable"]
    VALUE28 = 0x0,
    #[doc = "Enable"]
    VALUE29 = 0x01,
}
impl Req15En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req15En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req15En1 {
    #[inline(always)]
    fn from(val: u8) -> Req15En1 {
        Req15En1::from_bits(val)
    }
}
impl From<Req15En1> for u8 {
    #[inline(always)]
    fn from(val: Req15En1) -> u8 {
        Req15En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req16En0 {
    #[doc = "Disable"]
    VALUE30 = 0x0,
    #[doc = "Enable"]
    VALUE31 = 0x01,
}
impl Req16En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req16En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req16En0 {
    #[inline(always)]
    fn from(val: u8) -> Req16En0 {
        Req16En0::from_bits(val)
    }
}
impl From<Req16En0> for u8 {
    #[inline(always)]
    fn from(val: Req16En0) -> u8 {
        Req16En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req16En1 {
    #[doc = "Disable"]
    VALUE30 = 0x0,
    #[doc = "Enable"]
    VALUE31 = 0x01,
}
impl Req16En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req16En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req16En1 {
    #[inline(always)]
    fn from(val: u8) -> Req16En1 {
        Req16En1::from_bits(val)
    }
}
impl From<Req16En1> for u8 {
    #[inline(always)]
    fn from(val: Req16En1) -> u8 {
        Req16En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req17En0 {
    #[doc = "Disable"]
    VALUE32 = 0x0,
    #[doc = "Enable"]
    VALUE33 = 0x01,
}
impl Req17En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req17En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req17En0 {
    #[inline(always)]
    fn from(val: u8) -> Req17En0 {
        Req17En0::from_bits(val)
    }
}
impl From<Req17En0> for u8 {
    #[inline(always)]
    fn from(val: Req17En0) -> u8 {
        Req17En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req17En1 {
    #[doc = "Disable"]
    VALUE32 = 0x0,
    #[doc = "Enable"]
    VALUE33 = 0x01,
}
impl Req17En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req17En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req17En1 {
    #[inline(always)]
    fn from(val: u8) -> Req17En1 {
        Req17En1::from_bits(val)
    }
}
impl From<Req17En1> for u8 {
    #[inline(always)]
    fn from(val: Req17En1) -> u8 {
        Req17En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req18En0 {
    #[doc = "Disable"]
    VALUE34 = 0x0,
    #[doc = "Enable"]
    VALUE35 = 0x01,
}
impl Req18En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req18En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req18En0 {
    #[inline(always)]
    fn from(val: u8) -> Req18En0 {
        Req18En0::from_bits(val)
    }
}
impl From<Req18En0> for u8 {
    #[inline(always)]
    fn from(val: Req18En0) -> u8 {
        Req18En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req18En1 {
    #[doc = "Disable"]
    VALUE34 = 0x0,
    #[doc = "Enable"]
    VALUE35 = 0x01,
}
impl Req18En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req18En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req18En1 {
    #[inline(always)]
    fn from(val: u8) -> Req18En1 {
        Req18En1::from_bits(val)
    }
}
impl From<Req18En1> for u8 {
    #[inline(always)]
    fn from(val: Req18En1) -> u8 {
        Req18En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req19En0 {
    #[doc = "Disable"]
    VALUE36 = 0x0,
    #[doc = "Enable"]
    VALUE37 = 0x01,
}
impl Req19En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req19En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req19En0 {
    #[inline(always)]
    fn from(val: u8) -> Req19En0 {
        Req19En0::from_bits(val)
    }
}
impl From<Req19En0> for u8 {
    #[inline(always)]
    fn from(val: Req19En0) -> u8 {
        Req19En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req19En1 {
    #[doc = "Disable"]
    VALUE36 = 0x0,
    #[doc = "Enable"]
    VALUE37 = 0x01,
}
impl Req19En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req19En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req19En1 {
    #[inline(always)]
    fn from(val: u8) -> Req19En1 {
        Req19En1::from_bits(val)
    }
}
impl From<Req19En1> for u8 {
    #[inline(always)]
    fn from(val: Req19En1) -> u8 {
        Req19En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req1En0 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req1En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req1En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req1En0 {
    #[inline(always)]
    fn from(val: u8) -> Req1En0 {
        Req1En0::from_bits(val)
    }
}
impl From<Req1En0> for u8 {
    #[inline(always)]
    fn from(val: Req1En0) -> u8 {
        Req1En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req1En1 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req1En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req1En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req1En1 {
    #[inline(always)]
    fn from(val: u8) -> Req1En1 {
        Req1En1::from_bits(val)
    }
}
impl From<Req1En1> for u8 {
    #[inline(always)]
    fn from(val: Req1En1) -> u8 {
        Req1En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req20En0 {
    #[doc = "Disable"]
    VALUE38 = 0x0,
    #[doc = "Enable"]
    VALUE39 = 0x01,
}
impl Req20En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req20En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req20En0 {
    #[inline(always)]
    fn from(val: u8) -> Req20En0 {
        Req20En0::from_bits(val)
    }
}
impl From<Req20En0> for u8 {
    #[inline(always)]
    fn from(val: Req20En0) -> u8 {
        Req20En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req20En1 {
    #[doc = "Disable"]
    VALUE38 = 0x0,
    #[doc = "Enable"]
    VALUE39 = 0x01,
}
impl Req20En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req20En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req20En1 {
    #[inline(always)]
    fn from(val: u8) -> Req20En1 {
        Req20En1::from_bits(val)
    }
}
impl From<Req20En1> for u8 {
    #[inline(always)]
    fn from(val: Req20En1) -> u8 {
        Req20En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req21En0 {
    #[doc = "Disable"]
    VALUE40 = 0x0,
    #[doc = "Enable"]
    VALUE41 = 0x01,
}
impl Req21En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req21En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req21En0 {
    #[inline(always)]
    fn from(val: u8) -> Req21En0 {
        Req21En0::from_bits(val)
    }
}
impl From<Req21En0> for u8 {
    #[inline(always)]
    fn from(val: Req21En0) -> u8 {
        Req21En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req21En1 {
    #[doc = "Disable"]
    VALUE40 = 0x0,
    #[doc = "Enable"]
    VALUE41 = 0x01,
}
impl Req21En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req21En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req21En1 {
    #[inline(always)]
    fn from(val: u8) -> Req21En1 {
        Req21En1::from_bits(val)
    }
}
impl From<Req21En1> for u8 {
    #[inline(always)]
    fn from(val: Req21En1) -> u8 {
        Req21En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req22En0 {
    #[doc = "Disable"]
    VALUE42 = 0x0,
    #[doc = "Enable"]
    VALUE43 = 0x01,
}
impl Req22En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req22En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req22En0 {
    #[inline(always)]
    fn from(val: u8) -> Req22En0 {
        Req22En0::from_bits(val)
    }
}
impl From<Req22En0> for u8 {
    #[inline(always)]
    fn from(val: Req22En0) -> u8 {
        Req22En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req22En1 {
    #[doc = "Disable"]
    VALUE42 = 0x0,
    #[doc = "Enable"]
    VALUE43 = 0x01,
}
impl Req22En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req22En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req22En1 {
    #[inline(always)]
    fn from(val: u8) -> Req22En1 {
        Req22En1::from_bits(val)
    }
}
impl From<Req22En1> for u8 {
    #[inline(always)]
    fn from(val: Req22En1) -> u8 {
        Req22En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req23En0 {
    #[doc = "Disable"]
    VALUE44 = 0x0,
    #[doc = "Enable"]
    VALUE45 = 0x01,
}
impl Req23En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req23En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req23En0 {
    #[inline(always)]
    fn from(val: u8) -> Req23En0 {
        Req23En0::from_bits(val)
    }
}
impl From<Req23En0> for u8 {
    #[inline(always)]
    fn from(val: Req23En0) -> u8 {
        Req23En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req23En1 {
    #[doc = "Disable"]
    VALUE44 = 0x0,
    #[doc = "Enable"]
    VALUE45 = 0x01,
}
impl Req23En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req23En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req23En1 {
    #[inline(always)]
    fn from(val: u8) -> Req23En1 {
        Req23En1::from_bits(val)
    }
}
impl From<Req23En1> for u8 {
    #[inline(always)]
    fn from(val: Req23En1) -> u8 {
        Req23En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req24En0 {
    #[doc = "Disable"]
    VALUE46 = 0x0,
    #[doc = "Enable"]
    VALUE47 = 0x01,
}
impl Req24En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req24En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req24En0 {
    #[inline(always)]
    fn from(val: u8) -> Req24En0 {
        Req24En0::from_bits(val)
    }
}
impl From<Req24En0> for u8 {
    #[inline(always)]
    fn from(val: Req24En0) -> u8 {
        Req24En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req24En1 {
    #[doc = "Disable"]
    VALUE46 = 0x0,
    #[doc = "Enable"]
    VALUE47 = 0x01,
}
impl Req24En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req24En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req24En1 {
    #[inline(always)]
    fn from(val: u8) -> Req24En1 {
        Req24En1::from_bits(val)
    }
}
impl From<Req24En1> for u8 {
    #[inline(always)]
    fn from(val: Req24En1) -> u8 {
        Req24En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req25En0 {
    #[doc = "Disable"]
    VALUE48 = 0x0,
    #[doc = "Enable"]
    VALUE49 = 0x01,
}
impl Req25En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req25En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req25En0 {
    #[inline(always)]
    fn from(val: u8) -> Req25En0 {
        Req25En0::from_bits(val)
    }
}
impl From<Req25En0> for u8 {
    #[inline(always)]
    fn from(val: Req25En0) -> u8 {
        Req25En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req25En1 {
    #[doc = "Disable"]
    VALUE48 = 0x0,
    #[doc = "Enable"]
    VALUE49 = 0x01,
}
impl Req25En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req25En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req25En1 {
    #[inline(always)]
    fn from(val: u8) -> Req25En1 {
        Req25En1::from_bits(val)
    }
}
impl From<Req25En1> for u8 {
    #[inline(always)]
    fn from(val: Req25En1) -> u8 {
        Req25En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req26En0 {
    #[doc = "Disable"]
    VALUE50 = 0x0,
    #[doc = "Enable"]
    VALUE51 = 0x01,
}
impl Req26En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req26En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req26En0 {
    #[inline(always)]
    fn from(val: u8) -> Req26En0 {
        Req26En0::from_bits(val)
    }
}
impl From<Req26En0> for u8 {
    #[inline(always)]
    fn from(val: Req26En0) -> u8 {
        Req26En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req26En1 {
    #[doc = "Disable"]
    VALUE50 = 0x0,
    #[doc = "Enable"]
    VALUE51 = 0x01,
}
impl Req26En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req26En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req26En1 {
    #[inline(always)]
    fn from(val: u8) -> Req26En1 {
        Req26En1::from_bits(val)
    }
}
impl From<Req26En1> for u8 {
    #[inline(always)]
    fn from(val: Req26En1) -> u8 {
        Req26En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req27En0 {
    #[doc = "Disable"]
    VALUE52 = 0x0,
    #[doc = "Enable"]
    VALUE53 = 0x01,
}
impl Req27En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req27En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req27En0 {
    #[inline(always)]
    fn from(val: u8) -> Req27En0 {
        Req27En0::from_bits(val)
    }
}
impl From<Req27En0> for u8 {
    #[inline(always)]
    fn from(val: Req27En0) -> u8 {
        Req27En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req27En1 {
    #[doc = "Disable"]
    VALUE52 = 0x0,
    #[doc = "Enable"]
    VALUE53 = 0x01,
}
impl Req27En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req27En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req27En1 {
    #[inline(always)]
    fn from(val: u8) -> Req27En1 {
        Req27En1::from_bits(val)
    }
}
impl From<Req27En1> for u8 {
    #[inline(always)]
    fn from(val: Req27En1) -> u8 {
        Req27En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req28En0 {
    #[doc = "Disable"]
    VALUE54 = 0x0,
    #[doc = "Enable"]
    VALUE55 = 0x01,
}
impl Req28En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req28En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req28En0 {
    #[inline(always)]
    fn from(val: u8) -> Req28En0 {
        Req28En0::from_bits(val)
    }
}
impl From<Req28En0> for u8 {
    #[inline(always)]
    fn from(val: Req28En0) -> u8 {
        Req28En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req28En1 {
    #[doc = "Disable"]
    VALUE54 = 0x0,
    #[doc = "Enable"]
    VALUE55 = 0x01,
}
impl Req28En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req28En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req28En1 {
    #[inline(always)]
    fn from(val: u8) -> Req28En1 {
        Req28En1::from_bits(val)
    }
}
impl From<Req28En1> for u8 {
    #[inline(always)]
    fn from(val: Req28En1) -> u8 {
        Req28En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req29En0 {
    #[doc = "Disable"]
    VALUE56 = 0x0,
    #[doc = "Enable"]
    VALUE57 = 0x01,
}
impl Req29En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req29En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req29En0 {
    #[inline(always)]
    fn from(val: u8) -> Req29En0 {
        Req29En0::from_bits(val)
    }
}
impl From<Req29En0> for u8 {
    #[inline(always)]
    fn from(val: Req29En0) -> u8 {
        Req29En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req29En1 {
    #[doc = "Disable"]
    VALUE56 = 0x0,
    #[doc = "Enable"]
    VALUE57 = 0x01,
}
impl Req29En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req29En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req29En1 {
    #[inline(always)]
    fn from(val: u8) -> Req29En1 {
        Req29En1::from_bits(val)
    }
}
impl From<Req29En1> for u8 {
    #[inline(always)]
    fn from(val: Req29En1) -> u8 {
        Req29En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req2En0 {
    #[doc = "Disable"]
    VALUE2 = 0x0,
    #[doc = "Enable"]
    VALUE3 = 0x01,
}
impl Req2En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req2En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req2En0 {
    #[inline(always)]
    fn from(val: u8) -> Req2En0 {
        Req2En0::from_bits(val)
    }
}
impl From<Req2En0> for u8 {
    #[inline(always)]
    fn from(val: Req2En0) -> u8 {
        Req2En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req2En1 {
    #[doc = "Disable"]
    VALUE2 = 0x0,
    #[doc = "Enable"]
    VALUE3 = 0x01,
}
impl Req2En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req2En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req2En1 {
    #[inline(always)]
    fn from(val: u8) -> Req2En1 {
        Req2En1::from_bits(val)
    }
}
impl From<Req2En1> for u8 {
    #[inline(always)]
    fn from(val: Req2En1) -> u8 {
        Req2En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req30En0 {
    #[doc = "Disable"]
    VALUE58 = 0x0,
    #[doc = "Enable"]
    VALUE59 = 0x01,
}
impl Req30En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req30En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req30En0 {
    #[inline(always)]
    fn from(val: u8) -> Req30En0 {
        Req30En0::from_bits(val)
    }
}
impl From<Req30En0> for u8 {
    #[inline(always)]
    fn from(val: Req30En0) -> u8 {
        Req30En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req30En1 {
    #[doc = "Disable"]
    VALUE58 = 0x0,
    #[doc = "Enable"]
    VALUE59 = 0x01,
}
impl Req30En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req30En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req30En1 {
    #[inline(always)]
    fn from(val: u8) -> Req30En1 {
        Req30En1::from_bits(val)
    }
}
impl From<Req30En1> for u8 {
    #[inline(always)]
    fn from(val: Req30En1) -> u8 {
        Req30En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req31En0 {
    #[doc = "Disable"]
    VALUE60 = 0x0,
    #[doc = "Enable"]
    VALUE61 = 0x01,
}
impl Req31En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req31En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req31En0 {
    #[inline(always)]
    fn from(val: u8) -> Req31En0 {
        Req31En0::from_bits(val)
    }
}
impl From<Req31En0> for u8 {
    #[inline(always)]
    fn from(val: Req31En0) -> u8 {
        Req31En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req31En1 {
    #[doc = "Disable"]
    VALUE60 = 0x0,
    #[doc = "Enable"]
    VALUE61 = 0x01,
}
impl Req31En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req31En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req31En1 {
    #[inline(always)]
    fn from(val: u8) -> Req31En1 {
        Req31En1::from_bits(val)
    }
}
impl From<Req31En1> for u8 {
    #[inline(always)]
    fn from(val: Req31En1) -> u8 {
        Req31En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req32En0 {
    #[doc = "Disable"]
    VALUE62 = 0x0,
    #[doc = "Enable"]
    VALUE63 = 0x01,
}
impl Req32En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req32En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req32En0 {
    #[inline(always)]
    fn from(val: u8) -> Req32En0 {
        Req32En0::from_bits(val)
    }
}
impl From<Req32En0> for u8 {
    #[inline(always)]
    fn from(val: Req32En0) -> u8 {
        Req32En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req32En1 {
    #[doc = "Disable"]
    VALUE62 = 0x0,
    #[doc = "Enable"]
    VALUE63 = 0x01,
}
impl Req32En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req32En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req32En1 {
    #[inline(always)]
    fn from(val: u8) -> Req32En1 {
        Req32En1::from_bits(val)
    }
}
impl From<Req32En1> for u8 {
    #[inline(always)]
    fn from(val: Req32En1) -> u8 {
        Req32En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req33En0 {
    #[doc = "Disable"]
    VALUE64 = 0x0,
    #[doc = "Enable"]
    VALUE65 = 0x01,
}
impl Req33En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req33En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req33En0 {
    #[inline(always)]
    fn from(val: u8) -> Req33En0 {
        Req33En0::from_bits(val)
    }
}
impl From<Req33En0> for u8 {
    #[inline(always)]
    fn from(val: Req33En0) -> u8 {
        Req33En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req33En1 {
    #[doc = "Disable"]
    VALUE64 = 0x0,
    #[doc = "Enable"]
    VALUE65 = 0x01,
}
impl Req33En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req33En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req33En1 {
    #[inline(always)]
    fn from(val: u8) -> Req33En1 {
        Req33En1::from_bits(val)
    }
}
impl From<Req33En1> for u8 {
    #[inline(always)]
    fn from(val: Req33En1) -> u8 {
        Req33En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req34En0 {
    #[doc = "Disable"]
    VALUE66 = 0x0,
    #[doc = "Enable"]
    VALUE67 = 0x01,
}
impl Req34En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req34En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req34En0 {
    #[inline(always)]
    fn from(val: u8) -> Req34En0 {
        Req34En0::from_bits(val)
    }
}
impl From<Req34En0> for u8 {
    #[inline(always)]
    fn from(val: Req34En0) -> u8 {
        Req34En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req34En1 {
    #[doc = "Disable"]
    VALUE66 = 0x0,
    #[doc = "Enable"]
    VALUE67 = 0x01,
}
impl Req34En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req34En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req34En1 {
    #[inline(always)]
    fn from(val: u8) -> Req34En1 {
        Req34En1::from_bits(val)
    }
}
impl From<Req34En1> for u8 {
    #[inline(always)]
    fn from(val: Req34En1) -> u8 {
        Req34En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req35En0 {
    #[doc = "Disable"]
    VALUE68 = 0x0,
    #[doc = "Enable"]
    VALUE69 = 0x01,
}
impl Req35En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req35En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req35En0 {
    #[inline(always)]
    fn from(val: u8) -> Req35En0 {
        Req35En0::from_bits(val)
    }
}
impl From<Req35En0> for u8 {
    #[inline(always)]
    fn from(val: Req35En0) -> u8 {
        Req35En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req35En1 {
    #[doc = "Disable"]
    VALUE68 = 0x0,
    #[doc = "Enable"]
    VALUE69 = 0x01,
}
impl Req35En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req35En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req35En1 {
    #[inline(always)]
    fn from(val: u8) -> Req35En1 {
        Req35En1::from_bits(val)
    }
}
impl From<Req35En1> for u8 {
    #[inline(always)]
    fn from(val: Req35En1) -> u8 {
        Req35En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req36En0 {
    #[doc = "Disable"]
    VALUE70 = 0x0,
    #[doc = "Enable"]
    VALUE71 = 0x01,
}
impl Req36En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req36En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req36En0 {
    #[inline(always)]
    fn from(val: u8) -> Req36En0 {
        Req36En0::from_bits(val)
    }
}
impl From<Req36En0> for u8 {
    #[inline(always)]
    fn from(val: Req36En0) -> u8 {
        Req36En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req36En1 {
    #[doc = "Disable"]
    VALUE70 = 0x0,
    #[doc = "Enable"]
    VALUE71 = 0x01,
}
impl Req36En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req36En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req36En1 {
    #[inline(always)]
    fn from(val: u8) -> Req36En1 {
        Req36En1::from_bits(val)
    }
}
impl From<Req36En1> for u8 {
    #[inline(always)]
    fn from(val: Req36En1) -> u8 {
        Req36En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req37En0 {
    #[doc = "Disable"]
    VALUE72 = 0x0,
    #[doc = "Enable"]
    VALUE73 = 0x01,
}
impl Req37En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req37En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req37En0 {
    #[inline(always)]
    fn from(val: u8) -> Req37En0 {
        Req37En0::from_bits(val)
    }
}
impl From<Req37En0> for u8 {
    #[inline(always)]
    fn from(val: Req37En0) -> u8 {
        Req37En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req37En1 {
    #[doc = "Disable"]
    VALUE72 = 0x0,
    #[doc = "Enable"]
    VALUE73 = 0x01,
}
impl Req37En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req37En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req37En1 {
    #[inline(always)]
    fn from(val: u8) -> Req37En1 {
        Req37En1::from_bits(val)
    }
}
impl From<Req37En1> for u8 {
    #[inline(always)]
    fn from(val: Req37En1) -> u8 {
        Req37En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req38En0 {
    #[doc = "Disable"]
    VALUE74 = 0x0,
    #[doc = "Enable"]
    VALUE75 = 0x01,
}
impl Req38En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req38En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req38En0 {
    #[inline(always)]
    fn from(val: u8) -> Req38En0 {
        Req38En0::from_bits(val)
    }
}
impl From<Req38En0> for u8 {
    #[inline(always)]
    fn from(val: Req38En0) -> u8 {
        Req38En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req38En1 {
    #[doc = "Disable"]
    VALUE74 = 0x0,
    #[doc = "Enable"]
    VALUE75 = 0x01,
}
impl Req38En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req38En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req38En1 {
    #[inline(always)]
    fn from(val: u8) -> Req38En1 {
        Req38En1::from_bits(val)
    }
}
impl From<Req38En1> for u8 {
    #[inline(always)]
    fn from(val: Req38En1) -> u8 {
        Req38En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req39En0 {
    #[doc = "Disable"]
    VALUE76 = 0x0,
    #[doc = "Enable"]
    VALUE77 = 0x01,
}
impl Req39En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req39En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req39En0 {
    #[inline(always)]
    fn from(val: u8) -> Req39En0 {
        Req39En0::from_bits(val)
    }
}
impl From<Req39En0> for u8 {
    #[inline(always)]
    fn from(val: Req39En0) -> u8 {
        Req39En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req39En1 {
    #[doc = "Disable"]
    VALUE76 = 0x0,
    #[doc = "Enable"]
    VALUE77 = 0x01,
}
impl Req39En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req39En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req39En1 {
    #[inline(always)]
    fn from(val: u8) -> Req39En1 {
        Req39En1::from_bits(val)
    }
}
impl From<Req39En1> for u8 {
    #[inline(always)]
    fn from(val: Req39En1) -> u8 {
        Req39En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req3En0 {
    #[doc = "Disable"]
    VALUE4 = 0x0,
    #[doc = "Enable"]
    VALUE5 = 0x01,
}
impl Req3En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req3En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req3En0 {
    #[inline(always)]
    fn from(val: u8) -> Req3En0 {
        Req3En0::from_bits(val)
    }
}
impl From<Req3En0> for u8 {
    #[inline(always)]
    fn from(val: Req3En0) -> u8 {
        Req3En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req3En1 {
    #[doc = "Disable"]
    VALUE4 = 0x0,
    #[doc = "Enable"]
    VALUE5 = 0x01,
}
impl Req3En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req3En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req3En1 {
    #[inline(always)]
    fn from(val: u8) -> Req3En1 {
        Req3En1::from_bits(val)
    }
}
impl From<Req3En1> for u8 {
    #[inline(always)]
    fn from(val: Req3En1) -> u8 {
        Req3En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req40En0 {
    #[doc = "Disable"]
    VALUE78 = 0x0,
    #[doc = "Enable"]
    VALUE79 = 0x01,
}
impl Req40En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req40En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req40En0 {
    #[inline(always)]
    fn from(val: u8) -> Req40En0 {
        Req40En0::from_bits(val)
    }
}
impl From<Req40En0> for u8 {
    #[inline(always)]
    fn from(val: Req40En0) -> u8 {
        Req40En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req40En1 {
    #[doc = "Disable"]
    VALUE78 = 0x0,
    #[doc = "Enable"]
    VALUE79 = 0x01,
}
impl Req40En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req40En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req40En1 {
    #[inline(always)]
    fn from(val: u8) -> Req40En1 {
        Req40En1::from_bits(val)
    }
}
impl From<Req40En1> for u8 {
    #[inline(always)]
    fn from(val: Req40En1) -> u8 {
        Req40En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req41En0 {
    #[doc = "Disable"]
    VALUE80 = 0x0,
    #[doc = "Enable"]
    VALUE81 = 0x01,
}
impl Req41En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req41En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req41En0 {
    #[inline(always)]
    fn from(val: u8) -> Req41En0 {
        Req41En0::from_bits(val)
    }
}
impl From<Req41En0> for u8 {
    #[inline(always)]
    fn from(val: Req41En0) -> u8 {
        Req41En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req41En1 {
    #[doc = "Disable"]
    VALUE80 = 0x0,
    #[doc = "Enable"]
    VALUE81 = 0x01,
}
impl Req41En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req41En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req41En1 {
    #[inline(always)]
    fn from(val: u8) -> Req41En1 {
        Req41En1::from_bits(val)
    }
}
impl From<Req41En1> for u8 {
    #[inline(always)]
    fn from(val: Req41En1) -> u8 {
        Req41En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req42En0 {
    #[doc = "Disable"]
    VALUE82 = 0x0,
    #[doc = "Enable"]
    VALUE83 = 0x01,
}
impl Req42En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req42En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req42En0 {
    #[inline(always)]
    fn from(val: u8) -> Req42En0 {
        Req42En0::from_bits(val)
    }
}
impl From<Req42En0> for u8 {
    #[inline(always)]
    fn from(val: Req42En0) -> u8 {
        Req42En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req42En1 {
    #[doc = "Disable"]
    VALUE82 = 0x0,
    #[doc = "Enable"]
    VALUE83 = 0x01,
}
impl Req42En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req42En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req42En1 {
    #[inline(always)]
    fn from(val: u8) -> Req42En1 {
        Req42En1::from_bits(val)
    }
}
impl From<Req42En1> for u8 {
    #[inline(always)]
    fn from(val: Req42En1) -> u8 {
        Req42En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req43En0 {
    #[doc = "Disable"]
    VALUE84 = 0x0,
    #[doc = "Enable"]
    VALUE85 = 0x01,
}
impl Req43En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req43En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req43En0 {
    #[inline(always)]
    fn from(val: u8) -> Req43En0 {
        Req43En0::from_bits(val)
    }
}
impl From<Req43En0> for u8 {
    #[inline(always)]
    fn from(val: Req43En0) -> u8 {
        Req43En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req43En1 {
    #[doc = "Disable"]
    VALUE84 = 0x0,
    #[doc = "Enable"]
    VALUE85 = 0x01,
}
impl Req43En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req43En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req43En1 {
    #[inline(always)]
    fn from(val: u8) -> Req43En1 {
        Req43En1::from_bits(val)
    }
}
impl From<Req43En1> for u8 {
    #[inline(always)]
    fn from(val: Req43En1) -> u8 {
        Req43En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req44En0 {
    #[doc = "Disable"]
    VALUE86 = 0x0,
    #[doc = "Enable"]
    VALUE87 = 0x01,
}
impl Req44En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req44En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req44En0 {
    #[inline(always)]
    fn from(val: u8) -> Req44En0 {
        Req44En0::from_bits(val)
    }
}
impl From<Req44En0> for u8 {
    #[inline(always)]
    fn from(val: Req44En0) -> u8 {
        Req44En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req44En1 {
    #[doc = "Disable"]
    VALUE86 = 0x0,
    #[doc = "Enable"]
    VALUE87 = 0x01,
}
impl Req44En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req44En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req44En1 {
    #[inline(always)]
    fn from(val: u8) -> Req44En1 {
        Req44En1::from_bits(val)
    }
}
impl From<Req44En1> for u8 {
    #[inline(always)]
    fn from(val: Req44En1) -> u8 {
        Req44En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req45En0 {
    #[doc = "Disable"]
    VALUE88 = 0x0,
    #[doc = "Enable"]
    VALUE89 = 0x01,
}
impl Req45En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req45En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req45En0 {
    #[inline(always)]
    fn from(val: u8) -> Req45En0 {
        Req45En0::from_bits(val)
    }
}
impl From<Req45En0> for u8 {
    #[inline(always)]
    fn from(val: Req45En0) -> u8 {
        Req45En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req45En1 {
    #[doc = "Disable"]
    VALUE88 = 0x0,
    #[doc = "Enable"]
    VALUE89 = 0x01,
}
impl Req45En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req45En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req45En1 {
    #[inline(always)]
    fn from(val: u8) -> Req45En1 {
        Req45En1::from_bits(val)
    }
}
impl From<Req45En1> for u8 {
    #[inline(always)]
    fn from(val: Req45En1) -> u8 {
        Req45En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req46En0 {
    #[doc = "Disable"]
    VALUE90 = 0x0,
    #[doc = "Enable"]
    VALUE91 = 0x01,
}
impl Req46En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req46En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req46En0 {
    #[inline(always)]
    fn from(val: u8) -> Req46En0 {
        Req46En0::from_bits(val)
    }
}
impl From<Req46En0> for u8 {
    #[inline(always)]
    fn from(val: Req46En0) -> u8 {
        Req46En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req46En1 {
    #[doc = "Disable"]
    VALUE90 = 0x0,
    #[doc = "Enable"]
    VALUE91 = 0x01,
}
impl Req46En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req46En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req46En1 {
    #[inline(always)]
    fn from(val: u8) -> Req46En1 {
        Req46En1::from_bits(val)
    }
}
impl From<Req46En1> for u8 {
    #[inline(always)]
    fn from(val: Req46En1) -> u8 {
        Req46En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req47En0 {
    #[doc = "Disable"]
    VALUE92 = 0x0,
    #[doc = "Enable"]
    VALUE93 = 0x01,
}
impl Req47En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req47En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req47En0 {
    #[inline(always)]
    fn from(val: u8) -> Req47En0 {
        Req47En0::from_bits(val)
    }
}
impl From<Req47En0> for u8 {
    #[inline(always)]
    fn from(val: Req47En0) -> u8 {
        Req47En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req47En1 {
    #[doc = "Disable"]
    VALUE92 = 0x0,
    #[doc = "Enable"]
    VALUE93 = 0x01,
}
impl Req47En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req47En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req47En1 {
    #[inline(always)]
    fn from(val: u8) -> Req47En1 {
        Req47En1::from_bits(val)
    }
}
impl From<Req47En1> for u8 {
    #[inline(always)]
    fn from(val: Req47En1) -> u8 {
        Req47En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req48En0 {
    #[doc = "Disable"]
    VALUE94 = 0x0,
    #[doc = "Enable"]
    VALUE95 = 0x01,
}
impl Req48En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req48En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req48En0 {
    #[inline(always)]
    fn from(val: u8) -> Req48En0 {
        Req48En0::from_bits(val)
    }
}
impl From<Req48En0> for u8 {
    #[inline(always)]
    fn from(val: Req48En0) -> u8 {
        Req48En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req48En1 {
    #[doc = "Disable"]
    VALUE94 = 0x0,
    #[doc = "Enable"]
    VALUE95 = 0x01,
}
impl Req48En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req48En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req48En1 {
    #[inline(always)]
    fn from(val: u8) -> Req48En1 {
        Req48En1::from_bits(val)
    }
}
impl From<Req48En1> for u8 {
    #[inline(always)]
    fn from(val: Req48En1) -> u8 {
        Req48En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req49En0 {
    #[doc = "Disable"]
    VALUE96 = 0x0,
    #[doc = "Enable"]
    VALUE97 = 0x01,
}
impl Req49En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req49En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req49En0 {
    #[inline(always)]
    fn from(val: u8) -> Req49En0 {
        Req49En0::from_bits(val)
    }
}
impl From<Req49En0> for u8 {
    #[inline(always)]
    fn from(val: Req49En0) -> u8 {
        Req49En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req49En1 {
    #[doc = "Disable"]
    VALUE96 = 0x0,
    #[doc = "Enable"]
    VALUE97 = 0x01,
}
impl Req49En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req49En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req49En1 {
    #[inline(always)]
    fn from(val: u8) -> Req49En1 {
        Req49En1::from_bits(val)
    }
}
impl From<Req49En1> for u8 {
    #[inline(always)]
    fn from(val: Req49En1) -> u8 {
        Req49En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req4En0 {
    #[doc = "Disable"]
    VALUE6 = 0x0,
    #[doc = "Enable"]
    VALUE7 = 0x01,
}
impl Req4En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req4En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req4En0 {
    #[inline(always)]
    fn from(val: u8) -> Req4En0 {
        Req4En0::from_bits(val)
    }
}
impl From<Req4En0> for u8 {
    #[inline(always)]
    fn from(val: Req4En0) -> u8 {
        Req4En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req4En1 {
    #[doc = "Disable"]
    VALUE6 = 0x0,
    #[doc = "Enable"]
    VALUE7 = 0x01,
}
impl Req4En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req4En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req4En1 {
    #[inline(always)]
    fn from(val: u8) -> Req4En1 {
        Req4En1::from_bits(val)
    }
}
impl From<Req4En1> for u8 {
    #[inline(always)]
    fn from(val: Req4En1) -> u8 {
        Req4En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req50En0 {
    #[doc = "Disable"]
    VALUE98 = 0x0,
    #[doc = "Enable"]
    VALUE99 = 0x01,
}
impl Req50En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req50En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req50En0 {
    #[inline(always)]
    fn from(val: u8) -> Req50En0 {
        Req50En0::from_bits(val)
    }
}
impl From<Req50En0> for u8 {
    #[inline(always)]
    fn from(val: Req50En0) -> u8 {
        Req50En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req50En1 {
    #[doc = "Disable"]
    VALUE98 = 0x0,
    #[doc = "Enable"]
    VALUE99 = 0x01,
}
impl Req50En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req50En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req50En1 {
    #[inline(always)]
    fn from(val: u8) -> Req50En1 {
        Req50En1::from_bits(val)
    }
}
impl From<Req50En1> for u8 {
    #[inline(always)]
    fn from(val: Req50En1) -> u8 {
        Req50En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req51En0 {
    #[doc = "Disable"]
    VALUE100 = 0x0,
    #[doc = "Enable"]
    VALUE101 = 0x01,
}
impl Req51En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req51En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req51En0 {
    #[inline(always)]
    fn from(val: u8) -> Req51En0 {
        Req51En0::from_bits(val)
    }
}
impl From<Req51En0> for u8 {
    #[inline(always)]
    fn from(val: Req51En0) -> u8 {
        Req51En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req51En1 {
    #[doc = "Disable"]
    VALUE100 = 0x0,
    #[doc = "Enable"]
    VALUE101 = 0x01,
}
impl Req51En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req51En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req51En1 {
    #[inline(always)]
    fn from(val: u8) -> Req51En1 {
        Req51En1::from_bits(val)
    }
}
impl From<Req51En1> for u8 {
    #[inline(always)]
    fn from(val: Req51En1) -> u8 {
        Req51En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req52En0 {
    #[doc = "Disable"]
    VALUE102 = 0x0,
    #[doc = "Enable"]
    VALUE103 = 0x01,
}
impl Req52En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req52En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req52En0 {
    #[inline(always)]
    fn from(val: u8) -> Req52En0 {
        Req52En0::from_bits(val)
    }
}
impl From<Req52En0> for u8 {
    #[inline(always)]
    fn from(val: Req52En0) -> u8 {
        Req52En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req52En1 {
    #[doc = "Disable"]
    VALUE102 = 0x0,
    #[doc = "Enable"]
    VALUE103 = 0x01,
}
impl Req52En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req52En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req52En1 {
    #[inline(always)]
    fn from(val: u8) -> Req52En1 {
        Req52En1::from_bits(val)
    }
}
impl From<Req52En1> for u8 {
    #[inline(always)]
    fn from(val: Req52En1) -> u8 {
        Req52En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req53En0 {
    #[doc = "Disable"]
    VALUE104 = 0x0,
    #[doc = "Enable"]
    VALUE105 = 0x01,
}
impl Req53En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req53En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req53En0 {
    #[inline(always)]
    fn from(val: u8) -> Req53En0 {
        Req53En0::from_bits(val)
    }
}
impl From<Req53En0> for u8 {
    #[inline(always)]
    fn from(val: Req53En0) -> u8 {
        Req53En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req53En1 {
    #[doc = "Disable"]
    VALUE104 = 0x0,
    #[doc = "Enable"]
    VALUE105 = 0x01,
}
impl Req53En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req53En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req53En1 {
    #[inline(always)]
    fn from(val: u8) -> Req53En1 {
        Req53En1::from_bits(val)
    }
}
impl From<Req53En1> for u8 {
    #[inline(always)]
    fn from(val: Req53En1) -> u8 {
        Req53En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req54En0 {
    #[doc = "Disable"]
    VALUE106 = 0x0,
    #[doc = "Enable"]
    VALUE107 = 0x01,
}
impl Req54En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req54En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req54En0 {
    #[inline(always)]
    fn from(val: u8) -> Req54En0 {
        Req54En0::from_bits(val)
    }
}
impl From<Req54En0> for u8 {
    #[inline(always)]
    fn from(val: Req54En0) -> u8 {
        Req54En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req54En1 {
    #[doc = "Disable"]
    VALUE106 = 0x0,
    #[doc = "Enable"]
    VALUE107 = 0x01,
}
impl Req54En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req54En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req54En1 {
    #[inline(always)]
    fn from(val: u8) -> Req54En1 {
        Req54En1::from_bits(val)
    }
}
impl From<Req54En1> for u8 {
    #[inline(always)]
    fn from(val: Req54En1) -> u8 {
        Req54En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req57En0 {
    #[doc = "Disable"]
    VALUE112 = 0x0,
    #[doc = "Enable"]
    VALUE113 = 0x01,
}
impl Req57En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req57En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req57En0 {
    #[inline(always)]
    fn from(val: u8) -> Req57En0 {
        Req57En0::from_bits(val)
    }
}
impl From<Req57En0> for u8 {
    #[inline(always)]
    fn from(val: Req57En0) -> u8 {
        Req57En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req57En1 {
    #[doc = "Disable"]
    VALUE112 = 0x0,
    #[doc = "Enable"]
    VALUE113 = 0x01,
}
impl Req57En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req57En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req57En1 {
    #[inline(always)]
    fn from(val: u8) -> Req57En1 {
        Req57En1::from_bits(val)
    }
}
impl From<Req57En1> for u8 {
    #[inline(always)]
    fn from(val: Req57En1) -> u8 {
        Req57En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req58En0 {
    #[doc = "Disable"]
    VALUE114 = 0x0,
    #[doc = "Enable"]
    VALUE115 = 0x01,
}
impl Req58En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req58En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req58En0 {
    #[inline(always)]
    fn from(val: u8) -> Req58En0 {
        Req58En0::from_bits(val)
    }
}
impl From<Req58En0> for u8 {
    #[inline(always)]
    fn from(val: Req58En0) -> u8 {
        Req58En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req58En1 {
    #[doc = "Disable"]
    VALUE114 = 0x0,
    #[doc = "Enable"]
    VALUE115 = 0x01,
}
impl Req58En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req58En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req58En1 {
    #[inline(always)]
    fn from(val: u8) -> Req58En1 {
        Req58En1::from_bits(val)
    }
}
impl From<Req58En1> for u8 {
    #[inline(always)]
    fn from(val: Req58En1) -> u8 {
        Req58En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req59En0 {
    #[doc = "Disable"]
    VALUE116 = 0x0,
    #[doc = "Enable"]
    VALUE117 = 0x01,
}
impl Req59En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req59En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req59En0 {
    #[inline(always)]
    fn from(val: u8) -> Req59En0 {
        Req59En0::from_bits(val)
    }
}
impl From<Req59En0> for u8 {
    #[inline(always)]
    fn from(val: Req59En0) -> u8 {
        Req59En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req59En1 {
    #[doc = "Disable"]
    VALUE116 = 0x0,
    #[doc = "Enable"]
    VALUE117 = 0x01,
}
impl Req59En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req59En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req59En1 {
    #[inline(always)]
    fn from(val: u8) -> Req59En1 {
        Req59En1::from_bits(val)
    }
}
impl From<Req59En1> for u8 {
    #[inline(always)]
    fn from(val: Req59En1) -> u8 {
        Req59En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req5En0 {
    #[doc = "Disable"]
    VALUE8 = 0x0,
    #[doc = "Enable"]
    VALUE9 = 0x01,
}
impl Req5En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req5En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req5En0 {
    #[inline(always)]
    fn from(val: u8) -> Req5En0 {
        Req5En0::from_bits(val)
    }
}
impl From<Req5En0> for u8 {
    #[inline(always)]
    fn from(val: Req5En0) -> u8 {
        Req5En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req5En1 {
    #[doc = "Disable"]
    VALUE8 = 0x0,
    #[doc = "Enable"]
    VALUE9 = 0x01,
}
impl Req5En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req5En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req5En1 {
    #[inline(always)]
    fn from(val: u8) -> Req5En1 {
        Req5En1::from_bits(val)
    }
}
impl From<Req5En1> for u8 {
    #[inline(always)]
    fn from(val: Req5En1) -> u8 {
        Req5En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req60En0 {
    #[doc = "Disable"]
    VALUE118 = 0x0,
    #[doc = "Enable"]
    VALUE119 = 0x01,
}
impl Req60En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req60En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req60En0 {
    #[inline(always)]
    fn from(val: u8) -> Req60En0 {
        Req60En0::from_bits(val)
    }
}
impl From<Req60En0> for u8 {
    #[inline(always)]
    fn from(val: Req60En0) -> u8 {
        Req60En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req60En1 {
    #[doc = "Disable"]
    VALUE118 = 0x0,
    #[doc = "Enable"]
    VALUE119 = 0x01,
}
impl Req60En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req60En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req60En1 {
    #[inline(always)]
    fn from(val: u8) -> Req60En1 {
        Req60En1::from_bits(val)
    }
}
impl From<Req60En1> for u8 {
    #[inline(always)]
    fn from(val: Req60En1) -> u8 {
        Req60En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req61En0 {
    #[doc = "Disable"]
    VALUE120 = 0x0,
    #[doc = "Enable"]
    VALUE121 = 0x01,
}
impl Req61En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req61En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req61En0 {
    #[inline(always)]
    fn from(val: u8) -> Req61En0 {
        Req61En0::from_bits(val)
    }
}
impl From<Req61En0> for u8 {
    #[inline(always)]
    fn from(val: Req61En0) -> u8 {
        Req61En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req61En1 {
    #[doc = "Disable"]
    VALUE120 = 0x0,
    #[doc = "Enable"]
    VALUE121 = 0x01,
}
impl Req61En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req61En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req61En1 {
    #[inline(always)]
    fn from(val: u8) -> Req61En1 {
        Req61En1::from_bits(val)
    }
}
impl From<Req61En1> for u8 {
    #[inline(always)]
    fn from(val: Req61En1) -> u8 {
        Req61En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req62En0 {
    #[doc = "Disable"]
    VALUE122 = 0x0,
    #[doc = "Enable"]
    VALUE123 = 0x01,
}
impl Req62En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req62En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req62En0 {
    #[inline(always)]
    fn from(val: u8) -> Req62En0 {
        Req62En0::from_bits(val)
    }
}
impl From<Req62En0> for u8 {
    #[inline(always)]
    fn from(val: Req62En0) -> u8 {
        Req62En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req62En1 {
    #[doc = "Disable"]
    VALUE122 = 0x0,
    #[doc = "Enable"]
    VALUE123 = 0x01,
}
impl Req62En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req62En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req62En1 {
    #[inline(always)]
    fn from(val: u8) -> Req62En1 {
        Req62En1::from_bits(val)
    }
}
impl From<Req62En1> for u8 {
    #[inline(always)]
    fn from(val: Req62En1) -> u8 {
        Req62En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req63En0 {
    #[doc = "Disable"]
    VALUE124 = 0x0,
    #[doc = "Enable"]
    VALUE125 = 0x01,
}
impl Req63En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req63En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req63En0 {
    #[inline(always)]
    fn from(val: u8) -> Req63En0 {
        Req63En0::from_bits(val)
    }
}
impl From<Req63En0> for u8 {
    #[inline(always)]
    fn from(val: Req63En0) -> u8 {
        Req63En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req63En1 {
    #[doc = "Disable"]
    VALUE124 = 0x0,
    #[doc = "Enable"]
    VALUE125 = 0x01,
}
impl Req63En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req63En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req63En1 {
    #[inline(always)]
    fn from(val: u8) -> Req63En1 {
        Req63En1::from_bits(val)
    }
}
impl From<Req63En1> for u8 {
    #[inline(always)]
    fn from(val: Req63En1) -> u8 {
        Req63En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req64En0 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req64En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req64En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req64En0 {
    #[inline(always)]
    fn from(val: u8) -> Req64En0 {
        Req64En0::from_bits(val)
    }
}
impl From<Req64En0> for u8 {
    #[inline(always)]
    fn from(val: Req64En0) -> u8 {
        Req64En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req64En1 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req64En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req64En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req64En1 {
    #[inline(always)]
    fn from(val: u8) -> Req64En1 {
        Req64En1::from_bits(val)
    }
}
impl From<Req64En1> for u8 {
    #[inline(always)]
    fn from(val: Req64En1) -> u8 {
        Req64En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req65En0 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req65En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req65En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req65En0 {
    #[inline(always)]
    fn from(val: u8) -> Req65En0 {
        Req65En0::from_bits(val)
    }
}
impl From<Req65En0> for u8 {
    #[inline(always)]
    fn from(val: Req65En0) -> u8 {
        Req65En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req65En1 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req65En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req65En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req65En1 {
    #[inline(always)]
    fn from(val: u8) -> Req65En1 {
        Req65En1::from_bits(val)
    }
}
impl From<Req65En1> for u8 {
    #[inline(always)]
    fn from(val: Req65En1) -> u8 {
        Req65En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req66En0 {
    #[doc = "Disable"]
    VALUE2 = 0x0,
    #[doc = "Enable"]
    VALUE3 = 0x01,
}
impl Req66En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req66En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req66En0 {
    #[inline(always)]
    fn from(val: u8) -> Req66En0 {
        Req66En0::from_bits(val)
    }
}
impl From<Req66En0> for u8 {
    #[inline(always)]
    fn from(val: Req66En0) -> u8 {
        Req66En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req66En1 {
    #[doc = "Disable"]
    VALUE2 = 0x0,
    #[doc = "Enable"]
    VALUE3 = 0x01,
}
impl Req66En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req66En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req66En1 {
    #[inline(always)]
    fn from(val: u8) -> Req66En1 {
        Req66En1::from_bits(val)
    }
}
impl From<Req66En1> for u8 {
    #[inline(always)]
    fn from(val: Req66En1) -> u8 {
        Req66En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req67En0 {
    #[doc = "Disable"]
    VALUE4 = 0x0,
    #[doc = "Enable"]
    VALUE5 = 0x01,
}
impl Req67En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req67En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req67En0 {
    #[inline(always)]
    fn from(val: u8) -> Req67En0 {
        Req67En0::from_bits(val)
    }
}
impl From<Req67En0> for u8 {
    #[inline(always)]
    fn from(val: Req67En0) -> u8 {
        Req67En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req67En1 {
    #[doc = "Disable"]
    VALUE4 = 0x0,
    #[doc = "Enable"]
    VALUE5 = 0x01,
}
impl Req67En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req67En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req67En1 {
    #[inline(always)]
    fn from(val: u8) -> Req67En1 {
        Req67En1::from_bits(val)
    }
}
impl From<Req67En1> for u8 {
    #[inline(always)]
    fn from(val: Req67En1) -> u8 {
        Req67En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req68En0 {
    #[doc = "Disable"]
    VALUE6 = 0x0,
    #[doc = "Enable"]
    VALUE7 = 0x01,
}
impl Req68En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req68En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req68En0 {
    #[inline(always)]
    fn from(val: u8) -> Req68En0 {
        Req68En0::from_bits(val)
    }
}
impl From<Req68En0> for u8 {
    #[inline(always)]
    fn from(val: Req68En0) -> u8 {
        Req68En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req68En1 {
    #[doc = "Disable"]
    VALUE6 = 0x0,
    #[doc = "Enable"]
    VALUE7 = 0x01,
}
impl Req68En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req68En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req68En1 {
    #[inline(always)]
    fn from(val: u8) -> Req68En1 {
        Req68En1::from_bits(val)
    }
}
impl From<Req68En1> for u8 {
    #[inline(always)]
    fn from(val: Req68En1) -> u8 {
        Req68En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req69En0 {
    #[doc = "Disable"]
    VALUE8 = 0x0,
    #[doc = "Enable"]
    VALUE9 = 0x01,
}
impl Req69En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req69En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req69En0 {
    #[inline(always)]
    fn from(val: u8) -> Req69En0 {
        Req69En0::from_bits(val)
    }
}
impl From<Req69En0> for u8 {
    #[inline(always)]
    fn from(val: Req69En0) -> u8 {
        Req69En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req69En1 {
    #[doc = "Disable"]
    VALUE8 = 0x0,
    #[doc = "Enable"]
    VALUE9 = 0x01,
}
impl Req69En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req69En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req69En1 {
    #[inline(always)]
    fn from(val: u8) -> Req69En1 {
        Req69En1::from_bits(val)
    }
}
impl From<Req69En1> for u8 {
    #[inline(always)]
    fn from(val: Req69En1) -> u8 {
        Req69En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req6En0 {
    #[doc = "Disable"]
    VALUE10 = 0x0,
    #[doc = "Enable"]
    VALUE11 = 0x01,
}
impl Req6En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req6En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req6En0 {
    #[inline(always)]
    fn from(val: u8) -> Req6En0 {
        Req6En0::from_bits(val)
    }
}
impl From<Req6En0> for u8 {
    #[inline(always)]
    fn from(val: Req6En0) -> u8 {
        Req6En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req6En1 {
    #[doc = "Disable"]
    VALUE10 = 0x0,
    #[doc = "Enable"]
    VALUE11 = 0x01,
}
impl Req6En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req6En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req6En1 {
    #[inline(always)]
    fn from(val: u8) -> Req6En1 {
        Req6En1::from_bits(val)
    }
}
impl From<Req6En1> for u8 {
    #[inline(always)]
    fn from(val: Req6En1) -> u8 {
        Req6En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req70En0 {
    #[doc = "Disable"]
    VALUE10 = 0x0,
    #[doc = "Enable"]
    VALUE11 = 0x01,
}
impl Req70En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req70En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req70En0 {
    #[inline(always)]
    fn from(val: u8) -> Req70En0 {
        Req70En0::from_bits(val)
    }
}
impl From<Req70En0> for u8 {
    #[inline(always)]
    fn from(val: Req70En0) -> u8 {
        Req70En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req70En1 {
    #[doc = "Disable"]
    VALUE10 = 0x0,
    #[doc = "Enable"]
    VALUE11 = 0x01,
}
impl Req70En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req70En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req70En1 {
    #[inline(always)]
    fn from(val: u8) -> Req70En1 {
        Req70En1::from_bits(val)
    }
}
impl From<Req70En1> for u8 {
    #[inline(always)]
    fn from(val: Req70En1) -> u8 {
        Req70En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req71En0 {
    #[doc = "Disable"]
    VALUE12 = 0x0,
    #[doc = "Enable"]
    VALUE13 = 0x01,
}
impl Req71En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req71En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req71En0 {
    #[inline(always)]
    fn from(val: u8) -> Req71En0 {
        Req71En0::from_bits(val)
    }
}
impl From<Req71En0> for u8 {
    #[inline(always)]
    fn from(val: Req71En0) -> u8 {
        Req71En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req71En1 {
    #[doc = "Disable"]
    VALUE12 = 0x0,
    #[doc = "Enable"]
    VALUE13 = 0x01,
}
impl Req71En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req71En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req71En1 {
    #[inline(always)]
    fn from(val: u8) -> Req71En1 {
        Req71En1::from_bits(val)
    }
}
impl From<Req71En1> for u8 {
    #[inline(always)]
    fn from(val: Req71En1) -> u8 {
        Req71En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req72En0 {
    #[doc = "Disable"]
    VALUE14 = 0x0,
    #[doc = "Enable"]
    VALUE15 = 0x01,
}
impl Req72En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req72En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req72En0 {
    #[inline(always)]
    fn from(val: u8) -> Req72En0 {
        Req72En0::from_bits(val)
    }
}
impl From<Req72En0> for u8 {
    #[inline(always)]
    fn from(val: Req72En0) -> u8 {
        Req72En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req72En1 {
    #[doc = "Disable"]
    VALUE14 = 0x0,
    #[doc = "Enable"]
    VALUE15 = 0x01,
}
impl Req72En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req72En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req72En1 {
    #[inline(always)]
    fn from(val: u8) -> Req72En1 {
        Req72En1::from_bits(val)
    }
}
impl From<Req72En1> for u8 {
    #[inline(always)]
    fn from(val: Req72En1) -> u8 {
        Req72En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req73En0 {
    #[doc = "Disable"]
    VALUE16 = 0x0,
    #[doc = "Enable"]
    VALUE17 = 0x01,
}
impl Req73En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req73En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req73En0 {
    #[inline(always)]
    fn from(val: u8) -> Req73En0 {
        Req73En0::from_bits(val)
    }
}
impl From<Req73En0> for u8 {
    #[inline(always)]
    fn from(val: Req73En0) -> u8 {
        Req73En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req73En1 {
    #[doc = "Disable"]
    VALUE16 = 0x0,
    #[doc = "Enable"]
    VALUE17 = 0x01,
}
impl Req73En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req73En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req73En1 {
    #[inline(always)]
    fn from(val: u8) -> Req73En1 {
        Req73En1::from_bits(val)
    }
}
impl From<Req73En1> for u8 {
    #[inline(always)]
    fn from(val: Req73En1) -> u8 {
        Req73En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req74En0 {
    #[doc = "Disable"]
    VALUE18 = 0x0,
    #[doc = "Enable"]
    VALUE19 = 0x01,
}
impl Req74En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req74En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req74En0 {
    #[inline(always)]
    fn from(val: u8) -> Req74En0 {
        Req74En0::from_bits(val)
    }
}
impl From<Req74En0> for u8 {
    #[inline(always)]
    fn from(val: Req74En0) -> u8 {
        Req74En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req74En1 {
    #[doc = "Disable"]
    VALUE18 = 0x0,
    #[doc = "Enable"]
    VALUE19 = 0x01,
}
impl Req74En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req74En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req74En1 {
    #[inline(always)]
    fn from(val: u8) -> Req74En1 {
        Req74En1::from_bits(val)
    }
}
impl From<Req74En1> for u8 {
    #[inline(always)]
    fn from(val: Req74En1) -> u8 {
        Req74En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req75En0 {
    #[doc = "Disable"]
    VALUE20 = 0x0,
    #[doc = "Enable"]
    VALUE21 = 0x01,
}
impl Req75En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req75En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req75En0 {
    #[inline(always)]
    fn from(val: u8) -> Req75En0 {
        Req75En0::from_bits(val)
    }
}
impl From<Req75En0> for u8 {
    #[inline(always)]
    fn from(val: Req75En0) -> u8 {
        Req75En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req75En1 {
    #[doc = "Disable"]
    VALUE20 = 0x0,
    #[doc = "Enable"]
    VALUE21 = 0x01,
}
impl Req75En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req75En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req75En1 {
    #[inline(always)]
    fn from(val: u8) -> Req75En1 {
        Req75En1::from_bits(val)
    }
}
impl From<Req75En1> for u8 {
    #[inline(always)]
    fn from(val: Req75En1) -> u8 {
        Req75En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req76En0 {
    #[doc = "Disable"]
    VALUE22 = 0x0,
    #[doc = "Enable"]
    VALUE23 = 0x01,
}
impl Req76En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req76En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req76En0 {
    #[inline(always)]
    fn from(val: u8) -> Req76En0 {
        Req76En0::from_bits(val)
    }
}
impl From<Req76En0> for u8 {
    #[inline(always)]
    fn from(val: Req76En0) -> u8 {
        Req76En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req76En1 {
    #[doc = "Disable"]
    VALUE22 = 0x0,
    #[doc = "Enable"]
    VALUE23 = 0x01,
}
impl Req76En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req76En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req76En1 {
    #[inline(always)]
    fn from(val: u8) -> Req76En1 {
        Req76En1::from_bits(val)
    }
}
impl From<Req76En1> for u8 {
    #[inline(always)]
    fn from(val: Req76En1) -> u8 {
        Req76En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req77En0 {
    #[doc = "Disable"]
    VALUE24 = 0x0,
    #[doc = "Enable"]
    VALUE25 = 0x01,
}
impl Req77En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req77En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req77En0 {
    #[inline(always)]
    fn from(val: u8) -> Req77En0 {
        Req77En0::from_bits(val)
    }
}
impl From<Req77En0> for u8 {
    #[inline(always)]
    fn from(val: Req77En0) -> u8 {
        Req77En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req77En1 {
    #[doc = "Disable"]
    VALUE24 = 0x0,
    #[doc = "Enable"]
    VALUE25 = 0x01,
}
impl Req77En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req77En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req77En1 {
    #[inline(always)]
    fn from(val: u8) -> Req77En1 {
        Req77En1::from_bits(val)
    }
}
impl From<Req77En1> for u8 {
    #[inline(always)]
    fn from(val: Req77En1) -> u8 {
        Req77En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req78En0 {
    #[doc = "Disable"]
    VALUE26 = 0x0,
    #[doc = "Enable"]
    VALUE27 = 0x01,
}
impl Req78En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req78En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req78En0 {
    #[inline(always)]
    fn from(val: u8) -> Req78En0 {
        Req78En0::from_bits(val)
    }
}
impl From<Req78En0> for u8 {
    #[inline(always)]
    fn from(val: Req78En0) -> u8 {
        Req78En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req78En1 {
    #[doc = "Disable"]
    VALUE26 = 0x0,
    #[doc = "Enable"]
    VALUE27 = 0x01,
}
impl Req78En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req78En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req78En1 {
    #[inline(always)]
    fn from(val: u8) -> Req78En1 {
        Req78En1::from_bits(val)
    }
}
impl From<Req78En1> for u8 {
    #[inline(always)]
    fn from(val: Req78En1) -> u8 {
        Req78En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req79En0 {
    #[doc = "Disable"]
    VALUE28 = 0x0,
    #[doc = "Enable"]
    VALUE29 = 0x01,
}
impl Req79En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req79En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req79En0 {
    #[inline(always)]
    fn from(val: u8) -> Req79En0 {
        Req79En0::from_bits(val)
    }
}
impl From<Req79En0> for u8 {
    #[inline(always)]
    fn from(val: Req79En0) -> u8 {
        Req79En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req79En1 {
    #[doc = "Disable"]
    VALUE28 = 0x0,
    #[doc = "Enable"]
    VALUE29 = 0x01,
}
impl Req79En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req79En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req79En1 {
    #[inline(always)]
    fn from(val: u8) -> Req79En1 {
        Req79En1::from_bits(val)
    }
}
impl From<Req79En1> for u8 {
    #[inline(always)]
    fn from(val: Req79En1) -> u8 {
        Req79En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req7En0 {
    #[doc = "Disable"]
    VALUE12 = 0x0,
    #[doc = "Enable"]
    VALUE13 = 0x01,
}
impl Req7En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req7En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req7En0 {
    #[inline(always)]
    fn from(val: u8) -> Req7En0 {
        Req7En0::from_bits(val)
    }
}
impl From<Req7En0> for u8 {
    #[inline(always)]
    fn from(val: Req7En0) -> u8 {
        Req7En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req7En1 {
    #[doc = "Disable"]
    VALUE12 = 0x0,
    #[doc = "Enable"]
    VALUE13 = 0x01,
}
impl Req7En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req7En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req7En1 {
    #[inline(always)]
    fn from(val: u8) -> Req7En1 {
        Req7En1::from_bits(val)
    }
}
impl From<Req7En1> for u8 {
    #[inline(always)]
    fn from(val: Req7En1) -> u8 {
        Req7En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req80En0 {
    #[doc = "Disable"]
    VALUE30 = 0x0,
    #[doc = "Enable"]
    VALUE31 = 0x01,
}
impl Req80En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req80En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req80En0 {
    #[inline(always)]
    fn from(val: u8) -> Req80En0 {
        Req80En0::from_bits(val)
    }
}
impl From<Req80En0> for u8 {
    #[inline(always)]
    fn from(val: Req80En0) -> u8 {
        Req80En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req80En1 {
    #[doc = "Disable"]
    VALUE30 = 0x0,
    #[doc = "Enable"]
    VALUE31 = 0x01,
}
impl Req80En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req80En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req80En1 {
    #[inline(always)]
    fn from(val: u8) -> Req80En1 {
        Req80En1::from_bits(val)
    }
}
impl From<Req80En1> for u8 {
    #[inline(always)]
    fn from(val: Req80En1) -> u8 {
        Req80En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req81En0 {
    #[doc = "Disable"]
    VALUE32 = 0x0,
    #[doc = "Enable"]
    VALUE33 = 0x01,
}
impl Req81En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req81En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req81En0 {
    #[inline(always)]
    fn from(val: u8) -> Req81En0 {
        Req81En0::from_bits(val)
    }
}
impl From<Req81En0> for u8 {
    #[inline(always)]
    fn from(val: Req81En0) -> u8 {
        Req81En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req81En1 {
    #[doc = "Disable"]
    VALUE32 = 0x0,
    #[doc = "Enable"]
    VALUE33 = 0x01,
}
impl Req81En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req81En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req81En1 {
    #[inline(always)]
    fn from(val: u8) -> Req81En1 {
        Req81En1::from_bits(val)
    }
}
impl From<Req81En1> for u8 {
    #[inline(always)]
    fn from(val: Req81En1) -> u8 {
        Req81En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req82En0 {
    #[doc = "Disable"]
    VALUE34 = 0x0,
    #[doc = "Enable"]
    VALUE35 = 0x01,
}
impl Req82En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req82En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req82En0 {
    #[inline(always)]
    fn from(val: u8) -> Req82En0 {
        Req82En0::from_bits(val)
    }
}
impl From<Req82En0> for u8 {
    #[inline(always)]
    fn from(val: Req82En0) -> u8 {
        Req82En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req82En1 {
    #[doc = "Disable"]
    VALUE34 = 0x0,
    #[doc = "Enable"]
    VALUE35 = 0x01,
}
impl Req82En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req82En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req82En1 {
    #[inline(always)]
    fn from(val: u8) -> Req82En1 {
        Req82En1::from_bits(val)
    }
}
impl From<Req82En1> for u8 {
    #[inline(always)]
    fn from(val: Req82En1) -> u8 {
        Req82En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req83En0 {
    #[doc = "Disable"]
    VALUE36 = 0x0,
    #[doc = "Enable"]
    VALUE37 = 0x01,
}
impl Req83En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req83En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req83En0 {
    #[inline(always)]
    fn from(val: u8) -> Req83En0 {
        Req83En0::from_bits(val)
    }
}
impl From<Req83En0> for u8 {
    #[inline(always)]
    fn from(val: Req83En0) -> u8 {
        Req83En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req83En1 {
    #[doc = "Disable"]
    VALUE36 = 0x0,
    #[doc = "Enable"]
    VALUE37 = 0x01,
}
impl Req83En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req83En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req83En1 {
    #[inline(always)]
    fn from(val: u8) -> Req83En1 {
        Req83En1::from_bits(val)
    }
}
impl From<Req83En1> for u8 {
    #[inline(always)]
    fn from(val: Req83En1) -> u8 {
        Req83En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req84En0 {
    #[doc = "Disable"]
    VALUE38 = 0x0,
    #[doc = "Enable"]
    VALUE39 = 0x01,
}
impl Req84En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req84En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req84En0 {
    #[inline(always)]
    fn from(val: u8) -> Req84En0 {
        Req84En0::from_bits(val)
    }
}
impl From<Req84En0> for u8 {
    #[inline(always)]
    fn from(val: Req84En0) -> u8 {
        Req84En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req84En1 {
    #[doc = "Disable"]
    VALUE38 = 0x0,
    #[doc = "Enable"]
    VALUE39 = 0x01,
}
impl Req84En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req84En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req84En1 {
    #[inline(always)]
    fn from(val: u8) -> Req84En1 {
        Req84En1::from_bits(val)
    }
}
impl From<Req84En1> for u8 {
    #[inline(always)]
    fn from(val: Req84En1) -> u8 {
        Req84En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req85En0 {
    #[doc = "Disable"]
    VALUE40 = 0x0,
    #[doc = "Enable"]
    VALUE41 = 0x01,
}
impl Req85En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req85En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req85En0 {
    #[inline(always)]
    fn from(val: u8) -> Req85En0 {
        Req85En0::from_bits(val)
    }
}
impl From<Req85En0> for u8 {
    #[inline(always)]
    fn from(val: Req85En0) -> u8 {
        Req85En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req85En1 {
    #[doc = "Disable"]
    VALUE40 = 0x0,
    #[doc = "Enable"]
    VALUE41 = 0x01,
}
impl Req85En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req85En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req85En1 {
    #[inline(always)]
    fn from(val: u8) -> Req85En1 {
        Req85En1::from_bits(val)
    }
}
impl From<Req85En1> for u8 {
    #[inline(always)]
    fn from(val: Req85En1) -> u8 {
        Req85En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req86En0 {
    #[doc = "Disable"]
    VALUE42 = 0x0,
    #[doc = "Enable"]
    VALUE43 = 0x01,
}
impl Req86En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req86En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req86En0 {
    #[inline(always)]
    fn from(val: u8) -> Req86En0 {
        Req86En0::from_bits(val)
    }
}
impl From<Req86En0> for u8 {
    #[inline(always)]
    fn from(val: Req86En0) -> u8 {
        Req86En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req86En1 {
    #[doc = "Disable"]
    VALUE42 = 0x0,
    #[doc = "Enable"]
    VALUE43 = 0x01,
}
impl Req86En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req86En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req86En1 {
    #[inline(always)]
    fn from(val: u8) -> Req86En1 {
        Req86En1::from_bits(val)
    }
}
impl From<Req86En1> for u8 {
    #[inline(always)]
    fn from(val: Req86En1) -> u8 {
        Req86En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req87En0 {
    #[doc = "Disable"]
    VALUE44 = 0x0,
    #[doc = "Enable"]
    VALUE45 = 0x01,
}
impl Req87En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req87En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req87En0 {
    #[inline(always)]
    fn from(val: u8) -> Req87En0 {
        Req87En0::from_bits(val)
    }
}
impl From<Req87En0> for u8 {
    #[inline(always)]
    fn from(val: Req87En0) -> u8 {
        Req87En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req87En1 {
    #[doc = "Disable"]
    VALUE44 = 0x0,
    #[doc = "Enable"]
    VALUE45 = 0x01,
}
impl Req87En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req87En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req87En1 {
    #[inline(always)]
    fn from(val: u8) -> Req87En1 {
        Req87En1::from_bits(val)
    }
}
impl From<Req87En1> for u8 {
    #[inline(always)]
    fn from(val: Req87En1) -> u8 {
        Req87En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req88En0 {
    #[doc = "Disable"]
    VALUE46 = 0x0,
    #[doc = "Enable"]
    VALUE47 = 0x01,
}
impl Req88En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req88En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req88En0 {
    #[inline(always)]
    fn from(val: u8) -> Req88En0 {
        Req88En0::from_bits(val)
    }
}
impl From<Req88En0> for u8 {
    #[inline(always)]
    fn from(val: Req88En0) -> u8 {
        Req88En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req88En1 {
    #[doc = "Disable"]
    VALUE46 = 0x0,
    #[doc = "Enable"]
    VALUE47 = 0x01,
}
impl Req88En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req88En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req88En1 {
    #[inline(always)]
    fn from(val: u8) -> Req88En1 {
        Req88En1::from_bits(val)
    }
}
impl From<Req88En1> for u8 {
    #[inline(always)]
    fn from(val: Req88En1) -> u8 {
        Req88En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req8En0 {
    #[doc = "Disable"]
    VALUE14 = 0x0,
    #[doc = "Enable"]
    VALUE15 = 0x01,
}
impl Req8En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req8En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req8En0 {
    #[inline(always)]
    fn from(val: u8) -> Req8En0 {
        Req8En0::from_bits(val)
    }
}
impl From<Req8En0> for u8 {
    #[inline(always)]
    fn from(val: Req8En0) -> u8 {
        Req8En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req8En1 {
    #[doc = "Disable"]
    VALUE14 = 0x0,
    #[doc = "Enable"]
    VALUE15 = 0x01,
}
impl Req8En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req8En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req8En1 {
    #[inline(always)]
    fn from(val: u8) -> Req8En1 {
        Req8En1::from_bits(val)
    }
}
impl From<Req8En1> for u8 {
    #[inline(always)]
    fn from(val: Req8En1) -> u8 {
        Req8En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req91En0 {
    #[doc = "Disable"]
    VALUE52 = 0x0,
    #[doc = "Enable"]
    VALUE53 = 0x01,
}
impl Req91En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req91En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req91En0 {
    #[inline(always)]
    fn from(val: u8) -> Req91En0 {
        Req91En0::from_bits(val)
    }
}
impl From<Req91En0> for u8 {
    #[inline(always)]
    fn from(val: Req91En0) -> u8 {
        Req91En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req91En1 {
    #[doc = "Disable"]
    VALUE52 = 0x0,
    #[doc = "Enable"]
    VALUE53 = 0x01,
}
impl Req91En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req91En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req91En1 {
    #[inline(always)]
    fn from(val: u8) -> Req91En1 {
        Req91En1::from_bits(val)
    }
}
impl From<Req91En1> for u8 {
    #[inline(always)]
    fn from(val: Req91En1) -> u8 {
        Req91En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req92En0 {
    #[doc = "Disable"]
    VALUE54 = 0x0,
    #[doc = "Enable"]
    VALUE55 = 0x01,
}
impl Req92En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req92En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req92En0 {
    #[inline(always)]
    fn from(val: u8) -> Req92En0 {
        Req92En0::from_bits(val)
    }
}
impl From<Req92En0> for u8 {
    #[inline(always)]
    fn from(val: Req92En0) -> u8 {
        Req92En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req92En1 {
    #[doc = "Disable"]
    VALUE54 = 0x0,
    #[doc = "Enable"]
    VALUE55 = 0x01,
}
impl Req92En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req92En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req92En1 {
    #[inline(always)]
    fn from(val: u8) -> Req92En1 {
        Req92En1::from_bits(val)
    }
}
impl From<Req92En1> for u8 {
    #[inline(always)]
    fn from(val: Req92En1) -> u8 {
        Req92En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req93En0 {
    #[doc = "Disable"]
    VALUE56 = 0x0,
    #[doc = "Enable"]
    VALUE57 = 0x01,
}
impl Req93En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req93En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req93En0 {
    #[inline(always)]
    fn from(val: u8) -> Req93En0 {
        Req93En0::from_bits(val)
    }
}
impl From<Req93En0> for u8 {
    #[inline(always)]
    fn from(val: Req93En0) -> u8 {
        Req93En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req93En1 {
    #[doc = "Disable"]
    VALUE56 = 0x0,
    #[doc = "Enable"]
    VALUE57 = 0x01,
}
impl Req93En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req93En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req93En1 {
    #[inline(always)]
    fn from(val: u8) -> Req93En1 {
        Req93En1::from_bits(val)
    }
}
impl From<Req93En1> for u8 {
    #[inline(always)]
    fn from(val: Req93En1) -> u8 {
        Req93En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req94En0 {
    #[doc = "Disable"]
    VALUE58 = 0x0,
    #[doc = "Enable"]
    VALUE59 = 0x01,
}
impl Req94En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req94En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req94En0 {
    #[inline(always)]
    fn from(val: u8) -> Req94En0 {
        Req94En0::from_bits(val)
    }
}
impl From<Req94En0> for u8 {
    #[inline(always)]
    fn from(val: Req94En0) -> u8 {
        Req94En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req94En1 {
    #[doc = "Disable"]
    VALUE58 = 0x0,
    #[doc = "Enable"]
    VALUE59 = 0x01,
}
impl Req94En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req94En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req94En1 {
    #[inline(always)]
    fn from(val: u8) -> Req94En1 {
        Req94En1::from_bits(val)
    }
}
impl From<Req94En1> for u8 {
    #[inline(always)]
    fn from(val: Req94En1) -> u8 {
        Req94En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req95En0 {
    #[doc = "Disable"]
    VALUE60 = 0x0,
    #[doc = "Enable"]
    VALUE61 = 0x01,
}
impl Req95En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req95En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req95En0 {
    #[inline(always)]
    fn from(val: u8) -> Req95En0 {
        Req95En0::from_bits(val)
    }
}
impl From<Req95En0> for u8 {
    #[inline(always)]
    fn from(val: Req95En0) -> u8 {
        Req95En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req95En1 {
    #[doc = "Disable"]
    VALUE60 = 0x0,
    #[doc = "Enable"]
    VALUE61 = 0x01,
}
impl Req95En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req95En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req95En1 {
    #[inline(always)]
    fn from(val: u8) -> Req95En1 {
        Req95En1::from_bits(val)
    }
}
impl From<Req95En1> for u8 {
    #[inline(always)]
    fn from(val: Req95En1) -> u8 {
        Req95En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req96En0 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req96En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req96En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req96En0 {
    #[inline(always)]
    fn from(val: u8) -> Req96En0 {
        Req96En0::from_bits(val)
    }
}
impl From<Req96En0> for u8 {
    #[inline(always)]
    fn from(val: Req96En0) -> u8 {
        Req96En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req96En1 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req96En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req96En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req96En1 {
    #[inline(always)]
    fn from(val: u8) -> Req96En1 {
        Req96En1::from_bits(val)
    }
}
impl From<Req96En1> for u8 {
    #[inline(always)]
    fn from(val: Req96En1) -> u8 {
        Req96En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req97En0 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req97En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req97En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req97En0 {
    #[inline(always)]
    fn from(val: u8) -> Req97En0 {
        Req97En0::from_bits(val)
    }
}
impl From<Req97En0> for u8 {
    #[inline(always)]
    fn from(val: Req97En0) -> u8 {
        Req97En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req97En1 {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Req97En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req97En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req97En1 {
    #[inline(always)]
    fn from(val: u8) -> Req97En1 {
        Req97En1::from_bits(val)
    }
}
impl From<Req97En1> for u8 {
    #[inline(always)]
    fn from(val: Req97En1) -> u8 {
        Req97En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req98En0 {
    #[doc = "Disable"]
    VALUE2 = 0x0,
    #[doc = "Enable"]
    VALUE3 = 0x01,
}
impl Req98En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req98En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req98En0 {
    #[inline(always)]
    fn from(val: u8) -> Req98En0 {
        Req98En0::from_bits(val)
    }
}
impl From<Req98En0> for u8 {
    #[inline(always)]
    fn from(val: Req98En0) -> u8 {
        Req98En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req98En1 {
    #[doc = "Disable"]
    VALUE2 = 0x0,
    #[doc = "Enable"]
    VALUE3 = 0x01,
}
impl Req98En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req98En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req98En1 {
    #[inline(always)]
    fn from(val: u8) -> Req98En1 {
        Req98En1::from_bits(val)
    }
}
impl From<Req98En1> for u8 {
    #[inline(always)]
    fn from(val: Req98En1) -> u8 {
        Req98En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req99En0 {
    #[doc = "Disable"]
    VALUE4 = 0x0,
    #[doc = "Enable"]
    VALUE5 = 0x01,
}
impl Req99En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req99En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req99En0 {
    #[inline(always)]
    fn from(val: u8) -> Req99En0 {
        Req99En0::from_bits(val)
    }
}
impl From<Req99En0> for u8 {
    #[inline(always)]
    fn from(val: Req99En0) -> u8 {
        Req99En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req99En1 {
    #[doc = "Disable"]
    VALUE4 = 0x0,
    #[doc = "Enable"]
    VALUE5 = 0x01,
}
impl Req99En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req99En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req99En1 {
    #[inline(always)]
    fn from(val: u8) -> Req99En1 {
        Req99En1::from_bits(val)
    }
}
impl From<Req99En1> for u8 {
    #[inline(always)]
    fn from(val: Req99En1) -> u8 {
        Req99En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req9En0 {
    #[doc = "Disable"]
    VALUE16 = 0x0,
    #[doc = "Enable"]
    VALUE17 = 0x01,
}
impl Req9En0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req9En0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req9En0 {
    #[inline(always)]
    fn from(val: u8) -> Req9En0 {
        Req9En0::from_bits(val)
    }
}
impl From<Req9En0> for u8 {
    #[inline(always)]
    fn from(val: Req9En0) -> u8 {
        Req9En0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req9En1 {
    #[doc = "Disable"]
    VALUE16 = 0x0,
    #[doc = "Enable"]
    VALUE17 = 0x01,
}
impl Req9En1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req9En1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req9En1 {
    #[inline(always)]
    fn from(val: u8) -> Req9En1 {
        Req9En1::from_bits(val)
    }
}
impl From<Req9En1> for u8 {
    #[inline(always)]
    fn from(val: Req9En1) -> u8 {
        Req9En1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sct0InmuxInp {
    #[doc = "SCT0_IN0 input is selected"]
    VAL0 = 0x0,
    #[doc = "SCT0_IN1 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT0_IN2 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT0_IN3 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT0_IN4 input is selected"]
    VAL4 = 0x04,
    #[doc = "SCT0_IN5 input is selected"]
    VAL5 = 0x05,
    #[doc = "SCT0_IN6 input is selected"]
    VAL6 = 0x06,
    #[doc = "SCT0_IN7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER0_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER1_MAT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTIMER2_MAT0 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTIMER3_MAT0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTIMER4_MAT0 input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL13 = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL14 = 0x0e,
    #[doc = "usb0 start of frame input is selected"]
    VAL15 = 0x0f,
    #[doc = "usb1 start of frame input is selected"]
    VAL16 = 0x10,
    #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
    VAL17 = 0x11,
    #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
    VAL18 = 0x12,
    #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
    VAL19 = 0x13,
    #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
    VAL20 = 0x14,
    #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
    VAL21 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "DEBUG_HALTED input is selected"]
    VAL23 = 0x17,
    #[doc = "ADC1_IRQ input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL28 = 0x1c,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL29 = 0x1d,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL30 = 0x1e,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL31 = 0x1f,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL32 = 0x20,
    #[doc = "CMP0_OUT input is selected"]
    VAL33 = 0x21,
    #[doc = "CMP1_OUT input is selected"]
    VAL34 = 0x22,
    #[doc = "CMP2_OUT input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL38 = 0x26,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL39 = 0x27,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL40 = 0x28,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL41 = 0x29,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL42 = 0x2a,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL44 = 0x2c,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL47 = 0x2f,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL48 = 0x30,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL49 = 0x31,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL50 = 0x32,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL51 = 0x33,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL52 = 0x34,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL53 = 0x35,
    #[doc = "FC3_P0 (SDO, SDA) input is selected"]
    VAL54 = 0x36,
    #[doc = "FC3_P1 (SCK, TXD, SCL) input is selected"]
    VAL55 = 0x37,
    #[doc = "FC3_P2 (RTS, SCLS, TXD) input is selected"]
    VAL56 = 0x38,
    #[doc = "FC3_P3 (PCS\\[0\\], CTS, SDAS) input is selected"]
    VAL57 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "LP_FLEXCOMM0 trig 0 (lpuart_trg_txword) input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM0 trig 1 (lpuart_trg_rxword) input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM0 trig 2 (lpuart_trg_rxidle) input is selected"]
    VAL62 = 0x3e,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL63 = 0x3f,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL64 = 0x40,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL65 = 0x41,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL66 = 0x42,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL67 = 0x43,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL68 = 0x44,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL69 = 0x45,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL70 = 0x46,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL71 = 0x47,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL72 = 0x48,
    #[doc = "SAI0 TX BCLK input is selected"]
    VAL73 = 0x49,
    #[doc = "SAI0 RX BCLK input is selected"]
    VAL74 = 0x4a,
    #[doc = "SAI1 TX BCLK input is selected"]
    VAL75 = 0x4b,
    #[doc = "SAI1 RX BCLK input is selected"]
    VAL76 = 0x4c,
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
impl Sct0InmuxInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sct0InmuxInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sct0InmuxInp {
    #[inline(always)]
    fn from(val: u8) -> Sct0InmuxInp {
        Sct0InmuxInp::from_bits(val)
    }
}
impl From<Sct0InmuxInp> for u8 {
    #[inline(always)]
    fn from(val: Sct0InmuxInp) -> u8 {
        Sct0InmuxInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincFilterChInp {
    #[doc = "PINT PIN_INT0 input is selected"]
    VAL0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected"]
    VAL1 = 0x01,
    #[doc = "SCT_OUT4 input is selected"]
    VAL2 = 0x02,
    #[doc = "SCT_OUT5 input is selected"]
    VAL3 = 0x03,
    #[doc = "SCT_OUT9 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTIMER3_MAT3 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTIMER4_MAT3 input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected"]
    VAL12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL20 = 0x14,
    #[doc = "CMP0_OUT input is selected"]
    VAL21 = 0x15,
    #[doc = "CMP1_OUT input is selected"]
    VAL22 = 0x16,
    #[doc = "CMP2_OUT input is selected"]
    VAL23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    VAL24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected"]
    VAL38 = 0x26,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL40 = 0x28,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL47 = 0x2f,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL48 = 0x30,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL49 = 0x31,
    #[doc = "LPTMR0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LPTMR1 input is selected"]
    VAL51 = 0x33,
    #[doc = "FlexIO CH0 input is selected"]
    VAL52 = 0x34,
    #[doc = "FlexIO CH1 input is selected"]
    VAL53 = 0x35,
    #[doc = "FlexIO CH2 input is selected"]
    VAL54 = 0x36,
    #[doc = "FlexIO CH3 input is selected"]
    VAL55 = 0x37,
    #[doc = "WUU input is selected"]
    VAL56 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl SincFilterChInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincFilterChInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincFilterChInp {
    #[inline(always)]
    fn from(val: u8) -> SincFilterChInp {
        SincFilterChInp::from_bits(val)
    }
}
impl From<SincFilterChInp> for u8 {
    #[inline(always)]
    fn from(val: SincFilterChInp) -> u8 {
        SincFilterChInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartdmaarchbInmuxInp {
    #[doc = "FlexIO interrupt is selected as input"]
    VAL0 = 0x0,
    #[doc = "GPIO P0_1 input is selected"]
    VAL1 = 0x01,
    #[doc = "GPIO P0_2 input is selected"]
    VAL2 = 0x02,
    #[doc = "GPIO P0_3 input is selected"]
    VAL3 = 0x03,
    #[doc = "GPIO P0_4 input is selected"]
    VAL4 = 0x04,
    #[doc = "GPIO P0_5 input is selected"]
    VAL5 = 0x05,
    #[doc = "GPIO P0_6 input is selected"]
    VAL6 = 0x06,
    #[doc = "GPIO P0_7 input is selected"]
    VAL7 = 0x07,
    #[doc = "GPIO P0_8 input is selected"]
    VAL8 = 0x08,
    #[doc = "GPIO P0_9 input is selected"]
    VAL9 = 0x09,
    #[doc = "GPIO P0_10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "GPIO P0_11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "GPIO P0_12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "GPIO P0_13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "GPIO P0_14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "GPIO P0_15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "SCT0 SCT_OUT8 input is selected"]
    VAL16 = 0x10,
    #[doc = "SCT0 SCT_OUT9 input is selected"]
    VAL17 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "MRT0 MRT_CH0_IRQ input is selected"]
    VAL20 = 0x14,
    #[doc = "MRT0 MRT_CH1_IRQ input is selected"]
    VAL21 = 0x15,
    #[doc = "CTIMER4_MAT3 input is selected"]
    VAL22 = 0x16,
    #[doc = "CTIMER4_MAT2 input is selected"]
    VAL23 = 0x17,
    #[doc = "CTIMER3_MAT3 input is selected"]
    VAL24 = 0x18,
    #[doc = "CTIMER3_MAT2 input is selected"]
    VAL25 = 0x19,
    #[doc = "CTIMER1_MAT3 input is selected"]
    VAL26 = 0x1a,
    #[doc = "CTIMER1_MAT2 input is selected"]
    VAL27 = 0x1b,
    #[doc = "UTICK0 UTICK_IRQ input is selected"]
    VAL28 = 0x1c,
    #[doc = "WWDT0 WDT0_IRQ input is selected"]
    VAL29 = 0x1d,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL30 = 0x1e,
    #[doc = "CMP0_IRQ input is selected"]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "LP_FLEXCOMM7_IRQ input is selected"]
    VAL33 = 0x21,
    #[doc = "LP_FLEXCOMM6_IRQ input is selected"]
    VAL34 = 0x22,
    #[doc = "LP_FLEXCOMM5_IRQ input is selected"]
    VAL35 = 0x23,
    #[doc = "LP_FLEXCOMM4_IRQ input is selected"]
    VAL36 = 0x24,
    #[doc = "LP_FLEXCOMM3_IRQ input is selected"]
    VAL37 = 0x25,
    #[doc = "LP_FLEXCOMM2_IRQ input is selected"]
    VAL38 = 0x26,
    #[doc = "LP_FLEXCOMM1_IRQ input is selected"]
    VAL39 = 0x27,
    #[doc = "LP_FLEXCOMM0_IRQ input is selected"]
    VAL40 = 0x28,
    #[doc = "DMA0_IRQ input is selected"]
    VAL41 = 0x29,
    #[doc = "DMA1_IRQ input is selected"]
    VAL42 = 0x2a,
    #[doc = "SYS_IRQSYS_IRQ combines the CDOG IRQ, WWDT IRQ, MBC secure violation IRQ, Secure AHB Matrix secure violation IRQ, GDET IRQ, ELS S50 error IRQ, PKC error IRQ, and VBAT IRQ using the logical OR operation. input is selected"]
    VAL43 = 0x2b,
    #[doc = "RTC_COMBO_IRQ input is selected"]
    VAL44 = 0x2c,
    #[doc = "ARM_TXEV input is selected"]
    VAL45 = 0x2d,
    #[doc = "PINT0 GPIO_INT_BMATCH input is selected"]
    VAL46 = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CMP0_OUT input is selected"]
    VAL49 = 0x31,
    #[doc = "usb0 start of frame input is selected"]
    VAL50 = 0x32,
    #[doc = "usb1 start of frame input is selected"]
    VAL51 = 0x33,
    #[doc = "OSTIMER0 OS_EVENT_TIMER_IRQ input is selected"]
    VAL52 = 0x34,
    #[doc = "ADC1_IRQ input is selected"]
    VAL53 = 0x35,
    #[doc = "CMP0_IRQ/CMP1_IRQ/CMP2_IRQ input is selected"]
    VAL54 = 0x36,
    #[doc = "DAC0_IRQ input is selected"]
    VAL55 = 0x37,
    #[doc = "DAC1_IRQ/DAC2_IRQ input is selected"]
    VAL56 = 0x38,
    #[doc = "PWM0_IRQ input is selected"]
    VAL57 = 0x39,
    #[doc = "PWM1_IRQ input is selected"]
    VAL58 = 0x3a,
    #[doc = "QDC0_IRQ input is selected"]
    VAL59 = 0x3b,
    #[doc = "QDC1_IRQ input is selected"]
    VAL60 = 0x3c,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL61 = 0x3d,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL62 = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    #[doc = "GPIO1_alias0 GPIO1 Pin Event Trig 0 input is selected"]
    VAL65 = 0x41,
    #[doc = "GPIO1_alias1 GPIO1 Pin Event Trig 1 input is selected"]
    VAL66 = 0x42,
    #[doc = "GPIO2_alias0 GPIO2 Pin Event Trig 0 input is selected"]
    VAL67 = 0x43,
    #[doc = "GPIO2_alias1 GPIO2 Pin Event Trig 1 input is selected"]
    VAL68 = 0x44,
    #[doc = "GPIO3_alias0 GPIO3 Pin Event Trig 0 input is selected"]
    VAL69 = 0x45,
    #[doc = "GPIO3_alias1 GPIO3 Pin Event Trig 1 input is selected"]
    VAL70 = 0x46,
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
impl SmartdmaarchbInmuxInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartdmaarchbInmuxInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartdmaarchbInmuxInp {
    #[inline(always)]
    fn from(val: u8) -> SmartdmaarchbInmuxInp {
        SmartdmaarchbInmuxInp::from_bits(val)
    }
}
impl From<SmartdmaarchbInmuxInp> for u8 {
    #[inline(always)]
    fn from(val: SmartdmaarchbInmuxInp) -> u8 {
        SmartdmaarchbInmuxInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0trigInp {
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
    #[doc = "CT_INP0 input is selected"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 input is selected"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 input is selected"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 input is selected"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 input is selected"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 input is selected"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 input is selected"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 input is selected"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 input is selected"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 input is selected"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 input is selected"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 input is selected"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 input is selected"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 input is selected"]
    VAL19 = 0x13,
    #[doc = "usb0 start of frame input is selected"]
    VAL20 = 0x14,
    #[doc = "usb1 start of frame input is selected"]
    VAL21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected"]
    VAL22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected"]
    VAL25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected"]
    VAL26 = 0x1a,
    #[doc = "CMP0_OUT input is selected"]
    VAL27 = 0x1b,
    #[doc = "CMP1_OUT input is selected"]
    VAL28 = 0x1c,
    #[doc = "CMP2_OUT input is selected"]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected"]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected"]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected"]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected"]
    VAL33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected"]
    VAL34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected"]
    VAL35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected"]
    VAL36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected"]
    VAL37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected"]
    VAL38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected"]
    VAL39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected"]
    VAL40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected"]
    VAL41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected"]
    VAL42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected"]
    VAL43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected"]
    VAL44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected"]
    VAL45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected"]
    VAL46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected"]
    VAL47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
    VAL52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
    VAL53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
    VAL54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
    VAL55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
    VAL56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
    VAL59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
    VAL60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
    VAL61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
    VAL62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected"]
    VAL64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
pub enum TsiTrigInp {
    #[doc = "LPTMR0 input is selected"]
    VAL0 = 0x0,
    #[doc = "LPTMR1 input is selected"]
    VAL1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl TsiTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiTrigInp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiTrigInp {
    #[inline(always)]
    fn from(val: u8) -> TsiTrigInp {
        TsiTrigInp::from_bits(val)
    }
}
impl From<TsiTrigInp> for u8 {
    #[inline(always)]
    fn from(val: TsiTrigInp) -> u8 {
        TsiTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbfsTrigInp {
    #[doc = "LP_FLEXCOMM 0 trigger out \\[3\\] input is selected"]
    VAL0 = 0x0,
    #[doc = "LP_FLEXCOMM 1 trigger out \\[3\\] input is selected"]
    VAL1 = 0x01,
    #[doc = "LP_FLEXCOMM 2 trigger out \\[3\\] input is selected"]
    VAL2 = 0x02,
    #[doc = "LP_FLEXCOMM 3 trigger out \\[3\\] input is selected"]
    VAL3 = 0x03,
    #[doc = "LP_FLEXCOMM 4 trigger out \\[3\\] input is selected"]
    VAL4 = 0x04,
    #[doc = "LP_FLEXCOMM 5 trigger out \\[3\\] input is selected"]
    VAL5 = 0x05,
    #[doc = "LP_FLEXCOMM 6 trigger out \\[3\\] input is selected"]
    VAL6 = 0x06,
    #[doc = "LP_FLEXCOMM 7 trigger out \\[3\\] input is selected"]
    VAL7 = 0x07,
    #[doc = "LP_FLEXCOMM 8 trigger out \\[3\\] input is selected"]
    VAL8 = 0x08,
    #[doc = "LP_FLEXCOMM 9 trigger out \\[3\\] input is selected"]
    VAL9 = 0x09,
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
