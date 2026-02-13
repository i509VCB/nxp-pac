#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mulsize {
    _RESERVED_0 = 0x0,
    #[doc = "32-bit multiplier"]
    VALUE_32B = 0x01,
    #[doc = "64-bit multiplier"]
    VALUE_64B = 0x02,
    #[doc = "128-bit multiplier"]
    VALUE_128B = 0x03,
}
impl Mulsize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mulsize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mulsize {
    #[inline(always)]
    fn from(val: u8) -> Mulsize {
        Mulsize::from_bits(val)
    }
}
impl From<Mulsize> for u8 {
    #[inline(always)]
    fn from(val: Mulsize) -> u8 {
        Mulsize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Redmul {
    #[doc = "full size mode, 3 least significant bits of pointer and length are ignored, minimum supported length 0x0008"]
    FULLSZ = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "64-bit mode, 3 least significant bits of pointer and length are ignored, minimum supported length 0x0008"]
    VALUE_64BIT = 0x02,
    _RESERVED_3 = 0x03,
}
impl Redmul {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Redmul {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Redmul {
    #[inline(always)]
    fn from(val: u8) -> Redmul {
        Redmul::from_bits(val)
    }
}
impl From<Redmul> for u8 {
    #[inline(always)]
    fn from(val: Redmul) -> u8 {
        Redmul::to_bits(val)
    }
}
