#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remaplk {
    #[doc = "Lock disabled: can write to REMAP"]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to REMAP"]
    LOCK_ENABLED = 0x01,
}
impl Remaplk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remaplk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remaplk {
    #[inline(always)]
    fn from(val: u8) -> Remaplk {
        Remaplk::from_bits(val)
    }
}
impl From<Remaplk> for u8 {
    #[inline(always)]
    fn from(val: Remaplk) -> u8 {
        Remaplk::to_bits(val)
    }
}
