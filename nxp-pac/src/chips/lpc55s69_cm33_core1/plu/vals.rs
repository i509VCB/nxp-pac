#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterClksel {
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    FRO1MHZ = 0x0,
    #[doc = "Selects the 12 Mhz FRO as the filter clock."]
    FRO12MHZ = 0x01,
    #[doc = "Selects a third filter clock source, if provided."]
    OTHER_CLOCK = 0x02,
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
    BYPASS = 0x0,
    #[doc = "Filter 1 clock period."]
    FILTER1CLK = 0x01,
    #[doc = "Filter 2 clock period."]
    FILTER2CLK = 0x02,
    #[doc = "Filter 3 clock period."]
    FILTER3CLK = 0x03,
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
pub enum LutnInpx {
    #[doc = "The PLU primary inputs 0."]
    PLU_INPUTS0 = 0x0,
    #[doc = "The PLU primary inputs 1."]
    PLU_INPUTS1 = 0x01,
    #[doc = "The PLU primary inputs 2."]
    PLU_INPUTS2 = 0x02,
    #[doc = "The PLU primary inputs 3."]
    PLU_INPUTS3 = 0x03,
    #[doc = "The PLU primary inputs 4."]
    PLU_INPUTS4 = 0x04,
    #[doc = "The PLU primary inputs 5."]
    PLU_INPUTS5 = 0x05,
    #[doc = "The output of LUT0."]
    LUT_OUTPUTS0 = 0x06,
    #[doc = "The output of LUT1."]
    LUT_OUTPUTS1 = 0x07,
    #[doc = "The output of LUT2."]
    LUT_OUTPUTS2 = 0x08,
    #[doc = "The output of LUT3."]
    LUT_OUTPUTS3 = 0x09,
    #[doc = "The output of LUT4."]
    LUT_OUTPUTS4 = 0x0a,
    #[doc = "The output of LUT5."]
    LUT_OUTPUTS5 = 0x0b,
    #[doc = "The output of LUT6."]
    LUT_OUTPUTS6 = 0x0c,
    #[doc = "The output of LUT7."]
    LUT_OUTPUTS7 = 0x0d,
    #[doc = "The output of LUT8."]
    LUT_OUTPUTS8 = 0x0e,
    #[doc = "The output of LUT9."]
    LUT_OUTPUTS9 = 0x0f,
    #[doc = "The output of LUT10."]
    LUT_OUTPUTS10 = 0x10,
    #[doc = "The output of LUT11."]
    LUT_OUTPUTS11 = 0x11,
    #[doc = "The output of LUT12."]
    LUT_OUTPUTS12 = 0x12,
    #[doc = "The output of LUT13."]
    LUT_OUTPUTS13 = 0x13,
    #[doc = "The output of LUT14."]
    LUT_OUTPUTS14 = 0x14,
    #[doc = "The output of LUT15."]
    LUT_OUTPUTS15 = 0x15,
    #[doc = "The output of LUT16."]
    LUT_OUTPUTS16 = 0x16,
    #[doc = "The output of LUT17."]
    LUT_OUTPUTS17 = 0x17,
    #[doc = "The output of LUT18."]
    LUT_OUTPUTS18 = 0x18,
    #[doc = "The output of LUT19."]
    LUT_OUTPUTS19 = 0x19,
    #[doc = "The output of LUT20."]
    LUT_OUTPUTS20 = 0x1a,
    #[doc = "The output of LUT21."]
    LUT_OUTPUTS21 = 0x1b,
    #[doc = "The output of LUT22."]
    LUT_OUTPUTS22 = 0x1c,
    #[doc = "The output of LUT23."]
    LUT_OUTPUTS23 = 0x1d,
    #[doc = "The output of LUT24."]
    LUT_OUTPUTS24 = 0x1e,
    #[doc = "The output of LUT25."]
    LUT_OUTPUTS25 = 0x1f,
    #[doc = "state(0)."]
    STATE0 = 0x20,
    #[doc = "state(1)."]
    STATE1 = 0x21,
    #[doc = "state(2)."]
    STATE2 = 0x22,
    #[doc = "state(3)."]
    STATE3 = 0x23,
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
impl LutnInpx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LutnInpx {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LutnInpx {
    #[inline(always)]
    fn from(val: u8) -> LutnInpx {
        LutnInpx::from_bits(val)
    }
}
impl From<LutnInpx> for u8 {
    #[inline(always)]
    fn from(val: LutnInpx) -> u8 {
        LutnInpx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outputn {
    #[doc = "The PLU output 0."]
    PLU_OUTPUT0 = 0x0,
    #[doc = "The PLU output 1."]
    PLU_OUTPUT1 = 0x01,
    #[doc = "The PLU output 2."]
    PLU_OUTPUT2 = 0x02,
    #[doc = "The PLU output 3."]
    PLU_OUTPUT3 = 0x03,
    #[doc = "The PLU output 4."]
    PLU_OUTPUT4 = 0x04,
    #[doc = "The PLU output 5."]
    PLU_OUTPUT5 = 0x05,
    #[doc = "The PLU output 6."]
    PLU_OUTPUT6 = 0x06,
    #[doc = "The PLU output 7."]
    PLU_OUTPUT7 = 0x07,
    #[doc = "The PLU output 8."]
    PLU_OUTPUT8 = 0x08,
    #[doc = "The PLU output 9."]
    PLU_OUTPUT9 = 0x09,
    #[doc = "The PLU output 10."]
    PLU_OUTPUT10 = 0x0a,
    #[doc = "The PLU output 11."]
    PLU_OUTPUT11 = 0x0b,
    #[doc = "The PLU output 12."]
    PLU_OUTPUT12 = 0x0c,
    #[doc = "The PLU output 13."]
    PLU_OUTPUT13 = 0x0d,
    #[doc = "The PLU output 14."]
    PLU_OUTPUT14 = 0x0e,
    #[doc = "The PLU output 15."]
    PLU_OUTPUT15 = 0x0f,
    #[doc = "The PLU output 16."]
    PLU_OUTPUT16 = 0x10,
    #[doc = "The PLU output 17."]
    PLU_OUTPUT17 = 0x11,
    #[doc = "The PLU output 18."]
    PLU_OUTPUT18 = 0x12,
    #[doc = "The PLU output 19."]
    PLU_OUTPUT19 = 0x13,
    #[doc = "The PLU output 20."]
    PLU_OUTPUT20 = 0x14,
    #[doc = "The PLU output 21."]
    PLU_OUTPUT21 = 0x15,
    #[doc = "The PLU output 22."]
    PLU_OUTPUT22 = 0x16,
    #[doc = "The PLU output 23."]
    PLU_OUTPUT23 = 0x17,
    #[doc = "The PLU output 24."]
    PLU_OUTPUT24 = 0x18,
    #[doc = "The PLU output 25."]
    PLU_OUTPUT25 = 0x19,
    #[doc = "state(0)."]
    STATE0 = 0x1a,
    #[doc = "state(1)."]
    STATE1 = 0x1b,
    #[doc = "state(2)."]
    STATE2 = 0x1c,
    #[doc = "state(3)."]
    STATE3 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Outputn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outputn {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outputn {
    #[inline(always)]
    fn from(val: u8) -> Outputn {
        Outputn::from_bits(val)
    }
}
impl From<Outputn> for u8 {
    #[inline(always)]
    fn from(val: Outputn) -> u8 {
        Outputn::to_bits(val)
    }
}
