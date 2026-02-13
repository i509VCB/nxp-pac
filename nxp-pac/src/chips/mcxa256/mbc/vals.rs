#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbacsel {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
    GLBAC0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
    GLBAC1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
    GLBAC2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
    GLBAC3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
    GLBAC4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
    GLBAC5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
    GLBAC6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
    GLBAC7 = 0x07,
}
impl Mbacsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbacsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbacsel {
    #[inline(always)]
    fn from(val: u8) -> Mbacsel {
        Mbacsel::from_bits(val)
    }
}
impl From<Mbacsel> for u8 {
    #[inline(always)]
    fn from(val: Mbacsel) -> u8 {
        Mbacsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Nse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse {
    #[inline(always)]
    fn from(val: u8) -> Nse {
        Nse::from_bits(val)
    }
}
impl From<Nse> for u8 {
    #[inline(always)]
    fn from(val: Nse) -> u8 {
        Nse::to_bits(val)
    }
}
