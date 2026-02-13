#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cenaf(u8);
impl Cenaf {
    #[doc = "No operation"]
    pub const CENAF_0: Self = Self(0x0);
    #[doc = "LOW-active interrupt selected or falling-edge interrupt disabled"]
    pub const CENAF_1: Self = Self(0x01);
}
impl Cenaf {
    pub const fn from_bits(val: u8) -> Cenaf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Cenaf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CENAF_0"),
            0x01 => f.write_str("CENAF_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cenaf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CENAF_0"),
            0x01 => defmt::write!(f, "CENAF_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Cenaf {
    #[inline(always)]
    fn from(val: u8) -> Cenaf {
        Cenaf::from_bits(val)
    }
}
impl From<Cenaf> for u8 {
    #[inline(always)]
    fn from(val: Cenaf) -> u8 {
        Cenaf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cenrl(u8);
impl Cenrl {
    #[doc = "No operation"]
    pub const CENRL_0: Self = Self(0x0);
    #[doc = "Disable rising edge or level interrupt"]
    pub const CENRL_1: Self = Self(0x01);
}
impl Cenrl {
    pub const fn from_bits(val: u8) -> Cenrl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Cenrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CENRL_0"),
            0x01 => f.write_str("CENRL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cenrl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CENRL_0"),
            0x01 => defmt::write!(f, "CENRL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Cenrl {
    #[inline(always)]
    fn from(val: u8) -> Cenrl {
        Cenrl::from_bits(val)
    }
}
impl From<Cenrl> for u8 {
    #[inline(always)]
    fn from(val: Cenrl) -> u8 {
        Cenrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg0 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg0 {
    #[inline(always)]
    fn from(val: u8) -> Cfg0 {
        Cfg0::from_bits(val)
    }
}
impl From<Cfg0> for u8 {
    #[inline(always)]
    fn from(val: Cfg0) -> u8 {
        Cfg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg1 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg1 {
    #[inline(always)]
    fn from(val: u8) -> Cfg1 {
        Cfg1::from_bits(val)
    }
}
impl From<Cfg1> for u8 {
    #[inline(always)]
    fn from(val: Cfg1) -> u8 {
        Cfg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg2 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg2 {
    #[inline(always)]
    fn from(val: u8) -> Cfg2 {
        Cfg2::from_bits(val)
    }
}
impl From<Cfg2> for u8 {
    #[inline(always)]
    fn from(val: Cfg2) -> u8 {
        Cfg2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg3 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg3 {
    #[inline(always)]
    fn from(val: u8) -> Cfg3 {
        Cfg3::from_bits(val)
    }
}
impl From<Cfg3> for u8 {
    #[inline(always)]
    fn from(val: Cfg3) -> u8 {
        Cfg3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg4 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg4 {
    #[inline(always)]
    fn from(val: u8) -> Cfg4 {
        Cfg4::from_bits(val)
    }
}
impl From<Cfg4> for u8 {
    #[inline(always)]
    fn from(val: Cfg4) -> u8 {
        Cfg4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg5 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg5 {
    #[inline(always)]
    fn from(val: u8) -> Cfg5 {
        Cfg5::from_bits(val)
    }
}
impl From<Cfg5> for u8 {
    #[inline(always)]
    fn from(val: Cfg5) -> u8 {
        Cfg5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg6 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg6 {
    #[inline(always)]
    fn from(val: u8) -> Cfg6 {
        Cfg6::from_bits(val)
    }
}
impl From<Cfg6> for u8 {
    #[inline(always)]
    fn from(val: Cfg6) -> u8 {
        Cfg6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg7 {
    #[doc = "Constant HIGH"]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge"]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge"]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge"]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level"]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level"]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0"]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event (Nonsticky rising or falling edge)"]
    EVENT = 0x07,
}
impl Cfg7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg7 {
    #[inline(always)]
    fn from(val: u8) -> Cfg7 {
        Cfg7::from_bits(val)
    }
}
impl From<Cfg7> for u8 {
    #[inline(always)]
    fn from(val: Cfg7) -> u8 {
        Cfg7::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enaf(u8);
impl Enaf {
    #[doc = "Disable (set active interrupt level LOW)"]
    pub const ENAF_0: Self = Self(0x0);
    #[doc = "Enable (set active interrupt level HIGH)"]
    pub const ENAF_1: Self = Self(0x01);
}
impl Enaf {
    pub const fn from_bits(val: u8) -> Enaf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Enaf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENAF_0"),
            0x01 => f.write_str("ENAF_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enaf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENAF_0"),
            0x01 => defmt::write!(f, "ENAF_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Enaf {
    #[inline(always)]
    fn from(val: u8) -> Enaf {
        Enaf::from_bits(val)
    }
}
impl From<Enaf> for u8 {
    #[inline(always)]
    fn from(val: Enaf) -> u8 {
        Enaf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enrl(u8);
impl Enrl {
    #[doc = "In bit n disables the corresponding interrupt"]
    pub const ENRL_0: Self = Self(0x0);
    #[doc = "In bit n enables the corresponding interrupt"]
    pub const ENRL_1: Self = Self(0x01);
}
impl Enrl {
    pub const fn from_bits(val: u8) -> Enrl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Enrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ENRL_0"),
            0x01 => f.write_str("ENRL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enrl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ENRL_0"),
            0x01 => defmt::write!(f, "ENRL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Enrl {
    #[inline(always)]
    fn from(val: u8) -> Enrl {
        Enrl::from_bits(val)
    }
}
impl From<Enrl> for u8 {
    #[inline(always)]
    fn from(val: Enrl) -> u8 {
        Enrl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fdet(u8);
impl Fdet {
    #[doc = "Read 0- No falling edge (since Reset or you wrote a 1 to this field last time), Write 0- No operation"]
    pub const FDET_0: Self = Self(0x0);
    #[doc = "Read 1- Falling edge (since Reset or you wrote a 1 to this field last time), Write 1- Clear falling-edge detection for this bit"]
    pub const FDET_1: Self = Self(0x01);
}
impl Fdet {
    pub const fn from_bits(val: u8) -> Fdet {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fdet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FDET_0"),
            0x01 => f.write_str("FDET_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdet {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FDET_0"),
            0x01 => defmt::write!(f, "FDET_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Fdet {
    #[inline(always)]
    fn from(val: u8) -> Fdet {
        Fdet::from_bits(val)
    }
}
impl From<Fdet> for u8 {
    #[inline(always)]
    fn from(val: Fdet) -> u8 {
        Fdet::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pmat(u8);
impl Pmat {
    #[doc = "The corresponding product term is matched by the current state of the appropriate inputs"]
    pub const PMAT_1: Self = Self(0x01);
}
impl Pmat {
    pub const fn from_bits(val: u8) -> Pmat {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pmat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("PMAT_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "PMAT_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pmat {
    #[inline(always)]
    fn from(val: u8) -> Pmat {
        Pmat::from_bits(val)
    }
}
impl From<Pmat> for u8 {
    #[inline(always)]
    fn from(val: Pmat) -> u8 {
        Pmat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pmode(u8);
impl Pmode {
    #[doc = "In bit n configures the interrupt to be edge-sensitive"]
    pub const ISEL_0: Self = Self(0x0);
    #[doc = "In bit n configures the interrupt to be level-sensitive"]
    pub const ISEL_1: Self = Self(0x01);
}
impl Pmode {
    pub const fn from_bits(val: u8) -> Pmode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ISEL_0"),
            0x01 => f.write_str("ISEL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ISEL_0"),
            0x01 => defmt::write!(f, "ISEL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pmode {
    #[inline(always)]
    fn from(val: u8) -> Pmode {
        Pmode::from_bits(val)
    }
}
impl From<Pmode> for u8 {
    #[inline(always)]
    fn from(val: Pmode) -> u8 {
        Pmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Endpoint"]
    ENDPOINT = 0x01,
}
impl ProdEndpts0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts0 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts0 {
        ProdEndpts0::from_bits(val)
    }
}
impl From<ProdEndpts0> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts0) -> u8 {
        ProdEndpts0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Endpoint"]
    ENDPOINT = 0x01,
}
impl ProdEndpts1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts1 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts1 {
        ProdEndpts1::from_bits(val)
    }
}
impl From<ProdEndpts1> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts1) -> u8 {
        ProdEndpts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Endpoint"]
    ENDPOINT = 0x01,
}
impl ProdEndpts2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts2 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts2 {
        ProdEndpts2::from_bits(val)
    }
}
impl From<ProdEndpts2> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts2) -> u8 {
        ProdEndpts2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Endpoint"]
    ENDPOINT = 0x01,
}
impl ProdEndpts3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts3 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts3 {
        ProdEndpts3::from_bits(val)
    }
}
impl From<ProdEndpts3> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts3) -> u8 {
        ProdEndpts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Endpoint"]
    ENDPOINT = 0x01,
}
impl ProdEndpts4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts4 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts4 {
        ProdEndpts4::from_bits(val)
    }
}
impl From<ProdEndpts4> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts4) -> u8 {
        ProdEndpts4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts5 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Endpoint"]
    ENDPOINT = 0x01,
}
impl ProdEndpts5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts5 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts5 {
        ProdEndpts5::from_bits(val)
    }
}
impl From<ProdEndpts5> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts5) -> u8 {
        ProdEndpts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts6 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Endpoint"]
    ENDPOINT = 0x01,
}
impl ProdEndpts6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts6 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts6 {
        ProdEndpts6::from_bits(val)
    }
}
impl From<ProdEndpts6> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts6) -> u8 {
        ProdEndpts6::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pstat(u8);
impl Pstat {
    #[doc = "Read 0- Interrupt is not requested, Write 0- No operation"]
    pub const PSTAT_0: Self = Self(0x0);
    #[doc = "Read 1- Interrupt is requested, Write 1 (edge-sensitive)- clear rising- and falling-edge detection for this pin, Write 1 (level-sensitive)- switch the active level for this pin in"]
    pub const PSTAT_1: Self = Self(0x01);
}
impl Pstat {
    pub const fn from_bits(val: u8) -> Pstat {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("PSTAT_0"),
            0x01 => f.write_str("PSTAT_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pstat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "PSTAT_0"),
            0x01 => defmt::write!(f, "PSTAT_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pstat {
    #[inline(always)]
    fn from(val: u8) -> Pstat {
        Pstat::from_bits(val)
    }
}
impl From<Pstat> for u8 {
    #[inline(always)]
    fn from(val: Pstat) -> u8 {
        Pstat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rdet(u8);
impl Rdet {
    #[doc = "Read 0- No rising edge (since Reset or you wrote a 1 to this field last time), Write 0- No operation"]
    pub const RDET_0: Self = Self(0x0);
    #[doc = "Read 1- Rising edge (since Reset or you wrote a 1 to this field last time), Write 1- Clear rising-edge detection for this pin"]
    pub const RDET_1: Self = Self(0x01);
}
impl Rdet {
    pub const fn from_bits(val: u8) -> Rdet {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Rdet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("RDET_0"),
            0x01 => f.write_str("RDET_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdet {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "RDET_0"),
            0x01 => defmt::write!(f, "RDET_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Rdet {
    #[inline(always)]
    fn from(val: u8) -> Rdet {
        Rdet::from_bits(val)
    }
}
impl From<Rdet> for u8 {
    #[inline(always)]
    fn from(val: Rdet) -> u8 {
        Rdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelPmatch {
    #[doc = "Pin interrupt"]
    PIN_INTERRUPT = 0x0,
    #[doc = "Pattern match"]
    PATTERN_MATCH = 0x01,
}
impl SelPmatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelPmatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelPmatch {
    #[inline(always)]
    fn from(val: u8) -> SelPmatch {
        SelPmatch::from_bits(val)
    }
}
impl From<SelPmatch> for u8 {
    #[inline(always)]
    fn from(val: SelPmatch) -> u8 {
        SelPmatch::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Setenaf(u8);
impl Setenaf {
    #[doc = "Writes 0 to IENF."]
    pub const SETENAF_0: Self = Self(0x0);
    #[doc = "Select HIGH-active interrupt or enable falling-edge interrupt"]
    pub const SETENAF_1: Self = Self(0x01);
}
impl Setenaf {
    pub const fn from_bits(val: u8) -> Setenaf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Setenaf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("SETENAF_0"),
            0x01 => f.write_str("SETENAF_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setenaf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SETENAF_0"),
            0x01 => defmt::write!(f, "SETENAF_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Setenaf {
    #[inline(always)]
    fn from(val: u8) -> Setenaf {
        Setenaf::from_bits(val)
    }
}
impl From<Setenaf> for u8 {
    #[inline(always)]
    fn from(val: Setenaf) -> u8 {
        Setenaf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Setenrl(u8);
impl Setenrl {
    #[doc = "No operation for interrupt n"]
    pub const SETENRL_0: Self = Self(0x0);
    #[doc = "Enable rising edge or level interrupt for interrupt n"]
    pub const SETENRL_1: Self = Self(0x01);
}
impl Setenrl {
    pub const fn from_bits(val: u8) -> Setenrl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Setenrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("SETENRL_0"),
            0x01 => f.write_str("SETENRL_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setenrl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SETENRL_0"),
            0x01 => defmt::write!(f, "SETENRL_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Setenrl {
    #[inline(always)]
    fn from(val: u8) -> Setenrl {
        Setenrl::from_bits(val)
    }
}
impl From<Setenrl> for u8 {
    #[inline(always)]
    fn from(val: Setenrl) -> u8 {
        Setenrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src0 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src0 {
    #[inline(always)]
    fn from(val: u8) -> Src0 {
        Src0::from_bits(val)
    }
}
impl From<Src0> for u8 {
    #[inline(always)]
    fn from(val: Src0) -> u8 {
        Src0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src1 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src1 {
    #[inline(always)]
    fn from(val: u8) -> Src1 {
        Src1::from_bits(val)
    }
}
impl From<Src1> for u8 {
    #[inline(always)]
    fn from(val: Src1) -> u8 {
        Src1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src2 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src2 {
    #[inline(always)]
    fn from(val: u8) -> Src2 {
        Src2::from_bits(val)
    }
}
impl From<Src2> for u8 {
    #[inline(always)]
    fn from(val: Src2) -> u8 {
        Src2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src3 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src3 {
    #[inline(always)]
    fn from(val: u8) -> Src3 {
        Src3::from_bits(val)
    }
}
impl From<Src3> for u8 {
    #[inline(always)]
    fn from(val: Src3) -> u8 {
        Src3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src4 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src4 {
    #[inline(always)]
    fn from(val: u8) -> Src4 {
        Src4::from_bits(val)
    }
}
impl From<Src4> for u8 {
    #[inline(always)]
    fn from(val: Src4) -> u8 {
        Src4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src5 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src5 {
    #[inline(always)]
    fn from(val: u8) -> Src5 {
        Src5::from_bits(val)
    }
}
impl From<Src5> for u8 {
    #[inline(always)]
    fn from(val: Src5) -> u8 {
        Src5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src6 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src6 {
    #[inline(always)]
    fn from(val: u8) -> Src6 {
        Src6::from_bits(val)
    }
}
impl From<Src6> for u8 {
    #[inline(always)]
    fn from(val: Src6) -> u8 {
        Src6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src7 {
    #[doc = "Input 0 (selects the pin identified in PINSEL0)"]
    INPUT0 = 0x0,
    #[doc = "Input 1 (selects the pin identified in PINSEL1)"]
    INPUT1 = 0x01,
    #[doc = "Input 2 (selects the pin identified in PINSEL2)"]
    INPUT2 = 0x02,
    #[doc = "Input 3 (selects the pin identified in PINSEL3)"]
    INPUT3 = 0x03,
    #[doc = "Input 4 (selects the pin identified in PINSEL4)"]
    INPUT4 = 0x04,
    #[doc = "Input 5 (selects the pin identified in PINSEL5)"]
    INPUT5 = 0x05,
    #[doc = "Input 6 (selects the pin identified in PINSEL6)"]
    INPUT6 = 0x06,
    #[doc = "Input 7 (selects the pin identified in PINSEL7)"]
    INPUT7 = 0x07,
}
impl Src7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src7 {
    #[inline(always)]
    fn from(val: u8) -> Src7 {
        Src7::from_bits(val)
    }
}
impl From<Src7> for u8 {
    #[inline(always)]
    fn from(val: Src7) -> u8 {
        Src7::to_bits(val)
    }
}
