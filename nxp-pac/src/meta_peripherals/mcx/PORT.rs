#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Port Control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port {
    ptr: *mut u8,
}
unsafe impl Send for Port {}
unsafe impl Sync for Port {}
impl Port {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Global Pin Control Low."]
    #[inline(always)]
    pub const fn gpclr(self) -> crate::pac::common::Reg<Gpclr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Global Pin Control High."]
    #[inline(always)]
    pub const fn gpchr(self) -> crate::pac::common::Reg<Gpchr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::pac::common::Reg<Config, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pin Control 0."]
    #[inline(always)]
    pub const fn pcr(self, n: usize) -> crate::pac::common::Reg<Pcr, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
}
#[doc = "Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Port Voltage Range."]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> Range {
        let val = (self.0 >> 0usize) & 0x01;
        Range::from_bits(val as u8)
    }
    #[doc = "Port Voltage Range."]
    #[inline(always)]
    pub const fn set_range(&mut self, val: Range) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Config {{ range: {:?} }}", self.range())
    }
}
#[doc = "Global Pin Control High."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpchr(pub u32);
impl Gpchr {
    #[doc = "Global Pin Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data."]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe(&self, n: usize) -> Gpwe {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe(&mut self, n: usize, val: Gpwe) {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Gpchr {
    #[inline(always)]
    fn default() -> Gpchr {
        Gpchr(0)
    }
}
impl core::fmt::Debug for Gpchr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpchr")
            .field("gpwd", &self.gpwd())
            .field("gpwe[0]", &self.gpwe(0usize))
            .field("gpwe[1]", &self.gpwe(1usize))
            .field("gpwe[2]", &self.gpwe(2usize))
            .field("gpwe[3]", &self.gpwe(3usize))
            .field("gpwe[4]", &self.gpwe(4usize))
            .field("gpwe[5]", &self.gpwe(5usize))
            .field("gpwe[6]", &self.gpwe(6usize))
            .field("gpwe[7]", &self.gpwe(7usize))
            .field("gpwe[8]", &self.gpwe(8usize))
            .field("gpwe[9]", &self.gpwe(9usize))
            .field("gpwe[10]", &self.gpwe(10usize))
            .field("gpwe[11]", &self.gpwe(11usize))
            .field("gpwe[12]", &self.gpwe(12usize))
            .field("gpwe[13]", &self.gpwe(13usize))
            .field("gpwe[14]", &self.gpwe(14usize))
            .field("gpwe[15]", &self.gpwe(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpchr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpchr {{ gpwd: {=u16:?}, gpwe[0]: {:?}, gpwe[1]: {:?}, gpwe[2]: {:?}, gpwe[3]: {:?}, gpwe[4]: {:?}, gpwe[5]: {:?}, gpwe[6]: {:?}, gpwe[7]: {:?}, gpwe[8]: {:?}, gpwe[9]: {:?}, gpwe[10]: {:?}, gpwe[11]: {:?}, gpwe[12]: {:?}, gpwe[13]: {:?}, gpwe[14]: {:?}, gpwe[15]: {:?} }}",
            self.gpwd(),
            self.gpwe(0usize),
            self.gpwe(1usize),
            self.gpwe(2usize),
            self.gpwe(3usize),
            self.gpwe(4usize),
            self.gpwe(5usize),
            self.gpwe(6usize),
            self.gpwe(7usize),
            self.gpwe(8usize),
            self.gpwe(9usize),
            self.gpwe(10usize),
            self.gpwe(11usize),
            self.gpwe(12usize),
            self.gpwe(13usize),
            self.gpwe(14usize),
            self.gpwe(15usize)
        )
    }
}
#[doc = "Global Pin Control Low."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr(pub u32);
impl Gpclr {
    #[doc = "Global Pin Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data."]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe(&self, n: usize) -> Gpwe {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe(&mut self, n: usize, val: Gpwe) {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Gpclr {
    #[inline(always)]
    fn default() -> Gpclr {
        Gpclr(0)
    }
}
impl core::fmt::Debug for Gpclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpclr")
            .field("gpwd", &self.gpwd())
            .field("gpwe[0]", &self.gpwe(0usize))
            .field("gpwe[1]", &self.gpwe(1usize))
            .field("gpwe[2]", &self.gpwe(2usize))
            .field("gpwe[3]", &self.gpwe(3usize))
            .field("gpwe[4]", &self.gpwe(4usize))
            .field("gpwe[5]", &self.gpwe(5usize))
            .field("gpwe[6]", &self.gpwe(6usize))
            .field("gpwe[7]", &self.gpwe(7usize))
            .field("gpwe[8]", &self.gpwe(8usize))
            .field("gpwe[9]", &self.gpwe(9usize))
            .field("gpwe[10]", &self.gpwe(10usize))
            .field("gpwe[11]", &self.gpwe(11usize))
            .field("gpwe[12]", &self.gpwe(12usize))
            .field("gpwe[13]", &self.gpwe(13usize))
            .field("gpwe[14]", &self.gpwe(14usize))
            .field("gpwe[15]", &self.gpwe(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpclr {{ gpwd: {=u16:?}, gpwe[0]: {:?}, gpwe[1]: {:?}, gpwe[2]: {:?}, gpwe[3]: {:?}, gpwe[4]: {:?}, gpwe[5]: {:?}, gpwe[6]: {:?}, gpwe[7]: {:?}, gpwe[8]: {:?}, gpwe[9]: {:?}, gpwe[10]: {:?}, gpwe[11]: {:?}, gpwe[12]: {:?}, gpwe[13]: {:?}, gpwe[14]: {:?}, gpwe[15]: {:?} }}",
            self.gpwd(),
            self.gpwe(0usize),
            self.gpwe(1usize),
            self.gpwe(2usize),
            self.gpwe(3usize),
            self.gpwe(4usize),
            self.gpwe(5usize),
            self.gpwe(6usize),
            self.gpwe(7usize),
            self.gpwe(8usize),
            self.gpwe(9usize),
            self.gpwe(10usize),
            self.gpwe(11usize),
            self.gpwe(12usize),
            self.gpwe(13usize),
            self.gpwe(14usize),
            self.gpwe(15usize)
        )
    }
}
#[doc = "Pin Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr {
    #[inline(always)]
    fn default() -> Pcr {
        Pcr(0)
    }
}
impl core::fmt::Debug for Pcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dse {
    #[doc = "Low."]
    Dse0 = 0x0,
    #[doc = "High."]
    Dse1 = 0x01,
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
    #[doc = "Basic implementation."]
    pub const Feature0: Self = Self(0x0);
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
            0x0 => f.write_str("Feature0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Feature0"),
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
    #[doc = "Not updated."]
    Gpwe0 = 0x0,
    #[doc = "Updated."]
    Gpwe1 = 0x01,
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
    #[doc = "Disables."]
    Ibe0 = 0x0,
    #[doc = "Enables."]
    Ibe1 = 0x01,
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
    #[doc = "Does not invert."]
    Inv0 = 0x0,
    #[doc = "Inverts."]
    Inv1 = 0x01,
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
    #[doc = "Does not lock."]
    Lk0 = 0x0,
    #[doc = "Locks."]
    Lk1 = 0x01,
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
    Mux0 = 0x0,
    Mux1 = 0x01,
    Mux2 = 0x02,
    Mux3 = 0x03,
    Mux4 = 0x04,
    Mux5 = 0x05,
    Mux6 = 0x06,
    Mux7 = 0x07,
    Mux8 = 0x08,
    Mux9 = 0x09,
    Mux10 = 0x0a,
    Mux11 = 0x0b,
    Mux12 = 0x0c,
    Mux13 = 0x0d,
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
    #[doc = "Disables."]
    Ode0 = 0x0,
    #[doc = "Enables."]
    Ode1 = 0x01,
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
    #[doc = "Disables."]
    Pe0 = 0x0,
    #[doc = "Enables."]
    Pe1 = 0x01,
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
    #[doc = "Enables internal pulldown resistor."]
    Ps0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    Ps1 = 0x01,
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
    #[doc = "1.71 V-3.6 V."]
    Range0 = 0x0,
    #[doc = "2.70 V-3.6 V."]
    Range1 = 0x01,
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
    #[doc = "Fast."]
    Sre0 = 0x0,
    #[doc = "Slow."]
    Sre1 = 0x01,
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
