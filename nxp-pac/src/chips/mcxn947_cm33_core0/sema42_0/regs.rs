#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate0(pub u8);
impl Gate0 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate0Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate0Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate0Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate0 {
    #[inline(always)]
    fn default() -> Gate0 {
        Gate0(0)
    }
}
impl core::fmt::Debug for Gate0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate0")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate0 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate1(pub u8);
impl Gate1 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate1Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate1Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate1Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate1 {
    #[inline(always)]
    fn default() -> Gate1 {
        Gate1(0)
    }
}
impl core::fmt::Debug for Gate1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate1")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate1 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate10(pub u8);
impl Gate10 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate10Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate10Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate10Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate10 {
    #[inline(always)]
    fn default() -> Gate10 {
        Gate10(0)
    }
}
impl core::fmt::Debug for Gate10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate10")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate10 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate11(pub u8);
impl Gate11 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate11Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate11Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate11Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate11 {
    #[inline(always)]
    fn default() -> Gate11 {
        Gate11(0)
    }
}
impl core::fmt::Debug for Gate11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate11")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate11 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate12(pub u8);
impl Gate12 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate12Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate12Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate12Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate12 {
    #[inline(always)]
    fn default() -> Gate12 {
        Gate12(0)
    }
}
impl core::fmt::Debug for Gate12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate12")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate12 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate13(pub u8);
impl Gate13 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate13Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate13Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate13Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate13 {
    #[inline(always)]
    fn default() -> Gate13 {
        Gate13(0)
    }
}
impl core::fmt::Debug for Gate13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate13")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate13 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate14(pub u8);
impl Gate14 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate14Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate14Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate14Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate14 {
    #[inline(always)]
    fn default() -> Gate14 {
        Gate14(0)
    }
}
impl core::fmt::Debug for Gate14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate14")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate14 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate15(pub u8);
impl Gate15 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate15Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate15Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate15Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate15 {
    #[inline(always)]
    fn default() -> Gate15 {
        Gate15(0)
    }
}
impl core::fmt::Debug for Gate15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate15")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate15 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate2(pub u8);
impl Gate2 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate2Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate2Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate2Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate2 {
    #[inline(always)]
    fn default() -> Gate2 {
        Gate2(0)
    }
}
impl core::fmt::Debug for Gate2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate2")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate2 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate3(pub u8);
impl Gate3 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate3Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate3Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate3Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate3 {
    #[inline(always)]
    fn default() -> Gate3 {
        Gate3(0)
    }
}
impl core::fmt::Debug for Gate3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate3")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate3 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate4(pub u8);
impl Gate4 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate4Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate4Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate4Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate4 {
    #[inline(always)]
    fn default() -> Gate4 {
        Gate4(0)
    }
}
impl core::fmt::Debug for Gate4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate4")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate4 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate5(pub u8);
impl Gate5 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate5Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate5Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate5Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate5 {
    #[inline(always)]
    fn default() -> Gate5 {
        Gate5(0)
    }
}
impl core::fmt::Debug for Gate5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate5")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate5 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate6(pub u8);
impl Gate6 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate6Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate6Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate6Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate6 {
    #[inline(always)]
    fn default() -> Gate6 {
        Gate6(0)
    }
}
impl core::fmt::Debug for Gate6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate6")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate6 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate7(pub u8);
impl Gate7 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate7Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate7Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate7Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate7 {
    #[inline(always)]
    fn default() -> Gate7 {
        Gate7(0)
    }
}
impl core::fmt::Debug for Gate7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate7")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate7 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate8(pub u8);
impl Gate8 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate8Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate8Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate8Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate8 {
    #[inline(always)]
    fn default() -> Gate8 {
        Gate8(0)
    }
}
impl core::fmt::Debug for Gate8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate8")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate8 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Gate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate9(pub u8);
impl Gate9 {
    #[doc = "Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gate9Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gate9Gtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gate9Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate9 {
    #[inline(always)]
    fn default() -> Gate9 {
        Gate9(0)
    }
}
impl core::fmt::Debug for Gate9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate9")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate9 {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Reset Gate Read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtR(pub u16);
impl RstgtR {
    #[doc = "Reset Gate Number"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Number"]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Reset Gate Domain"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Reset Gate Domain"]
    #[inline(always)]
    pub const fn set_rstgms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Reset Gate Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgsm(&self) -> super::vals::Rstgsm {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Rstgsm::from_bits(val as u8)
    }
    #[doc = "Reset Gate Finite State Machine"]
    #[inline(always)]
    pub const fn set_rstgsm(&mut self, val: super::vals::Rstgsm) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
}
impl Default for RstgtR {
    #[inline(always)]
    fn default() -> RstgtR {
        RstgtR(0)
    }
}
impl core::fmt::Debug for RstgtR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtR")
            .field("rstgtn", &self.rstgtn())
            .field("rstgms", &self.rstgms())
            .field("rstgsm", &self.rstgsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RstgtR {{ rstgtn: {=u8:?}, rstgms: {=u8:?}, rstgsm: {:?} }}",
            self.rstgtn(),
            self.rstgms(),
            self.rstgsm()
        )
    }
}
#[doc = "Reset Gate Write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtW(pub u16);
impl RstgtW {
    #[doc = "Reset Gate Number"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Number"]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Reset Gate Data Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgdp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Data Pattern"]
    #[inline(always)]
    pub const fn set_rstgdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for RstgtW {
    #[inline(always)]
    fn default() -> RstgtW {
        RstgtW(0)
    }
}
impl core::fmt::Debug for RstgtW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtW")
            .field("rstgtn", &self.rstgtn())
            .field("rstgdp", &self.rstgdp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RstgtW {{ rstgtn: {=u8:?}, rstgdp: {=u8:?} }}",
            self.rstgtn(),
            self.rstgdp()
        )
    }
}
