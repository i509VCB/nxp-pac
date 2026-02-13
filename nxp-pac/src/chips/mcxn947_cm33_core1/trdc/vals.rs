#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ai {
    #[doc = "No effect."]
    LOGIC_0 = 0x0,
    #[doc = "Add 1 to the WNDX field after the register write."]
    LOGIC_1 = 0x01,
}
impl Ai {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ai {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ai {
    #[inline(always)]
    fn from(val: u8) -> Ai {
        Ai::from_bits(val)
    }
}
impl From<Ai> for u8 {
    #[inline(always)]
    fn from(val: Ai) -> u8 {
        Ai::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
pub enum Mbc0Dom0Mem0BlkNseW0Bit0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit0 {
        Mbc0Dom0Mem0BlkNseW0Bit0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit0) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit1 {
        Mbc0Dom0Mem0BlkNseW0Bit1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit1) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit10 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit10 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit10 {
        Mbc0Dom0Mem0BlkNseW0Bit10::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit10> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit10) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit11 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit11 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit11 {
        Mbc0Dom0Mem0BlkNseW0Bit11::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit11> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit11) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit12 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit12 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit12 {
        Mbc0Dom0Mem0BlkNseW0Bit12::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit12> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit12) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit13 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit13 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit13 {
        Mbc0Dom0Mem0BlkNseW0Bit13::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit13> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit13) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit14 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit14 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit14 {
        Mbc0Dom0Mem0BlkNseW0Bit14::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit14> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit14) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit15 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit15 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit15 {
        Mbc0Dom0Mem0BlkNseW0Bit15::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit15> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit15) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit16 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit16 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit16 {
        Mbc0Dom0Mem0BlkNseW0Bit16::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit16> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit16) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit17 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit17 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit17 {
        Mbc0Dom0Mem0BlkNseW0Bit17::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit17> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit17) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit18 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit18 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit18 {
        Mbc0Dom0Mem0BlkNseW0Bit18::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit18> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit18) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit19 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit19 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit19 {
        Mbc0Dom0Mem0BlkNseW0Bit19::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit19> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit19) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit2 {
        Mbc0Dom0Mem0BlkNseW0Bit2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit2) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit20 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit20 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit20 {
        Mbc0Dom0Mem0BlkNseW0Bit20::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit20> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit20) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit21 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit21 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit21 {
        Mbc0Dom0Mem0BlkNseW0Bit21::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit21> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit21) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit22 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit22 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit22 {
        Mbc0Dom0Mem0BlkNseW0Bit22::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit22> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit22) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit23 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit23 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit23 {
        Mbc0Dom0Mem0BlkNseW0Bit23::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit23> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit23) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit24 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit24 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit24 {
        Mbc0Dom0Mem0BlkNseW0Bit24::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit24> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit24) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit25 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit25 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit25 {
        Mbc0Dom0Mem0BlkNseW0Bit25::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit25> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit25) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit26 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit26 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit26 {
        Mbc0Dom0Mem0BlkNseW0Bit26::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit26> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit26) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit27 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit27 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit27 {
        Mbc0Dom0Mem0BlkNseW0Bit27::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit27> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit27) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit28 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit28 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit28 {
        Mbc0Dom0Mem0BlkNseW0Bit28::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit28> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit28) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit29 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit29 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit29 {
        Mbc0Dom0Mem0BlkNseW0Bit29::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit29> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit29) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit3 {
        Mbc0Dom0Mem0BlkNseW0Bit3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit3) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit30 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit30 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit30 {
        Mbc0Dom0Mem0BlkNseW0Bit30::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit30> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit30) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit31 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit31 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit31 {
        Mbc0Dom0Mem0BlkNseW0Bit31::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit31> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit31) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit4 {
        Mbc0Dom0Mem0BlkNseW0Bit4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit4) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit5 {
        Mbc0Dom0Mem0BlkNseW0Bit5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit5) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit6 {
        Mbc0Dom0Mem0BlkNseW0Bit6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit6) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit7 {
        Mbc0Dom0Mem0BlkNseW0Bit7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit7) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit8 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit8 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit8 {
        Mbc0Dom0Mem0BlkNseW0Bit8::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit8> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit8) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit9 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit9 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit9 {
        Mbc0Dom0Mem0BlkNseW0Bit9::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit9> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit9) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit0 {
        Mbc0Dom0Mem0BlkNseW1Bit0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit0) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit1 {
        Mbc0Dom0Mem0BlkNseW1Bit1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit1) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit10 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit10 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit10 {
        Mbc0Dom0Mem0BlkNseW1Bit10::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit10> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit10) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit11 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit11 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit11 {
        Mbc0Dom0Mem0BlkNseW1Bit11::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit11> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit11) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit12 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit12 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit12 {
        Mbc0Dom0Mem0BlkNseW1Bit12::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit12> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit12) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit13 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit13 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit13 {
        Mbc0Dom0Mem0BlkNseW1Bit13::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit13> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit13) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit14 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit14 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit14 {
        Mbc0Dom0Mem0BlkNseW1Bit14::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit14> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit14) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit15 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit15 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit15 {
        Mbc0Dom0Mem0BlkNseW1Bit15::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit15> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit15) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit16 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit16 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit16 {
        Mbc0Dom0Mem0BlkNseW1Bit16::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit16> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit16) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit17 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit17 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit17 {
        Mbc0Dom0Mem0BlkNseW1Bit17::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit17> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit17) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit18 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit18 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit18 {
        Mbc0Dom0Mem0BlkNseW1Bit18::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit18> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit18) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit19 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit19 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit19 {
        Mbc0Dom0Mem0BlkNseW1Bit19::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit19> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit19) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit2 {
        Mbc0Dom0Mem0BlkNseW1Bit2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit2) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit20 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit20 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit20 {
        Mbc0Dom0Mem0BlkNseW1Bit20::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit20> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit20) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit21 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit21 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit21 {
        Mbc0Dom0Mem0BlkNseW1Bit21::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit21> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit21) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit22 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit22 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit22 {
        Mbc0Dom0Mem0BlkNseW1Bit22::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit22> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit22) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit23 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit23 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit23 {
        Mbc0Dom0Mem0BlkNseW1Bit23::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit23> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit23) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit24 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit24 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit24 {
        Mbc0Dom0Mem0BlkNseW1Bit24::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit24> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit24) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit25 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit25 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit25 {
        Mbc0Dom0Mem0BlkNseW1Bit25::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit25> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit25) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit26 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit26 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit26 {
        Mbc0Dom0Mem0BlkNseW1Bit26::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit26> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit26) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit27 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit27 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit27 {
        Mbc0Dom0Mem0BlkNseW1Bit27::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit27> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit27) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit28 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit28 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit28 {
        Mbc0Dom0Mem0BlkNseW1Bit28::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit28> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit28) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit29 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit29 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit29 {
        Mbc0Dom0Mem0BlkNseW1Bit29::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit29> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit29) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit3 {
        Mbc0Dom0Mem0BlkNseW1Bit3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit3) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit30 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit30 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit30 {
        Mbc0Dom0Mem0BlkNseW1Bit30::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit30> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit30) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit31 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit31 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit31 {
        Mbc0Dom0Mem0BlkNseW1Bit31::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit31> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit31) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit4 {
        Mbc0Dom0Mem0BlkNseW1Bit4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit4) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit5 {
        Mbc0Dom0Mem0BlkNseW1Bit5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit5) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit6 {
        Mbc0Dom0Mem0BlkNseW1Bit6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit6) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit7 {
        Mbc0Dom0Mem0BlkNseW1Bit7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit7) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit8 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit8 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit8 {
        Mbc0Dom0Mem0BlkNseW1Bit8::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit8> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit8) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit9 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit9 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit9 {
        Mbc0Dom0Mem0BlkNseW1Bit9::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit9> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit9) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
pub enum Mbc0Dom0Mem1BlkNseW0Bit0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit0 {
        Mbc0Dom0Mem1BlkNseW0Bit0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit0) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit1 {
        Mbc0Dom0Mem1BlkNseW0Bit1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit1) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit10 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit10 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit10 {
        Mbc0Dom0Mem1BlkNseW0Bit10::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit10> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit10) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit11 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit11 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit11 {
        Mbc0Dom0Mem1BlkNseW0Bit11::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit11> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit11) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit12 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit12 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit12 {
        Mbc0Dom0Mem1BlkNseW0Bit12::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit12> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit12) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit13 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit13 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit13 {
        Mbc0Dom0Mem1BlkNseW0Bit13::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit13> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit13) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit14 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit14 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit14 {
        Mbc0Dom0Mem1BlkNseW0Bit14::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit14> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit14) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit15 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit15 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit15 {
        Mbc0Dom0Mem1BlkNseW0Bit15::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit15> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit15) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit16 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit16 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit16 {
        Mbc0Dom0Mem1BlkNseW0Bit16::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit16> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit16) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit17 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit17 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit17 {
        Mbc0Dom0Mem1BlkNseW0Bit17::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit17> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit17) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit18 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit18 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit18 {
        Mbc0Dom0Mem1BlkNseW0Bit18::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit18> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit18) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit19 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit19 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit19 {
        Mbc0Dom0Mem1BlkNseW0Bit19::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit19> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit19) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit2 {
        Mbc0Dom0Mem1BlkNseW0Bit2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit2) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit20 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit20 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit20 {
        Mbc0Dom0Mem1BlkNseW0Bit20::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit20> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit20) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit21 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit21 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit21 {
        Mbc0Dom0Mem1BlkNseW0Bit21::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit21> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit21) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit22 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit22 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit22 {
        Mbc0Dom0Mem1BlkNseW0Bit22::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit22> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit22) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit23 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit23 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit23 {
        Mbc0Dom0Mem1BlkNseW0Bit23::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit23> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit23) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit24 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit24 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit24 {
        Mbc0Dom0Mem1BlkNseW0Bit24::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit24> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit24) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit25 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit25 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit25 {
        Mbc0Dom0Mem1BlkNseW0Bit25::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit25> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit25) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit26 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit26 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit26 {
        Mbc0Dom0Mem1BlkNseW0Bit26::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit26> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit26) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit27 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit27 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit27 {
        Mbc0Dom0Mem1BlkNseW0Bit27::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit27> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit27) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit28 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit28 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit28 {
        Mbc0Dom0Mem1BlkNseW0Bit28::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit28> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit28) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit29 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit29 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit29 {
        Mbc0Dom0Mem1BlkNseW0Bit29::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit29> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit29) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit3 {
        Mbc0Dom0Mem1BlkNseW0Bit3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit3) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit30 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit30 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit30 {
        Mbc0Dom0Mem1BlkNseW0Bit30::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit30> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit30) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit31 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit31 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit31 {
        Mbc0Dom0Mem1BlkNseW0Bit31::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit31> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit31) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit4 {
        Mbc0Dom0Mem1BlkNseW0Bit4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit4) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit5 {
        Mbc0Dom0Mem1BlkNseW0Bit5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit5) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit6 {
        Mbc0Dom0Mem1BlkNseW0Bit6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit6) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit7 {
        Mbc0Dom0Mem1BlkNseW0Bit7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit7) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit8 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit8 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit8 {
        Mbc0Dom0Mem1BlkNseW0Bit8::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit8> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit8) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit9 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit9 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit9 {
        Mbc0Dom0Mem1BlkNseW0Bit9::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit9> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit9) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit0 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit0 {
        Mbc0Dom0Mem2BlkNseW0Bit0::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit0) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit1 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit1 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit1 {
        Mbc0Dom0Mem2BlkNseW0Bit1::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit1> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit1) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit10 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit10 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit10 {
        Mbc0Dom0Mem2BlkNseW0Bit10::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit10> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit10) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit11 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit11 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit11 {
        Mbc0Dom0Mem2BlkNseW0Bit11::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit11> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit11) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit12 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit12 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit12 {
        Mbc0Dom0Mem2BlkNseW0Bit12::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit12> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit12) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit13 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit13 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit13 {
        Mbc0Dom0Mem2BlkNseW0Bit13::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit13> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit13) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit14 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit14 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit14 {
        Mbc0Dom0Mem2BlkNseW0Bit14::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit14> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit14) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit15 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit15 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit15 {
        Mbc0Dom0Mem2BlkNseW0Bit15::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit15> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit15) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit16 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit16 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit16 {
        Mbc0Dom0Mem2BlkNseW0Bit16::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit16> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit16) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit17 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit17 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit17 {
        Mbc0Dom0Mem2BlkNseW0Bit17::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit17> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit17) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit18 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit18 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit18 {
        Mbc0Dom0Mem2BlkNseW0Bit18::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit18> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit18) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit19 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit19 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit19 {
        Mbc0Dom0Mem2BlkNseW0Bit19::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit19> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit19) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit2 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit2 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit2 {
        Mbc0Dom0Mem2BlkNseW0Bit2::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit2> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit2) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit20 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit20 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit20 {
        Mbc0Dom0Mem2BlkNseW0Bit20::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit20> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit20) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit21 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit21 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit21 {
        Mbc0Dom0Mem2BlkNseW0Bit21::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit21> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit21) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit22 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit22 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit22 {
        Mbc0Dom0Mem2BlkNseW0Bit22::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit22> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit22) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit23 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit23 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit23 {
        Mbc0Dom0Mem2BlkNseW0Bit23::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit23> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit23) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit24 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit24 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit24 {
        Mbc0Dom0Mem2BlkNseW0Bit24::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit24> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit24) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit25 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit25 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit25 {
        Mbc0Dom0Mem2BlkNseW0Bit25::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit25> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit25) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit26 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit26 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit26 {
        Mbc0Dom0Mem2BlkNseW0Bit26::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit26> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit26) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit27 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit27 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit27 {
        Mbc0Dom0Mem2BlkNseW0Bit27::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit27> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit27) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit28 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit28 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit28 {
        Mbc0Dom0Mem2BlkNseW0Bit28::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit28> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit28) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit29 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit29 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit29 {
        Mbc0Dom0Mem2BlkNseW0Bit29::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit29> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit29) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit3 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit3 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit3 {
        Mbc0Dom0Mem2BlkNseW0Bit3::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit3> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit3) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit30 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit30 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit30 {
        Mbc0Dom0Mem2BlkNseW0Bit30::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit30> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit30) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit31 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit31 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit31 {
        Mbc0Dom0Mem2BlkNseW0Bit31::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit31> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit31) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit4 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit4 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit4 {
        Mbc0Dom0Mem2BlkNseW0Bit4::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit4> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit4) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit5 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit5 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit5 {
        Mbc0Dom0Mem2BlkNseW0Bit5::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit5> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit5) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit6 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit6 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit6 {
        Mbc0Dom0Mem2BlkNseW0Bit6::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit6> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit6) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit7 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit7 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit7 {
        Mbc0Dom0Mem2BlkNseW0Bit7::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit7> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit7) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit8 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit8 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit8 {
        Mbc0Dom0Mem2BlkNseW0Bit8::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit8> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit8) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit9 {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    ALLOWED = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    NOTALLOWED = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit9 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit9 {
        Mbc0Dom0Mem2BlkNseW0Bit9::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit9> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit9) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0NseBlkClrAllDidSel0 {
    #[doc = "No effect."]
    LOGIC_0 = 0x0,
    #[doc = "Clear all NSE bits for this domain."]
    LOGIC_1 = 0x01,
}
impl Mbc0NseBlkClrAllDidSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0NseBlkClrAllDidSel0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0NseBlkClrAllDidSel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0NseBlkClrAllDidSel0 {
        Mbc0NseBlkClrAllDidSel0::from_bits(val)
    }
}
impl From<Mbc0NseBlkClrAllDidSel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0NseBlkClrAllDidSel0) -> u8 {
        Mbc0NseBlkClrAllDidSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0NseBlkIndexDidSel0 {
    #[doc = "No effect."]
    LOGIC_0 = 0x0,
    #[doc = "Selects NSE bits for this domain."]
    LOGIC_1 = 0x01,
}
impl Mbc0NseBlkIndexDidSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0NseBlkIndexDidSel0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0NseBlkIndexDidSel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0NseBlkIndexDidSel0 {
        Mbc0NseBlkIndexDidSel0::from_bits(val)
    }
}
impl From<Mbc0NseBlkIndexDidSel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0NseBlkIndexDidSel0) -> u8 {
        Mbc0NseBlkIndexDidSel0::to_bits(val)
    }
}
