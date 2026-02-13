#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dse {
    #[inline(always)]
    fn from(val: u8) -> Dse {
        Dse::from_bits(val)
    }
}
impl From<Dse> for u8 {
    #[inline(always)]
    fn from(val: Dse) -> u8 {
        Dse::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Basic implementation"]
    pub const FEATURE0: Self = Self(0x0);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FEATURE0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe {
    #[inline(always)]
    fn from(val: u8) -> Gpwe {
        Gpwe::from_bits(val)
    }
}
impl From<Gpwe> for u8 {
    #[inline(always)]
    fn from(val: Gpwe) -> u8 {
        Gpwe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibe {
    #[inline(always)]
    fn from(val: u8) -> Ibe {
        Ibe::from_bits(val)
    }
}
impl From<Ibe> for u8 {
    #[inline(always)]
    fn from(val: Ibe) -> u8 {
        Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inv {
    #[inline(always)]
    fn from(val: u8) -> Inv {
        Inv::from_bits(val)
    }
}
impl From<Inv> for u8 {
    #[inline(always)]
    fn from(val: Inv) -> u8 {
        Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lk {
    #[inline(always)]
    fn from(val: u8) -> Lk {
        Lk::from_bits(val)
    }
}
impl From<Lk> for u8 {
    #[inline(always)]
    fn from(val: Lk) -> u8 {
        Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mux {
    MUX0 = 0x0,
    MUX1 = 0x01,
    MUX2 = 0x02,
    MUX3 = 0x03,
    MUX4 = 0x04,
    MUX5 = 0x05,
    MUX6 = 0x06,
    MUX7 = 0x07,
    MUX8 = 0x08,
    MUX9 = 0x09,
    MUX10 = 0x0a,
    MUX11 = 0x0b,
    MUX12 = 0x0c,
    MUX13 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mux {
    #[inline(always)]
    fn from(val: u8) -> Mux {
        Mux::from_bits(val)
    }
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(val: Mux) -> u8 {
        Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ode {
    #[inline(always)]
    fn from(val: u8) -> Ode {
        Ode::from_bits(val)
    }
}
impl From<Ode> for u8 {
    #[inline(always)]
    fn from(val: Ode) -> u8 {
        Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe {
    #[inline(always)]
    fn from(val: u8) -> Pe {
        Pe::from_bits(val)
    }
}
impl From<Pe> for u8 {
    #[inline(always)]
    fn from(val: Pe) -> u8 {
        Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    #[doc = "1.71 V-3.6 V"]
    RANGE0 = 0x0,
    #[doc = "2.70 V-3.6 V"]
    RANGE1 = 0x01,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Range {
    #[inline(always)]
    fn from(val: u8) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u8 {
    #[inline(always)]
    fn from(val: Range) -> u8 {
        Range::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sre {
    #[inline(always)]
    fn from(val: u8) -> Sre {
        Sre::from_bits(val)
    }
}
impl From<Sre> for u8 {
    #[inline(always)]
    fn from(val: Sre) -> u8 {
        Sre::to_bits(val)
    }
}
