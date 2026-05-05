#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SMB {
    #[doc = "Low leakage."]
    LOW = 0x0,
    #[doc = "Medium leakage."]
    MEDIUM = 0x01,
    #[doc = "Highest leakage."]
    HIGHEST = 0x02,
    #[doc = "Disable."]
    DISABLE = 0x03,
}
impl SMB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SMB {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SMB {
    #[inline(always)]
    fn from(val: u8) -> SMB {
        SMB::from_bits(val)
    }
}
impl From<SMB> for u8 {
    #[inline(always)]
    fn from(val: SMB) -> u8 {
        SMB::to_bits(val)
    }
}
