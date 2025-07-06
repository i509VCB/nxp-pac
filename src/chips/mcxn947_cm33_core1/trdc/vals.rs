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
pub enum Mbc0MemnGlbac0Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Npr {
        Mbc0MemnGlbac0Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Npr) -> u8 {
        Mbc0MemnGlbac0Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Npw {
        Mbc0MemnGlbac0Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Npw) -> u8 {
        Mbc0MemnGlbac0Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Npx {
        Mbc0MemnGlbac0Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Npx) -> u8 {
        Mbc0MemnGlbac0Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Nur {
        Mbc0MemnGlbac0Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Nur) -> u8 {
        Mbc0MemnGlbac0Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Nuw {
        Mbc0MemnGlbac0Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Nuw) -> u8 {
        Mbc0MemnGlbac0Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Nux {
        Mbc0MemnGlbac0Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Nux) -> u8 {
        Mbc0MemnGlbac0Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Spr {
        Mbc0MemnGlbac0Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Spr) -> u8 {
        Mbc0MemnGlbac0Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Spw {
        Mbc0MemnGlbac0Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Spw) -> u8 {
        Mbc0MemnGlbac0Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Spx {
        Mbc0MemnGlbac0Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Spx) -> u8 {
        Mbc0MemnGlbac0Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Sur {
        Mbc0MemnGlbac0Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Sur) -> u8 {
        Mbc0MemnGlbac0Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Suw {
        Mbc0MemnGlbac0Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Suw) -> u8 {
        Mbc0MemnGlbac0Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac0Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac0Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac0Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac0Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac0Sux {
        Mbc0MemnGlbac0Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac0Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac0Sux) -> u8 {
        Mbc0MemnGlbac0Sux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Lk {
    #[doc = "This register is not locked and can be altered."]
    UNLOCKED = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    LOCKED = 0x01,
}
impl Mbc0MemnGlbac1Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Lk {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Lk {
        Mbc0MemnGlbac1Lk::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Lk> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Lk) -> u8 {
        Mbc0MemnGlbac1Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Npr {
        Mbc0MemnGlbac1Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Npr) -> u8 {
        Mbc0MemnGlbac1Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Npw {
        Mbc0MemnGlbac1Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Npw) -> u8 {
        Mbc0MemnGlbac1Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Npx {
        Mbc0MemnGlbac1Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Npx) -> u8 {
        Mbc0MemnGlbac1Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Nur {
        Mbc0MemnGlbac1Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Nur) -> u8 {
        Mbc0MemnGlbac1Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Nuw {
        Mbc0MemnGlbac1Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Nuw) -> u8 {
        Mbc0MemnGlbac1Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Nux {
        Mbc0MemnGlbac1Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Nux) -> u8 {
        Mbc0MemnGlbac1Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Spr {
        Mbc0MemnGlbac1Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Spr) -> u8 {
        Mbc0MemnGlbac1Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Spw {
        Mbc0MemnGlbac1Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Spw) -> u8 {
        Mbc0MemnGlbac1Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Spx {
        Mbc0MemnGlbac1Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Spx) -> u8 {
        Mbc0MemnGlbac1Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Sur {
        Mbc0MemnGlbac1Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Sur) -> u8 {
        Mbc0MemnGlbac1Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Suw {
        Mbc0MemnGlbac1Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Suw) -> u8 {
        Mbc0MemnGlbac1Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac1Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac1Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac1Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac1Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac1Sux {
        Mbc0MemnGlbac1Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac1Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac1Sux) -> u8 {
        Mbc0MemnGlbac1Sux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Lk {
    #[doc = "This register is not locked and can be altered."]
    UNLOCKED = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    LOCKED = 0x01,
}
impl Mbc0MemnGlbac2Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Lk {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Lk {
        Mbc0MemnGlbac2Lk::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Lk> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Lk) -> u8 {
        Mbc0MemnGlbac2Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Npr {
        Mbc0MemnGlbac2Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Npr) -> u8 {
        Mbc0MemnGlbac2Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Npw {
        Mbc0MemnGlbac2Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Npw) -> u8 {
        Mbc0MemnGlbac2Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Npx {
        Mbc0MemnGlbac2Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Npx) -> u8 {
        Mbc0MemnGlbac2Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Nur {
        Mbc0MemnGlbac2Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Nur) -> u8 {
        Mbc0MemnGlbac2Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Nuw {
        Mbc0MemnGlbac2Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Nuw) -> u8 {
        Mbc0MemnGlbac2Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Nux {
        Mbc0MemnGlbac2Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Nux) -> u8 {
        Mbc0MemnGlbac2Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Spr {
        Mbc0MemnGlbac2Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Spr) -> u8 {
        Mbc0MemnGlbac2Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Spw {
        Mbc0MemnGlbac2Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Spw) -> u8 {
        Mbc0MemnGlbac2Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Spx {
        Mbc0MemnGlbac2Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Spx) -> u8 {
        Mbc0MemnGlbac2Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Sur {
        Mbc0MemnGlbac2Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Sur) -> u8 {
        Mbc0MemnGlbac2Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Suw {
        Mbc0MemnGlbac2Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Suw) -> u8 {
        Mbc0MemnGlbac2Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac2Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac2Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac2Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac2Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac2Sux {
        Mbc0MemnGlbac2Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac2Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac2Sux) -> u8 {
        Mbc0MemnGlbac2Sux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Lk {
    #[doc = "This register is not locked and can be altered."]
    UNLOCKED = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    LOCKED = 0x01,
}
impl Mbc0MemnGlbac3Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Lk {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Lk {
        Mbc0MemnGlbac3Lk::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Lk> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Lk) -> u8 {
        Mbc0MemnGlbac3Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Npr {
        Mbc0MemnGlbac3Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Npr) -> u8 {
        Mbc0MemnGlbac3Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Npw {
        Mbc0MemnGlbac3Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Npw) -> u8 {
        Mbc0MemnGlbac3Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Npx {
        Mbc0MemnGlbac3Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Npx) -> u8 {
        Mbc0MemnGlbac3Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Nur {
        Mbc0MemnGlbac3Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Nur) -> u8 {
        Mbc0MemnGlbac3Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Nuw {
        Mbc0MemnGlbac3Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Nuw) -> u8 {
        Mbc0MemnGlbac3Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Nux {
        Mbc0MemnGlbac3Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Nux) -> u8 {
        Mbc0MemnGlbac3Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Spr {
        Mbc0MemnGlbac3Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Spr) -> u8 {
        Mbc0MemnGlbac3Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Spw {
        Mbc0MemnGlbac3Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Spw) -> u8 {
        Mbc0MemnGlbac3Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Spx {
        Mbc0MemnGlbac3Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Spx) -> u8 {
        Mbc0MemnGlbac3Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Sur {
        Mbc0MemnGlbac3Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Sur) -> u8 {
        Mbc0MemnGlbac3Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Suw {
        Mbc0MemnGlbac3Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Suw) -> u8 {
        Mbc0MemnGlbac3Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac3Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac3Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac3Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac3Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac3Sux {
        Mbc0MemnGlbac3Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac3Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac3Sux) -> u8 {
        Mbc0MemnGlbac3Sux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Lk {
    #[doc = "This register is not locked and can be altered."]
    UNLOCKED = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    LOCKED = 0x01,
}
impl Mbc0MemnGlbac4Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Lk {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Lk {
        Mbc0MemnGlbac4Lk::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Lk> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Lk) -> u8 {
        Mbc0MemnGlbac4Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Npr {
        Mbc0MemnGlbac4Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Npr) -> u8 {
        Mbc0MemnGlbac4Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Npw {
        Mbc0MemnGlbac4Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Npw) -> u8 {
        Mbc0MemnGlbac4Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Npx {
        Mbc0MemnGlbac4Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Npx) -> u8 {
        Mbc0MemnGlbac4Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Nur {
        Mbc0MemnGlbac4Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Nur) -> u8 {
        Mbc0MemnGlbac4Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Nuw {
        Mbc0MemnGlbac4Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Nuw) -> u8 {
        Mbc0MemnGlbac4Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Nux {
        Mbc0MemnGlbac4Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Nux) -> u8 {
        Mbc0MemnGlbac4Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Spr {
        Mbc0MemnGlbac4Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Spr) -> u8 {
        Mbc0MemnGlbac4Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Spw {
        Mbc0MemnGlbac4Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Spw) -> u8 {
        Mbc0MemnGlbac4Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Spx {
        Mbc0MemnGlbac4Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Spx) -> u8 {
        Mbc0MemnGlbac4Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Sur {
        Mbc0MemnGlbac4Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Sur) -> u8 {
        Mbc0MemnGlbac4Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Suw {
        Mbc0MemnGlbac4Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Suw) -> u8 {
        Mbc0MemnGlbac4Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac4Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac4Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac4Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac4Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac4Sux {
        Mbc0MemnGlbac4Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac4Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac4Sux) -> u8 {
        Mbc0MemnGlbac4Sux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Lk {
    #[doc = "This register is not locked and can be altered."]
    UNLOCKED = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    LOCKED = 0x01,
}
impl Mbc0MemnGlbac5Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Lk {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Lk {
        Mbc0MemnGlbac5Lk::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Lk> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Lk) -> u8 {
        Mbc0MemnGlbac5Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Npr {
        Mbc0MemnGlbac5Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Npr) -> u8 {
        Mbc0MemnGlbac5Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Npw {
        Mbc0MemnGlbac5Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Npw) -> u8 {
        Mbc0MemnGlbac5Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Npx {
        Mbc0MemnGlbac5Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Npx) -> u8 {
        Mbc0MemnGlbac5Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Nur {
        Mbc0MemnGlbac5Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Nur) -> u8 {
        Mbc0MemnGlbac5Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Nuw {
        Mbc0MemnGlbac5Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Nuw) -> u8 {
        Mbc0MemnGlbac5Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Nux {
        Mbc0MemnGlbac5Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Nux) -> u8 {
        Mbc0MemnGlbac5Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Spr {
        Mbc0MemnGlbac5Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Spr) -> u8 {
        Mbc0MemnGlbac5Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Spw {
        Mbc0MemnGlbac5Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Spw) -> u8 {
        Mbc0MemnGlbac5Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Spx {
        Mbc0MemnGlbac5Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Spx) -> u8 {
        Mbc0MemnGlbac5Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Sur {
        Mbc0MemnGlbac5Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Sur) -> u8 {
        Mbc0MemnGlbac5Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Suw {
        Mbc0MemnGlbac5Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Suw) -> u8 {
        Mbc0MemnGlbac5Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac5Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac5Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac5Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac5Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac5Sux {
        Mbc0MemnGlbac5Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac5Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac5Sux) -> u8 {
        Mbc0MemnGlbac5Sux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Lk {
    #[doc = "This register is not locked and can be altered."]
    UNLOCKED = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    LOCKED = 0x01,
}
impl Mbc0MemnGlbac6Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Lk {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Lk {
        Mbc0MemnGlbac6Lk::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Lk> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Lk) -> u8 {
        Mbc0MemnGlbac6Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Npr {
        Mbc0MemnGlbac6Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Npr) -> u8 {
        Mbc0MemnGlbac6Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Npw {
        Mbc0MemnGlbac6Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Npw) -> u8 {
        Mbc0MemnGlbac6Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Npx {
        Mbc0MemnGlbac6Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Npx) -> u8 {
        Mbc0MemnGlbac6Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Nur {
        Mbc0MemnGlbac6Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Nur) -> u8 {
        Mbc0MemnGlbac6Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Nuw {
        Mbc0MemnGlbac6Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Nuw) -> u8 {
        Mbc0MemnGlbac6Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Nux {
        Mbc0MemnGlbac6Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Nux) -> u8 {
        Mbc0MemnGlbac6Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Spr {
        Mbc0MemnGlbac6Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Spr) -> u8 {
        Mbc0MemnGlbac6Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Spw {
        Mbc0MemnGlbac6Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Spw) -> u8 {
        Mbc0MemnGlbac6Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Spx {
        Mbc0MemnGlbac6Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Spx) -> u8 {
        Mbc0MemnGlbac6Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Sur {
        Mbc0MemnGlbac6Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Sur) -> u8 {
        Mbc0MemnGlbac6Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Suw {
        Mbc0MemnGlbac6Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Suw) -> u8 {
        Mbc0MemnGlbac6Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac6Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac6Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac6Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac6Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac6Sux {
        Mbc0MemnGlbac6Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac6Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac6Sux) -> u8 {
        Mbc0MemnGlbac6Sux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Lk {
    #[doc = "This register is not locked and can be altered."]
    UNLOCKED = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    LOCKED = 0x01,
}
impl Mbc0MemnGlbac7Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Lk {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Lk {
        Mbc0MemnGlbac7Lk::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Lk> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Lk) -> u8 {
        Mbc0MemnGlbac7Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Npr {
    #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Npr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Npr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Npr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Npr {
        Mbc0MemnGlbac7Npr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Npr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Npr) -> u8 {
        Mbc0MemnGlbac7Npr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Npw {
    #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Npw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Npw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Npw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Npw {
        Mbc0MemnGlbac7Npw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Npw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Npw) -> u8 {
        Mbc0MemnGlbac7Npw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Npx {
    #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Npx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Npx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Npx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Npx {
        Mbc0MemnGlbac7Npx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Npx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Npx) -> u8 {
        Mbc0MemnGlbac7Npx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Nur {
    #[doc = "Read access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Nur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Nur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Nur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Nur {
        Mbc0MemnGlbac7Nur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Nur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Nur) -> u8 {
        Mbc0MemnGlbac7Nur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Nuw {
    #[doc = "Write access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Nuw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Nuw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Nuw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Nuw {
        Mbc0MemnGlbac7Nuw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Nuw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Nuw) -> u8 {
        Mbc0MemnGlbac7Nuw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Nux {
    #[doc = "Execute access is not allowed in Nonsecure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Nonsecure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Nux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Nux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Nux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Nux {
        Mbc0MemnGlbac7Nux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Nux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Nux) -> u8 {
        Mbc0MemnGlbac7Nux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Spr {
    #[doc = "Read access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Spr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Spr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Spr {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Spr {
        Mbc0MemnGlbac7Spr::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Spr> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Spr) -> u8 {
        Mbc0MemnGlbac7Spr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Spw {
    #[doc = "Write access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Spw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Spw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Spw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Spw {
        Mbc0MemnGlbac7Spw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Spw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Spw) -> u8 {
        Mbc0MemnGlbac7Spw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Spx {
    #[doc = "Execute access is not allowed in Secure Privilege mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure Privilege mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Spx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Spx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Spx {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Spx {
        Mbc0MemnGlbac7Spx::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Spx> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Spx) -> u8 {
        Mbc0MemnGlbac7Spx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Sur {
    #[doc = "Read access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Read access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Sur {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Sur {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Sur {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Sur {
        Mbc0MemnGlbac7Sur::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Sur> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Sur) -> u8 {
        Mbc0MemnGlbac7Sur::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Suw {
    #[doc = "Write access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Write access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Suw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Suw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Suw {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Suw {
        Mbc0MemnGlbac7Suw::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Suw> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Suw) -> u8 {
        Mbc0MemnGlbac7Suw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0MemnGlbac7Sux {
    #[doc = "Execute access is not allowed in Secure User mode."]
    NOTALLOWED = 0x0,
    #[doc = "Execute access is allowed in Secure User mode."]
    ALLOWED = 0x01,
}
impl Mbc0MemnGlbac7Sux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0MemnGlbac7Sux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0MemnGlbac7Sux {
    #[inline(always)]
    fn from(val: u8) -> Mbc0MemnGlbac7Sux {
        Mbc0MemnGlbac7Sux::from_bits(val)
    }
}
impl From<Mbc0MemnGlbac7Sux> for u8 {
    #[inline(always)]
    fn from(val: Mbc0MemnGlbac7Sux) -> u8 {
        Mbc0MemnGlbac7Sux::to_bits(val)
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
