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
