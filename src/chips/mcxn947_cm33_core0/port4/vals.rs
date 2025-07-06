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
pub enum Edf12 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf12 {
    #[inline(always)]
    fn from(val: u8) -> Edf12 {
        Edf12::from_bits(val)
    }
}
impl From<Edf12> for u8 {
    #[inline(always)]
    fn from(val: Edf12) -> u8 {
        Edf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf13 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf13 {
    #[inline(always)]
    fn from(val: u8) -> Edf13 {
        Edf13::from_bits(val)
    }
}
impl From<Edf13> for u8 {
    #[inline(always)]
    fn from(val: Edf13) -> u8 {
        Edf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf14 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf14 {
    #[inline(always)]
    fn from(val: u8) -> Edf14 {
        Edf14::from_bits(val)
    }
}
impl From<Edf14> for u8 {
    #[inline(always)]
    fn from(val: Edf14) -> u8 {
        Edf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf15 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf15 {
    #[inline(always)]
    fn from(val: u8) -> Edf15 {
        Edf15::from_bits(val)
    }
}
impl From<Edf15> for u8 {
    #[inline(always)]
    fn from(val: Edf15) -> u8 {
        Edf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf16 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf16 {
    #[inline(always)]
    fn from(val: u8) -> Edf16 {
        Edf16::from_bits(val)
    }
}
impl From<Edf16> for u8 {
    #[inline(always)]
    fn from(val: Edf16) -> u8 {
        Edf16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf17 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf17 {
    #[inline(always)]
    fn from(val: u8) -> Edf17 {
        Edf17::from_bits(val)
    }
}
impl From<Edf17> for u8 {
    #[inline(always)]
    fn from(val: Edf17) -> u8 {
        Edf17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf18 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf18 {
    #[inline(always)]
    fn from(val: u8) -> Edf18 {
        Edf18::from_bits(val)
    }
}
impl From<Edf18> for u8 {
    #[inline(always)]
    fn from(val: Edf18) -> u8 {
        Edf18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf19 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf19 {
    #[inline(always)]
    fn from(val: u8) -> Edf19 {
        Edf19::from_bits(val)
    }
}
impl From<Edf19> for u8 {
    #[inline(always)]
    fn from(val: Edf19) -> u8 {
        Edf19::to_bits(val)
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
pub enum Edf20 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf20 {
    #[inline(always)]
    fn from(val: u8) -> Edf20 {
        Edf20::from_bits(val)
    }
}
impl From<Edf20> for u8 {
    #[inline(always)]
    fn from(val: Edf20) -> u8 {
        Edf20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf21 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf21 {
    #[inline(always)]
    fn from(val: u8) -> Edf21 {
        Edf21::from_bits(val)
    }
}
impl From<Edf21> for u8 {
    #[inline(always)]
    fn from(val: Edf21) -> u8 {
        Edf21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf22 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf22 {
    #[inline(always)]
    fn from(val: u8) -> Edf22 {
        Edf22::from_bits(val)
    }
}
impl From<Edf22> for u8 {
    #[inline(always)]
    fn from(val: Edf22) -> u8 {
        Edf22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edf23 {
    #[doc = "No EFT event detected"]
    EDIE0 = 0x0,
    #[doc = "High or/and low EFT event detected"]
    EDIE1 = 0x01,
}
impl Edf23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edf23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edf23 {
    #[inline(always)]
    fn from(val: u8) -> Edf23 {
        Edf23::from_bits(val)
    }
}
impl From<Edf23> for u8 {
    #[inline(always)]
    fn from(val: Edf23) -> u8 {
        Edf23::to_bits(val)
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
pub enum Edie12 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie12 {
    #[inline(always)]
    fn from(val: u8) -> Edie12 {
        Edie12::from_bits(val)
    }
}
impl From<Edie12> for u8 {
    #[inline(always)]
    fn from(val: Edie12) -> u8 {
        Edie12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie13 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie13 {
    #[inline(always)]
    fn from(val: u8) -> Edie13 {
        Edie13::from_bits(val)
    }
}
impl From<Edie13> for u8 {
    #[inline(always)]
    fn from(val: Edie13) -> u8 {
        Edie13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie14 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie14 {
    #[inline(always)]
    fn from(val: u8) -> Edie14 {
        Edie14::from_bits(val)
    }
}
impl From<Edie14> for u8 {
    #[inline(always)]
    fn from(val: Edie14) -> u8 {
        Edie14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie15 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie15 {
    #[inline(always)]
    fn from(val: u8) -> Edie15 {
        Edie15::from_bits(val)
    }
}
impl From<Edie15> for u8 {
    #[inline(always)]
    fn from(val: Edie15) -> u8 {
        Edie15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie16 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie16 {
    #[inline(always)]
    fn from(val: u8) -> Edie16 {
        Edie16::from_bits(val)
    }
}
impl From<Edie16> for u8 {
    #[inline(always)]
    fn from(val: Edie16) -> u8 {
        Edie16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie17 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie17 {
    #[inline(always)]
    fn from(val: u8) -> Edie17 {
        Edie17::from_bits(val)
    }
}
impl From<Edie17> for u8 {
    #[inline(always)]
    fn from(val: Edie17) -> u8 {
        Edie17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie18 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie18 {
    #[inline(always)]
    fn from(val: u8) -> Edie18 {
        Edie18::from_bits(val)
    }
}
impl From<Edie18> for u8 {
    #[inline(always)]
    fn from(val: Edie18) -> u8 {
        Edie18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie19 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie19 {
    #[inline(always)]
    fn from(val: u8) -> Edie19 {
        Edie19::from_bits(val)
    }
}
impl From<Edie19> for u8 {
    #[inline(always)]
    fn from(val: Edie19) -> u8 {
        Edie19::to_bits(val)
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
pub enum Edie20 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie20 {
    #[inline(always)]
    fn from(val: u8) -> Edie20 {
        Edie20::from_bits(val)
    }
}
impl From<Edie20> for u8 {
    #[inline(always)]
    fn from(val: Edie20) -> u8 {
        Edie20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie21 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie21 {
    #[inline(always)]
    fn from(val: u8) -> Edie21 {
        Edie21::from_bits(val)
    }
}
impl From<Edie21> for u8 {
    #[inline(always)]
    fn from(val: Edie21) -> u8 {
        Edie21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie22 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie22 {
    #[inline(always)]
    fn from(val: u8) -> Edie22 {
        Edie22::from_bits(val)
    }
}
impl From<Edie22> for u8 {
    #[inline(always)]
    fn from(val: Edie22) -> u8 {
        Edie22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edie23 {
    #[doc = "Interrupt not generated upon detection of the EFT event"]
    EDIE0 = 0x0,
    #[doc = "Interrupt generated upon detection of the EFT event"]
    EDIE1 = 0x01,
}
impl Edie23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edie23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edie23 {
    #[inline(always)]
    fn from(val: u8) -> Edie23 {
        Edie23::from_bits(val)
    }
}
impl From<Edie23> for u8 {
    #[inline(always)]
    fn from(val: Edie23) -> u8 {
        Edie23::to_bits(val)
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
pub enum Pcr0Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr0Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Dse {
        Pcr0Dse::from_bits(val)
    }
}
impl From<Pcr0Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Dse) -> u8 {
        Pcr0Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr0Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr0Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr0Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Sre {
        Pcr0Sre::from_bits(val)
    }
}
impl From<Pcr0Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Sre) -> u8 {
        Pcr0Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr12Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Dse {
        Pcr12Dse::from_bits(val)
    }
}
impl From<Pcr12Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Dse) -> u8 {
        Pcr12Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr12Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ibe {
        Pcr12Ibe::from_bits(val)
    }
}
impl From<Pcr12Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ibe) -> u8 {
        Pcr12Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr12Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Inv {
        Pcr12Inv::from_bits(val)
    }
}
impl From<Pcr12Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Inv) -> u8 {
        Pcr12Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr12Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Lk {
        Pcr12Lk::from_bits(val)
    }
}
impl From<Pcr12Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Lk) -> u8 {
        Pcr12Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr12Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Mux {
        Pcr12Mux::from_bits(val)
    }
}
impl From<Pcr12Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Mux) -> u8 {
        Pcr12Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr12Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ode {
        Pcr12Ode::from_bits(val)
    }
}
impl From<Pcr12Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ode) -> u8 {
        Pcr12Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr12Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Pe {
        Pcr12Pe::from_bits(val)
    }
}
impl From<Pcr12Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Pe) -> u8 {
        Pcr12Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr12Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ps {
        Pcr12Ps::from_bits(val)
    }
}
impl From<Pcr12Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ps) -> u8 {
        Pcr12Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr12Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Sre {
        Pcr12Sre::from_bits(val)
    }
}
impl From<Pcr12Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Sre) -> u8 {
        Pcr12Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr13Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Dse {
        Pcr13Dse::from_bits(val)
    }
}
impl From<Pcr13Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Dse) -> u8 {
        Pcr13Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr13Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ibe {
        Pcr13Ibe::from_bits(val)
    }
}
impl From<Pcr13Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ibe) -> u8 {
        Pcr13Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr13Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Inv {
        Pcr13Inv::from_bits(val)
    }
}
impl From<Pcr13Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Inv) -> u8 {
        Pcr13Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr13Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Lk {
        Pcr13Lk::from_bits(val)
    }
}
impl From<Pcr13Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Lk) -> u8 {
        Pcr13Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr13Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Mux {
        Pcr13Mux::from_bits(val)
    }
}
impl From<Pcr13Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Mux) -> u8 {
        Pcr13Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr13Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ode {
        Pcr13Ode::from_bits(val)
    }
}
impl From<Pcr13Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ode) -> u8 {
        Pcr13Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr13Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Pe {
        Pcr13Pe::from_bits(val)
    }
}
impl From<Pcr13Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Pe) -> u8 {
        Pcr13Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr13Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ps {
        Pcr13Ps::from_bits(val)
    }
}
impl From<Pcr13Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ps) -> u8 {
        Pcr13Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr13Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Sre {
        Pcr13Sre::from_bits(val)
    }
}
impl From<Pcr13Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Sre) -> u8 {
        Pcr13Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr14Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Dse {
        Pcr14Dse::from_bits(val)
    }
}
impl From<Pcr14Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Dse) -> u8 {
        Pcr14Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr14Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ibe {
        Pcr14Ibe::from_bits(val)
    }
}
impl From<Pcr14Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ibe) -> u8 {
        Pcr14Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr14Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Inv {
        Pcr14Inv::from_bits(val)
    }
}
impl From<Pcr14Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Inv) -> u8 {
        Pcr14Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr14Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Lk {
        Pcr14Lk::from_bits(val)
    }
}
impl From<Pcr14Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Lk) -> u8 {
        Pcr14Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr14Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Mux {
        Pcr14Mux::from_bits(val)
    }
}
impl From<Pcr14Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Mux) -> u8 {
        Pcr14Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr14Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ode {
        Pcr14Ode::from_bits(val)
    }
}
impl From<Pcr14Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ode) -> u8 {
        Pcr14Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr14Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Pe {
        Pcr14Pe::from_bits(val)
    }
}
impl From<Pcr14Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Pe) -> u8 {
        Pcr14Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr14Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ps {
        Pcr14Ps::from_bits(val)
    }
}
impl From<Pcr14Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ps) -> u8 {
        Pcr14Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr14Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Sre {
        Pcr14Sre::from_bits(val)
    }
}
impl From<Pcr14Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Sre) -> u8 {
        Pcr14Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr15Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Dse {
        Pcr15Dse::from_bits(val)
    }
}
impl From<Pcr15Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Dse) -> u8 {
        Pcr15Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr15Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ibe {
        Pcr15Ibe::from_bits(val)
    }
}
impl From<Pcr15Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ibe) -> u8 {
        Pcr15Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr15Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Inv {
        Pcr15Inv::from_bits(val)
    }
}
impl From<Pcr15Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Inv) -> u8 {
        Pcr15Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr15Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Lk {
        Pcr15Lk::from_bits(val)
    }
}
impl From<Pcr15Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Lk) -> u8 {
        Pcr15Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr15Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Mux {
        Pcr15Mux::from_bits(val)
    }
}
impl From<Pcr15Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Mux) -> u8 {
        Pcr15Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr15Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ode {
        Pcr15Ode::from_bits(val)
    }
}
impl From<Pcr15Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ode) -> u8 {
        Pcr15Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr15Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Pe {
        Pcr15Pe::from_bits(val)
    }
}
impl From<Pcr15Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Pe) -> u8 {
        Pcr15Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr15Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ps {
        Pcr15Ps::from_bits(val)
    }
}
impl From<Pcr15Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ps) -> u8 {
        Pcr15Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr15Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Sre {
        Pcr15Sre::from_bits(val)
    }
}
impl From<Pcr15Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Sre) -> u8 {
        Pcr15Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr16Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Dse {
        Pcr16Dse::from_bits(val)
    }
}
impl From<Pcr16Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Dse) -> u8 {
        Pcr16Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr16Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ibe {
        Pcr16Ibe::from_bits(val)
    }
}
impl From<Pcr16Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ibe) -> u8 {
        Pcr16Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr16Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Inv {
        Pcr16Inv::from_bits(val)
    }
}
impl From<Pcr16Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Inv) -> u8 {
        Pcr16Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr16Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Lk {
        Pcr16Lk::from_bits(val)
    }
}
impl From<Pcr16Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Lk) -> u8 {
        Pcr16Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr16Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Mux {
        Pcr16Mux::from_bits(val)
    }
}
impl From<Pcr16Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Mux) -> u8 {
        Pcr16Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr16Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ode {
        Pcr16Ode::from_bits(val)
    }
}
impl From<Pcr16Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ode) -> u8 {
        Pcr16Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr16Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Pe {
        Pcr16Pe::from_bits(val)
    }
}
impl From<Pcr16Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Pe) -> u8 {
        Pcr16Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr16Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ps {
        Pcr16Ps::from_bits(val)
    }
}
impl From<Pcr16Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ps) -> u8 {
        Pcr16Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr16Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Sre {
        Pcr16Sre::from_bits(val)
    }
}
impl From<Pcr16Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Sre) -> u8 {
        Pcr16Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr17Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Dse {
        Pcr17Dse::from_bits(val)
    }
}
impl From<Pcr17Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Dse) -> u8 {
        Pcr17Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr17Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ibe {
        Pcr17Ibe::from_bits(val)
    }
}
impl From<Pcr17Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ibe) -> u8 {
        Pcr17Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr17Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Inv {
        Pcr17Inv::from_bits(val)
    }
}
impl From<Pcr17Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Inv) -> u8 {
        Pcr17Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr17Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Lk {
        Pcr17Lk::from_bits(val)
    }
}
impl From<Pcr17Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Lk) -> u8 {
        Pcr17Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr17Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Mux {
        Pcr17Mux::from_bits(val)
    }
}
impl From<Pcr17Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Mux) -> u8 {
        Pcr17Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr17Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ode {
        Pcr17Ode::from_bits(val)
    }
}
impl From<Pcr17Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ode) -> u8 {
        Pcr17Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr17Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Pe {
        Pcr17Pe::from_bits(val)
    }
}
impl From<Pcr17Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Pe) -> u8 {
        Pcr17Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr17Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ps {
        Pcr17Ps::from_bits(val)
    }
}
impl From<Pcr17Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ps) -> u8 {
        Pcr17Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr17Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Sre {
        Pcr17Sre::from_bits(val)
    }
}
impl From<Pcr17Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Sre) -> u8 {
        Pcr17Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr18Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Dse {
        Pcr18Dse::from_bits(val)
    }
}
impl From<Pcr18Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Dse) -> u8 {
        Pcr18Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr18Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ibe {
        Pcr18Ibe::from_bits(val)
    }
}
impl From<Pcr18Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ibe) -> u8 {
        Pcr18Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr18Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Inv {
        Pcr18Inv::from_bits(val)
    }
}
impl From<Pcr18Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Inv) -> u8 {
        Pcr18Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr18Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Lk {
        Pcr18Lk::from_bits(val)
    }
}
impl From<Pcr18Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Lk) -> u8 {
        Pcr18Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr18Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Mux {
        Pcr18Mux::from_bits(val)
    }
}
impl From<Pcr18Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Mux) -> u8 {
        Pcr18Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr18Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ode {
        Pcr18Ode::from_bits(val)
    }
}
impl From<Pcr18Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ode) -> u8 {
        Pcr18Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr18Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Pe {
        Pcr18Pe::from_bits(val)
    }
}
impl From<Pcr18Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Pe) -> u8 {
        Pcr18Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr18Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ps {
        Pcr18Ps::from_bits(val)
    }
}
impl From<Pcr18Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ps) -> u8 {
        Pcr18Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr18Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Sre {
        Pcr18Sre::from_bits(val)
    }
}
impl From<Pcr18Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Sre) -> u8 {
        Pcr18Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr19Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Dse {
        Pcr19Dse::from_bits(val)
    }
}
impl From<Pcr19Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Dse) -> u8 {
        Pcr19Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr19Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ibe {
        Pcr19Ibe::from_bits(val)
    }
}
impl From<Pcr19Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ibe) -> u8 {
        Pcr19Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr19Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Inv {
        Pcr19Inv::from_bits(val)
    }
}
impl From<Pcr19Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Inv) -> u8 {
        Pcr19Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr19Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Lk {
        Pcr19Lk::from_bits(val)
    }
}
impl From<Pcr19Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Lk) -> u8 {
        Pcr19Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr19Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Mux {
        Pcr19Mux::from_bits(val)
    }
}
impl From<Pcr19Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Mux) -> u8 {
        Pcr19Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr19Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ode {
        Pcr19Ode::from_bits(val)
    }
}
impl From<Pcr19Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ode) -> u8 {
        Pcr19Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr19Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Pe {
        Pcr19Pe::from_bits(val)
    }
}
impl From<Pcr19Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Pe) -> u8 {
        Pcr19Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr19Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ps {
        Pcr19Ps::from_bits(val)
    }
}
impl From<Pcr19Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ps) -> u8 {
        Pcr19Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr19Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Sre {
        Pcr19Sre::from_bits(val)
    }
}
impl From<Pcr19Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Sre) -> u8 {
        Pcr19Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr1Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Dse {
        Pcr1Dse::from_bits(val)
    }
}
impl From<Pcr1Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Dse) -> u8 {
        Pcr1Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr1Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr1Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr1Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Sre {
        Pcr1Sre::from_bits(val)
    }
}
impl From<Pcr1Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Sre) -> u8 {
        Pcr1Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr20Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Dse {
        Pcr20Dse::from_bits(val)
    }
}
impl From<Pcr20Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Dse) -> u8 {
        Pcr20Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr20Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Ibe {
        Pcr20Ibe::from_bits(val)
    }
}
impl From<Pcr20Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Ibe) -> u8 {
        Pcr20Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr20Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Inv {
        Pcr20Inv::from_bits(val)
    }
}
impl From<Pcr20Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Inv) -> u8 {
        Pcr20Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr20Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Lk {
        Pcr20Lk::from_bits(val)
    }
}
impl From<Pcr20Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Lk) -> u8 {
        Pcr20Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr20Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Mux {
        Pcr20Mux::from_bits(val)
    }
}
impl From<Pcr20Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Mux) -> u8 {
        Pcr20Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr20Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Ode {
        Pcr20Ode::from_bits(val)
    }
}
impl From<Pcr20Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Ode) -> u8 {
        Pcr20Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr20Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Pe {
        Pcr20Pe::from_bits(val)
    }
}
impl From<Pcr20Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Pe) -> u8 {
        Pcr20Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr20Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Ps {
        Pcr20Ps::from_bits(val)
    }
}
impl From<Pcr20Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Ps) -> u8 {
        Pcr20Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr20Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Sre {
        Pcr20Sre::from_bits(val)
    }
}
impl From<Pcr20Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Sre) -> u8 {
        Pcr20Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr21Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Dse {
        Pcr21Dse::from_bits(val)
    }
}
impl From<Pcr21Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Dse) -> u8 {
        Pcr21Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr21Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Ibe {
        Pcr21Ibe::from_bits(val)
    }
}
impl From<Pcr21Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Ibe) -> u8 {
        Pcr21Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr21Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Inv {
        Pcr21Inv::from_bits(val)
    }
}
impl From<Pcr21Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Inv) -> u8 {
        Pcr21Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr21Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Lk {
        Pcr21Lk::from_bits(val)
    }
}
impl From<Pcr21Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Lk) -> u8 {
        Pcr21Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr21Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Mux {
        Pcr21Mux::from_bits(val)
    }
}
impl From<Pcr21Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Mux) -> u8 {
        Pcr21Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr21Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Ode {
        Pcr21Ode::from_bits(val)
    }
}
impl From<Pcr21Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Ode) -> u8 {
        Pcr21Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr21Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Pe {
        Pcr21Pe::from_bits(val)
    }
}
impl From<Pcr21Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Pe) -> u8 {
        Pcr21Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr21Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Ps {
        Pcr21Ps::from_bits(val)
    }
}
impl From<Pcr21Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Ps) -> u8 {
        Pcr21Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr21Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Sre {
        Pcr21Sre::from_bits(val)
    }
}
impl From<Pcr21Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Sre) -> u8 {
        Pcr21Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr22Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Dse {
        Pcr22Dse::from_bits(val)
    }
}
impl From<Pcr22Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Dse) -> u8 {
        Pcr22Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr22Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Ibe {
        Pcr22Ibe::from_bits(val)
    }
}
impl From<Pcr22Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Ibe) -> u8 {
        Pcr22Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr22Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Inv {
        Pcr22Inv::from_bits(val)
    }
}
impl From<Pcr22Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Inv) -> u8 {
        Pcr22Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr22Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Lk {
        Pcr22Lk::from_bits(val)
    }
}
impl From<Pcr22Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Lk) -> u8 {
        Pcr22Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr22Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Mux {
        Pcr22Mux::from_bits(val)
    }
}
impl From<Pcr22Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Mux) -> u8 {
        Pcr22Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr22Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Ode {
        Pcr22Ode::from_bits(val)
    }
}
impl From<Pcr22Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Ode) -> u8 {
        Pcr22Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr22Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Pe {
        Pcr22Pe::from_bits(val)
    }
}
impl From<Pcr22Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Pe) -> u8 {
        Pcr22Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr22Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Ps {
        Pcr22Ps::from_bits(val)
    }
}
impl From<Pcr22Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Ps) -> u8 {
        Pcr22Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr22Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Sre {
        Pcr22Sre::from_bits(val)
    }
}
impl From<Pcr22Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Sre) -> u8 {
        Pcr22Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr23Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Dse {
        Pcr23Dse::from_bits(val)
    }
}
impl From<Pcr23Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Dse) -> u8 {
        Pcr23Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr23Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Ibe {
        Pcr23Ibe::from_bits(val)
    }
}
impl From<Pcr23Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Ibe) -> u8 {
        Pcr23Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr23Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Inv {
        Pcr23Inv::from_bits(val)
    }
}
impl From<Pcr23Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Inv) -> u8 {
        Pcr23Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr23Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Lk {
        Pcr23Lk::from_bits(val)
    }
}
impl From<Pcr23Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Lk) -> u8 {
        Pcr23Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr23Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Mux {
        Pcr23Mux::from_bits(val)
    }
}
impl From<Pcr23Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Mux) -> u8 {
        Pcr23Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr23Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Ode {
        Pcr23Ode::from_bits(val)
    }
}
impl From<Pcr23Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Ode) -> u8 {
        Pcr23Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr23Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Pe {
        Pcr23Pe::from_bits(val)
    }
}
impl From<Pcr23Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Pe) -> u8 {
        Pcr23Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr23Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Ps {
        Pcr23Ps::from_bits(val)
    }
}
impl From<Pcr23Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Ps) -> u8 {
        Pcr23Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr23Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Sre {
        Pcr23Sre::from_bits(val)
    }
}
impl From<Pcr23Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Sre) -> u8 {
        Pcr23Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr2Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Dse {
        Pcr2Dse::from_bits(val)
    }
}
impl From<Pcr2Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Dse) -> u8 {
        Pcr2Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr2Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr2Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr2Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Sre {
        Pcr2Sre::from_bits(val)
    }
}
impl From<Pcr2Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Sre) -> u8 {
        Pcr2Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr3Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Dse {
        Pcr3Dse::from_bits(val)
    }
}
impl From<Pcr3Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Dse) -> u8 {
        Pcr3Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr3Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr3Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr3Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Sre {
        Pcr3Sre::from_bits(val)
    }
}
impl From<Pcr3Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Sre) -> u8 {
        Pcr3Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr4Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Dse {
        Pcr4Dse::from_bits(val)
    }
}
impl From<Pcr4Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Dse) -> u8 {
        Pcr4Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr4Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr4Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr4Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Sre {
        Pcr4Sre::from_bits(val)
    }
}
impl From<Pcr4Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Sre) -> u8 {
        Pcr4Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr5Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Dse {
        Pcr5Dse::from_bits(val)
    }
}
impl From<Pcr5Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Dse) -> u8 {
        Pcr5Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr5Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr5Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr5Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Sre {
        Pcr5Sre::from_bits(val)
    }
}
impl From<Pcr5Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Sre) -> u8 {
        Pcr5Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr6Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Dse {
        Pcr6Dse::from_bits(val)
    }
}
impl From<Pcr6Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Dse) -> u8 {
        Pcr6Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr6Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr6Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr6Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Sre {
        Pcr6Sre::from_bits(val)
    }
}
impl From<Pcr6Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Sre) -> u8 {
        Pcr6Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr7Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Dse {
        Pcr7Dse::from_bits(val)
    }
}
impl From<Pcr7Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Dse) -> u8 {
        Pcr7Dse::to_bits(val)
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
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr7Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Pcr7Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr7Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Sre {
        Pcr7Sre::from_bits(val)
    }
}
impl From<Pcr7Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Sre) -> u8 {
        Pcr7Sre::to_bits(val)
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
