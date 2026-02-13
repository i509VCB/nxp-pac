#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr {
    #[doc = "Interrupt 0 is low-level sensitive."]
    LOW_LEVEL = 0x0,
    #[doc = "Interrupt 0 is high-level sensitive."]
    HIGH_LEVEL = 0x01,
    #[doc = "Interrupt 0 is rising-edge sensitive."]
    RISING_EDGE = 0x02,
    #[doc = "Interrupt 0 is falling-edge sensitive."]
    FALLING_EDGE = 0x03,
}
impl Icr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr {
    #[inline(always)]
    fn from(val: u8) -> Icr {
        Icr::from_bits(val)
    }
}
impl From<Icr> for u8 {
    #[inline(always)]
    fn from(val: Icr) -> u8 {
        Icr::to_bits(val)
    }
}
