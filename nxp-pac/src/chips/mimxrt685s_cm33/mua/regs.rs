#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Fn"]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> super::vals::CrFn {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CrFn::from_bits(val as u8)
    }
    #[doc = "Fn"]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: super::vals::CrFn) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "MUR"]
    #[must_use]
    #[inline(always)]
    pub const fn mur(&self) -> super::vals::Mur {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mur::from_bits(val as u8)
    }
    #[doc = "MUR"]
    #[inline(always)]
    pub const fn set_mur(&mut self, val: super::vals::Mur) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "BRDIE"]
    #[must_use]
    #[inline(always)]
    pub const fn rdie(&self) -> super::vals::Rdie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rdie::from_bits(val as u8)
    }
    #[doc = "BRDIE"]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: super::vals::Rdie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RAIE"]
    #[must_use]
    #[inline(always)]
    pub const fn raie(&self) -> super::vals::Raie {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Raie::from_bits(val as u8)
    }
    #[doc = "RAIE"]
    #[inline(always)]
    pub const fn set_raie(&mut self, val: super::vals::Raie) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GIRn"]
    #[must_use]
    #[inline(always)]
    pub const fn girn(&self) -> super::vals::Girn {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Girn::from_bits(val as u8)
    }
    #[doc = "GIRn"]
    #[inline(always)]
    pub const fn set_girn(&mut self, val: super::vals::Girn) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "TIEn"]
    #[must_use]
    #[inline(always)]
    pub const fn tien(&self) -> super::vals::Tien {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Tien::from_bits(val as u8)
    }
    #[doc = "TIEn"]
    #[inline(always)]
    pub const fn set_tien(&mut self, val: super::vals::Tien) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "RIEn"]
    #[must_use]
    #[inline(always)]
    pub const fn rien(&self) -> super::vals::Rien {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Rien::from_bits(val as u8)
    }
    #[doc = "RIEn"]
    #[inline(always)]
    pub const fn set_rien(&mut self, val: super::vals::Rien) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "GIEn"]
    #[must_use]
    #[inline(always)]
    pub const fn gien(&self) -> super::vals::Gien {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Gien::from_bits(val as u8)
    }
    #[doc = "GIEn"]
    #[inline(always)]
    pub const fn set_gien(&mut self, val: super::vals::Gien) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("fn_", &self.fn_())
            .field("mur", &self.mur())
            .field("rdie", &self.rdie())
            .field("raie", &self.raie())
            .field("girn", &self.girn())
            .field("tien", &self.tien())
            .field("rien", &self.rien())
            .field("gien", &self.gien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ fn_: {:?}, mur: {:?}, rdie: {:?}, raie: {:?}, girn: {:?}, tien: {:?}, rien: {:?}, gien: {:?} }}",
            self.fn_(),
            self.mur(),
            self.rdie(),
            self.raie(),
            self.girn(),
            self.tien(),
            self.rien(),
            self.gien()
        )
    }
}
#[doc = "Use Parameter register to determine the parameter settings of MUA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Par(pub u32);
impl Par {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn parameter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_parameter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Par {
    #[inline(always)]
    fn default() -> Par {
        Par(0)
    }
}
impl core::fmt::Debug for Par {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Par")
            .field("parameter", &self.parameter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Par {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Par {{ parameter: {=u32:?} }}", self.parameter())
    }
}
#[doc = "Receive Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rr(pub u32);
impl Rr {
    #[doc = "DATA"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DATA"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rr {
    #[inline(always)]
    fn default() -> Rr {
        Rr(0)
    }
}
impl core::fmt::Debug for Rr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Fn"]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> super::vals::SrFn {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SrFn::from_bits(val as u8)
    }
    #[doc = "Fn"]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: super::vals::SrFn) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "EP"]
    #[must_use]
    #[inline(always)]
    pub const fn ep(&self) -> super::vals::Ep {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ep::from_bits(val as u8)
    }
    #[doc = "EP"]
    #[inline(always)]
    pub const fn set_ep(&mut self, val: super::vals::Ep) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PM"]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> super::vals::Pm {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Pm::from_bits(val as u8)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: super::vals::Pm) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "RS"]
    #[must_use]
    #[inline(always)]
    pub const fn rs(&self) -> super::vals::Rs {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rs::from_bits(val as u8)
    }
    #[doc = "RS"]
    #[inline(always)]
    pub const fn set_rs(&mut self, val: super::vals::Rs) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FUP"]
    #[must_use]
    #[inline(always)]
    pub const fn fup(&self) -> super::vals::Fup {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fup::from_bits(val as u8)
    }
    #[doc = "FUP"]
    #[inline(always)]
    pub const fn set_fup(&mut self, val: super::vals::Fup) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "BRDIP"]
    #[must_use]
    #[inline(always)]
    pub const fn rdip(&self) -> super::vals::Rdip {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rdip::from_bits(val as u8)
    }
    #[doc = "BRDIP"]
    #[inline(always)]
    pub const fn set_rdip(&mut self, val: super::vals::Rdip) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "RAIP"]
    #[must_use]
    #[inline(always)]
    pub const fn raip(&self) -> super::vals::Raip {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Raip::from_bits(val as u8)
    }
    #[doc = "RAIP"]
    #[inline(always)]
    pub const fn set_raip(&mut self, val: super::vals::Raip) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "TEn"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> super::vals::Ten {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Ten::from_bits(val as u8)
    }
    #[doc = "TEn"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: super::vals::Ten) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "RFn"]
    #[must_use]
    #[inline(always)]
    pub const fn rfn(&self) -> super::vals::Rfn {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Rfn::from_bits(val as u8)
    }
    #[doc = "RFn"]
    #[inline(always)]
    pub const fn set_rfn(&mut self, val: super::vals::Rfn) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "GIPn"]
    #[must_use]
    #[inline(always)]
    pub const fn gipn(&self) -> super::vals::Gipn {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Gipn::from_bits(val as u8)
    }
    #[doc = "GIPn"]
    #[inline(always)]
    pub const fn set_gipn(&mut self, val: super::vals::Gipn) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("fn_", &self.fn_())
            .field("ep", &self.ep())
            .field("pm", &self.pm())
            .field("rs", &self.rs())
            .field("fup", &self.fup())
            .field("rdip", &self.rdip())
            .field("raip", &self.raip())
            .field("ten", &self.ten())
            .field("rfn", &self.rfn())
            .field("gipn", &self.gipn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ fn_: {:?}, ep: {:?}, pm: {:?}, rs: {:?}, fup: {:?}, rdip: {:?}, raip: {:?}, ten: {:?}, rfn: {:?}, gipn: {:?} }}",
            self.fn_(),
            self.ep(),
            self.pm(),
            self.rs(),
            self.fup(),
            self.rdip(),
            self.raip(),
            self.ten(),
            self.rfn(),
            self.gipn()
        )
    }
}
#[doc = "Transmit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr(pub u32);
impl Tr {
    #[doc = "DATA"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DATA"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tr {
    #[inline(always)]
    fn default() -> Tr {
        Tr(0)
    }
}
impl core::fmt::Debug for Tr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ver(pub u32);
impl Ver {
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
impl Default for Ver {
    #[inline(always)]
    fn default() -> Ver {
        Ver(0)
    }
}
impl core::fmt::Debug for Ver {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ver")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ver {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ver {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
