#[doc = "OPAMP Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpampCtrl(pub u32);
impl OpampCtrl {
    #[doc = "OPAMP Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn opa_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP Enable"]
    #[inline(always)]
    pub const fn set_opa_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Compensation capcitor config selection"]
    #[must_use]
    #[inline(always)]
    pub const fn opa_cc_sel(&self) -> super::vals::OpaCcSel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::OpaCcSel::from_bits(val as u8)
    }
    #[doc = "Compensation capcitor config selection"]
    #[inline(always)]
    pub const fn set_opa_cc_sel(&mut self, val: super::vals::OpaCcSel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Bias current config selection"]
    #[must_use]
    #[inline(always)]
    pub const fn opa_bc_sel(&self) -> super::vals::OpaBcSel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::OpaBcSel::from_bits(val as u8)
    }
    #[doc = "Bias current config selection"]
    #[inline(always)]
    pub const fn set_opa_bc_sel(&mut self, val: super::vals::OpaBcSel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for OpampCtrl {
    #[inline(always)]
    fn default() -> OpampCtrl {
        OpampCtrl(0)
    }
}
impl core::fmt::Debug for OpampCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OpampCtrl")
            .field("opa_en", &self.opa_en())
            .field("opa_cc_sel", &self.opa_cc_sel())
            .field("opa_bc_sel", &self.opa_bc_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OpampCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OpampCtrl {{ opa_en: {=bool:?}, opa_cc_sel: {:?}, opa_bc_sel: {:?} }}",
            self.opa_en(),
            self.opa_cc_sel(),
            self.opa_bc_sel()
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
