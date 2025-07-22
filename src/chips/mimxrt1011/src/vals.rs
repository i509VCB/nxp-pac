#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lockup {
    #[doc = "Reset is not a result of the mentioned case."]
    LOCKUP_0 = 0x0,
    #[doc = "Reset is a result of the mentioned case."]
    LOCKUP_1 = 0x01,
}
impl Lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockup {
    #[inline(always)]
    fn from(val: u8) -> Lockup {
        Lockup::from_bits(val)
    }
}
impl From<Lockup> for u8 {
    #[inline(always)]
    fn from(val: Lockup) -> u8 {
        Lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskWdog3Rst {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "wdog3_rst_b is masked"]
    MASK_WDOG3_RST_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "wdog3_rst_b is not masked"]
    MASK_WDOG3_RST_10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl MaskWdog3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskWdog3Rst {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskWdog3Rst {
    #[inline(always)]
    fn from(val: u8) -> MaskWdog3Rst {
        MaskWdog3Rst::from_bits(val)
    }
}
impl From<MaskWdog3Rst> for u8 {
    #[inline(always)]
    fn from(val: MaskWdog3Rst) -> u8 {
        MaskWdog3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskWdogRst {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "wdog_rst_b is masked"]
    MASK_WDOG_RST_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "wdog_rst_b is not masked (default)"]
    MASK_WDOG_RST_10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl MaskWdogRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskWdogRst {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskWdogRst {
    #[inline(always)]
    fn from(val: u8) -> MaskWdogRst {
        MaskWdogRst::from_bits(val)
    }
}
impl From<MaskWdogRst> for u8 {
    #[inline(always)]
    fn from(val: MaskWdogRst) -> u8 {
        MaskWdogRst::to_bits(val)
    }
}
