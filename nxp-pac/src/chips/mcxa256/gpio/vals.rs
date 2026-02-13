#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Basic implementation"]
    pub const FEATURE0: Self = Self(0x0);
    #[doc = "Protection registers implemented"]
    pub const FEATURE1: Self = Self(0x01);
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
            0x01 => f.write_str("FEATURE1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE0"),
            0x01 => defmt::write!(f, "FEATURE1"),
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
pub enum Giwe0 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe0 {
    #[inline(always)]
    fn from(val: u8) -> Giwe0 {
        Giwe0::from_bits(val)
    }
}
impl From<Giwe0> for u8 {
    #[inline(always)]
    fn from(val: Giwe0) -> u8 {
        Giwe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe1 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe1 {
    #[inline(always)]
    fn from(val: u8) -> Giwe1 {
        Giwe1::from_bits(val)
    }
}
impl From<Giwe1> for u8 {
    #[inline(always)]
    fn from(val: Giwe1) -> u8 {
        Giwe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe10 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe10 {
    #[inline(always)]
    fn from(val: u8) -> Giwe10 {
        Giwe10::from_bits(val)
    }
}
impl From<Giwe10> for u8 {
    #[inline(always)]
    fn from(val: Giwe10) -> u8 {
        Giwe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe11 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe11 {
    #[inline(always)]
    fn from(val: u8) -> Giwe11 {
        Giwe11::from_bits(val)
    }
}
impl From<Giwe11> for u8 {
    #[inline(always)]
    fn from(val: Giwe11) -> u8 {
        Giwe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe12 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe12 {
    #[inline(always)]
    fn from(val: u8) -> Giwe12 {
        Giwe12::from_bits(val)
    }
}
impl From<Giwe12> for u8 {
    #[inline(always)]
    fn from(val: Giwe12) -> u8 {
        Giwe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe13 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe13 {
    #[inline(always)]
    fn from(val: u8) -> Giwe13 {
        Giwe13::from_bits(val)
    }
}
impl From<Giwe13> for u8 {
    #[inline(always)]
    fn from(val: Giwe13) -> u8 {
        Giwe13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe14 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe14 {
    #[inline(always)]
    fn from(val: u8) -> Giwe14 {
        Giwe14::from_bits(val)
    }
}
impl From<Giwe14> for u8 {
    #[inline(always)]
    fn from(val: Giwe14) -> u8 {
        Giwe14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe15 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe15 {
    #[inline(always)]
    fn from(val: u8) -> Giwe15 {
        Giwe15::from_bits(val)
    }
}
impl From<Giwe15> for u8 {
    #[inline(always)]
    fn from(val: Giwe15) -> u8 {
        Giwe15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe16 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe16 {
    #[inline(always)]
    fn from(val: u8) -> Giwe16 {
        Giwe16::from_bits(val)
    }
}
impl From<Giwe16> for u8 {
    #[inline(always)]
    fn from(val: Giwe16) -> u8 {
        Giwe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe17 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe17 {
    #[inline(always)]
    fn from(val: u8) -> Giwe17 {
        Giwe17::from_bits(val)
    }
}
impl From<Giwe17> for u8 {
    #[inline(always)]
    fn from(val: Giwe17) -> u8 {
        Giwe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe18 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe18 {
    #[inline(always)]
    fn from(val: u8) -> Giwe18 {
        Giwe18::from_bits(val)
    }
}
impl From<Giwe18> for u8 {
    #[inline(always)]
    fn from(val: Giwe18) -> u8 {
        Giwe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe19 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe19 {
    #[inline(always)]
    fn from(val: u8) -> Giwe19 {
        Giwe19::from_bits(val)
    }
}
impl From<Giwe19> for u8 {
    #[inline(always)]
    fn from(val: Giwe19) -> u8 {
        Giwe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe2 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe2 {
    #[inline(always)]
    fn from(val: u8) -> Giwe2 {
        Giwe2::from_bits(val)
    }
}
impl From<Giwe2> for u8 {
    #[inline(always)]
    fn from(val: Giwe2) -> u8 {
        Giwe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe20 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe20 {
    #[inline(always)]
    fn from(val: u8) -> Giwe20 {
        Giwe20::from_bits(val)
    }
}
impl From<Giwe20> for u8 {
    #[inline(always)]
    fn from(val: Giwe20) -> u8 {
        Giwe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe21 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe21 {
    #[inline(always)]
    fn from(val: u8) -> Giwe21 {
        Giwe21::from_bits(val)
    }
}
impl From<Giwe21> for u8 {
    #[inline(always)]
    fn from(val: Giwe21) -> u8 {
        Giwe21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe22 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe22 {
    #[inline(always)]
    fn from(val: u8) -> Giwe22 {
        Giwe22::from_bits(val)
    }
}
impl From<Giwe22> for u8 {
    #[inline(always)]
    fn from(val: Giwe22) -> u8 {
        Giwe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe23 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe23 {
    #[inline(always)]
    fn from(val: u8) -> Giwe23 {
        Giwe23::from_bits(val)
    }
}
impl From<Giwe23> for u8 {
    #[inline(always)]
    fn from(val: Giwe23) -> u8 {
        Giwe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe24 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe24 {
    #[inline(always)]
    fn from(val: u8) -> Giwe24 {
        Giwe24::from_bits(val)
    }
}
impl From<Giwe24> for u8 {
    #[inline(always)]
    fn from(val: Giwe24) -> u8 {
        Giwe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe25 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe25 {
    #[inline(always)]
    fn from(val: u8) -> Giwe25 {
        Giwe25::from_bits(val)
    }
}
impl From<Giwe25> for u8 {
    #[inline(always)]
    fn from(val: Giwe25) -> u8 {
        Giwe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe26 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe26 {
    #[inline(always)]
    fn from(val: u8) -> Giwe26 {
        Giwe26::from_bits(val)
    }
}
impl From<Giwe26> for u8 {
    #[inline(always)]
    fn from(val: Giwe26) -> u8 {
        Giwe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe27 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe27 {
    #[inline(always)]
    fn from(val: u8) -> Giwe27 {
        Giwe27::from_bits(val)
    }
}
impl From<Giwe27> for u8 {
    #[inline(always)]
    fn from(val: Giwe27) -> u8 {
        Giwe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe28 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe28 {
    #[inline(always)]
    fn from(val: u8) -> Giwe28 {
        Giwe28::from_bits(val)
    }
}
impl From<Giwe28> for u8 {
    #[inline(always)]
    fn from(val: Giwe28) -> u8 {
        Giwe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe29 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe29 {
    #[inline(always)]
    fn from(val: u8) -> Giwe29 {
        Giwe29::from_bits(val)
    }
}
impl From<Giwe29> for u8 {
    #[inline(always)]
    fn from(val: Giwe29) -> u8 {
        Giwe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe3 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe3 {
    #[inline(always)]
    fn from(val: u8) -> Giwe3 {
        Giwe3::from_bits(val)
    }
}
impl From<Giwe3> for u8 {
    #[inline(always)]
    fn from(val: Giwe3) -> u8 {
        Giwe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe30 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe30 {
    #[inline(always)]
    fn from(val: u8) -> Giwe30 {
        Giwe30::from_bits(val)
    }
}
impl From<Giwe30> for u8 {
    #[inline(always)]
    fn from(val: Giwe30) -> u8 {
        Giwe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe31 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe31 {
    #[inline(always)]
    fn from(val: u8) -> Giwe31 {
        Giwe31::from_bits(val)
    }
}
impl From<Giwe31> for u8 {
    #[inline(always)]
    fn from(val: Giwe31) -> u8 {
        Giwe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe4 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe4 {
    #[inline(always)]
    fn from(val: u8) -> Giwe4 {
        Giwe4::from_bits(val)
    }
}
impl From<Giwe4> for u8 {
    #[inline(always)]
    fn from(val: Giwe4) -> u8 {
        Giwe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe5 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe5 {
    #[inline(always)]
    fn from(val: u8) -> Giwe5 {
        Giwe5::from_bits(val)
    }
}
impl From<Giwe5> for u8 {
    #[inline(always)]
    fn from(val: Giwe5) -> u8 {
        Giwe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe6 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe6 {
    #[inline(always)]
    fn from(val: u8) -> Giwe6 {
        Giwe6::from_bits(val)
    }
}
impl From<Giwe6> for u8 {
    #[inline(always)]
    fn from(val: Giwe6) -> u8 {
        Giwe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe7 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe7 {
    #[inline(always)]
    fn from(val: u8) -> Giwe7 {
        Giwe7::from_bits(val)
    }
}
impl From<Giwe7> for u8 {
    #[inline(always)]
    fn from(val: Giwe7) -> u8 {
        Giwe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe8 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe8 {
    #[inline(always)]
    fn from(val: u8) -> Giwe8 {
        Giwe8::from_bits(val)
    }
}
impl From<Giwe8> for u8 {
    #[inline(always)]
    fn from(val: Giwe8) -> u8 {
        Giwe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe9 {
    #[doc = "Not updated"]
    GIWE0 = 0x0,
    #[doc = "Updated"]
    GIWE1 = 0x01,
}
impl Giwe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe9 {
    #[inline(always)]
    fn from(val: u8) -> Giwe9 {
        Giwe9::from_bits(val)
    }
}
impl From<Giwe9> for u8 {
    #[inline(always)]
    fn from(val: Giwe9) -> u8 {
        Giwe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqc {
    #[doc = "ISF is disabled"]
    IRQC0 = 0x0,
    #[doc = "ISF and DMA request on rising edge"]
    IRQC1 = 0x01,
    #[doc = "ISF and DMA request on falling edge"]
    IRQC2 = 0x02,
    #[doc = "ISF and DMA request on either edge"]
    IRQC3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "ISF sets on rising edge"]
    IRQC5 = 0x05,
    #[doc = "ISF sets on falling edge"]
    IRQC6 = 0x06,
    #[doc = "ISF sets on either edge"]
    IRQC7 = 0x07,
    #[doc = "ISF and interrupt when logic 0"]
    IRQC8 = 0x08,
    #[doc = "ISF and interrupt on rising edge"]
    IRQC9 = 0x09,
    #[doc = "ISF and interrupt on falling edge"]
    IRQC10 = 0x0a,
    #[doc = "ISF and Interrupt on either edge"]
    IRQC11 = 0x0b,
    #[doc = "ISF and interrupt when logic 1"]
    IRQC12 = 0x0c,
    #[doc = "Enable active-high trigger output; ISF on rising edge (pin state is ORed with other enabled triggers to generate the output trigger for use by other peripherals)"]
    IRQC13 = 0x0d,
    #[doc = "Enable active-low trigger output; ISF on falling edge (pin state is inverted and ORed with other enabled triggers to generate the output trigger for use by other peripherals)"]
    IRQC14 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqc {
    #[inline(always)]
    fn from(val: u8) -> Irqc {
        Irqc::from_bits(val)
    }
}
impl From<Irqc> for u8 {
    #[inline(always)]
    fn from(val: Irqc) -> u8 {
        Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf {
    #[inline(always)]
    fn from(val: u8) -> Isf {
        Isf::from_bits(val)
    }
}
impl From<Isf> for u8 {
    #[inline(always)]
    fn from(val: Isf) -> u8 {
        Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf0 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf0 {
    #[inline(always)]
    fn from(val: u8) -> Isf0 {
        Isf0::from_bits(val)
    }
}
impl From<Isf0> for u8 {
    #[inline(always)]
    fn from(val: Isf0) -> u8 {
        Isf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf1 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf1 {
    #[inline(always)]
    fn from(val: u8) -> Isf1 {
        Isf1::from_bits(val)
    }
}
impl From<Isf1> for u8 {
    #[inline(always)]
    fn from(val: Isf1) -> u8 {
        Isf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf10 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf10 {
    #[inline(always)]
    fn from(val: u8) -> Isf10 {
        Isf10::from_bits(val)
    }
}
impl From<Isf10> for u8 {
    #[inline(always)]
    fn from(val: Isf10) -> u8 {
        Isf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf11 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf11 {
    #[inline(always)]
    fn from(val: u8) -> Isf11 {
        Isf11::from_bits(val)
    }
}
impl From<Isf11> for u8 {
    #[inline(always)]
    fn from(val: Isf11) -> u8 {
        Isf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf12 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf12 {
    #[inline(always)]
    fn from(val: u8) -> Isf12 {
        Isf12::from_bits(val)
    }
}
impl From<Isf12> for u8 {
    #[inline(always)]
    fn from(val: Isf12) -> u8 {
        Isf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf13 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf13 {
    #[inline(always)]
    fn from(val: u8) -> Isf13 {
        Isf13::from_bits(val)
    }
}
impl From<Isf13> for u8 {
    #[inline(always)]
    fn from(val: Isf13) -> u8 {
        Isf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf14 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf14 {
    #[inline(always)]
    fn from(val: u8) -> Isf14 {
        Isf14::from_bits(val)
    }
}
impl From<Isf14> for u8 {
    #[inline(always)]
    fn from(val: Isf14) -> u8 {
        Isf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf15 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf15 {
    #[inline(always)]
    fn from(val: u8) -> Isf15 {
        Isf15::from_bits(val)
    }
}
impl From<Isf15> for u8 {
    #[inline(always)]
    fn from(val: Isf15) -> u8 {
        Isf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf16 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf16 {
    #[inline(always)]
    fn from(val: u8) -> Isf16 {
        Isf16::from_bits(val)
    }
}
impl From<Isf16> for u8 {
    #[inline(always)]
    fn from(val: Isf16) -> u8 {
        Isf16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf17 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf17 {
    #[inline(always)]
    fn from(val: u8) -> Isf17 {
        Isf17::from_bits(val)
    }
}
impl From<Isf17> for u8 {
    #[inline(always)]
    fn from(val: Isf17) -> u8 {
        Isf17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf18 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf18 {
    #[inline(always)]
    fn from(val: u8) -> Isf18 {
        Isf18::from_bits(val)
    }
}
impl From<Isf18> for u8 {
    #[inline(always)]
    fn from(val: Isf18) -> u8 {
        Isf18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf19 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf19 {
    #[inline(always)]
    fn from(val: u8) -> Isf19 {
        Isf19::from_bits(val)
    }
}
impl From<Isf19> for u8 {
    #[inline(always)]
    fn from(val: Isf19) -> u8 {
        Isf19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf2 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf2 {
    #[inline(always)]
    fn from(val: u8) -> Isf2 {
        Isf2::from_bits(val)
    }
}
impl From<Isf2> for u8 {
    #[inline(always)]
    fn from(val: Isf2) -> u8 {
        Isf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf20 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf20 {
    #[inline(always)]
    fn from(val: u8) -> Isf20 {
        Isf20::from_bits(val)
    }
}
impl From<Isf20> for u8 {
    #[inline(always)]
    fn from(val: Isf20) -> u8 {
        Isf20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf21 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf21 {
    #[inline(always)]
    fn from(val: u8) -> Isf21 {
        Isf21::from_bits(val)
    }
}
impl From<Isf21> for u8 {
    #[inline(always)]
    fn from(val: Isf21) -> u8 {
        Isf21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf22 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf22 {
    #[inline(always)]
    fn from(val: u8) -> Isf22 {
        Isf22::from_bits(val)
    }
}
impl From<Isf22> for u8 {
    #[inline(always)]
    fn from(val: Isf22) -> u8 {
        Isf22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf23 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf23 {
    #[inline(always)]
    fn from(val: u8) -> Isf23 {
        Isf23::from_bits(val)
    }
}
impl From<Isf23> for u8 {
    #[inline(always)]
    fn from(val: Isf23) -> u8 {
        Isf23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf24 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf24 {
    #[inline(always)]
    fn from(val: u8) -> Isf24 {
        Isf24::from_bits(val)
    }
}
impl From<Isf24> for u8 {
    #[inline(always)]
    fn from(val: Isf24) -> u8 {
        Isf24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf25 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf25 {
    #[inline(always)]
    fn from(val: u8) -> Isf25 {
        Isf25::from_bits(val)
    }
}
impl From<Isf25> for u8 {
    #[inline(always)]
    fn from(val: Isf25) -> u8 {
        Isf25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf26 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf26 {
    #[inline(always)]
    fn from(val: u8) -> Isf26 {
        Isf26::from_bits(val)
    }
}
impl From<Isf26> for u8 {
    #[inline(always)]
    fn from(val: Isf26) -> u8 {
        Isf26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf27 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf27 {
    #[inline(always)]
    fn from(val: u8) -> Isf27 {
        Isf27::from_bits(val)
    }
}
impl From<Isf27> for u8 {
    #[inline(always)]
    fn from(val: Isf27) -> u8 {
        Isf27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf28 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf28 {
    #[inline(always)]
    fn from(val: u8) -> Isf28 {
        Isf28::from_bits(val)
    }
}
impl From<Isf28> for u8 {
    #[inline(always)]
    fn from(val: Isf28) -> u8 {
        Isf28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf29 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf29 {
    #[inline(always)]
    fn from(val: u8) -> Isf29 {
        Isf29::from_bits(val)
    }
}
impl From<Isf29> for u8 {
    #[inline(always)]
    fn from(val: Isf29) -> u8 {
        Isf29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf3 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf3 {
    #[inline(always)]
    fn from(val: u8) -> Isf3 {
        Isf3::from_bits(val)
    }
}
impl From<Isf3> for u8 {
    #[inline(always)]
    fn from(val: Isf3) -> u8 {
        Isf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf30 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf30 {
    #[inline(always)]
    fn from(val: u8) -> Isf30 {
        Isf30::from_bits(val)
    }
}
impl From<Isf30> for u8 {
    #[inline(always)]
    fn from(val: Isf30) -> u8 {
        Isf30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf31 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf31 {
    #[inline(always)]
    fn from(val: u8) -> Isf31 {
        Isf31::from_bits(val)
    }
}
impl From<Isf31> for u8 {
    #[inline(always)]
    fn from(val: Isf31) -> u8 {
        Isf31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf4 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf4 {
    #[inline(always)]
    fn from(val: u8) -> Isf4 {
        Isf4::from_bits(val)
    }
}
impl From<Isf4> for u8 {
    #[inline(always)]
    fn from(val: Isf4) -> u8 {
        Isf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf5 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf5 {
    #[inline(always)]
    fn from(val: u8) -> Isf5 {
        Isf5::from_bits(val)
    }
}
impl From<Isf5> for u8 {
    #[inline(always)]
    fn from(val: Isf5) -> u8 {
        Isf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf6 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf6 {
    #[inline(always)]
    fn from(val: u8) -> Isf6 {
        Isf6::from_bits(val)
    }
}
impl From<Isf6> for u8 {
    #[inline(always)]
    fn from(val: Isf6) -> u8 {
        Isf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf7 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf7 {
    #[inline(always)]
    fn from(val: u8) -> Isf7 {
        Isf7::from_bits(val)
    }
}
impl From<Isf7> for u8 {
    #[inline(always)]
    fn from(val: Isf7) -> u8 {
        Isf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf8 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf8 {
    #[inline(always)]
    fn from(val: u8) -> Isf8 {
        Isf8::from_bits(val)
    }
}
impl From<Isf8> for u8 {
    #[inline(always)]
    fn from(val: Isf8) -> u8 {
        Isf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf9 {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Isf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf9 {
    #[inline(always)]
    fn from(val: u8) -> Isf9 {
        Isf9::from_bits(val)
    }
}
impl From<Isf9> for u8 {
    #[inline(always)]
    fn from(val: Isf9) -> u8 {
        Isf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pd {
    #[doc = "Logic zero"]
    PD0 = 0x0,
    #[doc = "Logic one"]
    PD1 = 0x01,
}
impl Pd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd {
    #[inline(always)]
    fn from(val: u8) -> Pd {
        Pd::from_bits(val)
    }
}
impl From<Pd> for u8 {
    #[inline(always)]
    fn from(val: Pd) -> u8 {
        Pd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd {
    #[inline(always)]
    fn from(val: u8) -> Pdd {
        Pdd::from_bits(val)
    }
}
impl From<Pdd> for u8 {
    #[inline(always)]
    fn from(val: Pdd) -> u8 {
        Pdd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid {
    #[inline(always)]
    fn from(val: u8) -> Pid {
        Pid::from_bits(val)
    }
}
impl From<Pid> for u8 {
    #[inline(always)]
    fn from(val: Pid) -> u8 {
        Pid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco {
    #[inline(always)]
    fn from(val: u8) -> Ptco {
        Ptco::from_bits(val)
    }
}
impl From<Ptco> for u8 {
    #[inline(always)]
    fn from(val: Ptco) -> u8 {
        Ptco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso {
    #[inline(always)]
    fn from(val: u8) -> Ptso {
        Ptso::from_bits(val)
    }
}
impl From<Ptso> for u8 {
    #[inline(always)]
    fn from(val: Ptso) -> u8 {
        Ptso::to_bits(val)
    }
}
