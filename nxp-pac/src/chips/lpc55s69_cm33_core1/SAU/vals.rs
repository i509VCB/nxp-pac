#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ALLNS {
    #[doc = "Memory is marked as Secure and is not Non-secure callable."]
    SECURED_MEMORY = 0x0,
    #[doc = "Memory is marked as Non-secure."]
    NON_SECURED_MEMORY = 0x01,
}
impl ALLNS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ALLNS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ALLNS {
    #[inline(always)]
    fn from(val: u8) -> ALLNS {
        ALLNS::from_bits(val)
    }
}
impl From<ALLNS> for u8 {
    #[inline(always)]
    fn from(val: ALLNS) -> u8 {
        ALLNS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RLAR_ENABLE {
    #[doc = "SAU region is enabled."]
    ENABLED = 0x0,
    #[doc = "SAU region is disabled."]
    DISABLED = 0x01,
}
impl RLAR_ENABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RLAR_ENABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RLAR_ENABLE {
    #[inline(always)]
    fn from(val: u8) -> RLAR_ENABLE {
        RLAR_ENABLE::from_bits(val)
    }
}
impl From<RLAR_ENABLE> for u8 {
    #[inline(always)]
    fn from(val: RLAR_ENABLE) -> u8 {
        RLAR_ENABLE::to_bits(val)
    }
}
