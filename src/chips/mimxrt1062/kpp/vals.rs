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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Kdie {
    #[doc = "No interrupt request is generated when KPKD is set."]
    KDIE_0 = 0x0,
    #[doc = "An interrupt request is generated when KPKD is set."]
    KDIE_1 = 0x01,
}
impl Kdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Kdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Kdie {
    #[inline(always)]
    fn from(val: u8) -> Kdie {
        Kdie::from_bits(val)
    }
}
impl From<Kdie> for u8 {
    #[inline(always)]
    fn from(val: Kdie) -> u8 {
        Kdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Kdsc {
    #[doc = "No effect"]
    KDSC_0 = 0x0,
    #[doc = "Set bits that clear the keypad depress synchronizer chain"]
    KDSC_1 = 0x01,
}
impl Kdsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Kdsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Kdsc {
    #[inline(always)]
    fn from(val: u8) -> Kdsc {
        Kdsc::from_bits(val)
    }
}
impl From<Kdsc> for u8 {
    #[inline(always)]
    fn from(val: Kdsc) -> u8 {
        Kdsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Kpkd {
    #[doc = "No key presses detected"]
    KPKD_0 = 0x0,
    #[doc = "A key has been depressed"]
    KPKD_1 = 0x01,
}
impl Kpkd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Kpkd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Kpkd {
    #[inline(always)]
    fn from(val: u8) -> Kpkd {
        Kpkd::from_bits(val)
    }
}
impl From<Kpkd> for u8 {
    #[inline(always)]
    fn from(val: Kpkd) -> u8 {
        Kpkd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Kpkr {
    #[doc = "No key release detected"]
    KPKR_0 = 0x0,
    #[doc = "All keys have been released"]
    KPKR_1 = 0x01,
}
impl Kpkr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Kpkr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Kpkr {
    #[inline(always)]
    fn from(val: u8) -> Kpkr {
        Kpkr::from_bits(val)
    }
}
impl From<Kpkr> for u8 {
    #[inline(always)]
    fn from(val: Kpkr) -> u8 {
        Kpkr::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krie {
    #[doc = "No interrupt request is generated when KPKR is set."]
    KRIE_0 = 0x0,
    #[doc = "An interrupt request is generated when KPKR is set."]
    KRIE_1 = 0x01,
}
impl Krie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krie {
    #[inline(always)]
    fn from(val: u8) -> Krie {
        Krie::from_bits(val)
    }
}
impl From<Krie> for u8 {
    #[inline(always)]
    fn from(val: Krie) -> u8 {
        Krie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Krss {
    #[doc = "No effect"]
    KRSS_0 = 0x0,
    #[doc = "Set bits which sets keypad release synchronizer chain"]
    KRSS_1 = 0x01,
}
impl Krss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Krss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Krss {
    #[inline(always)]
    fn from(val: u8) -> Krss {
        Krss::from_bits(val)
    }
}
impl From<Krss> for u8 {
    #[inline(always)]
    fn from(val: Krss) -> u8 {
        Krss::to_bits(val)
    }
}
