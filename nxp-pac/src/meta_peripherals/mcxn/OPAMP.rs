#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "OPAMP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp {
    ptr: *mut u8,
}
unsafe impl Send for Opamp {}
unsafe impl Sync for Opamp {}
impl Opamp {
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
    #[doc = "Parameter."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OPAMP Control."]
    #[inline(always)]
    pub const fn opamp_ctr(self) -> crate::pac::common::Reg<OpampCtr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "OPAMP Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpampCtr(pub u32);
impl OpampCtr {
    #[doc = "OPAMP Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mode Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> Mode {
        let val = (self.0 >> 1usize) & 0x01;
        Mode::from_bits(val as u8)
    }
    #[doc = "Mode Selection."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: Mode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bias Current Trim Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn biasc(&self) -> Biasc {
        let val = (self.0 >> 2usize) & 0x03;
        Biasc::from_bits(val as u8)
    }
    #[doc = "Bias Current Trim Selection."]
    #[inline(always)]
    pub const fn set_biasc(&mut self, val: Biasc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Provide OPAMP rail to rail voltage selection."]
    #[must_use]
    #[inline(always)]
    pub const fn intref(&self) -> Intref {
        let val = (self.0 >> 4usize) & 0x03;
        Intref::from_bits(val as u8)
    }
    #[doc = "Provide OPAMP rail to rail voltage selection."]
    #[inline(always)]
    pub const fn set_intref(&mut self, val: Intref) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn trigmd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Mode."]
    #[inline(always)]
    pub const fn set_trigmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Positive Input Channel Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn inpsel(&self) -> Inpsel {
        let val = (self.0 >> 9usize) & 0x01;
        Inpsel::from_bits(val as u8)
    }
    #[doc = "Positive Input Channel Selection."]
    #[inline(always)]
    pub const fn set_inpsel(&mut self, val: Inpsel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Positive Input Connection Status."]
    #[must_use]
    #[inline(always)]
    pub const fn inpf(&self) -> Inpf {
        let val = (self.0 >> 12usize) & 0x01;
        Inpf::from_bits(val as u8)
    }
    #[doc = "Positive Input Connection Status."]
    #[inline(always)]
    pub const fn set_inpf(&mut self, val: Inpf) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Reference Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn bufen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Buffer."]
    #[inline(always)]
    pub const fn set_bufen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Positive Reference Voltage Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pref(&self) -> Pref {
        let val = (self.0 >> 17usize) & 0x03;
        Pref::from_bits(val as u8)
    }
    #[doc = "Positive Reference Voltage Selection."]
    #[inline(always)]
    pub const fn set_pref(&mut self, val: Pref) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "Measure Switch 1."]
    #[must_use]
    #[inline(always)]
    pub const fn adcsw1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Measure Switch 1."]
    #[inline(always)]
    pub const fn set_adcsw1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Measure Switch 2."]
    #[must_use]
    #[inline(always)]
    pub const fn adcsw2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Measure Switch 2."]
    #[inline(always)]
    pub const fn set_adcsw2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Output Switch."]
    #[must_use]
    #[inline(always)]
    pub const fn outsw(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Output Switch."]
    #[inline(always)]
    pub const fn set_outsw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Positive PGA Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pgain(&self) -> Pgain {
        let val = (self.0 >> 24usize) & 0x07;
        Pgain::from_bits(val as u8)
    }
    #[doc = "Positive PGA Selection."]
    #[inline(always)]
    pub const fn set_pgain(&mut self, val: Pgain) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Negative PGA Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn ngain(&self) -> Ngain {
        let val = (self.0 >> 28usize) & 0x07;
        Ngain::from_bits(val as u8)
    }
    #[doc = "Negative PGA Selection."]
    #[inline(always)]
    pub const fn set_ngain(&mut self, val: Ngain) {
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
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "PGA Function Option."]
    #[must_use]
    #[inline(always)]
    pub const fn pga_function(&self) -> PgaFunction {
        let val = (self.0 >> 0usize) & 0x01;
        PgaFunction::from_bits(val as u8)
    }
    #[doc = "PGA Function Option."]
    #[inline(always)]
    pub const fn set_pga_function(&mut self, val: PgaFunction) {
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
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Biasc {
    #[doc = "Default."]
    Def = 0x0,
    #[doc = "Increase current."]
    Inc = 0x01,
    #[doc = "Decrease current."]
    Dec = 0x02,
    #[doc = "Further decrease current."]
    FurDec = 0x03,
}
impl Biasc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Biasc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Biasc {
    #[inline(always)]
    fn from(val: u8) -> Biasc {
        Biasc::from_bits(val)
    }
}
impl From<Biasc> for u8 {
    #[inline(always)]
    fn from(val: Biasc) -> u8 {
        Biasc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inpf {
    #[doc = "Positive input 0 (INP0)."]
    Inp0 = 0x0,
    #[doc = "Positive input 1 (INP1)."]
    Inp1 = 0x01,
}
impl Inpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inpf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inpf {
    #[inline(always)]
    fn from(val: u8) -> Inpf {
        Inpf::from_bits(val)
    }
}
impl From<Inpf> for u8 {
    #[inline(always)]
    fn from(val: Inpf) -> u8 {
        Inpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inpsel {
    #[doc = "When OPAMP is not in trigger mode, select positive input 0 (INP0)."]
    Inp0 = 0x0,
    #[doc = "When OPAMP is not in trigger mode, select positive input 1 (INP1)."]
    Inp1 = 0x01,
}
impl Inpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inpsel {
    #[inline(always)]
    fn from(val: u8) -> Inpsel {
        Inpsel::from_bits(val)
    }
}
impl From<Inpsel> for u8 {
    #[inline(always)]
    fn from(val: Inpsel) -> u8 {
        Inpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intref {
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA."]
    Vdda2 = 0x0,
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA-0.8V."]
    Vdda3v = 0x01,
    #[doc = "Select OPAMP input rail to rail voltage from 0.8V to VDD_ANA."]
    Vssa3v = 0x02,
    #[doc = "Not allowed."]
    Not = 0x03,
}
impl Intref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intref {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intref {
    #[inline(always)]
    fn from(val: u8) -> Intref {
        Intref::from_bits(val)
    }
}
impl From<Intref> for u8 {
    #[inline(always)]
    fn from(val: Intref) -> u8 {
        Intref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "High performance mode."]
    Low = 0x0,
    #[doc = "Low power mode."]
    High = 0x01,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ngain {
    #[doc = "Buffer."]
    Buffer = 0x0,
    #[doc = "Ngain=1."]
    G1 = 0x01,
    #[doc = "Ngain=2."]
    G2 = 0x02,
    #[doc = "Ngain=4."]
    G4 = 0x03,
    #[doc = "Ngain=8."]
    G8 = 0x04,
    #[doc = "Ngain=16."]
    G16 = 0x05,
    #[doc = "Ngain=33."]
    G33 = 0x06,
    #[doc = "Ngain=64."]
    G64 = 0x07,
}
impl Ngain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ngain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ngain {
    #[inline(always)]
    fn from(val: u8) -> Ngain {
        Ngain::from_bits(val)
    }
}
impl From<Ngain> for u8 {
    #[inline(always)]
    fn from(val: Ngain) -> u8 {
        Ngain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgaFunction {
    #[doc = "Core amplifier enabled."]
    CoreAmp = 0x0,
    #[doc = "PGA function enabled."]
    Pga = 0x01,
}
impl PgaFunction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgaFunction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgaFunction {
    #[inline(always)]
    fn from(val: u8) -> PgaFunction {
        PgaFunction::from_bits(val)
    }
}
impl From<PgaFunction> for u8 {
    #[inline(always)]
    fn from(val: PgaFunction) -> u8 {
        PgaFunction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pgain {
    #[doc = "Positive input 1 (INP1)."]
    Inp1 = 0x0,
    #[doc = "Pgain=1."]
    G2 = 0x01,
    #[doc = "Pgain=2."]
    G3 = 0x02,
    #[doc = "Pgain=4."]
    G5 = 0x03,
    #[doc = "Pgain=8."]
    G9 = 0x04,
    #[doc = "Pgain=16."]
    G17 = 0x05,
    #[doc = "Pgain=33."]
    G34 = 0x06,
    #[doc = "Pgain=64."]
    G65 = 0x07,
}
impl Pgain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pgain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pgain {
    #[inline(always)]
    fn from(val: u8) -> Pgain {
        Pgain::from_bits(val)
    }
}
impl From<Pgain> for u8 {
    #[inline(always)]
    fn from(val: Pgain) -> u8 {
        Pgain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pref {
    #[doc = "Input 0."]
    Val0 = 0x0,
    #[doc = "Input 1."]
    Val1 = 0x01,
    #[doc = "Input 2."]
    Val2 = 0x02,
    #[doc = "Input 3."]
    Val3 = 0x03,
}
impl Pref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pref {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pref {
    #[inline(always)]
    fn from(val: u8) -> Pref {
        Pref::from_bits(val)
    }
}
impl From<Pref> for u8 {
    #[inline(always)]
    fn from(val: Pref) -> u8 {
        Pref::to_bits(val)
    }
}
