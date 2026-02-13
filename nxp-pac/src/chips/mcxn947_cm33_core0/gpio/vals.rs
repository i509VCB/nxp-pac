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
pub enum Icnp {
    #[doc = "Writable in Secure-Privilege state"]
    ICNP0 = 0x0,
    #[doc = "Not writable until the next reset"]
    ICNP1 = 0x01,
}
impl Icnp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icnp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icnp {
    #[inline(always)]
    fn from(val: u8) -> Icnp {
        Icnp::from_bits(val)
    }
}
impl From<Icnp> for u8 {
    #[inline(always)]
    fn from(val: Icnp) -> u8 {
        Icnp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnpNpe0 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl IcnpNpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnpNpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnpNpe0 {
    #[inline(always)]
    fn from(val: u8) -> IcnpNpe0 {
        IcnpNpe0::from_bits(val)
    }
}
impl From<IcnpNpe0> for u8 {
    #[inline(always)]
    fn from(val: IcnpNpe0) -> u8 {
        IcnpNpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnpNpe1 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl IcnpNpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnpNpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnpNpe1 {
    #[inline(always)]
    fn from(val: u8) -> IcnpNpe1 {
        IcnpNpe1::from_bits(val)
    }
}
impl From<IcnpNpe1> for u8 {
    #[inline(always)]
    fn from(val: IcnpNpe1) -> u8 {
        IcnpNpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icns {
    #[doc = "Writable in Secure-Privilege state"]
    ICNS0 = 0x0,
    #[doc = "Not writable until the next reset"]
    ICNS1 = 0x01,
}
impl Icns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icns {
    #[inline(always)]
    fn from(val: u8) -> Icns {
        Icns::from_bits(val)
    }
}
impl From<Icns> for u8 {
    #[inline(always)]
    fn from(val: Icns) -> u8 {
        Icns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnsNse0 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl IcnsNse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnsNse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnsNse0 {
    #[inline(always)]
    fn from(val: u8) -> IcnsNse0 {
        IcnsNse0::from_bits(val)
    }
}
impl From<IcnsNse0> for u8 {
    #[inline(always)]
    fn from(val: IcnsNse0) -> u8 {
        IcnsNse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnsNse1 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl IcnsNse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnsNse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnsNse1 {
    #[inline(always)]
    fn from(val: u8) -> IcnsNse1 {
        IcnsNse1::from_bits(val)
    }
}
impl From<IcnsNse1> for u8 {
    #[inline(always)]
    fn from(val: IcnsNse1) -> u8 {
        IcnsNse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr0Irqc {
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
impl Icr0Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr0Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr0Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr0Irqc {
        Icr0Irqc::from_bits(val)
    }
}
impl From<Icr0Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr0Irqc) -> u8 {
        Icr0Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr0Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr0Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr0Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr0Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr0Irqs {
        Icr0Irqs::from_bits(val)
    }
}
impl From<Icr0Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr0Irqs) -> u8 {
        Icr0Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr0Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr0Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr0Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr0Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr0Isf {
        Icr0Isf::from_bits(val)
    }
}
impl From<Icr0Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr0Isf) -> u8 {
        Icr0Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr0Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr0Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr0Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr0Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr0Lk {
        Icr0Lk::from_bits(val)
    }
}
impl From<Icr0Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr0Lk) -> u8 {
        Icr0Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr10Irqc {
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
impl Icr10Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr10Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr10Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr10Irqc {
        Icr10Irqc::from_bits(val)
    }
}
impl From<Icr10Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr10Irqc) -> u8 {
        Icr10Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr10Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr10Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr10Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr10Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr10Irqs {
        Icr10Irqs::from_bits(val)
    }
}
impl From<Icr10Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr10Irqs) -> u8 {
        Icr10Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr10Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr10Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr10Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr10Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr10Isf {
        Icr10Isf::from_bits(val)
    }
}
impl From<Icr10Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr10Isf) -> u8 {
        Icr10Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr10Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr10Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr10Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr10Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr10Lk {
        Icr10Lk::from_bits(val)
    }
}
impl From<Icr10Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr10Lk) -> u8 {
        Icr10Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr11Irqc {
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
impl Icr11Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr11Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr11Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr11Irqc {
        Icr11Irqc::from_bits(val)
    }
}
impl From<Icr11Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr11Irqc) -> u8 {
        Icr11Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr11Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr11Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr11Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr11Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr11Irqs {
        Icr11Irqs::from_bits(val)
    }
}
impl From<Icr11Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr11Irqs) -> u8 {
        Icr11Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr11Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr11Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr11Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr11Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr11Isf {
        Icr11Isf::from_bits(val)
    }
}
impl From<Icr11Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr11Isf) -> u8 {
        Icr11Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr11Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr11Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr11Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr11Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr11Lk {
        Icr11Lk::from_bits(val)
    }
}
impl From<Icr11Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr11Lk) -> u8 {
        Icr11Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr12Irqc {
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
impl Icr12Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr12Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr12Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr12Irqc {
        Icr12Irqc::from_bits(val)
    }
}
impl From<Icr12Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr12Irqc) -> u8 {
        Icr12Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr12Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr12Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr12Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr12Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr12Irqs {
        Icr12Irqs::from_bits(val)
    }
}
impl From<Icr12Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr12Irqs) -> u8 {
        Icr12Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr12Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr12Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr12Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr12Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr12Isf {
        Icr12Isf::from_bits(val)
    }
}
impl From<Icr12Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr12Isf) -> u8 {
        Icr12Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr12Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr12Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr12Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr12Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr12Lk {
        Icr12Lk::from_bits(val)
    }
}
impl From<Icr12Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr12Lk) -> u8 {
        Icr12Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr13Irqc {
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
impl Icr13Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr13Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr13Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr13Irqc {
        Icr13Irqc::from_bits(val)
    }
}
impl From<Icr13Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr13Irqc) -> u8 {
        Icr13Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr13Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr13Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr13Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr13Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr13Irqs {
        Icr13Irqs::from_bits(val)
    }
}
impl From<Icr13Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr13Irqs) -> u8 {
        Icr13Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr13Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr13Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr13Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr13Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr13Isf {
        Icr13Isf::from_bits(val)
    }
}
impl From<Icr13Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr13Isf) -> u8 {
        Icr13Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr13Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr13Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr13Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr13Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr13Lk {
        Icr13Lk::from_bits(val)
    }
}
impl From<Icr13Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr13Lk) -> u8 {
        Icr13Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr14Irqc {
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
impl Icr14Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr14Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr14Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr14Irqc {
        Icr14Irqc::from_bits(val)
    }
}
impl From<Icr14Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr14Irqc) -> u8 {
        Icr14Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr14Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr14Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr14Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr14Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr14Irqs {
        Icr14Irqs::from_bits(val)
    }
}
impl From<Icr14Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr14Irqs) -> u8 {
        Icr14Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr14Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr14Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr14Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr14Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr14Isf {
        Icr14Isf::from_bits(val)
    }
}
impl From<Icr14Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr14Isf) -> u8 {
        Icr14Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr14Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr14Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr14Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr14Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr14Lk {
        Icr14Lk::from_bits(val)
    }
}
impl From<Icr14Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr14Lk) -> u8 {
        Icr14Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr15Irqc {
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
impl Icr15Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr15Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr15Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr15Irqc {
        Icr15Irqc::from_bits(val)
    }
}
impl From<Icr15Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr15Irqc) -> u8 {
        Icr15Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr15Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr15Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr15Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr15Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr15Irqs {
        Icr15Irqs::from_bits(val)
    }
}
impl From<Icr15Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr15Irqs) -> u8 {
        Icr15Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr15Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr15Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr15Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr15Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr15Isf {
        Icr15Isf::from_bits(val)
    }
}
impl From<Icr15Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr15Isf) -> u8 {
        Icr15Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr15Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr15Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr15Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr15Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr15Lk {
        Icr15Lk::from_bits(val)
    }
}
impl From<Icr15Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr15Lk) -> u8 {
        Icr15Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr16Irqc {
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
impl Icr16Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr16Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr16Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr16Irqc {
        Icr16Irqc::from_bits(val)
    }
}
impl From<Icr16Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr16Irqc) -> u8 {
        Icr16Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr16Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr16Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr16Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr16Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr16Irqs {
        Icr16Irqs::from_bits(val)
    }
}
impl From<Icr16Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr16Irqs) -> u8 {
        Icr16Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr16Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr16Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr16Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr16Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr16Isf {
        Icr16Isf::from_bits(val)
    }
}
impl From<Icr16Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr16Isf) -> u8 {
        Icr16Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr16Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr16Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr16Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr16Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr16Lk {
        Icr16Lk::from_bits(val)
    }
}
impl From<Icr16Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr16Lk) -> u8 {
        Icr16Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr17Irqc {
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
impl Icr17Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr17Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr17Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr17Irqc {
        Icr17Irqc::from_bits(val)
    }
}
impl From<Icr17Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr17Irqc) -> u8 {
        Icr17Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr17Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr17Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr17Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr17Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr17Irqs {
        Icr17Irqs::from_bits(val)
    }
}
impl From<Icr17Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr17Irqs) -> u8 {
        Icr17Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr17Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr17Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr17Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr17Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr17Isf {
        Icr17Isf::from_bits(val)
    }
}
impl From<Icr17Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr17Isf) -> u8 {
        Icr17Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr17Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr17Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr17Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr17Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr17Lk {
        Icr17Lk::from_bits(val)
    }
}
impl From<Icr17Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr17Lk) -> u8 {
        Icr17Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr18Irqc {
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
impl Icr18Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr18Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr18Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr18Irqc {
        Icr18Irqc::from_bits(val)
    }
}
impl From<Icr18Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr18Irqc) -> u8 {
        Icr18Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr18Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr18Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr18Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr18Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr18Irqs {
        Icr18Irqs::from_bits(val)
    }
}
impl From<Icr18Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr18Irqs) -> u8 {
        Icr18Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr18Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr18Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr18Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr18Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr18Isf {
        Icr18Isf::from_bits(val)
    }
}
impl From<Icr18Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr18Isf) -> u8 {
        Icr18Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr18Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr18Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr18Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr18Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr18Lk {
        Icr18Lk::from_bits(val)
    }
}
impl From<Icr18Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr18Lk) -> u8 {
        Icr18Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr19Irqc {
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
impl Icr19Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr19Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr19Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr19Irqc {
        Icr19Irqc::from_bits(val)
    }
}
impl From<Icr19Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr19Irqc) -> u8 {
        Icr19Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr19Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr19Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr19Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr19Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr19Irqs {
        Icr19Irqs::from_bits(val)
    }
}
impl From<Icr19Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr19Irqs) -> u8 {
        Icr19Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr19Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr19Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr19Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr19Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr19Isf {
        Icr19Isf::from_bits(val)
    }
}
impl From<Icr19Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr19Isf) -> u8 {
        Icr19Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr19Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr19Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr19Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr19Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr19Lk {
        Icr19Lk::from_bits(val)
    }
}
impl From<Icr19Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr19Lk) -> u8 {
        Icr19Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr1Irqc {
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
impl Icr1Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr1Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr1Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr1Irqc {
        Icr1Irqc::from_bits(val)
    }
}
impl From<Icr1Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr1Irqc) -> u8 {
        Icr1Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr1Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr1Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr1Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr1Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr1Irqs {
        Icr1Irqs::from_bits(val)
    }
}
impl From<Icr1Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr1Irqs) -> u8 {
        Icr1Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr1Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr1Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr1Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr1Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr1Isf {
        Icr1Isf::from_bits(val)
    }
}
impl From<Icr1Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr1Isf) -> u8 {
        Icr1Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr1Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr1Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr1Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr1Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr1Lk {
        Icr1Lk::from_bits(val)
    }
}
impl From<Icr1Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr1Lk) -> u8 {
        Icr1Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr20Irqc {
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
impl Icr20Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr20Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr20Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr20Irqc {
        Icr20Irqc::from_bits(val)
    }
}
impl From<Icr20Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr20Irqc) -> u8 {
        Icr20Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr20Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr20Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr20Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr20Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr20Irqs {
        Icr20Irqs::from_bits(val)
    }
}
impl From<Icr20Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr20Irqs) -> u8 {
        Icr20Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr20Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr20Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr20Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr20Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr20Isf {
        Icr20Isf::from_bits(val)
    }
}
impl From<Icr20Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr20Isf) -> u8 {
        Icr20Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr20Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr20Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr20Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr20Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr20Lk {
        Icr20Lk::from_bits(val)
    }
}
impl From<Icr20Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr20Lk) -> u8 {
        Icr20Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr21Irqc {
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
impl Icr21Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr21Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr21Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr21Irqc {
        Icr21Irqc::from_bits(val)
    }
}
impl From<Icr21Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr21Irqc) -> u8 {
        Icr21Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr21Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr21Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr21Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr21Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr21Irqs {
        Icr21Irqs::from_bits(val)
    }
}
impl From<Icr21Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr21Irqs) -> u8 {
        Icr21Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr21Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr21Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr21Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr21Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr21Isf {
        Icr21Isf::from_bits(val)
    }
}
impl From<Icr21Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr21Isf) -> u8 {
        Icr21Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr21Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr21Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr21Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr21Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr21Lk {
        Icr21Lk::from_bits(val)
    }
}
impl From<Icr21Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr21Lk) -> u8 {
        Icr21Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr22Irqc {
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
impl Icr22Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr22Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr22Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr22Irqc {
        Icr22Irqc::from_bits(val)
    }
}
impl From<Icr22Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr22Irqc) -> u8 {
        Icr22Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr22Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr22Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr22Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr22Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr22Irqs {
        Icr22Irqs::from_bits(val)
    }
}
impl From<Icr22Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr22Irqs) -> u8 {
        Icr22Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr22Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr22Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr22Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr22Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr22Isf {
        Icr22Isf::from_bits(val)
    }
}
impl From<Icr22Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr22Isf) -> u8 {
        Icr22Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr22Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr22Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr22Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr22Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr22Lk {
        Icr22Lk::from_bits(val)
    }
}
impl From<Icr22Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr22Lk) -> u8 {
        Icr22Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr23Irqc {
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
impl Icr23Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr23Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr23Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr23Irqc {
        Icr23Irqc::from_bits(val)
    }
}
impl From<Icr23Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr23Irqc) -> u8 {
        Icr23Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr23Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr23Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr23Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr23Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr23Irqs {
        Icr23Irqs::from_bits(val)
    }
}
impl From<Icr23Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr23Irqs) -> u8 {
        Icr23Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr23Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr23Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr23Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr23Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr23Isf {
        Icr23Isf::from_bits(val)
    }
}
impl From<Icr23Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr23Isf) -> u8 {
        Icr23Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr23Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr23Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr23Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr23Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr23Lk {
        Icr23Lk::from_bits(val)
    }
}
impl From<Icr23Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr23Lk) -> u8 {
        Icr23Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr24Irqc {
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
impl Icr24Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr24Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr24Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr24Irqc {
        Icr24Irqc::from_bits(val)
    }
}
impl From<Icr24Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr24Irqc) -> u8 {
        Icr24Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr24Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr24Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr24Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr24Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr24Irqs {
        Icr24Irqs::from_bits(val)
    }
}
impl From<Icr24Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr24Irqs) -> u8 {
        Icr24Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr24Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr24Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr24Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr24Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr24Isf {
        Icr24Isf::from_bits(val)
    }
}
impl From<Icr24Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr24Isf) -> u8 {
        Icr24Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr24Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr24Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr24Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr24Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr24Lk {
        Icr24Lk::from_bits(val)
    }
}
impl From<Icr24Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr24Lk) -> u8 {
        Icr24Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr25Irqc {
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
impl Icr25Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr25Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr25Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr25Irqc {
        Icr25Irqc::from_bits(val)
    }
}
impl From<Icr25Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr25Irqc) -> u8 {
        Icr25Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr25Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr25Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr25Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr25Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr25Irqs {
        Icr25Irqs::from_bits(val)
    }
}
impl From<Icr25Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr25Irqs) -> u8 {
        Icr25Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr25Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr25Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr25Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr25Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr25Isf {
        Icr25Isf::from_bits(val)
    }
}
impl From<Icr25Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr25Isf) -> u8 {
        Icr25Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr25Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr25Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr25Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr25Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr25Lk {
        Icr25Lk::from_bits(val)
    }
}
impl From<Icr25Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr25Lk) -> u8 {
        Icr25Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr26Irqc {
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
impl Icr26Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr26Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr26Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr26Irqc {
        Icr26Irqc::from_bits(val)
    }
}
impl From<Icr26Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr26Irqc) -> u8 {
        Icr26Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr26Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr26Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr26Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr26Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr26Irqs {
        Icr26Irqs::from_bits(val)
    }
}
impl From<Icr26Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr26Irqs) -> u8 {
        Icr26Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr26Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr26Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr26Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr26Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr26Isf {
        Icr26Isf::from_bits(val)
    }
}
impl From<Icr26Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr26Isf) -> u8 {
        Icr26Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr26Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr26Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr26Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr26Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr26Lk {
        Icr26Lk::from_bits(val)
    }
}
impl From<Icr26Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr26Lk) -> u8 {
        Icr26Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr27Irqc {
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
impl Icr27Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr27Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr27Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr27Irqc {
        Icr27Irqc::from_bits(val)
    }
}
impl From<Icr27Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr27Irqc) -> u8 {
        Icr27Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr27Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr27Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr27Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr27Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr27Irqs {
        Icr27Irqs::from_bits(val)
    }
}
impl From<Icr27Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr27Irqs) -> u8 {
        Icr27Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr27Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr27Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr27Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr27Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr27Isf {
        Icr27Isf::from_bits(val)
    }
}
impl From<Icr27Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr27Isf) -> u8 {
        Icr27Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr27Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr27Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr27Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr27Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr27Lk {
        Icr27Lk::from_bits(val)
    }
}
impl From<Icr27Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr27Lk) -> u8 {
        Icr27Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr28Irqc {
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
impl Icr28Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr28Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr28Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr28Irqc {
        Icr28Irqc::from_bits(val)
    }
}
impl From<Icr28Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr28Irqc) -> u8 {
        Icr28Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr28Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr28Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr28Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr28Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr28Irqs {
        Icr28Irqs::from_bits(val)
    }
}
impl From<Icr28Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr28Irqs) -> u8 {
        Icr28Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr28Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr28Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr28Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr28Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr28Isf {
        Icr28Isf::from_bits(val)
    }
}
impl From<Icr28Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr28Isf) -> u8 {
        Icr28Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr28Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr28Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr28Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr28Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr28Lk {
        Icr28Lk::from_bits(val)
    }
}
impl From<Icr28Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr28Lk) -> u8 {
        Icr28Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr29Irqc {
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
impl Icr29Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr29Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr29Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr29Irqc {
        Icr29Irqc::from_bits(val)
    }
}
impl From<Icr29Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr29Irqc) -> u8 {
        Icr29Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr29Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr29Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr29Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr29Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr29Irqs {
        Icr29Irqs::from_bits(val)
    }
}
impl From<Icr29Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr29Irqs) -> u8 {
        Icr29Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr29Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr29Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr29Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr29Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr29Isf {
        Icr29Isf::from_bits(val)
    }
}
impl From<Icr29Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr29Isf) -> u8 {
        Icr29Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr29Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr29Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr29Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr29Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr29Lk {
        Icr29Lk::from_bits(val)
    }
}
impl From<Icr29Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr29Lk) -> u8 {
        Icr29Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr2Irqc {
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
impl Icr2Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr2Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr2Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr2Irqc {
        Icr2Irqc::from_bits(val)
    }
}
impl From<Icr2Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr2Irqc) -> u8 {
        Icr2Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr2Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr2Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr2Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr2Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr2Irqs {
        Icr2Irqs::from_bits(val)
    }
}
impl From<Icr2Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr2Irqs) -> u8 {
        Icr2Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr2Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr2Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr2Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr2Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr2Isf {
        Icr2Isf::from_bits(val)
    }
}
impl From<Icr2Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr2Isf) -> u8 {
        Icr2Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr2Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr2Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr2Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr2Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr2Lk {
        Icr2Lk::from_bits(val)
    }
}
impl From<Icr2Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr2Lk) -> u8 {
        Icr2Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr30Irqc {
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
impl Icr30Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr30Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr30Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr30Irqc {
        Icr30Irqc::from_bits(val)
    }
}
impl From<Icr30Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr30Irqc) -> u8 {
        Icr30Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr30Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr30Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr30Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr30Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr30Irqs {
        Icr30Irqs::from_bits(val)
    }
}
impl From<Icr30Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr30Irqs) -> u8 {
        Icr30Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr30Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr30Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr30Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr30Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr30Isf {
        Icr30Isf::from_bits(val)
    }
}
impl From<Icr30Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr30Isf) -> u8 {
        Icr30Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr30Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr30Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr30Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr30Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr30Lk {
        Icr30Lk::from_bits(val)
    }
}
impl From<Icr30Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr30Lk) -> u8 {
        Icr30Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr31Irqc {
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
impl Icr31Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr31Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr31Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr31Irqc {
        Icr31Irqc::from_bits(val)
    }
}
impl From<Icr31Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr31Irqc) -> u8 {
        Icr31Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr31Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr31Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr31Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr31Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr31Irqs {
        Icr31Irqs::from_bits(val)
    }
}
impl From<Icr31Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr31Irqs) -> u8 {
        Icr31Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr31Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr31Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr31Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr31Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr31Isf {
        Icr31Isf::from_bits(val)
    }
}
impl From<Icr31Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr31Isf) -> u8 {
        Icr31Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr31Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr31Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr31Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr31Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr31Lk {
        Icr31Lk::from_bits(val)
    }
}
impl From<Icr31Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr31Lk) -> u8 {
        Icr31Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr3Irqc {
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
impl Icr3Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr3Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr3Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr3Irqc {
        Icr3Irqc::from_bits(val)
    }
}
impl From<Icr3Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr3Irqc) -> u8 {
        Icr3Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr3Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr3Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr3Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr3Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr3Irqs {
        Icr3Irqs::from_bits(val)
    }
}
impl From<Icr3Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr3Irqs) -> u8 {
        Icr3Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr3Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr3Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr3Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr3Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr3Isf {
        Icr3Isf::from_bits(val)
    }
}
impl From<Icr3Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr3Isf) -> u8 {
        Icr3Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr3Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr3Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr3Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr3Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr3Lk {
        Icr3Lk::from_bits(val)
    }
}
impl From<Icr3Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr3Lk) -> u8 {
        Icr3Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr4Irqc {
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
impl Icr4Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr4Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr4Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr4Irqc {
        Icr4Irqc::from_bits(val)
    }
}
impl From<Icr4Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr4Irqc) -> u8 {
        Icr4Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr4Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr4Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr4Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr4Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr4Irqs {
        Icr4Irqs::from_bits(val)
    }
}
impl From<Icr4Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr4Irqs) -> u8 {
        Icr4Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr4Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr4Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr4Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr4Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr4Isf {
        Icr4Isf::from_bits(val)
    }
}
impl From<Icr4Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr4Isf) -> u8 {
        Icr4Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr4Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr4Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr4Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr4Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr4Lk {
        Icr4Lk::from_bits(val)
    }
}
impl From<Icr4Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr4Lk) -> u8 {
        Icr4Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr5Irqc {
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
impl Icr5Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr5Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr5Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr5Irqc {
        Icr5Irqc::from_bits(val)
    }
}
impl From<Icr5Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr5Irqc) -> u8 {
        Icr5Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr5Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr5Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr5Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr5Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr5Irqs {
        Icr5Irqs::from_bits(val)
    }
}
impl From<Icr5Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr5Irqs) -> u8 {
        Icr5Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr5Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr5Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr5Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr5Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr5Isf {
        Icr5Isf::from_bits(val)
    }
}
impl From<Icr5Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr5Isf) -> u8 {
        Icr5Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr5Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr5Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr5Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr5Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr5Lk {
        Icr5Lk::from_bits(val)
    }
}
impl From<Icr5Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr5Lk) -> u8 {
        Icr5Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr6Irqc {
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
impl Icr6Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr6Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr6Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr6Irqc {
        Icr6Irqc::from_bits(val)
    }
}
impl From<Icr6Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr6Irqc) -> u8 {
        Icr6Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr6Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr6Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr6Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr6Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr6Irqs {
        Icr6Irqs::from_bits(val)
    }
}
impl From<Icr6Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr6Irqs) -> u8 {
        Icr6Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr6Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr6Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr6Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr6Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr6Isf {
        Icr6Isf::from_bits(val)
    }
}
impl From<Icr6Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr6Isf) -> u8 {
        Icr6Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr6Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr6Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr6Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr6Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr6Lk {
        Icr6Lk::from_bits(val)
    }
}
impl From<Icr6Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr6Lk) -> u8 {
        Icr6Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr7Irqc {
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
impl Icr7Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr7Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr7Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr7Irqc {
        Icr7Irqc::from_bits(val)
    }
}
impl From<Icr7Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr7Irqc) -> u8 {
        Icr7Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr7Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr7Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr7Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr7Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr7Irqs {
        Icr7Irqs::from_bits(val)
    }
}
impl From<Icr7Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr7Irqs) -> u8 {
        Icr7Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr7Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr7Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr7Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr7Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr7Isf {
        Icr7Isf::from_bits(val)
    }
}
impl From<Icr7Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr7Isf) -> u8 {
        Icr7Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr7Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr7Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr7Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr7Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr7Lk {
        Icr7Lk::from_bits(val)
    }
}
impl From<Icr7Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr7Lk) -> u8 {
        Icr7Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr8Irqc {
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
impl Icr8Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr8Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr8Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr8Irqc {
        Icr8Irqc::from_bits(val)
    }
}
impl From<Icr8Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr8Irqc) -> u8 {
        Icr8Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr8Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr8Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr8Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr8Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr8Irqs {
        Icr8Irqs::from_bits(val)
    }
}
impl From<Icr8Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr8Irqs) -> u8 {
        Icr8Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr8Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr8Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr8Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr8Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr8Isf {
        Icr8Isf::from_bits(val)
    }
}
impl From<Icr8Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr8Isf) -> u8 {
        Icr8Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr8Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr8Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr8Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr8Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr8Lk {
        Icr8Lk::from_bits(val)
    }
}
impl From<Icr8Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr8Lk) -> u8 {
        Icr8Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr9Irqc {
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
impl Icr9Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr9Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr9Irqc {
    #[inline(always)]
    fn from(val: u8) -> Icr9Irqc {
        Icr9Irqc::from_bits(val)
    }
}
impl From<Icr9Irqc> for u8 {
    #[inline(always)]
    fn from(val: Icr9Irqc) -> u8 {
        Icr9Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr9Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0"]
    IRQS0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1"]
    IRQS1 = 0x01,
}
impl Icr9Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr9Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr9Irqs {
    #[inline(always)]
    fn from(val: u8) -> Icr9Irqs {
        Icr9Irqs::from_bits(val)
    }
}
impl From<Icr9Irqs> for u8 {
    #[inline(always)]
    fn from(val: Icr9Irqs) -> u8 {
        Icr9Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr9Isf {
    #[doc = "Not detected"]
    ISF0 = 0x0,
    #[doc = "Detected"]
    ISF1 = 0x01,
}
impl Icr9Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr9Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr9Isf {
    #[inline(always)]
    fn from(val: u8) -> Icr9Isf {
        Icr9Isf::from_bits(val)
    }
}
impl From<Icr9Isf> for u8 {
    #[inline(always)]
    fn from(val: Icr9Isf) -> u8 {
        Icr9Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icr9Lk {
    #[doc = "Lock"]
    LK0 = 0x0,
    #[doc = "Do not lock"]
    LK1 = 0x01,
}
impl Icr9Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icr9Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icr9Lk {
    #[inline(always)]
    fn from(val: u8) -> Icr9Lk {
        Icr9Lk::from_bits(val)
    }
}
impl From<Icr9Lk> for u8 {
    #[inline(always)]
    fn from(val: Icr9Lk) -> u8 {
        Icr9Lk::to_bits(val)
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
pub enum Npe10 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe10 {
    #[inline(always)]
    fn from(val: u8) -> Npe10 {
        Npe10::from_bits(val)
    }
}
impl From<Npe10> for u8 {
    #[inline(always)]
    fn from(val: Npe10) -> u8 {
        Npe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe11 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe11 {
    #[inline(always)]
    fn from(val: u8) -> Npe11 {
        Npe11::from_bits(val)
    }
}
impl From<Npe11> for u8 {
    #[inline(always)]
    fn from(val: Npe11) -> u8 {
        Npe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe12 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe12 {
    #[inline(always)]
    fn from(val: u8) -> Npe12 {
        Npe12::from_bits(val)
    }
}
impl From<Npe12> for u8 {
    #[inline(always)]
    fn from(val: Npe12) -> u8 {
        Npe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe13 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe13 {
    #[inline(always)]
    fn from(val: u8) -> Npe13 {
        Npe13::from_bits(val)
    }
}
impl From<Npe13> for u8 {
    #[inline(always)]
    fn from(val: Npe13) -> u8 {
        Npe13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe14 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe14 {
    #[inline(always)]
    fn from(val: u8) -> Npe14 {
        Npe14::from_bits(val)
    }
}
impl From<Npe14> for u8 {
    #[inline(always)]
    fn from(val: Npe14) -> u8 {
        Npe14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe15 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe15 {
    #[inline(always)]
    fn from(val: u8) -> Npe15 {
        Npe15::from_bits(val)
    }
}
impl From<Npe15> for u8 {
    #[inline(always)]
    fn from(val: Npe15) -> u8 {
        Npe15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe16 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe16 {
    #[inline(always)]
    fn from(val: u8) -> Npe16 {
        Npe16::from_bits(val)
    }
}
impl From<Npe16> for u8 {
    #[inline(always)]
    fn from(val: Npe16) -> u8 {
        Npe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe17 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe17 {
    #[inline(always)]
    fn from(val: u8) -> Npe17 {
        Npe17::from_bits(val)
    }
}
impl From<Npe17> for u8 {
    #[inline(always)]
    fn from(val: Npe17) -> u8 {
        Npe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe18 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe18 {
    #[inline(always)]
    fn from(val: u8) -> Npe18 {
        Npe18::from_bits(val)
    }
}
impl From<Npe18> for u8 {
    #[inline(always)]
    fn from(val: Npe18) -> u8 {
        Npe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe19 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe19 {
    #[inline(always)]
    fn from(val: u8) -> Npe19 {
        Npe19::from_bits(val)
    }
}
impl From<Npe19> for u8 {
    #[inline(always)]
    fn from(val: Npe19) -> u8 {
        Npe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe2 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe2 {
    #[inline(always)]
    fn from(val: u8) -> Npe2 {
        Npe2::from_bits(val)
    }
}
impl From<Npe2> for u8 {
    #[inline(always)]
    fn from(val: Npe2) -> u8 {
        Npe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe20 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe20 {
    #[inline(always)]
    fn from(val: u8) -> Npe20 {
        Npe20::from_bits(val)
    }
}
impl From<Npe20> for u8 {
    #[inline(always)]
    fn from(val: Npe20) -> u8 {
        Npe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe21 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe21 {
    #[inline(always)]
    fn from(val: u8) -> Npe21 {
        Npe21::from_bits(val)
    }
}
impl From<Npe21> for u8 {
    #[inline(always)]
    fn from(val: Npe21) -> u8 {
        Npe21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe22 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe22 {
    #[inline(always)]
    fn from(val: u8) -> Npe22 {
        Npe22::from_bits(val)
    }
}
impl From<Npe22> for u8 {
    #[inline(always)]
    fn from(val: Npe22) -> u8 {
        Npe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe23 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe23 {
    #[inline(always)]
    fn from(val: u8) -> Npe23 {
        Npe23::from_bits(val)
    }
}
impl From<Npe23> for u8 {
    #[inline(always)]
    fn from(val: Npe23) -> u8 {
        Npe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe24 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe24 {
    #[inline(always)]
    fn from(val: u8) -> Npe24 {
        Npe24::from_bits(val)
    }
}
impl From<Npe24> for u8 {
    #[inline(always)]
    fn from(val: Npe24) -> u8 {
        Npe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe25 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe25 {
    #[inline(always)]
    fn from(val: u8) -> Npe25 {
        Npe25::from_bits(val)
    }
}
impl From<Npe25> for u8 {
    #[inline(always)]
    fn from(val: Npe25) -> u8 {
        Npe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe26 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe26 {
    #[inline(always)]
    fn from(val: u8) -> Npe26 {
        Npe26::from_bits(val)
    }
}
impl From<Npe26> for u8 {
    #[inline(always)]
    fn from(val: Npe26) -> u8 {
        Npe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe27 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe27 {
    #[inline(always)]
    fn from(val: u8) -> Npe27 {
        Npe27::from_bits(val)
    }
}
impl From<Npe27> for u8 {
    #[inline(always)]
    fn from(val: Npe27) -> u8 {
        Npe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe28 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe28 {
    #[inline(always)]
    fn from(val: u8) -> Npe28 {
        Npe28::from_bits(val)
    }
}
impl From<Npe28> for u8 {
    #[inline(always)]
    fn from(val: Npe28) -> u8 {
        Npe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe29 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe29 {
    #[inline(always)]
    fn from(val: u8) -> Npe29 {
        Npe29::from_bits(val)
    }
}
impl From<Npe29> for u8 {
    #[inline(always)]
    fn from(val: Npe29) -> u8 {
        Npe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe3 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe3 {
    #[inline(always)]
    fn from(val: u8) -> Npe3 {
        Npe3::from_bits(val)
    }
}
impl From<Npe3> for u8 {
    #[inline(always)]
    fn from(val: Npe3) -> u8 {
        Npe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe30 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe30 {
    #[inline(always)]
    fn from(val: u8) -> Npe30 {
        Npe30::from_bits(val)
    }
}
impl From<Npe30> for u8 {
    #[inline(always)]
    fn from(val: Npe30) -> u8 {
        Npe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe31 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe31 {
    #[inline(always)]
    fn from(val: u8) -> Npe31 {
        Npe31::from_bits(val)
    }
}
impl From<Npe31> for u8 {
    #[inline(always)]
    fn from(val: Npe31) -> u8 {
        Npe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe4 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe4 {
    #[inline(always)]
    fn from(val: u8) -> Npe4 {
        Npe4::from_bits(val)
    }
}
impl From<Npe4> for u8 {
    #[inline(always)]
    fn from(val: Npe4) -> u8 {
        Npe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe5 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe5 {
    #[inline(always)]
    fn from(val: u8) -> Npe5 {
        Npe5::from_bits(val)
    }
}
impl From<Npe5> for u8 {
    #[inline(always)]
    fn from(val: Npe5) -> u8 {
        Npe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe6 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe6 {
    #[inline(always)]
    fn from(val: u8) -> Npe6 {
        Npe6::from_bits(val)
    }
}
impl From<Npe6> for u8 {
    #[inline(always)]
    fn from(val: Npe6) -> u8 {
        Npe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe7 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe7 {
    #[inline(always)]
    fn from(val: u8) -> Npe7 {
        Npe7::from_bits(val)
    }
}
impl From<Npe7> for u8 {
    #[inline(always)]
    fn from(val: Npe7) -> u8 {
        Npe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe8 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe8 {
    #[inline(always)]
    fn from(val: u8) -> Npe8 {
        Npe8::from_bits(val)
    }
}
impl From<Npe8> for u8 {
    #[inline(always)]
    fn from(val: Npe8) -> u8 {
        Npe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npe9 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl Npe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npe9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npe9 {
    #[inline(always)]
    fn from(val: u8) -> Npe9 {
        Npe9::from_bits(val)
    }
}
impl From<Npe9> for u8 {
    #[inline(always)]
    fn from(val: Npe9) -> u8 {
        Npe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse10 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse10 {
    #[inline(always)]
    fn from(val: u8) -> Nse10 {
        Nse10::from_bits(val)
    }
}
impl From<Nse10> for u8 {
    #[inline(always)]
    fn from(val: Nse10) -> u8 {
        Nse10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse11 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse11 {
    #[inline(always)]
    fn from(val: u8) -> Nse11 {
        Nse11::from_bits(val)
    }
}
impl From<Nse11> for u8 {
    #[inline(always)]
    fn from(val: Nse11) -> u8 {
        Nse11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse12 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse12 {
    #[inline(always)]
    fn from(val: u8) -> Nse12 {
        Nse12::from_bits(val)
    }
}
impl From<Nse12> for u8 {
    #[inline(always)]
    fn from(val: Nse12) -> u8 {
        Nse12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse13 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse13 {
    #[inline(always)]
    fn from(val: u8) -> Nse13 {
        Nse13::from_bits(val)
    }
}
impl From<Nse13> for u8 {
    #[inline(always)]
    fn from(val: Nse13) -> u8 {
        Nse13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse14 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse14 {
    #[inline(always)]
    fn from(val: u8) -> Nse14 {
        Nse14::from_bits(val)
    }
}
impl From<Nse14> for u8 {
    #[inline(always)]
    fn from(val: Nse14) -> u8 {
        Nse14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse15 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse15 {
    #[inline(always)]
    fn from(val: u8) -> Nse15 {
        Nse15::from_bits(val)
    }
}
impl From<Nse15> for u8 {
    #[inline(always)]
    fn from(val: Nse15) -> u8 {
        Nse15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse16 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse16 {
    #[inline(always)]
    fn from(val: u8) -> Nse16 {
        Nse16::from_bits(val)
    }
}
impl From<Nse16> for u8 {
    #[inline(always)]
    fn from(val: Nse16) -> u8 {
        Nse16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse17 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse17 {
    #[inline(always)]
    fn from(val: u8) -> Nse17 {
        Nse17::from_bits(val)
    }
}
impl From<Nse17> for u8 {
    #[inline(always)]
    fn from(val: Nse17) -> u8 {
        Nse17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse18 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse18 {
    #[inline(always)]
    fn from(val: u8) -> Nse18 {
        Nse18::from_bits(val)
    }
}
impl From<Nse18> for u8 {
    #[inline(always)]
    fn from(val: Nse18) -> u8 {
        Nse18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse19 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse19 {
    #[inline(always)]
    fn from(val: u8) -> Nse19 {
        Nse19::from_bits(val)
    }
}
impl From<Nse19> for u8 {
    #[inline(always)]
    fn from(val: Nse19) -> u8 {
        Nse19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse2 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse2 {
    #[inline(always)]
    fn from(val: u8) -> Nse2 {
        Nse2::from_bits(val)
    }
}
impl From<Nse2> for u8 {
    #[inline(always)]
    fn from(val: Nse2) -> u8 {
        Nse2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse20 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse20 {
    #[inline(always)]
    fn from(val: u8) -> Nse20 {
        Nse20::from_bits(val)
    }
}
impl From<Nse20> for u8 {
    #[inline(always)]
    fn from(val: Nse20) -> u8 {
        Nse20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse21 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse21 {
    #[inline(always)]
    fn from(val: u8) -> Nse21 {
        Nse21::from_bits(val)
    }
}
impl From<Nse21> for u8 {
    #[inline(always)]
    fn from(val: Nse21) -> u8 {
        Nse21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse22 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse22 {
    #[inline(always)]
    fn from(val: u8) -> Nse22 {
        Nse22::from_bits(val)
    }
}
impl From<Nse22> for u8 {
    #[inline(always)]
    fn from(val: Nse22) -> u8 {
        Nse22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse23 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse23 {
    #[inline(always)]
    fn from(val: u8) -> Nse23 {
        Nse23::from_bits(val)
    }
}
impl From<Nse23> for u8 {
    #[inline(always)]
    fn from(val: Nse23) -> u8 {
        Nse23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse24 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse24 {
    #[inline(always)]
    fn from(val: u8) -> Nse24 {
        Nse24::from_bits(val)
    }
}
impl From<Nse24> for u8 {
    #[inline(always)]
    fn from(val: Nse24) -> u8 {
        Nse24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse25 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse25 {
    #[inline(always)]
    fn from(val: u8) -> Nse25 {
        Nse25::from_bits(val)
    }
}
impl From<Nse25> for u8 {
    #[inline(always)]
    fn from(val: Nse25) -> u8 {
        Nse25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse26 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse26 {
    #[inline(always)]
    fn from(val: u8) -> Nse26 {
        Nse26::from_bits(val)
    }
}
impl From<Nse26> for u8 {
    #[inline(always)]
    fn from(val: Nse26) -> u8 {
        Nse26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse27 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse27 {
    #[inline(always)]
    fn from(val: u8) -> Nse27 {
        Nse27::from_bits(val)
    }
}
impl From<Nse27> for u8 {
    #[inline(always)]
    fn from(val: Nse27) -> u8 {
        Nse27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse28 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse28 {
    #[inline(always)]
    fn from(val: u8) -> Nse28 {
        Nse28::from_bits(val)
    }
}
impl From<Nse28> for u8 {
    #[inline(always)]
    fn from(val: Nse28) -> u8 {
        Nse28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse29 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse29 {
    #[inline(always)]
    fn from(val: u8) -> Nse29 {
        Nse29::from_bits(val)
    }
}
impl From<Nse29> for u8 {
    #[inline(always)]
    fn from(val: Nse29) -> u8 {
        Nse29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse3 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse3 {
    #[inline(always)]
    fn from(val: u8) -> Nse3 {
        Nse3::from_bits(val)
    }
}
impl From<Nse3> for u8 {
    #[inline(always)]
    fn from(val: Nse3) -> u8 {
        Nse3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse30 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse30 {
    #[inline(always)]
    fn from(val: u8) -> Nse30 {
        Nse30::from_bits(val)
    }
}
impl From<Nse30> for u8 {
    #[inline(always)]
    fn from(val: Nse30) -> u8 {
        Nse30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse31 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse31 {
    #[inline(always)]
    fn from(val: u8) -> Nse31 {
        Nse31::from_bits(val)
    }
}
impl From<Nse31> for u8 {
    #[inline(always)]
    fn from(val: Nse31) -> u8 {
        Nse31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse4 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse4 {
    #[inline(always)]
    fn from(val: u8) -> Nse4 {
        Nse4::from_bits(val)
    }
}
impl From<Nse4> for u8 {
    #[inline(always)]
    fn from(val: Nse4) -> u8 {
        Nse4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse5 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse5 {
    #[inline(always)]
    fn from(val: u8) -> Nse5 {
        Nse5::from_bits(val)
    }
}
impl From<Nse5> for u8 {
    #[inline(always)]
    fn from(val: Nse5) -> u8 {
        Nse5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse6 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse6 {
    #[inline(always)]
    fn from(val: u8) -> Nse6 {
        Nse6::from_bits(val)
    }
}
impl From<Nse6> for u8 {
    #[inline(always)]
    fn from(val: Nse6) -> u8 {
        Nse6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse7 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse7 {
    #[inline(always)]
    fn from(val: u8) -> Nse7 {
        Nse7::from_bits(val)
    }
}
impl From<Nse7> for u8 {
    #[inline(always)]
    fn from(val: Nse7) -> u8 {
        Nse7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse8 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse8 {
    #[inline(always)]
    fn from(val: u8) -> Nse8 {
        Nse8::from_bits(val)
    }
}
impl From<Nse8> for u8 {
    #[inline(always)]
    fn from(val: Nse8) -> u8 {
        Nse8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nse9 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl Nse9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nse9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nse9 {
    #[inline(always)]
    fn from(val: u8) -> Nse9 {
        Nse9::from_bits(val)
    }
}
impl From<Nse9> for u8 {
    #[inline(always)]
    fn from(val: Nse9) -> u8 {
        Nse9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcnp {
    #[doc = "Writable in Secure-Privilege state"]
    PCNP0 = 0x0,
    #[doc = "Not writable until the next reset"]
    PCNP1 = 0x01,
}
impl Pcnp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcnp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcnp {
    #[inline(always)]
    fn from(val: u8) -> Pcnp {
        Pcnp::from_bits(val)
    }
}
impl From<Pcnp> for u8 {
    #[inline(always)]
    fn from(val: Pcnp) -> u8 {
        Pcnp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcnpNpe0 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl PcnpNpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcnpNpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcnpNpe0 {
    #[inline(always)]
    fn from(val: u8) -> PcnpNpe0 {
        PcnpNpe0::from_bits(val)
    }
}
impl From<PcnpNpe0> for u8 {
    #[inline(always)]
    fn from(val: PcnpNpe0) -> u8 {
        PcnpNpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcnpNpe1 {
    #[doc = "Privilege access"]
    NPE0 = 0x0,
    #[doc = "Nonprivilege access"]
    NPE1 = 0x01,
}
impl PcnpNpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcnpNpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcnpNpe1 {
    #[inline(always)]
    fn from(val: u8) -> PcnpNpe1 {
        PcnpNpe1::from_bits(val)
    }
}
impl From<PcnpNpe1> for u8 {
    #[inline(always)]
    fn from(val: PcnpNpe1) -> u8 {
        PcnpNpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcns {
    #[doc = "Writable in Secure-Privilege state"]
    PCNS0 = 0x0,
    #[doc = "Not writable until the next reset"]
    PCNS1 = 0x01,
}
impl Pcns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcns {
    #[inline(always)]
    fn from(val: u8) -> Pcns {
        Pcns::from_bits(val)
    }
}
impl From<Pcns> for u8 {
    #[inline(always)]
    fn from(val: Pcns) -> u8 {
        Pcns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcnsNse0 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl PcnsNse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcnsNse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcnsNse0 {
    #[inline(always)]
    fn from(val: u8) -> PcnsNse0 {
        PcnsNse0::from_bits(val)
    }
}
impl From<PcnsNse0> for u8 {
    #[inline(always)]
    fn from(val: PcnsNse0) -> u8 {
        PcnsNse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcnsNse1 {
    #[doc = "Secure access"]
    NSE0 = 0x0,
    #[doc = "Nonsecure access"]
    NSE1 = 0x01,
}
impl PcnsNse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcnsNse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcnsNse1 {
    #[inline(always)]
    fn from(val: u8) -> PcnsNse1 {
        PcnsNse1::from_bits(val)
    }
}
impl From<PcnsNse1> for u8 {
    #[inline(always)]
    fn from(val: PcnsNse1) -> u8 {
        PcnsNse1::to_bits(val)
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
pub enum Pdd0 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd0 {
    #[inline(always)]
    fn from(val: u8) -> Pdd0 {
        Pdd0::from_bits(val)
    }
}
impl From<Pdd0> for u8 {
    #[inline(always)]
    fn from(val: Pdd0) -> u8 {
        Pdd0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd1 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd1 {
    #[inline(always)]
    fn from(val: u8) -> Pdd1 {
        Pdd1::from_bits(val)
    }
}
impl From<Pdd1> for u8 {
    #[inline(always)]
    fn from(val: Pdd1) -> u8 {
        Pdd1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd10 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd10 {
    #[inline(always)]
    fn from(val: u8) -> Pdd10 {
        Pdd10::from_bits(val)
    }
}
impl From<Pdd10> for u8 {
    #[inline(always)]
    fn from(val: Pdd10) -> u8 {
        Pdd10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd11 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd11 {
    #[inline(always)]
    fn from(val: u8) -> Pdd11 {
        Pdd11::from_bits(val)
    }
}
impl From<Pdd11> for u8 {
    #[inline(always)]
    fn from(val: Pdd11) -> u8 {
        Pdd11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd12 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd12 {
    #[inline(always)]
    fn from(val: u8) -> Pdd12 {
        Pdd12::from_bits(val)
    }
}
impl From<Pdd12> for u8 {
    #[inline(always)]
    fn from(val: Pdd12) -> u8 {
        Pdd12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd13 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd13 {
    #[inline(always)]
    fn from(val: u8) -> Pdd13 {
        Pdd13::from_bits(val)
    }
}
impl From<Pdd13> for u8 {
    #[inline(always)]
    fn from(val: Pdd13) -> u8 {
        Pdd13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd14 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd14 {
    #[inline(always)]
    fn from(val: u8) -> Pdd14 {
        Pdd14::from_bits(val)
    }
}
impl From<Pdd14> for u8 {
    #[inline(always)]
    fn from(val: Pdd14) -> u8 {
        Pdd14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd15 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd15 {
    #[inline(always)]
    fn from(val: u8) -> Pdd15 {
        Pdd15::from_bits(val)
    }
}
impl From<Pdd15> for u8 {
    #[inline(always)]
    fn from(val: Pdd15) -> u8 {
        Pdd15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd16 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd16 {
    #[inline(always)]
    fn from(val: u8) -> Pdd16 {
        Pdd16::from_bits(val)
    }
}
impl From<Pdd16> for u8 {
    #[inline(always)]
    fn from(val: Pdd16) -> u8 {
        Pdd16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd17 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd17 {
    #[inline(always)]
    fn from(val: u8) -> Pdd17 {
        Pdd17::from_bits(val)
    }
}
impl From<Pdd17> for u8 {
    #[inline(always)]
    fn from(val: Pdd17) -> u8 {
        Pdd17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd18 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd18 {
    #[inline(always)]
    fn from(val: u8) -> Pdd18 {
        Pdd18::from_bits(val)
    }
}
impl From<Pdd18> for u8 {
    #[inline(always)]
    fn from(val: Pdd18) -> u8 {
        Pdd18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd19 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd19 {
    #[inline(always)]
    fn from(val: u8) -> Pdd19 {
        Pdd19::from_bits(val)
    }
}
impl From<Pdd19> for u8 {
    #[inline(always)]
    fn from(val: Pdd19) -> u8 {
        Pdd19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd2 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd2 {
    #[inline(always)]
    fn from(val: u8) -> Pdd2 {
        Pdd2::from_bits(val)
    }
}
impl From<Pdd2> for u8 {
    #[inline(always)]
    fn from(val: Pdd2) -> u8 {
        Pdd2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd20 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd20 {
    #[inline(always)]
    fn from(val: u8) -> Pdd20 {
        Pdd20::from_bits(val)
    }
}
impl From<Pdd20> for u8 {
    #[inline(always)]
    fn from(val: Pdd20) -> u8 {
        Pdd20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd21 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd21 {
    #[inline(always)]
    fn from(val: u8) -> Pdd21 {
        Pdd21::from_bits(val)
    }
}
impl From<Pdd21> for u8 {
    #[inline(always)]
    fn from(val: Pdd21) -> u8 {
        Pdd21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd22 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd22 {
    #[inline(always)]
    fn from(val: u8) -> Pdd22 {
        Pdd22::from_bits(val)
    }
}
impl From<Pdd22> for u8 {
    #[inline(always)]
    fn from(val: Pdd22) -> u8 {
        Pdd22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd23 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd23 {
    #[inline(always)]
    fn from(val: u8) -> Pdd23 {
        Pdd23::from_bits(val)
    }
}
impl From<Pdd23> for u8 {
    #[inline(always)]
    fn from(val: Pdd23) -> u8 {
        Pdd23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd24 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd24 {
    #[inline(always)]
    fn from(val: u8) -> Pdd24 {
        Pdd24::from_bits(val)
    }
}
impl From<Pdd24> for u8 {
    #[inline(always)]
    fn from(val: Pdd24) -> u8 {
        Pdd24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd25 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd25 {
    #[inline(always)]
    fn from(val: u8) -> Pdd25 {
        Pdd25::from_bits(val)
    }
}
impl From<Pdd25> for u8 {
    #[inline(always)]
    fn from(val: Pdd25) -> u8 {
        Pdd25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd26 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd26 {
    #[inline(always)]
    fn from(val: u8) -> Pdd26 {
        Pdd26::from_bits(val)
    }
}
impl From<Pdd26> for u8 {
    #[inline(always)]
    fn from(val: Pdd26) -> u8 {
        Pdd26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd27 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd27 {
    #[inline(always)]
    fn from(val: u8) -> Pdd27 {
        Pdd27::from_bits(val)
    }
}
impl From<Pdd27> for u8 {
    #[inline(always)]
    fn from(val: Pdd27) -> u8 {
        Pdd27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd28 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd28 {
    #[inline(always)]
    fn from(val: u8) -> Pdd28 {
        Pdd28::from_bits(val)
    }
}
impl From<Pdd28> for u8 {
    #[inline(always)]
    fn from(val: Pdd28) -> u8 {
        Pdd28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd29 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd29 {
    #[inline(always)]
    fn from(val: u8) -> Pdd29 {
        Pdd29::from_bits(val)
    }
}
impl From<Pdd29> for u8 {
    #[inline(always)]
    fn from(val: Pdd29) -> u8 {
        Pdd29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd3 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd3 {
    #[inline(always)]
    fn from(val: u8) -> Pdd3 {
        Pdd3::from_bits(val)
    }
}
impl From<Pdd3> for u8 {
    #[inline(always)]
    fn from(val: Pdd3) -> u8 {
        Pdd3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd30 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd30 {
    #[inline(always)]
    fn from(val: u8) -> Pdd30 {
        Pdd30::from_bits(val)
    }
}
impl From<Pdd30> for u8 {
    #[inline(always)]
    fn from(val: Pdd30) -> u8 {
        Pdd30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd31 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd31 {
    #[inline(always)]
    fn from(val: u8) -> Pdd31 {
        Pdd31::from_bits(val)
    }
}
impl From<Pdd31> for u8 {
    #[inline(always)]
    fn from(val: Pdd31) -> u8 {
        Pdd31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd4 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd4 {
    #[inline(always)]
    fn from(val: u8) -> Pdd4 {
        Pdd4::from_bits(val)
    }
}
impl From<Pdd4> for u8 {
    #[inline(always)]
    fn from(val: Pdd4) -> u8 {
        Pdd4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd5 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd5 {
    #[inline(always)]
    fn from(val: u8) -> Pdd5 {
        Pdd5::from_bits(val)
    }
}
impl From<Pdd5> for u8 {
    #[inline(always)]
    fn from(val: Pdd5) -> u8 {
        Pdd5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd6 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd6 {
    #[inline(always)]
    fn from(val: u8) -> Pdd6 {
        Pdd6::from_bits(val)
    }
}
impl From<Pdd6> for u8 {
    #[inline(always)]
    fn from(val: Pdd6) -> u8 {
        Pdd6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd7 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd7 {
    #[inline(always)]
    fn from(val: u8) -> Pdd7 {
        Pdd7::from_bits(val)
    }
}
impl From<Pdd7> for u8 {
    #[inline(always)]
    fn from(val: Pdd7) -> u8 {
        Pdd7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd8 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd8 {
    #[inline(always)]
    fn from(val: u8) -> Pdd8 {
        Pdd8::from_bits(val)
    }
}
impl From<Pdd8> for u8 {
    #[inline(always)]
    fn from(val: Pdd8) -> u8 {
        Pdd8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd9 {
    #[doc = "Input"]
    PDD0 = 0x0,
    #[doc = "Output"]
    PDD1 = 0x01,
}
impl Pdd9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd9 {
    #[inline(always)]
    fn from(val: u8) -> Pdd9 {
        Pdd9::from_bits(val)
    }
}
impl From<Pdd9> for u8 {
    #[inline(always)]
    fn from(val: Pdd9) -> u8 {
        Pdd9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi0 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi0 {
    #[inline(always)]
    fn from(val: u8) -> Pdi0 {
        Pdi0::from_bits(val)
    }
}
impl From<Pdi0> for u8 {
    #[inline(always)]
    fn from(val: Pdi0) -> u8 {
        Pdi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi1 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi1 {
    #[inline(always)]
    fn from(val: u8) -> Pdi1 {
        Pdi1::from_bits(val)
    }
}
impl From<Pdi1> for u8 {
    #[inline(always)]
    fn from(val: Pdi1) -> u8 {
        Pdi1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi10 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi10 {
    #[inline(always)]
    fn from(val: u8) -> Pdi10 {
        Pdi10::from_bits(val)
    }
}
impl From<Pdi10> for u8 {
    #[inline(always)]
    fn from(val: Pdi10) -> u8 {
        Pdi10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi11 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi11 {
    #[inline(always)]
    fn from(val: u8) -> Pdi11 {
        Pdi11::from_bits(val)
    }
}
impl From<Pdi11> for u8 {
    #[inline(always)]
    fn from(val: Pdi11) -> u8 {
        Pdi11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi12 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi12 {
    #[inline(always)]
    fn from(val: u8) -> Pdi12 {
        Pdi12::from_bits(val)
    }
}
impl From<Pdi12> for u8 {
    #[inline(always)]
    fn from(val: Pdi12) -> u8 {
        Pdi12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi13 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi13 {
    #[inline(always)]
    fn from(val: u8) -> Pdi13 {
        Pdi13::from_bits(val)
    }
}
impl From<Pdi13> for u8 {
    #[inline(always)]
    fn from(val: Pdi13) -> u8 {
        Pdi13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi14 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi14 {
    #[inline(always)]
    fn from(val: u8) -> Pdi14 {
        Pdi14::from_bits(val)
    }
}
impl From<Pdi14> for u8 {
    #[inline(always)]
    fn from(val: Pdi14) -> u8 {
        Pdi14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi15 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi15 {
    #[inline(always)]
    fn from(val: u8) -> Pdi15 {
        Pdi15::from_bits(val)
    }
}
impl From<Pdi15> for u8 {
    #[inline(always)]
    fn from(val: Pdi15) -> u8 {
        Pdi15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi16 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi16 {
    #[inline(always)]
    fn from(val: u8) -> Pdi16 {
        Pdi16::from_bits(val)
    }
}
impl From<Pdi16> for u8 {
    #[inline(always)]
    fn from(val: Pdi16) -> u8 {
        Pdi16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi17 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi17 {
    #[inline(always)]
    fn from(val: u8) -> Pdi17 {
        Pdi17::from_bits(val)
    }
}
impl From<Pdi17> for u8 {
    #[inline(always)]
    fn from(val: Pdi17) -> u8 {
        Pdi17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi18 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi18 {
    #[inline(always)]
    fn from(val: u8) -> Pdi18 {
        Pdi18::from_bits(val)
    }
}
impl From<Pdi18> for u8 {
    #[inline(always)]
    fn from(val: Pdi18) -> u8 {
        Pdi18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi19 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi19 {
    #[inline(always)]
    fn from(val: u8) -> Pdi19 {
        Pdi19::from_bits(val)
    }
}
impl From<Pdi19> for u8 {
    #[inline(always)]
    fn from(val: Pdi19) -> u8 {
        Pdi19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi2 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi2 {
    #[inline(always)]
    fn from(val: u8) -> Pdi2 {
        Pdi2::from_bits(val)
    }
}
impl From<Pdi2> for u8 {
    #[inline(always)]
    fn from(val: Pdi2) -> u8 {
        Pdi2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi20 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi20 {
    #[inline(always)]
    fn from(val: u8) -> Pdi20 {
        Pdi20::from_bits(val)
    }
}
impl From<Pdi20> for u8 {
    #[inline(always)]
    fn from(val: Pdi20) -> u8 {
        Pdi20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi21 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi21 {
    #[inline(always)]
    fn from(val: u8) -> Pdi21 {
        Pdi21::from_bits(val)
    }
}
impl From<Pdi21> for u8 {
    #[inline(always)]
    fn from(val: Pdi21) -> u8 {
        Pdi21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi22 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi22 {
    #[inline(always)]
    fn from(val: u8) -> Pdi22 {
        Pdi22::from_bits(val)
    }
}
impl From<Pdi22> for u8 {
    #[inline(always)]
    fn from(val: Pdi22) -> u8 {
        Pdi22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi23 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi23 {
    #[inline(always)]
    fn from(val: u8) -> Pdi23 {
        Pdi23::from_bits(val)
    }
}
impl From<Pdi23> for u8 {
    #[inline(always)]
    fn from(val: Pdi23) -> u8 {
        Pdi23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi24 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi24 {
    #[inline(always)]
    fn from(val: u8) -> Pdi24 {
        Pdi24::from_bits(val)
    }
}
impl From<Pdi24> for u8 {
    #[inline(always)]
    fn from(val: Pdi24) -> u8 {
        Pdi24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi25 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi25 {
    #[inline(always)]
    fn from(val: u8) -> Pdi25 {
        Pdi25::from_bits(val)
    }
}
impl From<Pdi25> for u8 {
    #[inline(always)]
    fn from(val: Pdi25) -> u8 {
        Pdi25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi26 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi26 {
    #[inline(always)]
    fn from(val: u8) -> Pdi26 {
        Pdi26::from_bits(val)
    }
}
impl From<Pdi26> for u8 {
    #[inline(always)]
    fn from(val: Pdi26) -> u8 {
        Pdi26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi27 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi27 {
    #[inline(always)]
    fn from(val: u8) -> Pdi27 {
        Pdi27::from_bits(val)
    }
}
impl From<Pdi27> for u8 {
    #[inline(always)]
    fn from(val: Pdi27) -> u8 {
        Pdi27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi28 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi28 {
    #[inline(always)]
    fn from(val: u8) -> Pdi28 {
        Pdi28::from_bits(val)
    }
}
impl From<Pdi28> for u8 {
    #[inline(always)]
    fn from(val: Pdi28) -> u8 {
        Pdi28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi29 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi29 {
    #[inline(always)]
    fn from(val: u8) -> Pdi29 {
        Pdi29::from_bits(val)
    }
}
impl From<Pdi29> for u8 {
    #[inline(always)]
    fn from(val: Pdi29) -> u8 {
        Pdi29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi3 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi3 {
    #[inline(always)]
    fn from(val: u8) -> Pdi3 {
        Pdi3::from_bits(val)
    }
}
impl From<Pdi3> for u8 {
    #[inline(always)]
    fn from(val: Pdi3) -> u8 {
        Pdi3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi30 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi30 {
    #[inline(always)]
    fn from(val: u8) -> Pdi30 {
        Pdi30::from_bits(val)
    }
}
impl From<Pdi30> for u8 {
    #[inline(always)]
    fn from(val: Pdi30) -> u8 {
        Pdi30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi31 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi31 {
    #[inline(always)]
    fn from(val: u8) -> Pdi31 {
        Pdi31::from_bits(val)
    }
}
impl From<Pdi31> for u8 {
    #[inline(always)]
    fn from(val: Pdi31) -> u8 {
        Pdi31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi4 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi4 {
    #[inline(always)]
    fn from(val: u8) -> Pdi4 {
        Pdi4::from_bits(val)
    }
}
impl From<Pdi4> for u8 {
    #[inline(always)]
    fn from(val: Pdi4) -> u8 {
        Pdi4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi5 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi5 {
    #[inline(always)]
    fn from(val: u8) -> Pdi5 {
        Pdi5::from_bits(val)
    }
}
impl From<Pdi5> for u8 {
    #[inline(always)]
    fn from(val: Pdi5) -> u8 {
        Pdi5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi6 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi6 {
    #[inline(always)]
    fn from(val: u8) -> Pdi6 {
        Pdi6::from_bits(val)
    }
}
impl From<Pdi6> for u8 {
    #[inline(always)]
    fn from(val: Pdi6) -> u8 {
        Pdi6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi7 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi7 {
    #[inline(always)]
    fn from(val: u8) -> Pdi7 {
        Pdi7::from_bits(val)
    }
}
impl From<Pdi7> for u8 {
    #[inline(always)]
    fn from(val: Pdi7) -> u8 {
        Pdi7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi8 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi8 {
    #[inline(always)]
    fn from(val: u8) -> Pdi8 {
        Pdi8::from_bits(val)
    }
}
impl From<Pdi8> for u8 {
    #[inline(always)]
    fn from(val: Pdi8) -> u8 {
        Pdi8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi9 {
    #[doc = "Logic 0"]
    PDI0 = 0x0,
    #[doc = "Logic 1"]
    PDI1 = 0x01,
}
impl Pdi9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi9 {
    #[inline(always)]
    fn from(val: u8) -> Pdi9 {
        Pdi9::from_bits(val)
    }
}
impl From<Pdi9> for u8 {
    #[inline(always)]
    fn from(val: Pdi9) -> u8 {
        Pdi9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo0 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo0 {
    #[inline(always)]
    fn from(val: u8) -> Pdo0 {
        Pdo0::from_bits(val)
    }
}
impl From<Pdo0> for u8 {
    #[inline(always)]
    fn from(val: Pdo0) -> u8 {
        Pdo0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo1 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo1 {
    #[inline(always)]
    fn from(val: u8) -> Pdo1 {
        Pdo1::from_bits(val)
    }
}
impl From<Pdo1> for u8 {
    #[inline(always)]
    fn from(val: Pdo1) -> u8 {
        Pdo1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo10 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo10 {
    #[inline(always)]
    fn from(val: u8) -> Pdo10 {
        Pdo10::from_bits(val)
    }
}
impl From<Pdo10> for u8 {
    #[inline(always)]
    fn from(val: Pdo10) -> u8 {
        Pdo10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo11 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo11 {
    #[inline(always)]
    fn from(val: u8) -> Pdo11 {
        Pdo11::from_bits(val)
    }
}
impl From<Pdo11> for u8 {
    #[inline(always)]
    fn from(val: Pdo11) -> u8 {
        Pdo11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo12 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo12 {
    #[inline(always)]
    fn from(val: u8) -> Pdo12 {
        Pdo12::from_bits(val)
    }
}
impl From<Pdo12> for u8 {
    #[inline(always)]
    fn from(val: Pdo12) -> u8 {
        Pdo12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo13 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo13 {
    #[inline(always)]
    fn from(val: u8) -> Pdo13 {
        Pdo13::from_bits(val)
    }
}
impl From<Pdo13> for u8 {
    #[inline(always)]
    fn from(val: Pdo13) -> u8 {
        Pdo13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo14 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo14 {
    #[inline(always)]
    fn from(val: u8) -> Pdo14 {
        Pdo14::from_bits(val)
    }
}
impl From<Pdo14> for u8 {
    #[inline(always)]
    fn from(val: Pdo14) -> u8 {
        Pdo14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo15 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo15 {
    #[inline(always)]
    fn from(val: u8) -> Pdo15 {
        Pdo15::from_bits(val)
    }
}
impl From<Pdo15> for u8 {
    #[inline(always)]
    fn from(val: Pdo15) -> u8 {
        Pdo15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo16 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo16 {
    #[inline(always)]
    fn from(val: u8) -> Pdo16 {
        Pdo16::from_bits(val)
    }
}
impl From<Pdo16> for u8 {
    #[inline(always)]
    fn from(val: Pdo16) -> u8 {
        Pdo16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo17 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo17 {
    #[inline(always)]
    fn from(val: u8) -> Pdo17 {
        Pdo17::from_bits(val)
    }
}
impl From<Pdo17> for u8 {
    #[inline(always)]
    fn from(val: Pdo17) -> u8 {
        Pdo17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo18 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo18 {
    #[inline(always)]
    fn from(val: u8) -> Pdo18 {
        Pdo18::from_bits(val)
    }
}
impl From<Pdo18> for u8 {
    #[inline(always)]
    fn from(val: Pdo18) -> u8 {
        Pdo18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo19 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo19 {
    #[inline(always)]
    fn from(val: u8) -> Pdo19 {
        Pdo19::from_bits(val)
    }
}
impl From<Pdo19> for u8 {
    #[inline(always)]
    fn from(val: Pdo19) -> u8 {
        Pdo19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo2 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo2 {
    #[inline(always)]
    fn from(val: u8) -> Pdo2 {
        Pdo2::from_bits(val)
    }
}
impl From<Pdo2> for u8 {
    #[inline(always)]
    fn from(val: Pdo2) -> u8 {
        Pdo2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo20 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo20 {
    #[inline(always)]
    fn from(val: u8) -> Pdo20 {
        Pdo20::from_bits(val)
    }
}
impl From<Pdo20> for u8 {
    #[inline(always)]
    fn from(val: Pdo20) -> u8 {
        Pdo20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo21 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo21 {
    #[inline(always)]
    fn from(val: u8) -> Pdo21 {
        Pdo21::from_bits(val)
    }
}
impl From<Pdo21> for u8 {
    #[inline(always)]
    fn from(val: Pdo21) -> u8 {
        Pdo21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo22 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo22 {
    #[inline(always)]
    fn from(val: u8) -> Pdo22 {
        Pdo22::from_bits(val)
    }
}
impl From<Pdo22> for u8 {
    #[inline(always)]
    fn from(val: Pdo22) -> u8 {
        Pdo22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo23 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo23 {
    #[inline(always)]
    fn from(val: u8) -> Pdo23 {
        Pdo23::from_bits(val)
    }
}
impl From<Pdo23> for u8 {
    #[inline(always)]
    fn from(val: Pdo23) -> u8 {
        Pdo23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo24 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo24 {
    #[inline(always)]
    fn from(val: u8) -> Pdo24 {
        Pdo24::from_bits(val)
    }
}
impl From<Pdo24> for u8 {
    #[inline(always)]
    fn from(val: Pdo24) -> u8 {
        Pdo24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo25 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo25 {
    #[inline(always)]
    fn from(val: u8) -> Pdo25 {
        Pdo25::from_bits(val)
    }
}
impl From<Pdo25> for u8 {
    #[inline(always)]
    fn from(val: Pdo25) -> u8 {
        Pdo25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo26 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo26 {
    #[inline(always)]
    fn from(val: u8) -> Pdo26 {
        Pdo26::from_bits(val)
    }
}
impl From<Pdo26> for u8 {
    #[inline(always)]
    fn from(val: Pdo26) -> u8 {
        Pdo26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo27 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo27 {
    #[inline(always)]
    fn from(val: u8) -> Pdo27 {
        Pdo27::from_bits(val)
    }
}
impl From<Pdo27> for u8 {
    #[inline(always)]
    fn from(val: Pdo27) -> u8 {
        Pdo27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo28 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo28 {
    #[inline(always)]
    fn from(val: u8) -> Pdo28 {
        Pdo28::from_bits(val)
    }
}
impl From<Pdo28> for u8 {
    #[inline(always)]
    fn from(val: Pdo28) -> u8 {
        Pdo28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo29 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo29 {
    #[inline(always)]
    fn from(val: u8) -> Pdo29 {
        Pdo29::from_bits(val)
    }
}
impl From<Pdo29> for u8 {
    #[inline(always)]
    fn from(val: Pdo29) -> u8 {
        Pdo29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo3 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo3 {
    #[inline(always)]
    fn from(val: u8) -> Pdo3 {
        Pdo3::from_bits(val)
    }
}
impl From<Pdo3> for u8 {
    #[inline(always)]
    fn from(val: Pdo3) -> u8 {
        Pdo3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo30 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo30 {
    #[inline(always)]
    fn from(val: u8) -> Pdo30 {
        Pdo30::from_bits(val)
    }
}
impl From<Pdo30> for u8 {
    #[inline(always)]
    fn from(val: Pdo30) -> u8 {
        Pdo30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo31 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo31 {
    #[inline(always)]
    fn from(val: u8) -> Pdo31 {
        Pdo31::from_bits(val)
    }
}
impl From<Pdo31> for u8 {
    #[inline(always)]
    fn from(val: Pdo31) -> u8 {
        Pdo31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo4 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo4 {
    #[inline(always)]
    fn from(val: u8) -> Pdo4 {
        Pdo4::from_bits(val)
    }
}
impl From<Pdo4> for u8 {
    #[inline(always)]
    fn from(val: Pdo4) -> u8 {
        Pdo4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo5 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo5 {
    #[inline(always)]
    fn from(val: u8) -> Pdo5 {
        Pdo5::from_bits(val)
    }
}
impl From<Pdo5> for u8 {
    #[inline(always)]
    fn from(val: Pdo5) -> u8 {
        Pdo5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo6 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo6 {
    #[inline(always)]
    fn from(val: u8) -> Pdo6 {
        Pdo6::from_bits(val)
    }
}
impl From<Pdo6> for u8 {
    #[inline(always)]
    fn from(val: Pdo6) -> u8 {
        Pdo6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo7 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo7 {
    #[inline(always)]
    fn from(val: u8) -> Pdo7 {
        Pdo7::from_bits(val)
    }
}
impl From<Pdo7> for u8 {
    #[inline(always)]
    fn from(val: Pdo7) -> u8 {
        Pdo7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo8 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo8 {
    #[inline(always)]
    fn from(val: u8) -> Pdo8 {
        Pdo8::from_bits(val)
    }
}
impl From<Pdo8> for u8 {
    #[inline(always)]
    fn from(val: Pdo8) -> u8 {
        Pdo8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo9 {
    #[doc = "Logic level 0"]
    PDO0 = 0x0,
    #[doc = "Logic level 1"]
    PDO1 = 0x01,
}
impl Pdo9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo9 {
    #[inline(always)]
    fn from(val: u8) -> Pdo9 {
        Pdo9::from_bits(val)
    }
}
impl From<Pdo9> for u8 {
    #[inline(always)]
    fn from(val: Pdo9) -> u8 {
        Pdo9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid0 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid0 {
    #[inline(always)]
    fn from(val: u8) -> Pid0 {
        Pid0::from_bits(val)
    }
}
impl From<Pid0> for u8 {
    #[inline(always)]
    fn from(val: Pid0) -> u8 {
        Pid0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid1 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid1 {
    #[inline(always)]
    fn from(val: u8) -> Pid1 {
        Pid1::from_bits(val)
    }
}
impl From<Pid1> for u8 {
    #[inline(always)]
    fn from(val: Pid1) -> u8 {
        Pid1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid10 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid10 {
    #[inline(always)]
    fn from(val: u8) -> Pid10 {
        Pid10::from_bits(val)
    }
}
impl From<Pid10> for u8 {
    #[inline(always)]
    fn from(val: Pid10) -> u8 {
        Pid10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid11 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid11 {
    #[inline(always)]
    fn from(val: u8) -> Pid11 {
        Pid11::from_bits(val)
    }
}
impl From<Pid11> for u8 {
    #[inline(always)]
    fn from(val: Pid11) -> u8 {
        Pid11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid12 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid12 {
    #[inline(always)]
    fn from(val: u8) -> Pid12 {
        Pid12::from_bits(val)
    }
}
impl From<Pid12> for u8 {
    #[inline(always)]
    fn from(val: Pid12) -> u8 {
        Pid12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid13 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid13 {
    #[inline(always)]
    fn from(val: u8) -> Pid13 {
        Pid13::from_bits(val)
    }
}
impl From<Pid13> for u8 {
    #[inline(always)]
    fn from(val: Pid13) -> u8 {
        Pid13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid14 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid14 {
    #[inline(always)]
    fn from(val: u8) -> Pid14 {
        Pid14::from_bits(val)
    }
}
impl From<Pid14> for u8 {
    #[inline(always)]
    fn from(val: Pid14) -> u8 {
        Pid14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid15 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid15 {
    #[inline(always)]
    fn from(val: u8) -> Pid15 {
        Pid15::from_bits(val)
    }
}
impl From<Pid15> for u8 {
    #[inline(always)]
    fn from(val: Pid15) -> u8 {
        Pid15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid16 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid16 {
    #[inline(always)]
    fn from(val: u8) -> Pid16 {
        Pid16::from_bits(val)
    }
}
impl From<Pid16> for u8 {
    #[inline(always)]
    fn from(val: Pid16) -> u8 {
        Pid16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid17 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid17 {
    #[inline(always)]
    fn from(val: u8) -> Pid17 {
        Pid17::from_bits(val)
    }
}
impl From<Pid17> for u8 {
    #[inline(always)]
    fn from(val: Pid17) -> u8 {
        Pid17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid18 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid18 {
    #[inline(always)]
    fn from(val: u8) -> Pid18 {
        Pid18::from_bits(val)
    }
}
impl From<Pid18> for u8 {
    #[inline(always)]
    fn from(val: Pid18) -> u8 {
        Pid18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid19 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid19 {
    #[inline(always)]
    fn from(val: u8) -> Pid19 {
        Pid19::from_bits(val)
    }
}
impl From<Pid19> for u8 {
    #[inline(always)]
    fn from(val: Pid19) -> u8 {
        Pid19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid2 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid2 {
    #[inline(always)]
    fn from(val: u8) -> Pid2 {
        Pid2::from_bits(val)
    }
}
impl From<Pid2> for u8 {
    #[inline(always)]
    fn from(val: Pid2) -> u8 {
        Pid2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid20 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid20 {
    #[inline(always)]
    fn from(val: u8) -> Pid20 {
        Pid20::from_bits(val)
    }
}
impl From<Pid20> for u8 {
    #[inline(always)]
    fn from(val: Pid20) -> u8 {
        Pid20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid21 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid21 {
    #[inline(always)]
    fn from(val: u8) -> Pid21 {
        Pid21::from_bits(val)
    }
}
impl From<Pid21> for u8 {
    #[inline(always)]
    fn from(val: Pid21) -> u8 {
        Pid21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid22 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid22 {
    #[inline(always)]
    fn from(val: u8) -> Pid22 {
        Pid22::from_bits(val)
    }
}
impl From<Pid22> for u8 {
    #[inline(always)]
    fn from(val: Pid22) -> u8 {
        Pid22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid23 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid23 {
    #[inline(always)]
    fn from(val: u8) -> Pid23 {
        Pid23::from_bits(val)
    }
}
impl From<Pid23> for u8 {
    #[inline(always)]
    fn from(val: Pid23) -> u8 {
        Pid23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid24 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid24 {
    #[inline(always)]
    fn from(val: u8) -> Pid24 {
        Pid24::from_bits(val)
    }
}
impl From<Pid24> for u8 {
    #[inline(always)]
    fn from(val: Pid24) -> u8 {
        Pid24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid25 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid25 {
    #[inline(always)]
    fn from(val: u8) -> Pid25 {
        Pid25::from_bits(val)
    }
}
impl From<Pid25> for u8 {
    #[inline(always)]
    fn from(val: Pid25) -> u8 {
        Pid25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid26 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid26 {
    #[inline(always)]
    fn from(val: u8) -> Pid26 {
        Pid26::from_bits(val)
    }
}
impl From<Pid26> for u8 {
    #[inline(always)]
    fn from(val: Pid26) -> u8 {
        Pid26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid27 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid27 {
    #[inline(always)]
    fn from(val: u8) -> Pid27 {
        Pid27::from_bits(val)
    }
}
impl From<Pid27> for u8 {
    #[inline(always)]
    fn from(val: Pid27) -> u8 {
        Pid27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid28 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid28 {
    #[inline(always)]
    fn from(val: u8) -> Pid28 {
        Pid28::from_bits(val)
    }
}
impl From<Pid28> for u8 {
    #[inline(always)]
    fn from(val: Pid28) -> u8 {
        Pid28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid29 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid29 {
    #[inline(always)]
    fn from(val: u8) -> Pid29 {
        Pid29::from_bits(val)
    }
}
impl From<Pid29> for u8 {
    #[inline(always)]
    fn from(val: Pid29) -> u8 {
        Pid29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid3 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid3 {
    #[inline(always)]
    fn from(val: u8) -> Pid3 {
        Pid3::from_bits(val)
    }
}
impl From<Pid3> for u8 {
    #[inline(always)]
    fn from(val: Pid3) -> u8 {
        Pid3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid30 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid30 {
    #[inline(always)]
    fn from(val: u8) -> Pid30 {
        Pid30::from_bits(val)
    }
}
impl From<Pid30> for u8 {
    #[inline(always)]
    fn from(val: Pid30) -> u8 {
        Pid30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid31 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid31 {
    #[inline(always)]
    fn from(val: u8) -> Pid31 {
        Pid31::from_bits(val)
    }
}
impl From<Pid31> for u8 {
    #[inline(always)]
    fn from(val: Pid31) -> u8 {
        Pid31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid4 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid4 {
    #[inline(always)]
    fn from(val: u8) -> Pid4 {
        Pid4::from_bits(val)
    }
}
impl From<Pid4> for u8 {
    #[inline(always)]
    fn from(val: Pid4) -> u8 {
        Pid4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid5 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid5 {
    #[inline(always)]
    fn from(val: u8) -> Pid5 {
        Pid5::from_bits(val)
    }
}
impl From<Pid5> for u8 {
    #[inline(always)]
    fn from(val: Pid5) -> u8 {
        Pid5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid6 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid6 {
    #[inline(always)]
    fn from(val: u8) -> Pid6 {
        Pid6::from_bits(val)
    }
}
impl From<Pid6> for u8 {
    #[inline(always)]
    fn from(val: Pid6) -> u8 {
        Pid6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid7 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid7 {
    #[inline(always)]
    fn from(val: u8) -> Pid7 {
        Pid7::from_bits(val)
    }
}
impl From<Pid7> for u8 {
    #[inline(always)]
    fn from(val: Pid7) -> u8 {
        Pid7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid8 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid8 {
    #[inline(always)]
    fn from(val: u8) -> Pid8 {
        Pid8::from_bits(val)
    }
}
impl From<Pid8> for u8 {
    #[inline(always)]
    fn from(val: Pid8) -> u8 {
        Pid8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid9 {
    #[doc = "Configured for general-purpose input"]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input"]
    PID1 = 0x01,
}
impl Pid9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid9 {
    #[inline(always)]
    fn from(val: u8) -> Pid9 {
        Pid9::from_bits(val)
    }
}
impl From<Pid9> for u8 {
    #[inline(always)]
    fn from(val: Pid9) -> u8 {
        Pid9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco0 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco0 {
    #[inline(always)]
    fn from(val: u8) -> Ptco0 {
        Ptco0::from_bits(val)
    }
}
impl From<Ptco0> for u8 {
    #[inline(always)]
    fn from(val: Ptco0) -> u8 {
        Ptco0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco1 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco1 {
    #[inline(always)]
    fn from(val: u8) -> Ptco1 {
        Ptco1::from_bits(val)
    }
}
impl From<Ptco1> for u8 {
    #[inline(always)]
    fn from(val: Ptco1) -> u8 {
        Ptco1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco10 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco10 {
    #[inline(always)]
    fn from(val: u8) -> Ptco10 {
        Ptco10::from_bits(val)
    }
}
impl From<Ptco10> for u8 {
    #[inline(always)]
    fn from(val: Ptco10) -> u8 {
        Ptco10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco11 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco11 {
    #[inline(always)]
    fn from(val: u8) -> Ptco11 {
        Ptco11::from_bits(val)
    }
}
impl From<Ptco11> for u8 {
    #[inline(always)]
    fn from(val: Ptco11) -> u8 {
        Ptco11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco12 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco12 {
    #[inline(always)]
    fn from(val: u8) -> Ptco12 {
        Ptco12::from_bits(val)
    }
}
impl From<Ptco12> for u8 {
    #[inline(always)]
    fn from(val: Ptco12) -> u8 {
        Ptco12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco13 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco13 {
    #[inline(always)]
    fn from(val: u8) -> Ptco13 {
        Ptco13::from_bits(val)
    }
}
impl From<Ptco13> for u8 {
    #[inline(always)]
    fn from(val: Ptco13) -> u8 {
        Ptco13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco14 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco14 {
    #[inline(always)]
    fn from(val: u8) -> Ptco14 {
        Ptco14::from_bits(val)
    }
}
impl From<Ptco14> for u8 {
    #[inline(always)]
    fn from(val: Ptco14) -> u8 {
        Ptco14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco15 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco15 {
    #[inline(always)]
    fn from(val: u8) -> Ptco15 {
        Ptco15::from_bits(val)
    }
}
impl From<Ptco15> for u8 {
    #[inline(always)]
    fn from(val: Ptco15) -> u8 {
        Ptco15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco16 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco16 {
    #[inline(always)]
    fn from(val: u8) -> Ptco16 {
        Ptco16::from_bits(val)
    }
}
impl From<Ptco16> for u8 {
    #[inline(always)]
    fn from(val: Ptco16) -> u8 {
        Ptco16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco17 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco17 {
    #[inline(always)]
    fn from(val: u8) -> Ptco17 {
        Ptco17::from_bits(val)
    }
}
impl From<Ptco17> for u8 {
    #[inline(always)]
    fn from(val: Ptco17) -> u8 {
        Ptco17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco18 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco18 {
    #[inline(always)]
    fn from(val: u8) -> Ptco18 {
        Ptco18::from_bits(val)
    }
}
impl From<Ptco18> for u8 {
    #[inline(always)]
    fn from(val: Ptco18) -> u8 {
        Ptco18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco19 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco19 {
    #[inline(always)]
    fn from(val: u8) -> Ptco19 {
        Ptco19::from_bits(val)
    }
}
impl From<Ptco19> for u8 {
    #[inline(always)]
    fn from(val: Ptco19) -> u8 {
        Ptco19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco2 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco2 {
    #[inline(always)]
    fn from(val: u8) -> Ptco2 {
        Ptco2::from_bits(val)
    }
}
impl From<Ptco2> for u8 {
    #[inline(always)]
    fn from(val: Ptco2) -> u8 {
        Ptco2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco20 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco20 {
    #[inline(always)]
    fn from(val: u8) -> Ptco20 {
        Ptco20::from_bits(val)
    }
}
impl From<Ptco20> for u8 {
    #[inline(always)]
    fn from(val: Ptco20) -> u8 {
        Ptco20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco21 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco21 {
    #[inline(always)]
    fn from(val: u8) -> Ptco21 {
        Ptco21::from_bits(val)
    }
}
impl From<Ptco21> for u8 {
    #[inline(always)]
    fn from(val: Ptco21) -> u8 {
        Ptco21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco22 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco22 {
    #[inline(always)]
    fn from(val: u8) -> Ptco22 {
        Ptco22::from_bits(val)
    }
}
impl From<Ptco22> for u8 {
    #[inline(always)]
    fn from(val: Ptco22) -> u8 {
        Ptco22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco23 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco23 {
    #[inline(always)]
    fn from(val: u8) -> Ptco23 {
        Ptco23::from_bits(val)
    }
}
impl From<Ptco23> for u8 {
    #[inline(always)]
    fn from(val: Ptco23) -> u8 {
        Ptco23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco24 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco24 {
    #[inline(always)]
    fn from(val: u8) -> Ptco24 {
        Ptco24::from_bits(val)
    }
}
impl From<Ptco24> for u8 {
    #[inline(always)]
    fn from(val: Ptco24) -> u8 {
        Ptco24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco25 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco25 {
    #[inline(always)]
    fn from(val: u8) -> Ptco25 {
        Ptco25::from_bits(val)
    }
}
impl From<Ptco25> for u8 {
    #[inline(always)]
    fn from(val: Ptco25) -> u8 {
        Ptco25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco26 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco26 {
    #[inline(always)]
    fn from(val: u8) -> Ptco26 {
        Ptco26::from_bits(val)
    }
}
impl From<Ptco26> for u8 {
    #[inline(always)]
    fn from(val: Ptco26) -> u8 {
        Ptco26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco27 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco27 {
    #[inline(always)]
    fn from(val: u8) -> Ptco27 {
        Ptco27::from_bits(val)
    }
}
impl From<Ptco27> for u8 {
    #[inline(always)]
    fn from(val: Ptco27) -> u8 {
        Ptco27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco28 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco28 {
    #[inline(always)]
    fn from(val: u8) -> Ptco28 {
        Ptco28::from_bits(val)
    }
}
impl From<Ptco28> for u8 {
    #[inline(always)]
    fn from(val: Ptco28) -> u8 {
        Ptco28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco29 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco29 {
    #[inline(always)]
    fn from(val: u8) -> Ptco29 {
        Ptco29::from_bits(val)
    }
}
impl From<Ptco29> for u8 {
    #[inline(always)]
    fn from(val: Ptco29) -> u8 {
        Ptco29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco3 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco3 {
    #[inline(always)]
    fn from(val: u8) -> Ptco3 {
        Ptco3::from_bits(val)
    }
}
impl From<Ptco3> for u8 {
    #[inline(always)]
    fn from(val: Ptco3) -> u8 {
        Ptco3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco30 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco30 {
    #[inline(always)]
    fn from(val: u8) -> Ptco30 {
        Ptco30::from_bits(val)
    }
}
impl From<Ptco30> for u8 {
    #[inline(always)]
    fn from(val: Ptco30) -> u8 {
        Ptco30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco31 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco31 {
    #[inline(always)]
    fn from(val: u8) -> Ptco31 {
        Ptco31::from_bits(val)
    }
}
impl From<Ptco31> for u8 {
    #[inline(always)]
    fn from(val: Ptco31) -> u8 {
        Ptco31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco4 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco4 {
    #[inline(always)]
    fn from(val: u8) -> Ptco4 {
        Ptco4::from_bits(val)
    }
}
impl From<Ptco4> for u8 {
    #[inline(always)]
    fn from(val: Ptco4) -> u8 {
        Ptco4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco5 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco5 {
    #[inline(always)]
    fn from(val: u8) -> Ptco5 {
        Ptco5::from_bits(val)
    }
}
impl From<Ptco5> for u8 {
    #[inline(always)]
    fn from(val: Ptco5) -> u8 {
        Ptco5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco6 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco6 {
    #[inline(always)]
    fn from(val: u8) -> Ptco6 {
        Ptco6::from_bits(val)
    }
}
impl From<Ptco6> for u8 {
    #[inline(always)]
    fn from(val: Ptco6) -> u8 {
        Ptco6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco7 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco7 {
    #[inline(always)]
    fn from(val: u8) -> Ptco7 {
        Ptco7::from_bits(val)
    }
}
impl From<Ptco7> for u8 {
    #[inline(always)]
    fn from(val: Ptco7) -> u8 {
        Ptco7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco8 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco8 {
    #[inline(always)]
    fn from(val: u8) -> Ptco8 {
        Ptco8::from_bits(val)
    }
}
impl From<Ptco8> for u8 {
    #[inline(always)]
    fn from(val: Ptco8) -> u8 {
        Ptco8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco9 {
    #[doc = "No change"]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0"]
    PTCO1 = 0x01,
}
impl Ptco9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco9 {
    #[inline(always)]
    fn from(val: u8) -> Ptco9 {
        Ptco9::from_bits(val)
    }
}
impl From<Ptco9> for u8 {
    #[inline(always)]
    fn from(val: Ptco9) -> u8 {
        Ptco9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso0 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso0 {
    #[inline(always)]
    fn from(val: u8) -> Ptso0 {
        Ptso0::from_bits(val)
    }
}
impl From<Ptso0> for u8 {
    #[inline(always)]
    fn from(val: Ptso0) -> u8 {
        Ptso0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso1 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso1 {
    #[inline(always)]
    fn from(val: u8) -> Ptso1 {
        Ptso1::from_bits(val)
    }
}
impl From<Ptso1> for u8 {
    #[inline(always)]
    fn from(val: Ptso1) -> u8 {
        Ptso1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso10 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso10 {
    #[inline(always)]
    fn from(val: u8) -> Ptso10 {
        Ptso10::from_bits(val)
    }
}
impl From<Ptso10> for u8 {
    #[inline(always)]
    fn from(val: Ptso10) -> u8 {
        Ptso10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso11 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso11 {
    #[inline(always)]
    fn from(val: u8) -> Ptso11 {
        Ptso11::from_bits(val)
    }
}
impl From<Ptso11> for u8 {
    #[inline(always)]
    fn from(val: Ptso11) -> u8 {
        Ptso11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso12 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso12 {
    #[inline(always)]
    fn from(val: u8) -> Ptso12 {
        Ptso12::from_bits(val)
    }
}
impl From<Ptso12> for u8 {
    #[inline(always)]
    fn from(val: Ptso12) -> u8 {
        Ptso12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso13 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso13 {
    #[inline(always)]
    fn from(val: u8) -> Ptso13 {
        Ptso13::from_bits(val)
    }
}
impl From<Ptso13> for u8 {
    #[inline(always)]
    fn from(val: Ptso13) -> u8 {
        Ptso13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso14 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso14 {
    #[inline(always)]
    fn from(val: u8) -> Ptso14 {
        Ptso14::from_bits(val)
    }
}
impl From<Ptso14> for u8 {
    #[inline(always)]
    fn from(val: Ptso14) -> u8 {
        Ptso14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso15 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso15 {
    #[inline(always)]
    fn from(val: u8) -> Ptso15 {
        Ptso15::from_bits(val)
    }
}
impl From<Ptso15> for u8 {
    #[inline(always)]
    fn from(val: Ptso15) -> u8 {
        Ptso15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso16 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso16 {
    #[inline(always)]
    fn from(val: u8) -> Ptso16 {
        Ptso16::from_bits(val)
    }
}
impl From<Ptso16> for u8 {
    #[inline(always)]
    fn from(val: Ptso16) -> u8 {
        Ptso16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso17 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso17 {
    #[inline(always)]
    fn from(val: u8) -> Ptso17 {
        Ptso17::from_bits(val)
    }
}
impl From<Ptso17> for u8 {
    #[inline(always)]
    fn from(val: Ptso17) -> u8 {
        Ptso17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso18 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso18 {
    #[inline(always)]
    fn from(val: u8) -> Ptso18 {
        Ptso18::from_bits(val)
    }
}
impl From<Ptso18> for u8 {
    #[inline(always)]
    fn from(val: Ptso18) -> u8 {
        Ptso18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso19 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso19 {
    #[inline(always)]
    fn from(val: u8) -> Ptso19 {
        Ptso19::from_bits(val)
    }
}
impl From<Ptso19> for u8 {
    #[inline(always)]
    fn from(val: Ptso19) -> u8 {
        Ptso19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso2 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso2 {
    #[inline(always)]
    fn from(val: u8) -> Ptso2 {
        Ptso2::from_bits(val)
    }
}
impl From<Ptso2> for u8 {
    #[inline(always)]
    fn from(val: Ptso2) -> u8 {
        Ptso2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso20 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso20 {
    #[inline(always)]
    fn from(val: u8) -> Ptso20 {
        Ptso20::from_bits(val)
    }
}
impl From<Ptso20> for u8 {
    #[inline(always)]
    fn from(val: Ptso20) -> u8 {
        Ptso20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso21 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso21 {
    #[inline(always)]
    fn from(val: u8) -> Ptso21 {
        Ptso21::from_bits(val)
    }
}
impl From<Ptso21> for u8 {
    #[inline(always)]
    fn from(val: Ptso21) -> u8 {
        Ptso21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso22 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso22 {
    #[inline(always)]
    fn from(val: u8) -> Ptso22 {
        Ptso22::from_bits(val)
    }
}
impl From<Ptso22> for u8 {
    #[inline(always)]
    fn from(val: Ptso22) -> u8 {
        Ptso22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso23 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso23 {
    #[inline(always)]
    fn from(val: u8) -> Ptso23 {
        Ptso23::from_bits(val)
    }
}
impl From<Ptso23> for u8 {
    #[inline(always)]
    fn from(val: Ptso23) -> u8 {
        Ptso23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso24 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso24 {
    #[inline(always)]
    fn from(val: u8) -> Ptso24 {
        Ptso24::from_bits(val)
    }
}
impl From<Ptso24> for u8 {
    #[inline(always)]
    fn from(val: Ptso24) -> u8 {
        Ptso24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso25 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso25 {
    #[inline(always)]
    fn from(val: u8) -> Ptso25 {
        Ptso25::from_bits(val)
    }
}
impl From<Ptso25> for u8 {
    #[inline(always)]
    fn from(val: Ptso25) -> u8 {
        Ptso25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso26 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso26 {
    #[inline(always)]
    fn from(val: u8) -> Ptso26 {
        Ptso26::from_bits(val)
    }
}
impl From<Ptso26> for u8 {
    #[inline(always)]
    fn from(val: Ptso26) -> u8 {
        Ptso26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso27 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso27 {
    #[inline(always)]
    fn from(val: u8) -> Ptso27 {
        Ptso27::from_bits(val)
    }
}
impl From<Ptso27> for u8 {
    #[inline(always)]
    fn from(val: Ptso27) -> u8 {
        Ptso27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso28 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso28 {
    #[inline(always)]
    fn from(val: u8) -> Ptso28 {
        Ptso28::from_bits(val)
    }
}
impl From<Ptso28> for u8 {
    #[inline(always)]
    fn from(val: Ptso28) -> u8 {
        Ptso28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso29 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso29 {
    #[inline(always)]
    fn from(val: u8) -> Ptso29 {
        Ptso29::from_bits(val)
    }
}
impl From<Ptso29> for u8 {
    #[inline(always)]
    fn from(val: Ptso29) -> u8 {
        Ptso29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso3 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso3 {
    #[inline(always)]
    fn from(val: u8) -> Ptso3 {
        Ptso3::from_bits(val)
    }
}
impl From<Ptso3> for u8 {
    #[inline(always)]
    fn from(val: Ptso3) -> u8 {
        Ptso3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso30 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso30 {
    #[inline(always)]
    fn from(val: u8) -> Ptso30 {
        Ptso30::from_bits(val)
    }
}
impl From<Ptso30> for u8 {
    #[inline(always)]
    fn from(val: Ptso30) -> u8 {
        Ptso30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso31 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso31 {
    #[inline(always)]
    fn from(val: u8) -> Ptso31 {
        Ptso31::from_bits(val)
    }
}
impl From<Ptso31> for u8 {
    #[inline(always)]
    fn from(val: Ptso31) -> u8 {
        Ptso31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso4 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso4 {
    #[inline(always)]
    fn from(val: u8) -> Ptso4 {
        Ptso4::from_bits(val)
    }
}
impl From<Ptso4> for u8 {
    #[inline(always)]
    fn from(val: Ptso4) -> u8 {
        Ptso4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso5 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso5 {
    #[inline(always)]
    fn from(val: u8) -> Ptso5 {
        Ptso5::from_bits(val)
    }
}
impl From<Ptso5> for u8 {
    #[inline(always)]
    fn from(val: Ptso5) -> u8 {
        Ptso5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso6 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso6 {
    #[inline(always)]
    fn from(val: u8) -> Ptso6 {
        Ptso6::from_bits(val)
    }
}
impl From<Ptso6> for u8 {
    #[inline(always)]
    fn from(val: Ptso6) -> u8 {
        Ptso6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso7 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso7 {
    #[inline(always)]
    fn from(val: u8) -> Ptso7 {
        Ptso7::from_bits(val)
    }
}
impl From<Ptso7> for u8 {
    #[inline(always)]
    fn from(val: Ptso7) -> u8 {
        Ptso7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso8 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso8 {
    #[inline(always)]
    fn from(val: u8) -> Ptso8 {
        Ptso8::from_bits(val)
    }
}
impl From<Ptso8> for u8 {
    #[inline(always)]
    fn from(val: Ptso8) -> u8 {
        Ptso8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso9 {
    #[doc = "No change"]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1"]
    PTSO1 = 0x01,
}
impl Ptso9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso9 {
    #[inline(always)]
    fn from(val: u8) -> Ptso9 {
        Ptso9::from_bits(val)
    }
}
impl From<Ptso9> for u8 {
    #[inline(always)]
    fn from(val: Ptso9) -> u8 {
        Ptso9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto0 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto0 {
    #[inline(always)]
    fn from(val: u8) -> Ptto0 {
        Ptto0::from_bits(val)
    }
}
impl From<Ptto0> for u8 {
    #[inline(always)]
    fn from(val: Ptto0) -> u8 {
        Ptto0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto1 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto1 {
    #[inline(always)]
    fn from(val: u8) -> Ptto1 {
        Ptto1::from_bits(val)
    }
}
impl From<Ptto1> for u8 {
    #[inline(always)]
    fn from(val: Ptto1) -> u8 {
        Ptto1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto10 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto10 {
    #[inline(always)]
    fn from(val: u8) -> Ptto10 {
        Ptto10::from_bits(val)
    }
}
impl From<Ptto10> for u8 {
    #[inline(always)]
    fn from(val: Ptto10) -> u8 {
        Ptto10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto11 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto11 {
    #[inline(always)]
    fn from(val: u8) -> Ptto11 {
        Ptto11::from_bits(val)
    }
}
impl From<Ptto11> for u8 {
    #[inline(always)]
    fn from(val: Ptto11) -> u8 {
        Ptto11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto12 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto12 {
    #[inline(always)]
    fn from(val: u8) -> Ptto12 {
        Ptto12::from_bits(val)
    }
}
impl From<Ptto12> for u8 {
    #[inline(always)]
    fn from(val: Ptto12) -> u8 {
        Ptto12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto13 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto13 {
    #[inline(always)]
    fn from(val: u8) -> Ptto13 {
        Ptto13::from_bits(val)
    }
}
impl From<Ptto13> for u8 {
    #[inline(always)]
    fn from(val: Ptto13) -> u8 {
        Ptto13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto14 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto14 {
    #[inline(always)]
    fn from(val: u8) -> Ptto14 {
        Ptto14::from_bits(val)
    }
}
impl From<Ptto14> for u8 {
    #[inline(always)]
    fn from(val: Ptto14) -> u8 {
        Ptto14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto15 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto15 {
    #[inline(always)]
    fn from(val: u8) -> Ptto15 {
        Ptto15::from_bits(val)
    }
}
impl From<Ptto15> for u8 {
    #[inline(always)]
    fn from(val: Ptto15) -> u8 {
        Ptto15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto16 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto16 {
    #[inline(always)]
    fn from(val: u8) -> Ptto16 {
        Ptto16::from_bits(val)
    }
}
impl From<Ptto16> for u8 {
    #[inline(always)]
    fn from(val: Ptto16) -> u8 {
        Ptto16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto17 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto17 {
    #[inline(always)]
    fn from(val: u8) -> Ptto17 {
        Ptto17::from_bits(val)
    }
}
impl From<Ptto17> for u8 {
    #[inline(always)]
    fn from(val: Ptto17) -> u8 {
        Ptto17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto18 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto18 {
    #[inline(always)]
    fn from(val: u8) -> Ptto18 {
        Ptto18::from_bits(val)
    }
}
impl From<Ptto18> for u8 {
    #[inline(always)]
    fn from(val: Ptto18) -> u8 {
        Ptto18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto19 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto19 {
    #[inline(always)]
    fn from(val: u8) -> Ptto19 {
        Ptto19::from_bits(val)
    }
}
impl From<Ptto19> for u8 {
    #[inline(always)]
    fn from(val: Ptto19) -> u8 {
        Ptto19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto2 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto2 {
    #[inline(always)]
    fn from(val: u8) -> Ptto2 {
        Ptto2::from_bits(val)
    }
}
impl From<Ptto2> for u8 {
    #[inline(always)]
    fn from(val: Ptto2) -> u8 {
        Ptto2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto20 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto20 {
    #[inline(always)]
    fn from(val: u8) -> Ptto20 {
        Ptto20::from_bits(val)
    }
}
impl From<Ptto20> for u8 {
    #[inline(always)]
    fn from(val: Ptto20) -> u8 {
        Ptto20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto21 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto21 {
    #[inline(always)]
    fn from(val: u8) -> Ptto21 {
        Ptto21::from_bits(val)
    }
}
impl From<Ptto21> for u8 {
    #[inline(always)]
    fn from(val: Ptto21) -> u8 {
        Ptto21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto22 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto22 {
    #[inline(always)]
    fn from(val: u8) -> Ptto22 {
        Ptto22::from_bits(val)
    }
}
impl From<Ptto22> for u8 {
    #[inline(always)]
    fn from(val: Ptto22) -> u8 {
        Ptto22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto23 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto23 {
    #[inline(always)]
    fn from(val: u8) -> Ptto23 {
        Ptto23::from_bits(val)
    }
}
impl From<Ptto23> for u8 {
    #[inline(always)]
    fn from(val: Ptto23) -> u8 {
        Ptto23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto24 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto24 {
    #[inline(always)]
    fn from(val: u8) -> Ptto24 {
        Ptto24::from_bits(val)
    }
}
impl From<Ptto24> for u8 {
    #[inline(always)]
    fn from(val: Ptto24) -> u8 {
        Ptto24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto25 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto25 {
    #[inline(always)]
    fn from(val: u8) -> Ptto25 {
        Ptto25::from_bits(val)
    }
}
impl From<Ptto25> for u8 {
    #[inline(always)]
    fn from(val: Ptto25) -> u8 {
        Ptto25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto26 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto26 {
    #[inline(always)]
    fn from(val: u8) -> Ptto26 {
        Ptto26::from_bits(val)
    }
}
impl From<Ptto26> for u8 {
    #[inline(always)]
    fn from(val: Ptto26) -> u8 {
        Ptto26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto27 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto27 {
    #[inline(always)]
    fn from(val: u8) -> Ptto27 {
        Ptto27::from_bits(val)
    }
}
impl From<Ptto27> for u8 {
    #[inline(always)]
    fn from(val: Ptto27) -> u8 {
        Ptto27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto28 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto28 {
    #[inline(always)]
    fn from(val: u8) -> Ptto28 {
        Ptto28::from_bits(val)
    }
}
impl From<Ptto28> for u8 {
    #[inline(always)]
    fn from(val: Ptto28) -> u8 {
        Ptto28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto29 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto29 {
    #[inline(always)]
    fn from(val: u8) -> Ptto29 {
        Ptto29::from_bits(val)
    }
}
impl From<Ptto29> for u8 {
    #[inline(always)]
    fn from(val: Ptto29) -> u8 {
        Ptto29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto3 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto3 {
    #[inline(always)]
    fn from(val: u8) -> Ptto3 {
        Ptto3::from_bits(val)
    }
}
impl From<Ptto3> for u8 {
    #[inline(always)]
    fn from(val: Ptto3) -> u8 {
        Ptto3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto30 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto30 {
    #[inline(always)]
    fn from(val: u8) -> Ptto30 {
        Ptto30::from_bits(val)
    }
}
impl From<Ptto30> for u8 {
    #[inline(always)]
    fn from(val: Ptto30) -> u8 {
        Ptto30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto31 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto31 {
    #[inline(always)]
    fn from(val: u8) -> Ptto31 {
        Ptto31::from_bits(val)
    }
}
impl From<Ptto31> for u8 {
    #[inline(always)]
    fn from(val: Ptto31) -> u8 {
        Ptto31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto4 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto4 {
    #[inline(always)]
    fn from(val: u8) -> Ptto4 {
        Ptto4::from_bits(val)
    }
}
impl From<Ptto4> for u8 {
    #[inline(always)]
    fn from(val: Ptto4) -> u8 {
        Ptto4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto5 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto5 {
    #[inline(always)]
    fn from(val: u8) -> Ptto5 {
        Ptto5::from_bits(val)
    }
}
impl From<Ptto5> for u8 {
    #[inline(always)]
    fn from(val: Ptto5) -> u8 {
        Ptto5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto6 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto6 {
    #[inline(always)]
    fn from(val: u8) -> Ptto6 {
        Ptto6::from_bits(val)
    }
}
impl From<Ptto6> for u8 {
    #[inline(always)]
    fn from(val: Ptto6) -> u8 {
        Ptto6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto7 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto7 {
    #[inline(always)]
    fn from(val: u8) -> Ptto7 {
        Ptto7::from_bits(val)
    }
}
impl From<Ptto7> for u8 {
    #[inline(always)]
    fn from(val: Ptto7) -> u8 {
        Ptto7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto8 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto8 {
    #[inline(always)]
    fn from(val: u8) -> Ptto8 {
        Ptto8::from_bits(val)
    }
}
impl From<Ptto8> for u8 {
    #[inline(always)]
    fn from(val: Ptto8) -> u8 {
        Ptto8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto9 {
    #[doc = "No change"]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state"]
    PTTO1 = 0x01,
}
impl Ptto9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto9 {
    #[inline(always)]
    fn from(val: u8) -> Ptto9 {
        Ptto9::from_bits(val)
    }
}
impl From<Ptto9> for u8 {
    #[inline(always)]
    fn from(val: Ptto9) -> u8 {
        Ptto9::to_bits(val)
    }
}
