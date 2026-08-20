#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "VREF."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vref {
    ptr: *mut u8,
}
unsafe impl Send for Vref {}
unsafe impl Sync for Vref {}
impl Vref {
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
    #[doc = "Control and Status."]
    #[inline(always)]
    pub const fn csr(self) -> crate::pac::common::Reg<Csr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "User Trim."]
    #[inline(always)]
    pub const fn utrim(self) -> crate::pac::common::Reg<Utrim, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
#[doc = "Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "HC Bandgap Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn hcbgen(&self) -> Hcbgen {
        let val = (self.0 >> 0usize) & 0x01;
        Hcbgen::from_bits(val as u8)
    }
    #[doc = "HC Bandgap Enabled."]
    #[inline(always)]
    pub const fn set_hcbgen(&mut self, val: Hcbgen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Low-Power Bandgap Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbgen(&self) -> Lpbgen {
        let val = (self.0 >> 1usize) & 0x01;
        Lpbgen::from_bits(val as u8)
    }
    #[doc = "Low-Power Bandgap Enable."]
    #[inline(always)]
    pub const fn set_lpbgen(&mut self, val: Lpbgen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Low-Power Bandgap Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_buf_en(&self) -> LpbgBufEn {
        let val = (self.0 >> 2usize) & 0x01;
        LpbgBufEn::from_bits(val as u8)
    }
    #[doc = "Low-Power Bandgap Buffer Enable."]
    #[inline(always)]
    pub const fn set_lpbg_buf_en(&mut self, val: LpbgBufEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Chop Oscillator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chopen(&self) -> Chopen {
        let val = (self.0 >> 3usize) & 0x01;
        Chopen::from_bits(val as u8)
    }
    #[doc = "Chop Oscillator Enable."]
    #[inline(always)]
    pub const fn set_chopen(&mut self, val: Chopen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Current Compensation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn icompen(&self) -> Icompen {
        let val = (self.0 >> 4usize) & 0x01;
        Icompen::from_bits(val as u8)
    }
    #[doc = "Current Compensation Enable."]
    #[inline(always)]
    pub const fn set_icompen(&mut self, val: Icompen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Regulator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regen(&self) -> Regen {
        let val = (self.0 >> 5usize) & 0x01;
        Regen::from_bits(val as u8)
    }
    #[doc = "Regulator Enable."]
    #[inline(always)]
    pub const fn set_regen(&mut self, val: Regen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "High-Power Level."]
    #[must_use]
    #[inline(always)]
    pub const fn hi_pwr_lv(&self) -> HiPwrLv {
        let val = (self.0 >> 11usize) & 0x01;
        HiPwrLv::from_bits(val as u8)
    }
    #[doc = "High-Power Level."]
    #[inline(always)]
    pub const fn set_hi_pwr_lv(&mut self, val: HiPwrLv) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Internal Buffer21 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn buf21en(&self) -> Buf21en {
        let val = (self.0 >> 16usize) & 0x01;
        Buf21en::from_bits(val as u8)
    }
    #[doc = "Internal Buffer21 Enable."]
    #[inline(always)]
    pub const fn set_buf21en(&mut self, val: Buf21en) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Internal HC Voltage Reference Stable."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefst(&self) -> Vrefst {
        let val = (self.0 >> 31usize) & 0x01;
        Vrefst::from_bits(val as u8)
    }
    #[doc = "Internal HC Voltage Reference Stable."]
    #[inline(always)]
    pub const fn set_vrefst(&mut self, val: Vrefst) {
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
#[doc = "User Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utrim(pub u32);
impl Utrim {
    #[doc = "VREF 2.1 V Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn trim2v1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "VREF 2.1 V Trim."]
    #[inline(always)]
    pub const fn set_trim2v1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "VREF Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn vreftrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "VREF Trim."]
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
pub enum Buf21en {
    #[doc = "Disables."]
    Dis = 0x0,
    #[doc = "Enables."]
    Ena = 0x01,
}
impl Buf21en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf21en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf21en {
    #[inline(always)]
    fn from(val: u8) -> Buf21en {
        Buf21en::from_bits(val)
    }
}
impl From<Buf21en> for u8 {
    #[inline(always)]
    fn from(val: Buf21en) -> u8 {
        Buf21en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chopen {
    #[doc = "Disables."]
    Dis = 0x0,
    #[doc = "Enables."]
    Ena = 0x01,
}
impl Chopen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chopen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chopen {
    #[inline(always)]
    fn from(val: u8) -> Chopen {
        Chopen::from_bits(val)
    }
}
impl From<Chopen> for u8 {
    #[inline(always)]
    fn from(val: Chopen) -> u8 {
        Chopen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hcbgen {
    #[doc = "Disables."]
    Dis = 0x0,
    #[doc = "Enables."]
    Ena = 0x01,
}
impl Hcbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hcbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hcbgen {
    #[inline(always)]
    fn from(val: u8) -> Hcbgen {
        Hcbgen::from_bits(val)
    }
}
impl From<Hcbgen> for u8 {
    #[inline(always)]
    fn from(val: Hcbgen) -> u8 {
        Hcbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HiPwrLv {
    #[doc = "Low-power."]
    Low = 0x0,
    #[doc = "High-power."]
    High = 0x01,
}
impl HiPwrLv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HiPwrLv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HiPwrLv {
    #[inline(always)]
    fn from(val: u8) -> HiPwrLv {
        HiPwrLv::from_bits(val)
    }
}
impl From<HiPwrLv> for u8 {
    #[inline(always)]
    fn from(val: HiPwrLv) -> u8 {
        HiPwrLv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icompen {
    #[doc = "Disables."]
    Dis = 0x0,
    #[doc = "Enables."]
    Ena = 0x01,
}
impl Icompen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icompen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icompen {
    #[inline(always)]
    fn from(val: u8) -> Icompen {
        Icompen::from_bits(val)
    }
}
impl From<Icompen> for u8 {
    #[inline(always)]
    fn from(val: Icompen) -> u8 {
        Icompen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpbgBufEn {
    #[doc = "Disables."]
    Dis = 0x0,
    #[doc = "Enables."]
    Ena = 0x01,
}
impl LpbgBufEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpbgBufEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpbgBufEn {
    #[inline(always)]
    fn from(val: u8) -> LpbgBufEn {
        LpbgBufEn::from_bits(val)
    }
}
impl From<LpbgBufEn> for u8 {
    #[inline(always)]
    fn from(val: LpbgBufEn) -> u8 {
        LpbgBufEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpbgen {
    #[doc = "Disables."]
    Dis = 0x0,
    #[doc = "Enables."]
    Ena = 0x01,
}
impl Lpbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpbgen {
    #[inline(always)]
    fn from(val: u8) -> Lpbgen {
        Lpbgen::from_bits(val)
    }
}
impl From<Lpbgen> for u8 {
    #[inline(always)]
    fn from(val: Lpbgen) -> u8 {
        Lpbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Regen {
    #[doc = "Disables."]
    Dis = 0x0,
    #[doc = "Enables."]
    Ena = 0x01,
}
impl Regen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Regen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Regen {
    #[inline(always)]
    fn from(val: u8) -> Regen {
        Regen::from_bits(val)
    }
}
impl From<Regen> for u8 {
    #[inline(always)]
    fn from(val: Regen) -> u8 {
        Regen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrefst {
    #[doc = "Disabled and unstable."]
    DisNotstable = 0x0,
    #[doc = "Stable."]
    Stable = 0x01,
}
impl Vrefst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrefst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrefst {
    #[inline(always)]
    fn from(val: u8) -> Vrefst {
        Vrefst::from_bits(val)
    }
}
impl From<Vrefst> for u8 {
    #[inline(always)]
    fn from(val: Vrefst) -> u8 {
        Vrefst::to_bits(val)
    }
}
