#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wden {
    #[doc = "Timer stopped"]
    STOP = 0x0,
    #[doc = "Timer running"]
    RUN = 0x01,
}
impl Wden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wden {
    #[inline(always)]
    fn from(val: u8) -> Wden {
        Wden::from_bits(val)
    }
}
impl From<Wden> for u8 {
    #[inline(always)]
    fn from(val: Wden) -> u8 {
        Wden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdprotect {
    #[doc = "Flexible"]
    FLEXIBLE = 0x0,
    #[doc = "Threshold"]
    THRESHOLD = 0x01,
}
impl Wdprotect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdprotect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdprotect {
    #[inline(always)]
    fn from(val: u8) -> Wdprotect {
        Wdprotect::from_bits(val)
    }
}
impl From<Wdprotect> for u8 {
    #[inline(always)]
    fn from(val: Wdprotect) -> u8 {
        Wdprotect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdreset {
    #[doc = "Interrupt"]
    INTERRUPT = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl Wdreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdreset {
    #[inline(always)]
    fn from(val: u8) -> Wdreset {
        Wdreset::from_bits(val)
    }
}
impl From<Wdreset> for u8 {
    #[inline(always)]
    fn from(val: Wdreset) -> u8 {
        Wdreset::to_bits(val)
    }
}
