#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kcdd(u8);
impl Kcdd {
    #[doc = "COLn pin is configured as an input."]
    pub const INPUT: Self = Self(0x0);
    #[doc = "COLn pin is configured as an output."]
    pub const OUTPUT: Self = Self(0x01);
}
impl Kcdd {
    pub const fn from_bits(val: u8) -> Kcdd {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Kcdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("INPUT"),
            0x01 => f.write_str("OUTPUT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kcdd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "INPUT"),
            0x01 => defmt::write!(f, "OUTPUT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Kcdd {
    #[inline(always)]
    fn from(val: u8) -> Kcdd {
        Kcdd::from_bits(val)
    }
}
impl From<Kcdd> for u8 {
    #[inline(always)]
    fn from(val: Kcdd) -> u8 {
        Kcdd::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kco(u8);
impl Kco {
    #[doc = "Column strobe output is totem pole drive."]
    pub const TOTEM_POLE: Self = Self(0x0);
    #[doc = "Column strobe output is open drain."]
    pub const OPEN_DRAIN: Self = Self(0x01);
}
impl Kco {
    pub const fn from_bits(val: u8) -> Kco {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Kco {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TOTEM_POLE"),
            0x01 => f.write_str("OPEN_DRAIN"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kco {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TOTEM_POLE"),
            0x01 => defmt::write!(f, "OPEN_DRAIN"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Kco {
    #[inline(always)]
    fn from(val: u8) -> Kco {
        Kco::from_bits(val)
    }
}
impl From<Kco> for u8 {
    #[inline(always)]
    fn from(val: Kco) -> u8 {
        Kco::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Krdd(u8);
impl Krdd {
    #[doc = "ROWn pin configured as an input."]
    pub const INPUT: Self = Self(0x0);
    #[doc = "ROWn pin configured as an output."]
    pub const OUTPUT: Self = Self(0x01);
}
impl Krdd {
    pub const fn from_bits(val: u8) -> Krdd {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Krdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("INPUT"),
            0x01 => f.write_str("OUTPUT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Krdd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "INPUT"),
            0x01 => defmt::write!(f, "OUTPUT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Krdd {
    #[inline(always)]
    fn from(val: u8) -> Krdd {
        Krdd::from_bits(val)
    }
}
impl From<Krdd> for u8 {
    #[inline(always)]
    fn from(val: Krdd) -> u8 {
        Krdd::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kre(u8);
impl Kre {
    #[doc = "Row is not included in the keypad key press detect."]
    pub const KRE_0: Self = Self(0x0);
    #[doc = "Row is included in the keypad key press detect."]
    pub const KRE_1: Self = Self(0x01);
}
impl Kre {
    pub const fn from_bits(val: u8) -> Kre {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Kre {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("KRE_0"),
            0x01 => f.write_str("KRE_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kre {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "KRE_0"),
            0x01 => defmt::write!(f, "KRE_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Kre {
    #[inline(always)]
    fn from(val: u8) -> Kre {
        Kre::from_bits(val)
    }
}
impl From<Kre> for u8 {
    #[inline(always)]
    fn from(val: Kre) -> u8 {
        Kre::to_bits(val)
    }
}
