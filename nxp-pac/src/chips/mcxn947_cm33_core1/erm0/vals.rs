#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc5 {
    #[doc = "No single-bit correction event on Memory 5 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 5 detected."]
    CORR_EVENT = 0x01,
}
impl Sbc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc5 {
    #[inline(always)]
    fn from(val: u8) -> Sbc5 {
        Sbc5::from_bits(val)
    }
}
impl From<Sbc5> for u8 {
    #[inline(always)]
    fn from(val: Sbc5) -> u8 {
        Sbc5::to_bits(val)
    }
}
