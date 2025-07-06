#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf0 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf0 {
    #[inline(always)]
    fn from(val: u8) -> Edf0 {
        Edf0::from_bits(val)
    }
}
impl From<Edf0> for u8 {
    #[inline(always)]
    fn from(val: Edf0) -> u8 {
        Edf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf1 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf1 {
    #[inline(always)]
    fn from(val: u8) -> Edf1 {
        Edf1::from_bits(val)
    }
}
impl From<Edf1> for u8 {
    #[inline(always)]
    fn from(val: Edf1) -> u8 {
        Edf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf2 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf2 {
    #[inline(always)]
    fn from(val: u8) -> Edf2 {
        Edf2::from_bits(val)
    }
}
impl From<Edf2> for u8 {
    #[inline(always)]
    fn from(val: Edf2) -> u8 {
        Edf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf3 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf3 {
    #[inline(always)]
    fn from(val: u8) -> Edf3 {
        Edf3::from_bits(val)
    }
}
impl From<Edf3> for u8 {
    #[inline(always)]
    fn from(val: Edf3) -> u8 {
        Edf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf4 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf4 {
    #[inline(always)]
    fn from(val: u8) -> Edf4 {
        Edf4::from_bits(val)
    }
}
impl From<Edf4> for u8 {
    #[inline(always)]
    fn from(val: Edf4) -> u8 {
        Edf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf5 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf5 {
    #[inline(always)]
    fn from(val: u8) -> Edf5 {
        Edf5::from_bits(val)
    }
}
impl From<Edf5> for u8 {
    #[inline(always)]
    fn from(val: Edf5) -> u8 {
        Edf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf6 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf6 {
    #[inline(always)]
    fn from(val: u8) -> Edf6 {
        Edf6::from_bits(val)
    }
}
impl From<Edf6> for u8 {
    #[inline(always)]
    fn from(val: Edf6) -> u8 {
        Edf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf7 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf7 {
    #[inline(always)]
    fn from(val: u8) -> Edf7 {
        Edf7::from_bits(val)
    }
}
impl From<Edf7> for u8 {
    #[inline(always)]
    fn from(val: Edf7) -> u8 {
        Edf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf8 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf8 {
    #[inline(always)]
    fn from(val: u8) -> Edf8 {
        Edf8::from_bits(val)
    }
}
impl From<Edf8> for u8 {
    #[inline(always)]
    fn from(val: Edf8) -> u8 {
        Edf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf9 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf9 {
    #[inline(always)]
    fn from(val: u8) -> Edf9 {
        Edf9::from_bits(val)
    }
}
impl From<Edf9> for u8 {
    #[inline(always)]
    fn from(val: Edf9) -> u8 {
        Edf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edhc {
    #[doc = "Does not clear"]
    EDHC0 = 0x0,
    #[doc = "Clears"]
    EDHC1 = 0x01,
}
impl Edhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edhc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edhc {
    #[inline(always)]
    fn from(val: u8) -> Edhc {
        Edhc::from_bits(val)
    }
}
impl From<Edhc> for u8 {
    #[inline(always)]
    fn from(val: Edhc) -> u8 {
        Edhc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie0 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie0 {
    #[inline(always)]
    fn from(val: u8) -> Edie0 {
        Edie0::from_bits(val)
    }
}
impl From<Edie0> for u8 {
    #[inline(always)]
    fn from(val: Edie0) -> u8 {
        Edie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie1 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie1 {
    #[inline(always)]
    fn from(val: u8) -> Edie1 {
        Edie1::from_bits(val)
    }
}
impl From<Edie1> for u8 {
    #[inline(always)]
    fn from(val: Edie1) -> u8 {
        Edie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie2 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie2 {
    #[inline(always)]
    fn from(val: u8) -> Edie2 {
        Edie2::from_bits(val)
    }
}
impl From<Edie2> for u8 {
    #[inline(always)]
    fn from(val: Edie2) -> u8 {
        Edie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie3 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie3 {
    #[inline(always)]
    fn from(val: u8) -> Edie3 {
        Edie3::from_bits(val)
    }
}
impl From<Edie3> for u8 {
    #[inline(always)]
    fn from(val: Edie3) -> u8 {
        Edie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie4 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie4 {
    #[inline(always)]
    fn from(val: u8) -> Edie4 {
        Edie4::from_bits(val)
    }
}
impl From<Edie4> for u8 {
    #[inline(always)]
    fn from(val: Edie4) -> u8 {
        Edie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie5 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie5 {
    #[inline(always)]
    fn from(val: u8) -> Edie5 {
        Edie5::from_bits(val)
    }
}
impl From<Edie5> for u8 {
    #[inline(always)]
    fn from(val: Edie5) -> u8 {
        Edie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie6 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie6 {
    #[inline(always)]
    fn from(val: u8) -> Edie6 {
        Edie6::from_bits(val)
    }
}
impl From<Edie6> for u8 {
    #[inline(always)]
    fn from(val: Edie6) -> u8 {
        Edie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie7 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie7 {
    #[inline(always)]
    fn from(val: u8) -> Edie7 {
        Edie7::from_bits(val)
    }
}
impl From<Edie7> for u8 {
    #[inline(always)]
    fn from(val: Edie7) -> u8 {
        Edie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie8 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie8 {
    #[inline(always)]
    fn from(val: u8) -> Edie8 {
        Edie8::from_bits(val)
    }
}
impl From<Edie8> for u8 {
    #[inline(always)]
    fn from(val: Edie8) -> u8 {
        Edie8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie9 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie9 {
    #[inline(always)]
    fn from(val: u8) -> Edie9 {
        Edie9::from_bits(val)
    }
}
impl From<Edie9> for u8 {
    #[inline(always)]
    fn from(val: Edie9) -> u8 {
        Edie9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edlc {
    #[doc = "Does not clear"]
    EDLC0 = 0x0,
    #[doc = "Clears"]
    EDLC1 = 0x01,
}
impl Edlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edlc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edlc {
    #[inline(always)]
    fn from(val: u8) -> Edlc {
        Edlc::from_bits(val)
    }
}
impl From<Edlc> for u8 {
    #[inline(always)]
    fn from(val: Edlc) -> u8 {
        Edlc::to_bits(val)
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
pub enum Gpwe0 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe0 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe0 {
        Gpwe0::from_bits(val)
    }
}
impl From<Gpwe0> for u8 {
    #[inline(always)]
    fn from(val: Gpwe0) -> u8 {
        Gpwe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe1 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe1 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe1 {
        Gpwe1::from_bits(val)
    }
}
impl From<Gpwe1> for u8 {
    #[inline(always)]
    fn from(val: Gpwe1) -> u8 {
        Gpwe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe10 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe10 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe10 {
        Gpwe10::from_bits(val)
    }
}
impl From<Gpwe10> for u8 {
    #[inline(always)]
    fn from(val: Gpwe10) -> u8 {
        Gpwe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe11 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe11 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe11 {
        Gpwe11::from_bits(val)
    }
}
impl From<Gpwe11> for u8 {
    #[inline(always)]
    fn from(val: Gpwe11) -> u8 {
        Gpwe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe12 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe12 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe12 {
        Gpwe12::from_bits(val)
    }
}
impl From<Gpwe12> for u8 {
    #[inline(always)]
    fn from(val: Gpwe12) -> u8 {
        Gpwe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe13 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe13 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe13 {
        Gpwe13::from_bits(val)
    }
}
impl From<Gpwe13> for u8 {
    #[inline(always)]
    fn from(val: Gpwe13) -> u8 {
        Gpwe13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe14 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe14 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe14 {
        Gpwe14::from_bits(val)
    }
}
impl From<Gpwe14> for u8 {
    #[inline(always)]
    fn from(val: Gpwe14) -> u8 {
        Gpwe14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe15 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe15 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe15 {
        Gpwe15::from_bits(val)
    }
}
impl From<Gpwe15> for u8 {
    #[inline(always)]
    fn from(val: Gpwe15) -> u8 {
        Gpwe15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe16 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe16 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe16 {
        Gpwe16::from_bits(val)
    }
}
impl From<Gpwe16> for u8 {
    #[inline(always)]
    fn from(val: Gpwe16) -> u8 {
        Gpwe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe17 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe17 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe17 {
        Gpwe17::from_bits(val)
    }
}
impl From<Gpwe17> for u8 {
    #[inline(always)]
    fn from(val: Gpwe17) -> u8 {
        Gpwe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe18 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe18 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe18 {
        Gpwe18::from_bits(val)
    }
}
impl From<Gpwe18> for u8 {
    #[inline(always)]
    fn from(val: Gpwe18) -> u8 {
        Gpwe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe19 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe19 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe19 {
        Gpwe19::from_bits(val)
    }
}
impl From<Gpwe19> for u8 {
    #[inline(always)]
    fn from(val: Gpwe19) -> u8 {
        Gpwe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe2 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe2 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe2 {
        Gpwe2::from_bits(val)
    }
}
impl From<Gpwe2> for u8 {
    #[inline(always)]
    fn from(val: Gpwe2) -> u8 {
        Gpwe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe20 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe20 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe20 {
        Gpwe20::from_bits(val)
    }
}
impl From<Gpwe20> for u8 {
    #[inline(always)]
    fn from(val: Gpwe20) -> u8 {
        Gpwe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe21 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe21 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe21 {
        Gpwe21::from_bits(val)
    }
}
impl From<Gpwe21> for u8 {
    #[inline(always)]
    fn from(val: Gpwe21) -> u8 {
        Gpwe21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe22 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe22 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe22 {
        Gpwe22::from_bits(val)
    }
}
impl From<Gpwe22> for u8 {
    #[inline(always)]
    fn from(val: Gpwe22) -> u8 {
        Gpwe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe23 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe23 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe23 {
        Gpwe23::from_bits(val)
    }
}
impl From<Gpwe23> for u8 {
    #[inline(always)]
    fn from(val: Gpwe23) -> u8 {
        Gpwe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe24 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe24 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe24 {
        Gpwe24::from_bits(val)
    }
}
impl From<Gpwe24> for u8 {
    #[inline(always)]
    fn from(val: Gpwe24) -> u8 {
        Gpwe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe25 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe25 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe25 {
        Gpwe25::from_bits(val)
    }
}
impl From<Gpwe25> for u8 {
    #[inline(always)]
    fn from(val: Gpwe25) -> u8 {
        Gpwe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe26 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe26 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe26 {
        Gpwe26::from_bits(val)
    }
}
impl From<Gpwe26> for u8 {
    #[inline(always)]
    fn from(val: Gpwe26) -> u8 {
        Gpwe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe27 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe27 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe27 {
        Gpwe27::from_bits(val)
    }
}
impl From<Gpwe27> for u8 {
    #[inline(always)]
    fn from(val: Gpwe27) -> u8 {
        Gpwe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe28 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe28 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe28 {
        Gpwe28::from_bits(val)
    }
}
impl From<Gpwe28> for u8 {
    #[inline(always)]
    fn from(val: Gpwe28) -> u8 {
        Gpwe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe29 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe29 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe29 {
        Gpwe29::from_bits(val)
    }
}
impl From<Gpwe29> for u8 {
    #[inline(always)]
    fn from(val: Gpwe29) -> u8 {
        Gpwe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe3 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe3 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe3 {
        Gpwe3::from_bits(val)
    }
}
impl From<Gpwe3> for u8 {
    #[inline(always)]
    fn from(val: Gpwe3) -> u8 {
        Gpwe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe30 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe30 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe30 {
        Gpwe30::from_bits(val)
    }
}
impl From<Gpwe30> for u8 {
    #[inline(always)]
    fn from(val: Gpwe30) -> u8 {
        Gpwe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe31 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe31 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe31 {
        Gpwe31::from_bits(val)
    }
}
impl From<Gpwe31> for u8 {
    #[inline(always)]
    fn from(val: Gpwe31) -> u8 {
        Gpwe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe4 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe4 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe4 {
        Gpwe4::from_bits(val)
    }
}
impl From<Gpwe4> for u8 {
    #[inline(always)]
    fn from(val: Gpwe4) -> u8 {
        Gpwe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe5 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe5 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe5 {
        Gpwe5::from_bits(val)
    }
}
impl From<Gpwe5> for u8 {
    #[inline(always)]
    fn from(val: Gpwe5) -> u8 {
        Gpwe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe6 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe6 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe6 {
        Gpwe6::from_bits(val)
    }
}
impl From<Gpwe6> for u8 {
    #[inline(always)]
    fn from(val: Gpwe6) -> u8 {
        Gpwe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe7 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe7 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe7 {
        Gpwe7::from_bits(val)
    }
}
impl From<Gpwe7> for u8 {
    #[inline(always)]
    fn from(val: Gpwe7) -> u8 {
        Gpwe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe8 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe8 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe8 {
        Gpwe8::from_bits(val)
    }
}
impl From<Gpwe8> for u8 {
    #[inline(always)]
    fn from(val: Gpwe8) -> u8 {
        Gpwe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe9 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe9 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe9 {
        Gpwe9::from_bits(val)
    }
}
impl From<Gpwe9> for u8 {
    #[inline(always)]
    fn from(val: Gpwe9) -> u8 {
        Gpwe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr0Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ibe {
        Pcr0Ibe::from_bits(val)
    }
}
impl From<Pcr0Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ibe) -> u8 {
        Pcr0Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr0Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Inv {
        Pcr0Inv::from_bits(val)
    }
}
impl From<Pcr0Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Inv) -> u8 {
        Pcr0Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr0Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Lk {
        Pcr0Lk::from_bits(val)
    }
}
impl From<Pcr0Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Lk) -> u8 {
        Pcr0Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr0Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Mux {
        Pcr0Mux::from_bits(val)
    }
}
impl From<Pcr0Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Mux) -> u8 {
        Pcr0Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr0Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ode {
        Pcr0Ode::from_bits(val)
    }
}
impl From<Pcr0Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ode) -> u8 {
        Pcr0Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr0Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Pe {
        Pcr0Pe::from_bits(val)
    }
}
impl From<Pcr0Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Pe) -> u8 {
        Pcr0Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr0Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Pfe {
        Pcr0Pfe::from_bits(val)
    }
}
impl From<Pcr0Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Pfe) -> u8 {
        Pcr0Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ps {
        Pcr0Ps::from_bits(val)
    }
}
impl From<Pcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ps) -> u8 {
        Pcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr0Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Pv {
        Pcr0Pv::from_bits(val)
    }
}
impl From<Pcr0Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Pv) -> u8 {
        Pcr0Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr1Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ibe {
        Pcr1Ibe::from_bits(val)
    }
}
impl From<Pcr1Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ibe) -> u8 {
        Pcr1Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr1Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Inv {
        Pcr1Inv::from_bits(val)
    }
}
impl From<Pcr1Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Inv) -> u8 {
        Pcr1Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr1Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Lk {
        Pcr1Lk::from_bits(val)
    }
}
impl From<Pcr1Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Lk) -> u8 {
        Pcr1Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr1Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Mux {
        Pcr1Mux::from_bits(val)
    }
}
impl From<Pcr1Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Mux) -> u8 {
        Pcr1Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr1Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ode {
        Pcr1Ode::from_bits(val)
    }
}
impl From<Pcr1Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ode) -> u8 {
        Pcr1Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr1Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Pe {
        Pcr1Pe::from_bits(val)
    }
}
impl From<Pcr1Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Pe) -> u8 {
        Pcr1Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr1Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Pfe {
        Pcr1Pfe::from_bits(val)
    }
}
impl From<Pcr1Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Pfe) -> u8 {
        Pcr1Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr1Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ps {
        Pcr1Ps::from_bits(val)
    }
}
impl From<Pcr1Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ps) -> u8 {
        Pcr1Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr1Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Pv {
        Pcr1Pv::from_bits(val)
    }
}
impl From<Pcr1Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Pv) -> u8 {
        Pcr1Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr2Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ibe {
        Pcr2Ibe::from_bits(val)
    }
}
impl From<Pcr2Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ibe) -> u8 {
        Pcr2Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr2Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Inv {
        Pcr2Inv::from_bits(val)
    }
}
impl From<Pcr2Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Inv) -> u8 {
        Pcr2Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr2Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Lk {
        Pcr2Lk::from_bits(val)
    }
}
impl From<Pcr2Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Lk) -> u8 {
        Pcr2Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr2Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Mux {
        Pcr2Mux::from_bits(val)
    }
}
impl From<Pcr2Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Mux) -> u8 {
        Pcr2Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr2Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ode {
        Pcr2Ode::from_bits(val)
    }
}
impl From<Pcr2Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ode) -> u8 {
        Pcr2Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr2Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Pe {
        Pcr2Pe::from_bits(val)
    }
}
impl From<Pcr2Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Pe) -> u8 {
        Pcr2Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr2Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Pfe {
        Pcr2Pfe::from_bits(val)
    }
}
impl From<Pcr2Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Pfe) -> u8 {
        Pcr2Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr2Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ps {
        Pcr2Ps::from_bits(val)
    }
}
impl From<Pcr2Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ps) -> u8 {
        Pcr2Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr2Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Pv {
        Pcr2Pv::from_bits(val)
    }
}
impl From<Pcr2Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Pv) -> u8 {
        Pcr2Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr3Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ibe {
        Pcr3Ibe::from_bits(val)
    }
}
impl From<Pcr3Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ibe) -> u8 {
        Pcr3Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr3Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Inv {
        Pcr3Inv::from_bits(val)
    }
}
impl From<Pcr3Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Inv) -> u8 {
        Pcr3Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr3Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Lk {
        Pcr3Lk::from_bits(val)
    }
}
impl From<Pcr3Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Lk) -> u8 {
        Pcr3Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr3Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Mux {
        Pcr3Mux::from_bits(val)
    }
}
impl From<Pcr3Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Mux) -> u8 {
        Pcr3Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr3Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ode {
        Pcr3Ode::from_bits(val)
    }
}
impl From<Pcr3Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ode) -> u8 {
        Pcr3Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr3Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Pe {
        Pcr3Pe::from_bits(val)
    }
}
impl From<Pcr3Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Pe) -> u8 {
        Pcr3Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr3Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Pfe {
        Pcr3Pfe::from_bits(val)
    }
}
impl From<Pcr3Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Pfe) -> u8 {
        Pcr3Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr3Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ps {
        Pcr3Ps::from_bits(val)
    }
}
impl From<Pcr3Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ps) -> u8 {
        Pcr3Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr3Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Pv {
        Pcr3Pv::from_bits(val)
    }
}
impl From<Pcr3Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Pv) -> u8 {
        Pcr3Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr4Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ibe {
        Pcr4Ibe::from_bits(val)
    }
}
impl From<Pcr4Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ibe) -> u8 {
        Pcr4Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr4Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Inv {
        Pcr4Inv::from_bits(val)
    }
}
impl From<Pcr4Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Inv) -> u8 {
        Pcr4Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr4Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Lk {
        Pcr4Lk::from_bits(val)
    }
}
impl From<Pcr4Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Lk) -> u8 {
        Pcr4Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr4Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Mux {
        Pcr4Mux::from_bits(val)
    }
}
impl From<Pcr4Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Mux) -> u8 {
        Pcr4Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr4Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ode {
        Pcr4Ode::from_bits(val)
    }
}
impl From<Pcr4Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ode) -> u8 {
        Pcr4Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr4Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Pe {
        Pcr4Pe::from_bits(val)
    }
}
impl From<Pcr4Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Pe) -> u8 {
        Pcr4Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr4Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Pfe {
        Pcr4Pfe::from_bits(val)
    }
}
impl From<Pcr4Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Pfe) -> u8 {
        Pcr4Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr4Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ps {
        Pcr4Ps::from_bits(val)
    }
}
impl From<Pcr4Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ps) -> u8 {
        Pcr4Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr4Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Pv {
        Pcr4Pv::from_bits(val)
    }
}
impl From<Pcr4Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Pv) -> u8 {
        Pcr4Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr5Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ibe {
        Pcr5Ibe::from_bits(val)
    }
}
impl From<Pcr5Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ibe) -> u8 {
        Pcr5Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr5Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Inv {
        Pcr5Inv::from_bits(val)
    }
}
impl From<Pcr5Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Inv) -> u8 {
        Pcr5Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr5Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Lk {
        Pcr5Lk::from_bits(val)
    }
}
impl From<Pcr5Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Lk) -> u8 {
        Pcr5Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr5Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Mux {
        Pcr5Mux::from_bits(val)
    }
}
impl From<Pcr5Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Mux) -> u8 {
        Pcr5Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr5Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ode {
        Pcr5Ode::from_bits(val)
    }
}
impl From<Pcr5Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ode) -> u8 {
        Pcr5Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr5Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Pe {
        Pcr5Pe::from_bits(val)
    }
}
impl From<Pcr5Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Pe) -> u8 {
        Pcr5Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr5Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Pfe {
        Pcr5Pfe::from_bits(val)
    }
}
impl From<Pcr5Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Pfe) -> u8 {
        Pcr5Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr5Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ps {
        Pcr5Ps::from_bits(val)
    }
}
impl From<Pcr5Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ps) -> u8 {
        Pcr5Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr5Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Pv {
        Pcr5Pv::from_bits(val)
    }
}
impl From<Pcr5Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Pv) -> u8 {
        Pcr5Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr6Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ibe {
        Pcr6Ibe::from_bits(val)
    }
}
impl From<Pcr6Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ibe) -> u8 {
        Pcr6Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr6Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Inv {
        Pcr6Inv::from_bits(val)
    }
}
impl From<Pcr6Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Inv) -> u8 {
        Pcr6Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr6Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Lk {
        Pcr6Lk::from_bits(val)
    }
}
impl From<Pcr6Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Lk) -> u8 {
        Pcr6Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr6Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Mux {
        Pcr6Mux::from_bits(val)
    }
}
impl From<Pcr6Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Mux) -> u8 {
        Pcr6Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr6Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ode {
        Pcr6Ode::from_bits(val)
    }
}
impl From<Pcr6Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ode) -> u8 {
        Pcr6Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr6Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Pe {
        Pcr6Pe::from_bits(val)
    }
}
impl From<Pcr6Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Pe) -> u8 {
        Pcr6Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr6Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Pfe {
        Pcr6Pfe::from_bits(val)
    }
}
impl From<Pcr6Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Pfe) -> u8 {
        Pcr6Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr6Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ps {
        Pcr6Ps::from_bits(val)
    }
}
impl From<Pcr6Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ps) -> u8 {
        Pcr6Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr6Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Pv {
        Pcr6Pv::from_bits(val)
    }
}
impl From<Pcr6Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Pv) -> u8 {
        Pcr6Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr7Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ibe {
        Pcr7Ibe::from_bits(val)
    }
}
impl From<Pcr7Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ibe) -> u8 {
        Pcr7Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr7Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Inv {
        Pcr7Inv::from_bits(val)
    }
}
impl From<Pcr7Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Inv) -> u8 {
        Pcr7Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr7Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Lk {
        Pcr7Lk::from_bits(val)
    }
}
impl From<Pcr7Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Lk) -> u8 {
        Pcr7Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr7Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Mux {
        Pcr7Mux::from_bits(val)
    }
}
impl From<Pcr7Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Mux) -> u8 {
        Pcr7Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr7Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ode {
        Pcr7Ode::from_bits(val)
    }
}
impl From<Pcr7Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ode) -> u8 {
        Pcr7Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr7Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Pe {
        Pcr7Pe::from_bits(val)
    }
}
impl From<Pcr7Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Pe) -> u8 {
        Pcr7Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr7Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Pfe {
        Pcr7Pfe::from_bits(val)
    }
}
impl From<Pcr7Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Pfe) -> u8 {
        Pcr7Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr7Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ps {
        Pcr7Ps::from_bits(val)
    }
}
impl From<Pcr7Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ps) -> u8 {
        Pcr7Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr7Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Pv {
        Pcr7Pv::from_bits(val)
    }
}
impl From<Pcr7Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Pv) -> u8 {
        Pcr7Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr8Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ibe {
        Pcr8Ibe::from_bits(val)
    }
}
impl From<Pcr8Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ibe) -> u8 {
        Pcr8Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr8Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Inv {
        Pcr8Inv::from_bits(val)
    }
}
impl From<Pcr8Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Inv) -> u8 {
        Pcr8Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr8Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Lk {
        Pcr8Lk::from_bits(val)
    }
}
impl From<Pcr8Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Lk) -> u8 {
        Pcr8Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr8Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Mux {
        Pcr8Mux::from_bits(val)
    }
}
impl From<Pcr8Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Mux) -> u8 {
        Pcr8Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr8Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ode {
        Pcr8Ode::from_bits(val)
    }
}
impl From<Pcr8Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ode) -> u8 {
        Pcr8Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr8Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pe {
        Pcr8Pe::from_bits(val)
    }
}
impl From<Pcr8Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pe) -> u8 {
        Pcr8Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr8Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pfe {
        Pcr8Pfe::from_bits(val)
    }
}
impl From<Pcr8Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pfe) -> u8 {
        Pcr8Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr8Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ps {
        Pcr8Ps::from_bits(val)
    }
}
impl From<Pcr8Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ps) -> u8 {
        Pcr8Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr8Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pv {
        Pcr8Pv::from_bits(val)
    }
}
impl From<Pcr8Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pv) -> u8 {
        Pcr8Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr9Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ibe {
        Pcr9Ibe::from_bits(val)
    }
}
impl From<Pcr9Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ibe) -> u8 {
        Pcr9Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr9Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Inv {
        Pcr9Inv::from_bits(val)
    }
}
impl From<Pcr9Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Inv) -> u8 {
        Pcr9Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr9Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Lk {
        Pcr9Lk::from_bits(val)
    }
}
impl From<Pcr9Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Lk) -> u8 {
        Pcr9Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr9Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Mux {
        Pcr9Mux::from_bits(val)
    }
}
impl From<Pcr9Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Mux) -> u8 {
        Pcr9Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr9Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ode {
        Pcr9Ode::from_bits(val)
    }
}
impl From<Pcr9Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ode) -> u8 {
        Pcr9Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr9Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Pe {
        Pcr9Pe::from_bits(val)
    }
}
impl From<Pcr9Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Pe) -> u8 {
        Pcr9Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr9Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Pfe {
        Pcr9Pfe::from_bits(val)
    }
}
impl From<Pcr9Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Pfe) -> u8 {
        Pcr9Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr9Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ps {
        Pcr9Ps::from_bits(val)
    }
}
impl From<Pcr9Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ps) -> u8 {
        Pcr9Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr9Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Pv {
        Pcr9Pv::from_bits(val)
    }
}
impl From<Pcr9Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Pv) -> u8 {
        Pcr9Pv::to_bits(val)
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
