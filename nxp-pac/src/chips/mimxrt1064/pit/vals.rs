#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frz {
    #[doc = "Timers continue to run in Debug mode."]
    T000001 = 0x0,
    #[doc = "Timers are stopped in Debug mode."]
    T0000011 = 0x01,
}
impl Frz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frz {
    #[inline(always)]
    fn from(val: u8) -> Frz {
        Frz::from_bits(val)
    }
}
impl From<Frz> for u8 {
    #[inline(always)]
    fn from(val: Frz) -> u8 {
        Frz::to_bits(val)
    }
}
