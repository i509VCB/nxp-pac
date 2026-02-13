#[doc = "Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "HC Bandgap Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn hcbgen(&self) -> super::vals::Hcbgen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hcbgen::from_bits(val as u8)
    }
    #[doc = "HC Bandgap Enabled"]
    #[inline(always)]
    pub const fn set_hcbgen(&mut self, val: super::vals::Hcbgen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Low-Power Bandgap Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbgen(&self) -> super::vals::Lpbgen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lpbgen::from_bits(val as u8)
    }
    #[doc = "Low-Power Bandgap Enable"]
    #[inline(always)]
    pub const fn set_lpbgen(&mut self, val: super::vals::Lpbgen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Low-Power Bandgap Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_buf_en(&self) -> super::vals::LpbgBufEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::LpbgBufEn::from_bits(val as u8)
    }
    #[doc = "Low-Power Bandgap Buffer Enable"]
    #[inline(always)]
    pub const fn set_lpbg_buf_en(&mut self, val: super::vals::LpbgBufEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Chop Oscillator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chopen(&self) -> super::vals::Chopen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Chopen::from_bits(val as u8)
    }
    #[doc = "Chop Oscillator Enable"]
    #[inline(always)]
    pub const fn set_chopen(&mut self, val: super::vals::Chopen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Current Compensation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn icompen(&self) -> super::vals::Icompen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Icompen::from_bits(val as u8)
    }
    #[doc = "Current Compensation Enable"]
    #[inline(always)]
    pub const fn set_icompen(&mut self, val: super::vals::Icompen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Regulator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regen(&self) -> super::vals::Regen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Regen::from_bits(val as u8)
    }
    #[doc = "Regulator Enable"]
    #[inline(always)]
    pub const fn set_regen(&mut self, val: super::vals::Regen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "High-Power Level"]
    #[must_use]
    #[inline(always)]
    pub const fn hi_pwr_lv(&self) -> super::vals::HiPwrLv {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::HiPwrLv::from_bits(val as u8)
    }
    #[doc = "High-Power Level"]
    #[inline(always)]
    pub const fn set_hi_pwr_lv(&mut self, val: super::vals::HiPwrLv) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Internal Buffer21 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn buf21en(&self) -> super::vals::Buf21en {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Buf21en::from_bits(val as u8)
    }
    #[doc = "Internal Buffer21 Enable"]
    #[inline(always)]
    pub const fn set_buf21en(&mut self, val: super::vals::Buf21en) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Internal HC Voltage Reference Stable"]
    #[must_use]
    #[inline(always)]
    pub const fn vrefst(&self) -> super::vals::Vrefst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Vrefst::from_bits(val as u8)
    }
    #[doc = "Internal HC Voltage Reference Stable"]
    #[inline(always)]
    pub const fn set_vrefst(&mut self, val: super::vals::Vrefst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("hcbgen", &self.hcbgen())
            .field("lpbgen", &self.lpbgen())
            .field("lpbg_buf_en", &self.lpbg_buf_en())
            .field("chopen", &self.chopen())
            .field("icompen", &self.icompen())
            .field("regen", &self.regen())
            .field("hi_pwr_lv", &self.hi_pwr_lv())
            .field("buf21en", &self.buf21en())
            .field("vrefst", &self.vrefst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ hcbgen: {:?}, lpbgen: {:?}, lpbg_buf_en: {:?}, chopen: {:?}, icompen: {:?}, regen: {:?}, hi_pwr_lv: {:?}, buf21en: {:?}, vrefst: {:?} }}",
            self.hcbgen(),
            self.lpbgen(),
            self.lpbg_buf_en(),
            self.chopen(),
            self.icompen(),
            self.regen(),
            self.hi_pwr_lv(),
            self.buf21en(),
            self.vrefst()
        )
    }
}
#[doc = "User Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utrim(pub u32);
impl Utrim {
    #[doc = "VREF 2.1 V Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn trim2v1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "VREF 2.1 V Trim"]
    #[inline(always)]
    pub const fn set_trim2v1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "VREF Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn vreftrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "VREF Trim"]
    #[inline(always)]
    pub const fn set_vreftrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Utrim {
    #[inline(always)]
    fn default() -> Utrim {
        Utrim(0)
    }
}
impl core::fmt::Debug for Utrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Utrim")
            .field("trim2v1", &self.trim2v1())
            .field("vreftrim", &self.vreftrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Utrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Utrim {{ trim2v1: {=u8:?}, vreftrim: {=u8:?} }}",
            self.trim2v1(),
            self.vreftrim()
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
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
