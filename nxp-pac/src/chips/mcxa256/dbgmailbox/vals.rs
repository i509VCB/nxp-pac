#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChipResetReq {
    #[doc = "No effect"]
    NO_EFF = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl ChipResetReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChipResetReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChipResetReq {
    #[inline(always)]
    fn from(val: u8) -> ChipResetReq {
        ChipResetReq::from_bits(val)
    }
}
impl From<ChipResetReq> for u8 {
    #[inline(always)]
    fn from(val: ChipResetReq) -> u8 {
        ChipResetReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SoftReset {
    #[doc = "No effect"]
    NO_EFF = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl SoftReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SoftReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SoftReset {
    #[inline(always)]
    fn from(val: u8) -> SoftReset {
        SoftReset::from_bits(val)
    }
}
impl From<SoftReset> for u8 {
    #[inline(always)]
    fn from(val: SoftReset) -> u8 {
        SoftReset::to_bits(val)
    }
}
