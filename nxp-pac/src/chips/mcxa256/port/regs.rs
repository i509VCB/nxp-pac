#[doc = "Calibration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib0(pub u32);
impl Calib0 {
    #[doc = "Calibration of NMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver"]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib0 {
    #[inline(always)]
    fn default() -> Calib0 {
        Calib0(0)
    }
}
impl core::fmt::Debug for Calib0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib0")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib0 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Calibration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib1(pub u32);
impl Calib1 {
    #[doc = "Calibration of NMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver"]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib1 {
    #[inline(always)]
    fn default() -> Calib1 {
        Calib1(0)
    }
}
impl core::fmt::Debug for Calib1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib1")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib1 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Port Voltage Range"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Range::from_bits(val as u8)
    }
    #[doc = "Port Voltage Range"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: super::vals::Range) {
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
#[doc = "Global Pin Control High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpchr(pub u32);
impl Gpchr {
    #[doc = "Global Pin Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data"]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe(&self, n: usize) -> super::vals::Gpwe {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe(&mut self, n: usize, val: super::vals::Gpwe) {
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
#[doc = "Global Pin Control Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr(pub u32);
impl Gpclr {
    #[doc = "Global Pin Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data"]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe(&self, n: usize) -> super::vals::Gpwe {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::Gpwe::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe(&mut self, n: usize, val: super::vals::Gpwe) {
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
#[doc = "Pin Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Lk) {
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
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
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
