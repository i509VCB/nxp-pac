#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Allns {
    #[doc = "Memory is marked as Secure and is not Non-secure callable."]
    SECURED_MEMORY = 0x0,
    #[doc = "Memory is marked as Non-secure."]
    NON_SECURED_MEMORY = 0x01,
}
impl Allns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Allns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Allns {
    #[inline(always)]
    fn from(val: u8) -> Allns {
        Allns::from_bits(val)
    }
}
impl From<Allns> for u8 {
    #[inline(always)]
    fn from(val: Allns) -> u8 {
        Allns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RlarEnable {
    #[doc = "SAU region is enabled."]
    ENABLED = 0x0,
    #[doc = "SAU region is disabled."]
    DISABLED = 0x01,
}
impl RlarEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RlarEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RlarEnable {
    #[inline(always)]
    fn from(val: u8) -> RlarEnable {
        RlarEnable::from_bits(val)
    }
}
impl From<RlarEnable> for u8 {
    #[inline(always)]
    fn from(val: RlarEnable) -> u8 {
        RlarEnable::to_bits(val)
    }
}
