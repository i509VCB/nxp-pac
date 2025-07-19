#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CurSnsThrsh {
    #[doc = "150 mA"]
    SELECT_ZERO = 0x0,
    #[doc = "250 mA"]
    SELECT_ONE = 0x01,
    #[doc = "350 mA"]
    SELECT_TWO = 0x02,
    #[doc = "450 mA"]
    SELECT_THREE = 0x03,
    #[doc = "550 mA"]
    SELECT_FOUR = 0x04,
    #[doc = "650 mA"]
    SELECT_FIVE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CurSnsThrsh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CurSnsThrsh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CurSnsThrsh {
    #[inline(always)]
    fn from(val: u8) -> CurSnsThrsh {
        CurSnsThrsh::from_bits(val)
    }
}
impl From<CurSnsThrsh> for u8 {
    #[inline(always)]
    fn from(val: CurSnsThrsh) -> u8 {
        CurSnsThrsh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCmpIsrcSel {
    #[doc = "50 nA"]
    SEL0 = 0x0,
    #[doc = "100 nA"]
    SEL1 = 0x01,
    #[doc = "200 nA"]
    SEL2 = 0x02,
    #[doc = "400 nA"]
    SEL3 = 0x03,
}
impl LpCmpIsrcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCmpIsrcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCmpIsrcSel {
    #[inline(always)]
    fn from(val: u8) -> LpCmpIsrcSel {
        LpCmpIsrcSel::from_bits(val)
    }
}
impl From<LpCmpIsrcSel> for u8 {
    #[inline(always)]
    fn from(val: LpCmpIsrcSel) -> u8 {
        LpCmpIsrcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpOverloadThrsh {
    #[doc = "32"]
    THRSH_32 = 0x0,
    #[doc = "64"]
    THRSH_64 = 0x01,
    #[doc = "16"]
    THRSH_16 = 0x02,
    #[doc = "8"]
    THRSH_8 = 0x03,
}
impl LpOverloadThrsh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpOverloadThrsh {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpOverloadThrsh {
    #[inline(always)]
    fn from(val: u8) -> LpOverloadThrsh {
        LpOverloadThrsh::from_bits(val)
    }
}
impl From<LpOverloadThrsh> for u8 {
    #[inline(always)]
    fn from(val: LpOverloadThrsh) -> u8 {
        LpOverloadThrsh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvercurTrigAdj {
    #[doc = "In Run Mode, 1 A. In Power Save Mode, 0.25 A"]
    SELECT_ZERO = 0x0,
    #[doc = "In Run Mode, 2 A. In Power Save Mode, 0.25 A"]
    SELECT_ONE = 0x01,
    #[doc = "In Run Mode, 1 A. In Power Save Mode, 0.2 A"]
    SELECT_TWO = 0x02,
    #[doc = "In Run Mode, 2 A. In Power Save Mode, 0.2 A"]
    SELECT_THREE = 0x03,
}
impl OvercurTrigAdj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvercurTrigAdj {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvercurTrigAdj {
    #[inline(always)]
    fn from(val: u8) -> OvercurTrigAdj {
        OvercurTrigAdj::from_bits(val)
    }
}
impl From<OvercurTrigAdj> for u8 {
    #[inline(always)]
    fn from(val: OvercurTrigAdj) -> u8 {
        OvercurTrigAdj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegFbkSel {
    #[doc = "The regulator outputs 1.0V with 1.2V reference voltage"]
    REG_FBK_SEL0 = 0x0,
    #[doc = "The regulator outputs 1.1V with 1.2V reference voltage"]
    REG_FBK_SEL1 = 0x01,
    #[doc = "The regulator outputs 1.0V with 1.3V reference voltage"]
    REG_FBK_SEL2 = 0x02,
    #[doc = "The regulator outputs 1.1V with 1.3V reference voltage"]
    REG_FBK_SEL3 = 0x03,
}
impl RegFbkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegFbkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegFbkSel {
    #[inline(always)]
    fn from(val: u8) -> RegFbkSel {
        RegFbkSel::from_bits(val)
    }
}
impl From<RegFbkSel> for u8 {
    #[inline(always)]
    fn from(val: RegFbkSel) -> u8 {
        RegFbkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetLp {
    #[doc = "0.9 V"]
    SEL0 = 0x0,
    #[doc = "0.925 V"]
    SEL1 = 0x01,
    #[doc = "0.95 V"]
    SEL2 = 0x02,
    #[doc = "0.975 V"]
    SEL3 = 0x03,
    #[doc = "1.0 V"]
    SEL4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TargetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TargetLp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TargetLp {
    #[inline(always)]
    fn from(val: u8) -> TargetLp {
        TargetLp::from_bits(val)
    }
}
impl From<TargetLp> for u8 {
    #[inline(always)]
    fn from(val: TargetLp) -> u8 {
        TargetLp::to_bits(val)
    }
}
