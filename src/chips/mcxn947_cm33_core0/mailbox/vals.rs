#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ex {
    #[doc = "Resource unavailable"]
    UNAVAILABLE = 0x0,
    #[doc = "Resource available"]
    AVAILABLE = 0x01,
}
impl Ex {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ex {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ex {
    #[inline(always)]
    fn from(val: u8) -> Ex {
        Ex::from_bits(val)
    }
}
impl From<Ex> for u8 {
    #[inline(always)]
    fn from(val: Ex) -> u8 {
        Ex::to_bits(val)
    }
}
