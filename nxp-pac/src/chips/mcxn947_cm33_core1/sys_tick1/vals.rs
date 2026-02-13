#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksource {
    #[doc = "external clock"]
    EXTERNAL_CLOCK = 0x0,
    #[doc = "processor clock"]
    PROCESSOR_CLOCK = 0x01,
}
impl Clksource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksource {
    #[inline(always)]
    fn from(val: u8) -> Clksource {
        Clksource::from_bits(val)
    }
}
impl From<Clksource> for u8 {
    #[inline(always)]
    fn from(val: Clksource) -> u8 {
        Clksource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enable {
    #[doc = "counter disabled"]
    COUNTER_DISABLED = 0x0,
    #[doc = "counter enabled"]
    COUNTER_ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Noref {
    #[doc = "The alternative reference clock is provided"]
    CLOCK_PROVIDED = 0x0,
    #[doc = "The alternative reference clock is not provided"]
    CLOCK_DISABLED = 0x01,
}
impl Noref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Noref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Noref {
    #[inline(always)]
    fn from(val: u8) -> Noref {
        Noref::from_bits(val)
    }
}
impl From<Noref> for u8 {
    #[inline(always)]
    fn from(val: Noref) -> u8 {
        Noref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skew {
    #[doc = "10ms calibration value is exact"]
    EXACT_VALUE = 0x0,
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    INEXACT_VALUE = 0x01,
}
impl Skew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skew {
    #[inline(always)]
    fn from(val: u8) -> Skew {
        Skew::from_bits(val)
    }
}
impl From<Skew> for u8 {
    #[inline(always)]
    fn from(val: Skew) -> u8 {
        Skew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tickint {
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    INTERRUPT_DISABLED = 0x0,
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    INTERRUPT_ENABLED = 0x01,
}
impl Tickint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tickint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tickint {
    #[inline(always)]
    fn from(val: u8) -> Tickint {
        Tickint::from_bits(val)
    }
}
impl From<Tickint> for u8 {
    #[inline(always)]
    fn from(val: Tickint) -> u8 {
        Tickint::to_bits(val)
    }
}
