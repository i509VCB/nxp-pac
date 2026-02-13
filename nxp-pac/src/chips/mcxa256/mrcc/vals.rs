#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FUNC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl AdcClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcClkselMux {
    #[inline(always)]
    fn from(val: u8) -> AdcClkselMux {
        AdcClkselMux::from_bits(val)
    }
}
impl From<AdcClkselMux> for u8 {
    #[inline(always)]
    fn from(val: AdcClkselMux) -> u8 {
        AdcClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkdivHalt {
        ClkdivHalt::from_bits(val)
    }
}
impl From<ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkdivHalt) -> u8 {
        ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkdivReset {
        ClkdivReset::from_bits(val)
    }
}
impl From<ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkdivReset) -> u8 {
        ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> ClkdivUnstab {
        ClkdivUnstab::from_bits(val)
    }
}
impl From<ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: ClkdivUnstab) -> u8 {
        ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutClkselMux {
    #[doc = "FRO_12M"]
    CLKROOT_12M = 0x0,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FIRC_DIV = 0x01,
    #[doc = "CLK_IN"]
    CLKROOT_SOSC = 0x02,
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "PLL1_CLK"]
    CLKROOT_SPLL = 0x05,
    #[doc = "SLOW_CLK"]
    CLKROOT_SLOW = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkoutClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutClkselMux {
    #[inline(always)]
    fn from(val: u8) -> ClkoutClkselMux {
        ClkoutClkselMux::from_bits(val)
    }
}
impl From<ClkoutClkselMux> for u8 {
    #[inline(always)]
    fn from(val: ClkoutClkselMux) -> u8 {
        ClkoutClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FUNC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    #[doc = "CLK_16K"]
    CLKROOT_FUNC_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CtimerClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerClkselMux {
    #[inline(always)]
    fn from(val: u8) -> CtimerClkselMux {
        CtimerClkselMux::from_bits(val)
    }
}
impl From<CtimerClkselMux> for u8 {
    #[inline(always)]
    fn from(val: CtimerClkselMux) -> u8 {
        CtimerClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl DacClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacClkselMux {
    #[inline(always)]
    fn from(val: u8) -> DacClkselMux {
        DacClkselMux::from_bits(val)
    }
}
impl From<DacClkselMux> for u8 {
    #[inline(always)]
    fn from(val: DacClkselMux) -> u8 {
        DacClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgTraceClkselMux {
    #[doc = "CPU_CLK"]
    CLKROOT_CPU = 0x0,
    #[doc = "CLK_1M"]
    CLKROOT_1M = 0x01,
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x02,
    _RESERVED_3 = 0x03,
}
impl DbgTraceClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgTraceClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgTraceClkselMux {
    #[inline(always)]
    fn from(val: u8) -> DbgTraceClkselMux {
        DbgTraceClkselMux::from_bits(val)
    }
}
impl From<DbgTraceClkselMux> for u8 {
    #[inline(always)]
    fn from(val: DbgTraceClkselMux) -> u8 {
        DbgTraceClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FclkClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl FclkClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FclkClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FclkClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FclkClkselMux {
        FclkClkselMux::from_bits(val)
    }
}
impl From<FclkClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FclkClkselMux) -> u8 {
        FclkClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcanClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FIRC_GATED = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FIRC_DIV = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_SOSC = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK"]
    CLKROOT_SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexcanClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcanClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcanClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexcanClkselMux {
        FlexcanClkselMux::from_bits(val)
    }
}
impl From<FlexcanClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexcanClkselMux) -> u8 {
        FlexcanClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    CLKROOT_FUNC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexioClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexioClkselMux {
        FlexioClkselMux::from_bits(val)
    }
}
impl From<FlexioClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexioClkselMux) -> u8 {
        FlexioClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lpi2cClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkselMux {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkselMux {
        Lpi2cClkselMux::from_bits(val)
    }
}
impl From<Lpi2cClkselMux> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkselMux) -> u8 {
        Lpi2cClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LpspiClkselMux {
        LpspiClkselMux::from_bits(val)
    }
}
impl From<LpspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LpspiClkselMux) -> u8 {
        LpspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LptmrClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LptmrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LptmrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LptmrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LptmrClkselMux {
        LptmrClkselMux::from_bits(val)
    }
}
impl From<LptmrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LptmrClkselMux) -> u8 {
        LptmrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpuartClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    #[doc = "CLK_16K"]
    CLKROOT_FUNC_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpuartClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpuartClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpuartClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LpuartClkselMux {
        LpuartClkselMux::from_bits(val)
    }
}
impl From<LpuartClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LpuartClkselMux) -> u8 {
        LpuartClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerClkselMux {
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "CLK_1M"]
    CLKROOT_1M = 0x02,
    _RESERVED_3 = 0x03,
}
impl OstimerClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerClkselMux {
    #[inline(always)]
    fn from(val: u8) -> OstimerClkselMux {
        OstimerClkselMux::from_bits(val)
    }
}
impl From<OstimerClkselMux> for u8 {
    #[inline(always)]
    fn from(val: OstimerClkselMux) -> u8 {
        OstimerClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrClkselMux {
    #[doc = "FRO_LF_DIV"]
    CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> RrClkselMux {
        RrClkselMux::from_bits(val)
    }
}
impl From<RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: RrClkselMux) -> u8 {
        RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystickClkselMux {
    #[doc = "CPU_CLK"]
    CLKROOT_CPU = 0x0,
    #[doc = "CLK_1M"]
    CLKROOT_1M = 0x01,
    #[doc = "CLK_16K"]
    CLKROOT_16K = 0x02,
    _RESERVED_3 = 0x03,
}
impl SystickClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystickClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystickClkselMux {
    #[inline(always)]
    fn from(val: u8) -> SystickClkselMux {
        SystickClkselMux::from_bits(val)
    }
}
impl From<SystickClkselMux> for u8 {
    #[inline(always)]
    fn from(val: SystickClkselMux) -> u8 {
        SystickClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbClkselMux {
    #[doc = "PLL1_CLK"]
    CLKROOT_SPLL = 0x0,
    #[doc = "CLK_48M"]
    SCG_SCG_FIRC_48MHZ_CLK = 0x01,
    #[doc = "CLK_IN"]
    CLKROOT_SOSC = 0x02,
    _RESERVED_3 = 0x03,
}
impl UsbClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbClkselMux {
    #[inline(always)]
    fn from(val: u8) -> UsbClkselMux {
        UsbClkselMux::from_bits(val)
    }
}
impl From<UsbClkselMux> for u8 {
    #[inline(always)]
    fn from(val: UsbClkselMux) -> u8 {
        UsbClkselMux::to_bits(val)
    }
}
