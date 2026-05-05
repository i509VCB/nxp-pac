#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterClksel {
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    Fro1mhz = 0x0,
    #[doc = "Selects the 12 MHz FRO as the filter clock."]
    Fro12mhz = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl FilterClksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterClksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterClksel {
    #[inline(always)]
    fn from(val: u8) -> FilterClksel {
        FilterClksel::from_bits(val)
    }
}
impl From<FilterClksel> for u8 {
    #[inline(always)]
    fn from(val: FilterClksel) -> u8 {
        FilterClksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterMode {
    #[doc = "Bypass mode."]
    Bypass = 0x0,
    #[doc = "Filter 1 clock period."]
    Filter1clk = 0x01,
    #[doc = "Filter 2 clock period."]
    Filter2clk = 0x02,
    #[doc = "Filter 3 clock period."]
    Filter3clk = 0x03,
}
impl FilterMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterMode {
    #[inline(always)]
    fn from(val: u8) -> FilterMode {
        FilterMode::from_bits(val)
    }
}
impl From<FilterMode> for u8 {
    #[inline(always)]
    fn from(val: FilterMode) -> u8 {
        FilterMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LuTnInPx {
    #[doc = "PLU primary inputs 0."]
    PluInputs0 = 0x0,
    #[doc = "PLU primary inputs 1."]
    PluInputs1 = 0x01,
    #[doc = "PLU primary inputs 2."]
    PluInputs2 = 0x02,
    #[doc = "PLU primary inputs 3."]
    PluInputs3 = 0x03,
    #[doc = "PLU primary inputs 4."]
    PluInputs4 = 0x04,
    #[doc = "PLU primary inputs 5."]
    PluInputs5 = 0x05,
    #[doc = "Output of LUT0."]
    LutOutputs0 = 0x06,
    #[doc = "Output of LUT1."]
    LutOutputs1 = 0x07,
    #[doc = "Output of LUT2."]
    LutOutputs2 = 0x08,
    #[doc = "Output of LUT3."]
    LutOutputs3 = 0x09,
    #[doc = "Output of LUT4."]
    LutOutputs4 = 0x0a,
    #[doc = "Output of LUT5."]
    LutOutputs5 = 0x0b,
    #[doc = "Output of LUT6."]
    LutOutputs6 = 0x0c,
    #[doc = "Output of LUT7."]
    LutOutputs7 = 0x0d,
    #[doc = "Output of LUT8."]
    LutOutputs8 = 0x0e,
    #[doc = "Output of LUT9."]
    LutOutputs9 = 0x0f,
    #[doc = "Output of LUT10."]
    LutOutputs10 = 0x10,
    #[doc = "Output of LUT11."]
    LutOutputs11 = 0x11,
    #[doc = "Output of LUT12."]
    LutOutputs12 = 0x12,
    #[doc = "Output of LUT13."]
    LutOutputs13 = 0x13,
    #[doc = "Output of LUT14."]
    LutOutputs14 = 0x14,
    #[doc = "Output of LUT15."]
    LutOutputs15 = 0x15,
    #[doc = "Output of LUT16."]
    LutOutputs16 = 0x16,
    #[doc = "Output of LUT17."]
    LutOutputs17 = 0x17,
    #[doc = "Output of LUT18."]
    LutOutputs18 = 0x18,
    #[doc = "Output of LUT19."]
    LutOutputs19 = 0x19,
    #[doc = "Output of LUT20."]
    LutOutputs20 = 0x1a,
    #[doc = "Output of LUT21."]
    LutOutputs21 = 0x1b,
    #[doc = "Output of LUT22."]
    LutOutputs22 = 0x1c,
    #[doc = "Output of LUT23."]
    LutOutputs23 = 0x1d,
    #[doc = "Output of LUT24."]
    LutOutputs24 = 0x1e,
    #[doc = "Output of LUT25."]
    LutOutputs25 = 0x1f,
    #[doc = "State\\[0\\]."]
    State0 = 0x20,
    #[doc = "State\\[1\\]."]
    State1 = 0x21,
    #[doc = "State\\[2\\]."]
    State2 = 0x22,
    #[doc = "State\\[3\\]."]
    State3 = 0x23,
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
impl LuTnInPx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LuTnInPx {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LuTnInPx {
    #[inline(always)]
    fn from(val: u8) -> LuTnInPx {
        LuTnInPx::from_bits(val)
    }
}
impl From<LuTnInPx> for u8 {
    #[inline(always)]
    fn from(val: LuTnInPx) -> u8 {
        LuTnInPx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Output {
    #[doc = "LUT output 0."]
    PluOutput0 = 0x0,
    #[doc = "LUT output 1."]
    PluOutput1 = 0x01,
    #[doc = "LUT output 2."]
    PluOutput2 = 0x02,
    #[doc = "LUT output 3."]
    PluOutput3 = 0x03,
    #[doc = "LUT output 4."]
    PluOutput4 = 0x04,
    #[doc = "LUT output 5."]
    PluOutput5 = 0x05,
    #[doc = "LUT output 6."]
    PluOutput6 = 0x06,
    #[doc = "LUT output 7."]
    PluOutput7 = 0x07,
    #[doc = "LUT output 8."]
    PluOutput8 = 0x08,
    #[doc = "LUT output 9."]
    PluOutput9 = 0x09,
    #[doc = "LUT output 10."]
    PluOutput10 = 0x0a,
    #[doc = "LUT output 11."]
    PluOutput11 = 0x0b,
    #[doc = "LUT output 12."]
    PluOutput12 = 0x0c,
    #[doc = "LUT output 13."]
    PluOutput13 = 0x0d,
    #[doc = "LUT output 14."]
    PluOutput14 = 0x0e,
    #[doc = "LUT output 15."]
    PluOutput15 = 0x0f,
    #[doc = "LUT output 16."]
    PluOutput16 = 0x10,
    #[doc = "LUT output 17."]
    PluOutput17 = 0x11,
    #[doc = "LUT output 18."]
    PluOutput18 = 0x12,
    #[doc = "LUT output 19."]
    PluOutput19 = 0x13,
    #[doc = "LUT output 20."]
    PluOutput20 = 0x14,
    #[doc = "LUT output 21."]
    PluOutput21 = 0x15,
    #[doc = "LUT output 22."]
    PluOutput22 = 0x16,
    #[doc = "LUT output 23."]
    PluOutput23 = 0x17,
    #[doc = "LUT output 24."]
    PluOutput24 = 0x18,
    #[doc = "LUT output 25."]
    PluOutput25 = 0x19,
    #[doc = "State\\[0\\]."]
    State0 = 0x1a,
    #[doc = "State\\[1\\]."]
    State1 = 0x1b,
    #[doc = "State\\[2\\]."]
    State2 = 0x1c,
    #[doc = "State\\[3\\]."]
    State3 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Output {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Output {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Output {
    #[inline(always)]
    fn from(val: u8) -> Output {
        Output::from_bits(val)
    }
}
impl From<Output> for u8 {
    #[inline(always)]
    fn from(val: Output) -> u8 {
        Output::to_bits(val)
    }
}
