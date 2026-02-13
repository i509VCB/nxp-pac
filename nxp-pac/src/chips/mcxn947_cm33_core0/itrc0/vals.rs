#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum In2421Status {
    #[doc = "Output not triggered."]
    DISABLE = 0x0,
    #[doc = "Output has been triggered."]
    ENABLE = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl In2421Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> In2421Status {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for In2421Status {
    #[inline(always)]
    fn from(val: u8) -> In2421Status {
        In2421Status::from_bits(val)
    }
}
impl From<In2421Status> for u8 {
    #[inline(always)]
    fn from(val: In2421Status) -> u8 {
        In2421Status::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct In3225Status(u8);
impl In3225Status {
    #[doc = "Output not triggered."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Output has been triggered."]
    pub const ENABLE: Self = Self(0x01);
}
impl In3225Status {
    pub const fn from_bits(val: u8) -> In3225Status {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for In3225Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for In3225Status {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for In3225Status {
    #[inline(always)]
    fn from(val: u8) -> In3225Status {
        In3225Status::from_bits(val)
    }
}
impl From<In3225Status> for u8 {
    #[inline(always)]
    fn from(val: In3225Status) -> u8 {
        In3225Status::to_bits(val)
    }
}
