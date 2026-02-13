#[doc = "OPAMP Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpampCtr(pub u32);
impl OpampCtr {
    #[doc = "OPAMP Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mode Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Mode Selection"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bias Current Trim Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn biasc(&self) -> super::vals::Biasc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Biasc::from_bits(val as u8)
    }
    #[doc = "Bias Current Trim Selection"]
    #[inline(always)]
    pub const fn set_biasc(&mut self, val: super::vals::Biasc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Provide OPAMP rail to rail voltage selection"]
    #[must_use]
    #[inline(always)]
    pub const fn intref(&self) -> super::vals::Intref {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Intref::from_bits(val as u8)
    }
    #[doc = "Provide OPAMP rail to rail voltage selection"]
    #[inline(always)]
    pub const fn set_intref(&mut self, val: super::vals::Intref) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn trigmd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Mode"]
    #[inline(always)]
    pub const fn set_trigmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Positive Input Channel Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inpsel(&self) -> super::vals::Inpsel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Inpsel::from_bits(val as u8)
    }
    #[doc = "Positive Input Channel Selection"]
    #[inline(always)]
    pub const fn set_inpsel(&mut self, val: super::vals::Inpsel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Positive Input Connection Status"]
    #[must_use]
    #[inline(always)]
    pub const fn inpf(&self) -> super::vals::Inpf {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Inpf::from_bits(val as u8)
    }
    #[doc = "Positive Input Connection Status"]
    #[inline(always)]
    pub const fn set_inpf(&mut self, val: super::vals::Inpf) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Reference Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn bufen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Buffer"]
    #[inline(always)]
    pub const fn set_bufen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Positive Reference Voltage Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pref(&self) -> super::vals::Pref {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Pref::from_bits(val as u8)
    }
    #[doc = "Positive Reference Voltage Selection"]
    #[inline(always)]
    pub const fn set_pref(&mut self, val: super::vals::Pref) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "Measure Switch 1"]
    #[must_use]
    #[inline(always)]
    pub const fn adcsw1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Measure Switch 1"]
    #[inline(always)]
    pub const fn set_adcsw1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Measure Switch 2"]
    #[must_use]
    #[inline(always)]
    pub const fn adcsw2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Measure Switch 2"]
    #[inline(always)]
    pub const fn set_adcsw2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Output Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn outsw(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Output Switch"]
    #[inline(always)]
    pub const fn set_outsw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Positive PGA Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pgain(&self) -> super::vals::Pgain {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Pgain::from_bits(val as u8)
    }
    #[doc = "Positive PGA Selection"]
    #[inline(always)]
    pub const fn set_pgain(&mut self, val: super::vals::Pgain) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Negative PGA Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ngain(&self) -> super::vals::Ngain {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Ngain::from_bits(val as u8)
    }
    #[doc = "Negative PGA Selection"]
    #[inline(always)]
    pub const fn set_ngain(&mut self, val: super::vals::Ngain) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for OpampCtr {
    #[inline(always)]
    fn default() -> OpampCtr {
        OpampCtr(0)
    }
}
impl core::fmt::Debug for OpampCtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OpampCtr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("biasc", &self.biasc())
            .field("intref", &self.intref())
            .field("trigmd", &self.trigmd())
            .field("inpsel", &self.inpsel())
            .field("inpf", &self.inpf())
            .field("bufen", &self.bufen())
            .field("pref", &self.pref())
            .field("adcsw1", &self.adcsw1())
            .field("adcsw2", &self.adcsw2())
            .field("outsw", &self.outsw())
            .field("pgain", &self.pgain())
            .field("ngain", &self.ngain())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OpampCtr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OpampCtr {{ en: {=bool:?}, mode: {:?}, biasc: {:?}, intref: {:?}, trigmd: {=bool:?}, inpsel: {:?}, inpf: {:?}, bufen: {=bool:?}, pref: {:?}, adcsw1: {=bool:?}, adcsw2: {=bool:?}, outsw: {=bool:?}, pgain: {:?}, ngain: {:?} }}",
            self.en(),
            self.mode(),
            self.biasc(),
            self.intref(),
            self.trigmd(),
            self.inpsel(),
            self.inpf(),
            self.bufen(),
            self.pref(),
            self.adcsw1(),
            self.adcsw2(),
            self.outsw(),
            self.pgain(),
            self.ngain()
        )
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "PGA Function Option"]
    #[must_use]
    #[inline(always)]
    pub const fn pga_function(&self) -> super::vals::PgaFunction {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PgaFunction::from_bits(val as u8)
    }
    #[doc = "PGA Function Option"]
    #[inline(always)]
    pub const fn set_pga_function(&mut self, val: super::vals::PgaFunction) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("pga_function", &self.pga_function())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ pga_function: {:?} }}", self.pga_function())
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
