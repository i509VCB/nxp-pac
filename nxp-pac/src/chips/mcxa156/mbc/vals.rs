#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse0 {
        Mbc0Dom0Mem0BlkCfgW0Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse1 {
        Mbc0Dom0Mem0BlkCfgW0Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse2 {
        Mbc0Dom0Mem0BlkCfgW0Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse3 {
        Mbc0Dom0Mem0BlkCfgW0Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse4 {
        Mbc0Dom0Mem0BlkCfgW0Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse5 {
        Mbc0Dom0Mem0BlkCfgW0Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse6 {
        Mbc0Dom0Mem0BlkCfgW0Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW0Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW0Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW0Nse7 {
        Mbc0Dom0Mem0BlkCfgW0Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW0Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW0Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW0Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse0 {
        Mbc0Dom0Mem0BlkCfgW1Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse1 {
        Mbc0Dom0Mem0BlkCfgW1Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse2 {
        Mbc0Dom0Mem0BlkCfgW1Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse3 {
        Mbc0Dom0Mem0BlkCfgW1Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse4 {
        Mbc0Dom0Mem0BlkCfgW1Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse5 {
        Mbc0Dom0Mem0BlkCfgW1Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse6 {
        Mbc0Dom0Mem0BlkCfgW1Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW1Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW1Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW1Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW1Nse7 {
        Mbc0Dom0Mem0BlkCfgW1Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW1Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW1Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW1Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse0 {
        Mbc0Dom0Mem0BlkCfgW2Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse1 {
        Mbc0Dom0Mem0BlkCfgW2Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse2 {
        Mbc0Dom0Mem0BlkCfgW2Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse3 {
        Mbc0Dom0Mem0BlkCfgW2Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse4 {
        Mbc0Dom0Mem0BlkCfgW2Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse5 {
        Mbc0Dom0Mem0BlkCfgW2Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse6 {
        Mbc0Dom0Mem0BlkCfgW2Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW2Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW2Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW2Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW2Nse7 {
        Mbc0Dom0Mem0BlkCfgW2Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW2Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW2Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW2Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse0 {
        Mbc0Dom0Mem0BlkCfgW3Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse1 {
        Mbc0Dom0Mem0BlkCfgW3Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse2 {
        Mbc0Dom0Mem0BlkCfgW3Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse3 {
        Mbc0Dom0Mem0BlkCfgW3Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse4 {
        Mbc0Dom0Mem0BlkCfgW3Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse5 {
        Mbc0Dom0Mem0BlkCfgW3Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse6 {
        Mbc0Dom0Mem0BlkCfgW3Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW3Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW3Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW3Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW3Nse7 {
        Mbc0Dom0Mem0BlkCfgW3Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW3Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW3Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW3Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse0 {
        Mbc0Dom0Mem0BlkCfgW4Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse1 {
        Mbc0Dom0Mem0BlkCfgW4Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse2 {
        Mbc0Dom0Mem0BlkCfgW4Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse3 {
        Mbc0Dom0Mem0BlkCfgW4Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse4 {
        Mbc0Dom0Mem0BlkCfgW4Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse5 {
        Mbc0Dom0Mem0BlkCfgW4Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse6 {
        Mbc0Dom0Mem0BlkCfgW4Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW4Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW4Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW4Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW4Nse7 {
        Mbc0Dom0Mem0BlkCfgW4Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW4Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW4Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW4Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse0 {
        Mbc0Dom0Mem0BlkCfgW5Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse1 {
        Mbc0Dom0Mem0BlkCfgW5Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse2 {
        Mbc0Dom0Mem0BlkCfgW5Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse3 {
        Mbc0Dom0Mem0BlkCfgW5Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse4 {
        Mbc0Dom0Mem0BlkCfgW5Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse5 {
        Mbc0Dom0Mem0BlkCfgW5Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse6 {
        Mbc0Dom0Mem0BlkCfgW5Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW5Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW5Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW5Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW5Nse7 {
        Mbc0Dom0Mem0BlkCfgW5Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW5Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW5Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW5Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse0 {
        Mbc0Dom0Mem0BlkCfgW6Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse1 {
        Mbc0Dom0Mem0BlkCfgW6Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse2 {
        Mbc0Dom0Mem0BlkCfgW6Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse3 {
        Mbc0Dom0Mem0BlkCfgW6Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse4 {
        Mbc0Dom0Mem0BlkCfgW6Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse5 {
        Mbc0Dom0Mem0BlkCfgW6Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse6 {
        Mbc0Dom0Mem0BlkCfgW6Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW6Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW6Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW6Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW6Nse7 {
        Mbc0Dom0Mem0BlkCfgW6Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW6Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW6Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW6Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Mbacsel7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse0 {
        Mbc0Dom0Mem0BlkCfgW7Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse0) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse1 {
        Mbc0Dom0Mem0BlkCfgW7Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse1) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse2 {
        Mbc0Dom0Mem0BlkCfgW7Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse2) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse3 {
        Mbc0Dom0Mem0BlkCfgW7Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse3) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse4 {
        Mbc0Dom0Mem0BlkCfgW7Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse4) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse5 {
        Mbc0Dom0Mem0BlkCfgW7Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse5) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse6 {
        Mbc0Dom0Mem0BlkCfgW7Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse6) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW7Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgW7Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgW7Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgW7Nse7 {
        Mbc0Dom0Mem0BlkCfgW7Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgW7Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgW7Nse7) -> u8 {
        Mbc0Dom0Mem0BlkCfgW7Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel0) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel1) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel2) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel3) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel4) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel5) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel6) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel7) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse0 {
        Mbc0Dom0Mem1BlkCfgW0Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse0) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse1 {
        Mbc0Dom0Mem1BlkCfgW0Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse1) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse2 {
        Mbc0Dom0Mem1BlkCfgW0Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse2) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse3 {
        Mbc0Dom0Mem1BlkCfgW0Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse3) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse4 {
        Mbc0Dom0Mem1BlkCfgW0Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse4) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse5 {
        Mbc0Dom0Mem1BlkCfgW0Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse5) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse6 {
        Mbc0Dom0Mem1BlkCfgW0Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse6) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse7 {
        Mbc0Dom0Mem1BlkCfgW0Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse7) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel0) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel1) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel2) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel3) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel4) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel5) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel6) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel7) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse0 {
        Mbc0Dom0Mem2BlkCfgW0Nse0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse0) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse1 {
        Mbc0Dom0Mem2BlkCfgW0Nse1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse1) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse2 {
        Mbc0Dom0Mem2BlkCfgW0Nse2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse2) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse3 {
        Mbc0Dom0Mem2BlkCfgW0Nse3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse3) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse4 {
        Mbc0Dom0Mem2BlkCfgW0Nse4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse4) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse5 {
        Mbc0Dom0Mem2BlkCfgW0Nse5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse5) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse6 {
        Mbc0Dom0Mem2BlkCfgW0Nse6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse6) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse7 {
        Mbc0Dom0Mem2BlkCfgW0Nse7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse7) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse7::to_bits(val)
    }
}
