#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FILTER_CLKSEL {
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    FRO1MHZ = 0x0,
    #[doc = "Selects the 12 Mhz FRO as the filter clock."]
    FRO12MHZ = 0x01,
    #[doc = "Selects a third filter clock source, if provided."]
    OTHER_CLOCK = 0x02,
    _RESERVED_3 = 0x03,
}
impl FILTER_CLKSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FILTER_CLKSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FILTER_CLKSEL {
    #[inline(always)]
    fn from(val: u8) -> FILTER_CLKSEL {
        FILTER_CLKSEL::from_bits(val)
    }
}
impl From<FILTER_CLKSEL> for u8 {
    #[inline(always)]
    fn from(val: FILTER_CLKSEL) -> u8 {
        FILTER_CLKSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FILTER_MODE {
    #[doc = "Bypass mode."]
    BYPASS = 0x0,
    #[doc = "Filter 1 clock period."]
    FILTER1CLK = 0x01,
    #[doc = "Filter 2 clock period."]
    FILTER2CLK = 0x02,
    #[doc = "Filter 3 clock period."]
    FILTER3CLK = 0x03,
}
impl FILTER_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FILTER_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FILTER_MODE {
    #[inline(always)]
    fn from(val: u8) -> FILTER_MODE {
        FILTER_MODE::from_bits(val)
    }
}
impl From<FILTER_MODE> for u8 {
    #[inline(always)]
    fn from(val: FILTER_MODE) -> u8 {
        FILTER_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LUTn_INPx {
    #[doc = "The PLU primary inputs 0."]
    plu_inputs0 = 0x0,
    #[doc = "The PLU primary inputs 1."]
    plu_inputs1 = 0x01,
    #[doc = "The PLU primary inputs 2."]
    plu_inputs2 = 0x02,
    #[doc = "The PLU primary inputs 3."]
    plu_inputs3 = 0x03,
    #[doc = "The PLU primary inputs 4."]
    plu_inputs4 = 0x04,
    #[doc = "The PLU primary inputs 5."]
    plu_inputs5 = 0x05,
    #[doc = "The output of LUT0."]
    lut_outputs0 = 0x06,
    #[doc = "The output of LUT1."]
    lut_outputs1 = 0x07,
    #[doc = "The output of LUT2."]
    lut_outputs2 = 0x08,
    #[doc = "The output of LUT3."]
    lut_outputs3 = 0x09,
    #[doc = "The output of LUT4."]
    lut_outputs4 = 0x0a,
    #[doc = "The output of LUT5."]
    lut_outputs5 = 0x0b,
    #[doc = "The output of LUT6."]
    lut_outputs6 = 0x0c,
    #[doc = "The output of LUT7."]
    lut_outputs7 = 0x0d,
    #[doc = "The output of LUT8."]
    lut_outputs8 = 0x0e,
    #[doc = "The output of LUT9."]
    lut_outputs9 = 0x0f,
    #[doc = "The output of LUT10."]
    lut_outputs10 = 0x10,
    #[doc = "The output of LUT11."]
    lut_outputs11 = 0x11,
    #[doc = "The output of LUT12."]
    lut_outputs12 = 0x12,
    #[doc = "The output of LUT13."]
    lut_outputs13 = 0x13,
    #[doc = "The output of LUT14."]
    lut_outputs14 = 0x14,
    #[doc = "The output of LUT15."]
    lut_outputs15 = 0x15,
    #[doc = "The output of LUT16."]
    lut_outputs16 = 0x16,
    #[doc = "The output of LUT17."]
    lut_outputs17 = 0x17,
    #[doc = "The output of LUT18."]
    lut_outputs18 = 0x18,
    #[doc = "The output of LUT19."]
    lut_outputs19 = 0x19,
    #[doc = "The output of LUT20."]
    lut_outputs20 = 0x1a,
    #[doc = "The output of LUT21."]
    lut_outputs21 = 0x1b,
    #[doc = "The output of LUT22."]
    lut_outputs22 = 0x1c,
    #[doc = "The output of LUT23."]
    lut_outputs23 = 0x1d,
    #[doc = "The output of LUT24."]
    lut_outputs24 = 0x1e,
    #[doc = "The output of LUT25."]
    lut_outputs25 = 0x1f,
    #[doc = "state(0)."]
    state0 = 0x20,
    #[doc = "state(1)."]
    state1 = 0x21,
    #[doc = "state(2)."]
    state2 = 0x22,
    #[doc = "state(3)."]
    state3 = 0x23,
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
impl LUTn_INPx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LUTn_INPx {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LUTn_INPx {
    #[inline(always)]
    fn from(val: u8) -> LUTn_INPx {
        LUTn_INPx::from_bits(val)
    }
}
impl From<LUTn_INPx> for u8 {
    #[inline(always)]
    fn from(val: LUTn_INPx) -> u8 {
        LUTn_INPx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OUTPUTn {
    #[doc = "The PLU output 0."]
    plu_output0 = 0x0,
    #[doc = "The PLU output 1."]
    plu_output1 = 0x01,
    #[doc = "The PLU output 2."]
    plu_output2 = 0x02,
    #[doc = "The PLU output 3."]
    plu_output3 = 0x03,
    #[doc = "The PLU output 4."]
    plu_output4 = 0x04,
    #[doc = "The PLU output 5."]
    plu_output5 = 0x05,
    #[doc = "The PLU output 6."]
    plu_output6 = 0x06,
    #[doc = "The PLU output 7."]
    plu_output7 = 0x07,
    #[doc = "The PLU output 8."]
    plu_output8 = 0x08,
    #[doc = "The PLU output 9."]
    plu_output9 = 0x09,
    #[doc = "The PLU output 10."]
    plu_output10 = 0x0a,
    #[doc = "The PLU output 11."]
    plu_output11 = 0x0b,
    #[doc = "The PLU output 12."]
    plu_output12 = 0x0c,
    #[doc = "The PLU output 13."]
    plu_output13 = 0x0d,
    #[doc = "The PLU output 14."]
    plu_output14 = 0x0e,
    #[doc = "The PLU output 15."]
    plu_output15 = 0x0f,
    #[doc = "The PLU output 16."]
    plu_output16 = 0x10,
    #[doc = "The PLU output 17."]
    plu_output17 = 0x11,
    #[doc = "The PLU output 18."]
    plu_output18 = 0x12,
    #[doc = "The PLU output 19."]
    plu_output19 = 0x13,
    #[doc = "The PLU output 20."]
    plu_output20 = 0x14,
    #[doc = "The PLU output 21."]
    plu_output21 = 0x15,
    #[doc = "The PLU output 22."]
    plu_output22 = 0x16,
    #[doc = "The PLU output 23."]
    plu_output23 = 0x17,
    #[doc = "The PLU output 24."]
    plu_output24 = 0x18,
    #[doc = "The PLU output 25."]
    plu_output25 = 0x19,
    #[doc = "state(0)."]
    state0 = 0x1a,
    #[doc = "state(1)."]
    state1 = 0x1b,
    #[doc = "state(2)."]
    state2 = 0x1c,
    #[doc = "state(3)."]
    state3 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl OUTPUTn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OUTPUTn {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OUTPUTn {
    #[inline(always)]
    fn from(val: u8) -> OUTPUTn {
        OUTPUTn::from_bits(val)
    }
}
impl From<OUTPUTn> for u8 {
    #[inline(always)]
    fn from(val: OUTPUTn) -> u8 {
        OUTPUTn::to_bits(val)
    }
}
