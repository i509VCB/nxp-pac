#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Datafix(u8);
impl Datafix {
    #[doc = "Address comparator triggers a opcode patch"]
    pub const DATAFIX_0: Self = Self(0x0);
    #[doc = "Address comparator triggers a data fix"]
    pub const DATAFIX_1: Self = Self(0x01);
}
impl Datafix {
    pub const fn from_bits(val: u8) -> Datafix {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Datafix {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DATAFIX_0"),
            0x01 => f.write_str("DATAFIX_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Datafix {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DATAFIX_0"),
            0x01 => defmt::write!(f, "DATAFIX_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Datafix {
    #[inline(always)]
    fn from(val: u8) -> Datafix {
        Datafix::from_bits(val)
    }
}
impl From<Datafix> for u8 {
    #[inline(always)]
    fn from(val: Datafix) -> u8 {
        Datafix::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis {
    #[doc = "Does not affect any ROMC functions (default)"]
    DIS_0 = 0x0,
    #[doc = "Disable all ROMC functions: data fixing, and opcode patching"]
    DIS_1 = 0x01,
}
impl Dis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis {
    #[inline(always)]
    fn from(val: u8) -> Dis {
        Dis::from_bits(val)
    }
}
impl From<Dis> for u8 {
    #[inline(always)]
    fn from(val: Dis) -> u8 {
        Dis::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(u16);
impl Enable {
    #[doc = "Address comparator disabled"]
    pub const ENABLE_0: Self = Self(0x0);
    #[doc = "Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address"]
    pub const ENABLE_1: Self = Self(0x01);
}
impl Enable {
    pub const fn from_bits(val: u16) -> Enable {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Enable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENABLE_0"),
            0x01 => f.write_str("ENABLE_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENABLE_0"),
            0x01 => defmt::write!(f, "ENABLE_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Enable {
    #[inline(always)]
    fn from(val: u16) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u16 {
    #[inline(always)]
    fn from(val: Enable) -> u16 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Source {
    #[doc = "Address Comparator 0 matched"]
    SOURCE_0 = 0x0,
    #[doc = "Address Comparator 1 matched"]
    SOURCE_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Address Comparator 15 matched"]
    SOURCE_15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Source {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Source {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Source {
    #[inline(always)]
    fn from(val: u8) -> Source {
        Source::from_bits(val)
    }
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(val: Source) -> u8 {
        Source::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sw {
    #[doc = "no event or comparator collisions"]
    SW_0 = 0x0,
    #[doc = "a collision has occurred"]
    SW_1 = 0x01,
}
impl Sw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sw {
    #[inline(always)]
    fn from(val: u8) -> Sw {
        Sw::from_bits(val)
    }
}
impl From<Sw> for u8 {
    #[inline(always)]
    fn from(val: Sw) -> u8 {
        Sw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Thumbx {
    #[doc = "Arm patch"]
    THUMBX_0 = 0x0,
    #[doc = "THUMB patch (ignore if data fix)"]
    THUMBX_1 = 0x01,
}
impl Thumbx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Thumbx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Thumbx {
    #[inline(always)]
    fn from(val: u8) -> Thumbx {
        Thumbx::from_bits(val)
    }
}
impl From<Thumbx> for u8 {
    #[inline(always)]
    fn from(val: Thumbx) -> u8 {
        Thumbx::to_bits(val)
    }
}
