#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcInLowVol {
    #[doc = "DCDC_IN is ok"]
    DCDC_IN_LOW_VOL_0 = 0x0,
    #[doc = "DCDC_IN is too low"]
    DCDC_IN_LOW_VOL_1 = 0x01,
}
impl DcdcInLowVol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcInLowVol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcInLowVol {
    #[inline(always)]
    fn from(val: u8) -> DcdcInLowVol {
        DcdcInLowVol::from_bits(val)
    }
}
impl From<DcdcInLowVol> for u8 {
    #[inline(always)]
    fn from(val: DcdcInLowVol) -> u8 {
        DcdcInLowVol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcOverCur {
    #[doc = "No over current detected"]
    DCDC_OVER_CUR_0 = 0x0,
    #[doc = "Over current detected"]
    DCDC_OVER_CUR_1 = 0x01,
}
impl DcdcOverCur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcOverCur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcOverCur {
    #[inline(always)]
    fn from(val: u8) -> DcdcOverCur {
        DcdcOverCur::from_bits(val)
    }
}
impl From<DcdcOverCur> for u8 {
    #[inline(always)]
    fn from(val: DcdcOverCur) -> u8 {
        DcdcOverCur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcOverVol {
    #[doc = "No over voltage detected"]
    DCDC_OVER_VOL_0 = 0x0,
    #[doc = "Over voltage detected"]
    DCDC_OVER_VOL_1 = 0x01,
}
impl DcdcOverVol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcOverVol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcOverVol {
    #[inline(always)]
    fn from(val: u8) -> DcdcOverVol {
        DcdcOverVol::from_bits(val)
    }
}
impl From<DcdcOverVol> for u8 {
    #[inline(always)]
    fn from(val: DcdcOverVol) -> u8 {
        DcdcOverVol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcStsDcOk {
    #[doc = "DCDC is ramping up and not ready"]
    DCDC_STS_DC_OK_0 = 0x0,
    #[doc = "DCDC is ready"]
    DCDC_STS_DC_OK_1 = 0x01,
}
impl DcdcStsDcOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcStsDcOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcStsDcOk {
    #[inline(always)]
    fn from(val: u8) -> DcdcStsDcOk {
        DcdcStsDcOk::from_bits(val)
    }
}
impl From<DcdcStsDcOk> for u8 {
    #[inline(always)]
    fn from(val: DcdcStsDcOk) -> u8 {
        DcdcStsDcOk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpsrModeEnable {
    #[doc = "SNVS domain will reset when system reset happens"]
    LPSR_MODE_ENABLE_0 = 0x0,
    #[doc = "SNVS domain will only reset with SNVS POR"]
    LPSR_MODE_ENABLE_1 = 0x01,
}
impl LpsrModeEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpsrModeEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpsrModeEnable {
    #[inline(always)]
    fn from(val: u8) -> LpsrModeEnable {
        LpsrModeEnable::from_bits(val)
    }
}
impl From<LpsrModeEnable> for u8 {
    #[inline(always)]
    fn from(val: LpsrModeEnable) -> u8 {
        LpsrModeEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PorPullType {
    #[doc = "100 Ohm pull up enabled for POR_B always"]
    POR_PULL_TYPE_0 = 0x0,
    #[doc = "Disable pull in SNVS mode, 100 Ohm pull up enabled otherwise"]
    POR_PULL_TYPE_1 = 0x01,
    #[doc = "Disable pull of POR_B always"]
    POR_PULL_TYPE_2 = 0x02,
    #[doc = "100 Ohm pull down enabled in SNVS mode, 100 Ohm pull up enabled otherwise"]
    POR_PULL_TYPE_3 = 0x03,
}
impl PorPullType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PorPullType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PorPullType {
    #[inline(always)]
    fn from(val: u8) -> PorPullType {
        PorPullType::from_bits(val)
    }
}
impl From<PorPullType> for u8 {
    #[inline(always)]
    fn from(val: PorPullType) -> u8 {
        PorPullType::to_bits(val)
    }
}
