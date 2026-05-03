#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "System Clock Generator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scg {
    ptr: *mut u8,
}
unsafe impl Send for Scg {}
unsafe impl Sync for Scg {}
impl Scg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID Register."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter Register."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Trim Lock register."]
    #[inline(always)]
    pub const fn trim_lock(self) -> crate::pac::common::Reg<TrimLock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock Status Register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::pac::common::Reg<Csr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Run Clock Control Register."]
    #[inline(always)]
    pub const fn rccr(self) -> crate::pac::common::Reg<Rccr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SOSC Control Status Register."]
    #[inline(always)]
    pub const fn sosccsr(self) -> crate::pac::common::Reg<Sosccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SOSC Configuration Register."]
    #[inline(always)]
    pub const fn sosccfg(self) -> crate::pac::common::Reg<Sosccfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SOSC Test Register."]
    #[inline(always)]
    pub const fn sosctest(self) -> crate::pac::common::Reg<Sosctest, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "SIRC Control Status Register."]
    #[inline(always)]
    pub const fn sirccsr(self) -> crate::pac::common::Reg<Sirccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SIRC Trim Configuration Register."]
    #[inline(always)]
    pub const fn sirctcfg(self) -> crate::pac::common::Reg<Sirctcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "SIRC Trim Register."]
    #[inline(always)]
    pub const fn sirctrim(self) -> crate::pac::common::Reg<Sirctrim, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SIRC Auto-trimming Status Register."]
    #[inline(always)]
    pub const fn sircstat(self) -> crate::pac::common::Reg<Sircstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "SIRC Test Register."]
    #[inline(always)]
    pub const fn sirctest(self) -> crate::pac::common::Reg<Sirctest, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "FIRC Control Status Register."]
    #[inline(always)]
    pub const fn firccsr(self) -> crate::pac::common::Reg<Firccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "FIRC Configuration Register."]
    #[inline(always)]
    pub const fn firccfg(self) -> crate::pac::common::Reg<Firccfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "FIRC Trim Configuration Register."]
    #[inline(always)]
    pub const fn firctcfg(self) -> crate::pac::common::Reg<Firctcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "FIRC Trim Register."]
    #[inline(always)]
    pub const fn firctrim(self) -> crate::pac::common::Reg<Firctrim, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Status Register."]
    #[inline(always)]
    pub const fn fircstat(self) -> crate::pac::common::Reg<Fircstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "FIRC Test Register."]
    #[inline(always)]
    pub const fn firctest(self) -> crate::pac::common::Reg<Firctest, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "ROSC Control Status Register."]
    #[inline(always)]
    pub const fn rosccsr(self) -> crate::pac::common::Reg<Rosccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "APLL Control Status Register."]
    #[inline(always)]
    pub const fn apllcsr(self) -> crate::pac::common::Reg<Apllcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "APLL Control Register."]
    #[inline(always)]
    pub const fn apllctrl(self) -> crate::pac::common::Reg<Apllctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "APLL Status Register."]
    #[inline(always)]
    pub const fn apllstat(self) -> crate::pac::common::Reg<Apllstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "APLL N Divider Register."]
    #[inline(always)]
    pub const fn apllndiv(self) -> crate::pac::common::Reg<Apllndiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "APLL M Divider Register."]
    #[inline(always)]
    pub const fn apllmdiv(self) -> crate::pac::common::Reg<Apllmdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "APLL P Divider Register."]
    #[inline(always)]
    pub const fn apllpdiv(self) -> crate::pac::common::Reg<Apllpdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "APLL LOCK Configuration Register."]
    #[inline(always)]
    pub const fn aplllock_cnfg(
        self,
    ) -> crate::pac::common::Reg<AplllockCnfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "APLL SSCG Status Register."]
    #[inline(always)]
    pub const fn apllsscgstat(
        self,
    ) -> crate::pac::common::Reg<Apllsscgstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "APLL Spread Spectrum Control 0 Register."]
    #[inline(always)]
    pub const fn apllsscg0(self) -> crate::pac::common::Reg<Apllsscg0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "APLL Spread Spectrum Control 1 Register."]
    #[inline(always)]
    pub const fn apllsscg1(self) -> crate::pac::common::Reg<Apllsscg1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "APLL Test Status Register."]
    #[inline(always)]
    pub const fn apllteststat(
        self,
    ) -> crate::pac::common::Reg<Apllteststat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "APLL Test Control Register."]
    #[inline(always)]
    pub const fn aplltestctrl(
        self,
    ) -> crate::pac::common::Reg<Aplltestctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "SPLL Control Status Register."]
    #[inline(always)]
    pub const fn spllcsr(self) -> crate::pac::common::Reg<Spllcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "SPLL Control Register."]
    #[inline(always)]
    pub const fn spllctrl(self) -> crate::pac::common::Reg<Spllctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "SPLL Status Register."]
    #[inline(always)]
    pub const fn spllstat(self) -> crate::pac::common::Reg<Spllstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "SPLL N Divider Register."]
    #[inline(always)]
    pub const fn spllndiv(self) -> crate::pac::common::Reg<Spllndiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "SPLL M Divider Register."]
    #[inline(always)]
    pub const fn spllmdiv(self) -> crate::pac::common::Reg<Spllmdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "SPLL P Divider Register."]
    #[inline(always)]
    pub const fn spllpdiv(self) -> crate::pac::common::Reg<Spllpdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "SPLL LOCK Configuration Register."]
    #[inline(always)]
    pub const fn splllock_cnfg(
        self,
    ) -> crate::pac::common::Reg<SplllockCnfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "SPLL SSCG Status Register."]
    #[inline(always)]
    pub const fn spllsscgstat(
        self,
    ) -> crate::pac::common::Reg<Spllsscgstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "SPLL Spread Spectrum Control 0 Register."]
    #[inline(always)]
    pub const fn spllsscg0(self) -> crate::pac::common::Reg<Spllsscg0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "SPLL Spread Spectrum Control 1 Register."]
    #[inline(always)]
    pub const fn spllsscg1(self) -> crate::pac::common::Reg<Spllsscg1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "SPLL Test Status Register."]
    #[inline(always)]
    pub const fn spllteststat(
        self,
    ) -> crate::pac::common::Reg<Spllteststat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f8usize) as _) }
    }
    #[doc = "SPLL Test Control Register."]
    #[inline(always)]
    pub const fn splltestctrl(
        self,
    ) -> crate::pac::common::Reg<Splltestctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06fcusize) as _) }
    }
    #[doc = "UPLL Control Status Register."]
    #[inline(always)]
    pub const fn upllcsr(self) -> crate::pac::common::Reg<Upllcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "LDO Control and Status Register."]
    #[inline(always)]
    pub const fn ldocsr(self) -> crate::pac::common::Reg<Ldocsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "LDO Test Register."]
    #[inline(always)]
    pub const fn ldotest(self) -> crate::pac::common::Reg<Ldotest, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08fcusize) as _) }
    }
}
#[doc = "APLL Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllcsr(pub u32);
impl Apllcsr {
    #[doc = "APLL Power Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn apllpwren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "APLL Power Enable."]
    #[inline(always)]
    pub const fn set_apllpwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APLL Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn apllclken(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APLL Clock Enable."]
    #[inline(always)]
    pub const fn set_apllclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "APLL Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn apllsten(&self) -> Apllsten {
        let val = (self.0 >> 2usize) & 0x01;
        Apllsten::from_bits(val as u8)
    }
    #[doc = "APLL Stop Enable."]
    #[inline(always)]
    pub const fn set_apllsten(&mut self, val: Apllsten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Free running mode clock stable."]
    #[must_use]
    #[inline(always)]
    pub const fn frm_clockstable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Free running mode clock stable."]
    #[inline(always)]
    pub const fn set_frm_clockstable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "APLL Clock Monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn apllcm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "APLL Clock Monitor."]
    #[inline(always)]
    pub const fn set_apllcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "APLL Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn apllcmre(&self) -> Apllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        Apllcmre::from_bits(val as u8)
    }
    #[doc = "APLL Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_apllcmre(&mut self, val: Apllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> ApllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        ApllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: ApllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "APLL LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn apll_lock(&self) -> ApllLock {
        let val = (self.0 >> 24usize) & 0x01;
        ApllLock::from_bits(val as u8)
    }
    #[doc = "APLL LOCK."]
    #[inline(always)]
    pub const fn set_apll_lock(&mut self, val: ApllLock) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "APLL Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn apllsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "APLL Selected."]
    #[inline(always)]
    pub const fn set_apllsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "APLL Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn apllerr(&self) -> Apllerr {
        let val = (self.0 >> 26usize) & 0x01;
        Apllerr::from_bits(val as u8)
    }
    #[doc = "APLL Clock Error."]
    #[inline(always)]
    pub const fn set_apllerr(&mut self, val: Apllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "APLL LOCK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn apll_lock_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "APLL LOCK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_apll_lock_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Apllcsr {
    #[inline(always)]
    fn default() -> Apllcsr {
        Apllcsr(0)
    }
}
impl core::fmt::Debug for Apllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllcsr")
            .field("apllpwren", &self.apllpwren())
            .field("apllclken", &self.apllclken())
            .field("apllsten", &self.apllsten())
            .field("frm_clockstable", &self.frm_clockstable())
            .field("apllcm", &self.apllcm())
            .field("apllcmre", &self.apllcmre())
            .field("lk", &self.lk())
            .field("apll_lock", &self.apll_lock())
            .field("apllsel", &self.apllsel())
            .field("apllerr", &self.apllerr())
            .field("apll_lock_ie", &self.apll_lock_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllcsr {{ apllpwren: {=bool:?}, apllclken: {=bool:?}, apllsten: {:?}, frm_clockstable: {=bool:?}, apllcm: {=bool:?}, apllcmre: {:?}, lk: {:?}, apll_lock: {:?}, apllsel: {=bool:?}, apllerr: {:?}, apll_lock_ie: {=bool:?} }}",
            self.apllpwren(),
            self.apllclken(),
            self.apllsten(),
            self.frm_clockstable(),
            self.apllcm(),
            self.apllcmre(),
            self.lk(),
            self.apll_lock(),
            self.apllsel(),
            self.apllerr(),
            self.apll_lock_ie()
        )
    }
}
#[doc = "APLL Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllctrl(pub u32);
impl Apllctrl {
    #[doc = "Bandwidth select R (resistor) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R (resistor) value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass of Divide-by-2 Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of Divide-by-2 Divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Up Limiter."]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Up Limiter."]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn banddirect(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_banddirect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bypass of the predivider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the predivider."]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bypass of the postdivider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the postdivider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Free Running Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Free Running Mode Enable."]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Skew mode."]
    #[must_use]
    #[inline(always)]
    pub const fn skew_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Skew mode."]
    #[inline(always)]
    pub const fn set_skew_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> ApllctrlSource {
        let val = (self.0 >> 25usize) & 0x03;
        ApllctrlSource::from_bits(val as u8)
    }
    #[doc = "Clock Source."]
    #[inline(always)]
    pub const fn set_source(&mut self, val: ApllctrlSource) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for Apllctrl {
    #[inline(always)]
    fn default() -> Apllctrl {
        Apllctrl(0)
    }
}
impl core::fmt::Debug for Apllctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("banddirect", &self.banddirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("frm", &self.frm())
            .field("skew_en", &self.skew_en())
            .field("source", &self.source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspostdiv2: {=bool:?}, limupoff: {=bool:?}, banddirect: {=bool:?}, bypassprediv: {=bool:?}, bypasspostdiv: {=bool:?}, frm: {=bool:?}, skew_en: {=bool:?}, source: {:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.banddirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.frm(),
            self.skew_en(),
            self.source()
        )
    }
}
#[doc = "APLL LOCK Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AplllockCnfg(pub u32);
impl AplllockCnfg {
    #[doc = "Configures the number of reference clocks to count before APLL is considered locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Configures the number of reference clocks to count before APLL is considered locked."]
    #[inline(always)]
    pub const fn set_lock_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for AplllockCnfg {
    #[inline(always)]
    fn default() -> AplllockCnfg {
        AplllockCnfg(0)
    }
}
impl core::fmt::Debug for AplllockCnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AplllockCnfg")
            .field("lock_time", &self.lock_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AplllockCnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AplllockCnfg {{ lock_time: {=u32:?} }}",
            self.lock_time()
        )
    }
}
#[doc = "APLL M Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllmdiv(pub u32);
impl Apllmdiv {
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_mdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback ratio change request."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllmdiv {
    #[inline(always)]
    fn default() -> Apllmdiv {
        Apllmdiv(0)
    }
}
impl core::fmt::Debug for Apllmdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllmdiv")
            .field("mdiv", &self.mdiv())
            .field("mreq", &self.mreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllmdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllmdiv {{ mdiv: {=u16:?}, mreq: {=bool:?} }}",
            self.mdiv(),
            self.mreq()
        )
    }
}
#[doc = "APLL N Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllndiv(pub u32);
impl Apllndiv {
    #[doc = "Predivider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Predivider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Predivider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllndiv {
    #[inline(always)]
    fn default() -> Apllndiv {
        Apllndiv(0)
    }
}
impl core::fmt::Debug for Apllndiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllndiv")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllndiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllndiv {{ ndiv: {=u8:?}, nreq: {=bool:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "APLL P Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllpdiv(pub u32);
impl Apllpdiv {
    #[doc = "Postdivider divider ratio (P-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Postdivider divider ratio (P-divider)."]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Postdivider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Postdivider ratio change request."]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllpdiv {
    #[inline(always)]
    fn default() -> Apllpdiv {
        Apllpdiv(0)
    }
}
impl core::fmt::Debug for Apllpdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllpdiv")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllpdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllpdiv {{ pdiv: {=u8:?}, preq: {=bool:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "APLL Spread Spectrum Control 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllsscg0(pub u32);
impl Apllsscg0 {
    #[doc = "SS_MDIV."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_lsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SS_MDIV."]
    #[inline(always)]
    pub const fn set_ss_mdiv_lsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Apllsscg0 {
    #[inline(always)]
    fn default() -> Apllsscg0 {
        Apllsscg0(0)
    }
}
impl core::fmt::Debug for Apllsscg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllsscg0")
            .field("ss_mdiv_lsb", &self.ss_mdiv_lsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllsscg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllsscg0 {{ ss_mdiv_lsb: {=u32:?} }}",
            self.ss_mdiv_lsb()
        )
    }
}
#[doc = "APLL Spread Spectrum Control 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllsscg1(pub u32);
impl Apllsscg1 {
    #[doc = "SS_MDIV\\[32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32\\]."]
    #[inline(always)]
    pub const fn set_ss_mdiv_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[inline(always)]
    pub const fn set_ss_mdiv_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Modulation Frequency Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Frequency Control."]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Modulation Depth Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Depth Control."]
    #[inline(always)]
    pub const fn set_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Modulation Waveform Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> Apllsscg1Mc {
        let val = (self.0 >> 8usize) & 0x03;
        Apllsscg1Mc::from_bits(val as u8)
    }
    #[doc = "Modulation Waveform Control."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: Apllsscg1Mc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Dither Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Dither Enable."]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SS_MDIV select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ss_mdiv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV select."]
    #[inline(always)]
    pub const fn set_sel_ss_mdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SSCG Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_pd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SSCG Power Down."]
    #[inline(always)]
    pub const fn set_ss_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllsscg1 {
    #[inline(always)]
    fn default() -> Apllsscg1 {
        Apllsscg1(0)
    }
}
impl core::fmt::Debug for Apllsscg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllsscg1")
            .field("ss_mdiv_msb", &self.ss_mdiv_msb())
            .field("ss_mdiv_req", &self.ss_mdiv_req())
            .field("mf", &self.mf())
            .field("mr", &self.mr())
            .field("mc", &self.mc())
            .field("dither", &self.dither())
            .field("sel_ss_mdiv", &self.sel_ss_mdiv())
            .field("ss_pd", &self.ss_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllsscg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllsscg1 {{ ss_mdiv_msb: {=bool:?}, ss_mdiv_req: {=bool:?}, mf: {=u8:?}, mr: {=u8:?}, mc: {:?}, dither: {=bool:?}, sel_ss_mdiv: {=bool:?}, ss_pd: {=bool:?} }}",
            self.ss_mdiv_msb(),
            self.ss_mdiv_req(),
            self.mf(),
            self.mr(),
            self.mc(),
            self.dither(),
            self.sel_ss_mdiv(),
            self.ss_pd()
        )
    }
}
#[doc = "APLL SSCG Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllsscgstat(pub u32);
impl Apllsscgstat {
    #[doc = "SS_MDIV change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV change acknowledge."]
    #[inline(always)]
    pub const fn set_ss_mdiv_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Apllsscgstat {
    #[inline(always)]
    fn default() -> Apllsscgstat {
        Apllsscgstat(0)
    }
}
impl core::fmt::Debug for Apllsscgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllsscgstat")
            .field("ss_mdiv_ack", &self.ss_mdiv_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllsscgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllsscgstat {{ ss_mdiv_ack: {=bool:?} }}",
            self.ss_mdiv_ack()
        )
    }
}
#[doc = "APLL Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllstat(pub u32);
impl Apllstat {
    #[doc = "Lock detector output (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock detector output (active high)."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Predivider(N) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ndivack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider(N) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_ndivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Feedback(M) divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn mdivack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback(M) divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_mdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Postdivider(P) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn pdivack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Postdivider(P) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_pdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Free running detector (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn frmdet(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Free running detector (active high)."]
    #[inline(always)]
    pub const fn set_frmdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Apllstat {
    #[inline(always)]
    fn default() -> Apllstat {
        Apllstat(0)
    }
}
impl core::fmt::Debug for Apllstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllstat")
            .field("lock", &self.lock())
            .field("ndivack", &self.ndivack())
            .field("mdivack", &self.mdivack())
            .field("pdivack", &self.pdivack())
            .field("frmdet", &self.frmdet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllstat {{ lock: {=bool:?}, ndivack: {=bool:?}, mdivack: {=bool:?}, pdivack: {=bool:?}, frmdet: {=bool:?} }}",
            self.lock(),
            self.ndivack(),
            self.mdivack(),
            self.pdivack(),
            self.frmdet()
        )
    }
}
#[doc = "APLL Test Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aplltestctrl(pub u32);
impl Aplltestctrl {
    #[doc = "Input to functional test the predivider (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn nfunctest(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input to functional test the predivider (N-divider)."]
    #[inline(always)]
    pub const fn set_nfunctest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Input to functional test the feedback-divider (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mfunctest(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Input to functional test the feedback-divider (M-divider)."]
    #[inline(always)]
    pub const fn set_mfunctest(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "Input to functional test the postdivider (P-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn pfunctest(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "Input to functional test the postdivider (P-divider)."]
    #[inline(always)]
    pub const fn set_pfunctest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Enable test mux."]
    #[must_use]
    #[inline(always)]
    pub const fn testv_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable test mux."]
    #[inline(always)]
    pub const fn set_testv_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Select analog signal channel that need to be test."]
    #[must_use]
    #[inline(always)]
    pub const fn testv_sel(&self) -> AplltestctrlTestvSel {
        let val = (self.0 >> 27usize) & 0x07;
        AplltestctrlTestvSel::from_bits(val as u8)
    }
    #[doc = "Select analog signal channel that need to be test."]
    #[inline(always)]
    pub const fn set_testv_sel(&mut self, val: AplltestctrlTestvSel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Bypass PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspll(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass PLL."]
    #[inline(always)]
    pub const fn set_bypasspll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Aplltestctrl {
    #[inline(always)]
    fn default() -> Aplltestctrl {
        Aplltestctrl(0)
    }
}
impl core::fmt::Debug for Aplltestctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aplltestctrl")
            .field("nfunctest", &self.nfunctest())
            .field("mfunctest", &self.mfunctest())
            .field("pfunctest", &self.pfunctest())
            .field("testv_en", &self.testv_en())
            .field("testv_sel", &self.testv_sel())
            .field("bypasspll", &self.bypasspll())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aplltestctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aplltestctrl {{ nfunctest: {=u8:?}, mfunctest: {=u16:?}, pfunctest: {=u8:?}, testv_en: {=bool:?}, testv_sel: {:?}, bypasspll: {=bool:?} }}",
            self.nfunctest(),
            self.mfunctest(),
            self.pfunctest(),
            self.testv_en(),
            self.testv_sel(),
            self.bypasspll()
        )
    }
}
#[doc = "APLL Test Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apllteststat(pub u32);
impl Apllteststat {
    #[doc = "Output to observe the functional predivider test."]
    #[must_use]
    #[inline(always)]
    pub const fn nmotest(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Output to observe the functional predivider test."]
    #[inline(always)]
    pub const fn set_nmotest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output to observe the functional feedback-divider test."]
    #[must_use]
    #[inline(always)]
    pub const fn mmotest(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Output to observe the functional feedback-divider test."]
    #[inline(always)]
    pub const fn set_mmotest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Output to observe the functional postdivider test."]
    #[must_use]
    #[inline(always)]
    pub const fn pmotest(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Output to observe the functional postdivider test."]
    #[inline(always)]
    pub const fn set_pmotest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "PLL functional divider test clock pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_xfunc_test_pulse(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "PLL functional divider test clock pulse."]
    #[inline(always)]
    pub const fn set_pll_xfunc_test_pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable PLL functional divider test clock pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_xfunc_test_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL functional divider test clock pulse."]
    #[inline(always)]
    pub const fn set_pll_xfunc_test_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable to observe the xMOTEST flags."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_xmo_test_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable to observe the xMOTEST flags."]
    #[inline(always)]
    pub const fn set_pll_xmo_test_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Apllteststat {
    #[inline(always)]
    fn default() -> Apllteststat {
        Apllteststat(0)
    }
}
impl core::fmt::Debug for Apllteststat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Apllteststat")
            .field("nmotest", &self.nmotest())
            .field("mmotest", &self.mmotest())
            .field("pmotest", &self.pmotest())
            .field("pll_xfunc_test_pulse", &self.pll_xfunc_test_pulse())
            .field("pll_xfunc_test_en", &self.pll_xfunc_test_en())
            .field("pll_xmo_test_en", &self.pll_xmo_test_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Apllteststat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Apllteststat {{ nmotest: {=u8:?}, mmotest: {=u8:?}, pmotest: {=u8:?}, pll_xfunc_test_pulse: {=bool:?}, pll_xfunc_test_en: {=bool:?}, pll_xmo_test_en: {=bool:?} }}",
            self.nmotest(),
            self.mmotest(),
            self.pmotest(),
            self.pll_xfunc_test_pulse(),
            self.pll_xfunc_test_en(),
            self.pll_xmo_test_en()
        )
    }
}
#[doc = "Clock Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "System Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> Scs {
        let val = (self.0 >> 24usize) & 0x0f;
        Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: Scs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
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
        f.debug_struct("Csr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Csr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "FIRC Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccfg(pub u32);
impl Firccfg {
    #[doc = "Frequency Range."]
    #[must_use]
    #[inline(always)]
    pub const fn freq_sel(&self) -> FreqSel {
        let val = (self.0 >> 1usize) & 0x07;
        FreqSel::from_bits(val as u8)
    }
    #[doc = "Frequency Range."]
    #[inline(always)]
    pub const fn set_freq_sel(&mut self, val: FreqSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
}
impl Default for Firccfg {
    #[inline(always)]
    fn default() -> Firccfg {
        Firccfg(0)
    }
}
impl core::fmt::Debug for Firccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccfg")
            .field("freq_sel", &self.freq_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Firccfg {{ freq_sel: {:?} }}", self.freq_sel())
    }
}
#[doc = "FIRC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccsr(pub u32);
impl Firccsr {
    #[doc = "FIRC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Enable."]
    #[inline(always)]
    pub const fn set_fircen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIRC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircsten(&self) -> Fircsten {
        let val = (self.0 >> 1usize) & 0x01;
        Fircsten::from_bits(val as u8)
    }
    #[doc = "FIRC Stop Enable."]
    #[inline(always)]
    pub const fn set_fircsten(&mut self, val: Fircsten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firc_sclk_periph_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable."]
    #[inline(always)]
    pub const fn set_firc_sclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FIRC 144 MHz Clock to peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firc_fclk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC 144 MHz Clock to peripherals Enable."]
    #[inline(always)]
    pub const fn set_firc_fclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "FIRC 144 MHz Trim Enable (FIRCCFG\\[RANGE\\]=1)."]
    #[must_use]
    #[inline(always)]
    pub const fn firctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC 144 MHz Trim Enable (FIRCCFG\\[RANGE\\]=1)."]
    #[inline(always)]
    pub const fn set_firctren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIRC Trim Update."]
    #[must_use]
    #[inline(always)]
    pub const fn firctrup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Trim Update."]
    #[inline(always)]
    pub const fn set_firctrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIRC TRIM LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> FirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        FirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "FIRC TRIM LOCK."]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: FirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> FirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        FirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: FirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FIRC Valid status."]
    #[must_use]
    #[inline(always)]
    pub const fn fircvld(&self) -> Fircvld {
        let val = (self.0 >> 24usize) & 0x01;
        Fircvld::from_bits(val as u8)
    }
    #[doc = "FIRC Valid status."]
    #[inline(always)]
    pub const fn set_fircvld(&mut self, val: Fircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FIRC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn fircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Selected."]
    #[inline(always)]
    pub const fn set_fircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "FIRC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr(&self) -> Fircerr {
        let val = (self.0 >> 26usize) & 0x01;
        Fircerr::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error."]
    #[inline(always)]
    pub const fn set_fircerr(&mut self, val: Fircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FIRC Clock Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr_ie(&self) -> FircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        FircerrIe::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fircerr_ie(&mut self, val: FircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "FIRC Accurate Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc_ie(&self) -> FircaccIe {
        let val = (self.0 >> 30usize) & 0x01;
        FircaccIe::from_bits(val as u8)
    }
    #[doc = "FIRC Accurate Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fircacc_ie(&mut self, val: FircaccIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FIRC Frequency Accurate."]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc(&self) -> Fircacc {
        let val = (self.0 >> 31usize) & 0x01;
        Fircacc::from_bits(val as u8)
    }
    #[doc = "FIRC Frequency Accurate."]
    #[inline(always)]
    pub const fn set_fircacc(&mut self, val: Fircacc) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Firccsr {
    #[inline(always)]
    fn default() -> Firccsr {
        Firccsr(0)
    }
}
impl core::fmt::Debug for Firccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccsr")
            .field("fircen", &self.fircen())
            .field("fircsten", &self.fircsten())
            .field("firc_sclk_periph_en", &self.firc_sclk_periph_en())
            .field("firc_fclk_periph_en", &self.firc_fclk_periph_en())
            .field("firctren", &self.firctren())
            .field("firctrup", &self.firctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("fircvld", &self.fircvld())
            .field("fircsel", &self.fircsel())
            .field("fircerr", &self.fircerr())
            .field("fircerr_ie", &self.fircerr_ie())
            .field("fircacc_ie", &self.fircacc_ie())
            .field("fircacc", &self.fircacc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firccsr {{ fircen: {=bool:?}, fircsten: {:?}, firc_sclk_periph_en: {=bool:?}, firc_fclk_periph_en: {=bool:?}, firctren: {=bool:?}, firctrup: {=bool:?}, trim_lock: {:?}, coarse_trim_bypass: {=bool:?}, lk: {:?}, fircvld: {:?}, fircsel: {=bool:?}, fircerr: {:?}, fircerr_ie: {:?}, fircacc_ie: {:?}, fircacc: {:?} }}",
            self.fircen(),
            self.fircsten(),
            self.firc_sclk_periph_en(),
            self.firc_fclk_periph_en(),
            self.firctren(),
            self.firctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.fircvld(),
            self.fircsel(),
            self.fircerr(),
            self.fircerr_ie(),
            self.fircacc_ie(),
            self.fircacc()
        )
    }
}
#[doc = "FIRC Auto-trimming Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircstat(pub u32);
impl Fircstat {
    #[doc = "Trim Fine."]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine."]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse."]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse."]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Fircstat {
    #[inline(always)]
    fn default() -> Fircstat {
        Fircstat(0)
    }
}
impl core::fmt::Debug for Fircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircstat")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircstat {{ trimfine: {=u8:?}, trimcoar: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar()
        )
    }
}
#[doc = "FIRC Trim Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctcfg(pub u32);
impl Firctcfg {
    #[doc = "Trim Source."]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> FirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        FirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source."]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: FirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FIRC Trim Predivider."]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "FIRC Trim Predivider."]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Firctcfg {
    #[inline(always)]
    fn default() -> Firctcfg {
        Firctcfg(0)
    }
}
impl core::fmt::Debug for Firctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "FIRC Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctest(pub u32);
impl Firctest {
    #[doc = "Test Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn testen(&self) -> Testen {
        let val = (self.0 >> 0usize) & 0x01;
        Testen::from_bits(val as u8)
    }
    #[doc = "Test Enable."]
    #[inline(always)]
    pub const fn set_testen(&mut self, val: Testen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Select."]
    #[must_use]
    #[inline(always)]
    pub const fn testsel(&self) -> Testsel {
        let val = (self.0 >> 3usize) & 0x03;
        Testsel::from_bits(val as u8)
    }
    #[doc = "Test Select."]
    #[inline(always)]
    pub const fn set_testsel(&mut self, val: Testsel) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Over Stress Test."]
    #[must_use]
    #[inline(always)]
    pub const fn overstress(&self) -> Overstress {
        let val = (self.0 >> 5usize) & 0x01;
        Overstress::from_bits(val as u8)
    }
    #[doc = "Over Stress Test."]
    #[inline(always)]
    pub const fn set_overstress(&mut self, val: Overstress) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Test Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn test_buf_en(&self) -> TestBufEn {
        let val = (self.0 >> 6usize) & 0x01;
        TestBufEn::from_bits(val as u8)
    }
    #[doc = "Test Buffer Enable."]
    #[inline(always)]
    pub const fn set_test_buf_en(&mut self, val: TestBufEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Test Buffer Flip."]
    #[must_use]
    #[inline(always)]
    pub const fn test_buf_flip(&self) -> TestBufFlip {
        let val = (self.0 >> 7usize) & 0x01;
        TestBufFlip::from_bits(val as u8)
    }
    #[doc = "Test Buffer Flip."]
    #[inline(always)]
    pub const fn set_test_buf_flip(&mut self, val: TestBufFlip) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Analog output clk_valid_lv."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_valid(&self) -> ClkValid {
        let val = (self.0 >> 16usize) & 0x01;
        ClkValid::from_bits(val as u8)
    }
    #[doc = "Analog output clk_valid_lv."]
    #[inline(always)]
    pub const fn set_clk_valid(&mut self, val: ClkValid) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Analog output start_valid_lv."]
    #[must_use]
    #[inline(always)]
    pub const fn start_valid(&self) -> StartValid {
        let val = (self.0 >> 17usize) & 0x01;
        StartValid::from_bits(val as u8)
    }
    #[doc = "Analog output start_valid_lv."]
    #[inline(always)]
    pub const fn set_start_valid(&mut self, val: StartValid) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for Firctest {
    #[inline(always)]
    fn default() -> Firctest {
        Firctest(0)
    }
}
impl core::fmt::Debug for Firctest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctest")
            .field("testen", &self.testen())
            .field("testsel", &self.testsel())
            .field("overstress", &self.overstress())
            .field("test_buf_en", &self.test_buf_en())
            .field("test_buf_flip", &self.test_buf_flip())
            .field("clk_valid", &self.clk_valid())
            .field("start_valid", &self.start_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctest {{ testen: {:?}, testsel: {:?}, overstress: {:?}, test_buf_en: {:?}, test_buf_flip: {:?}, clk_valid: {:?}, start_valid: {:?} }}",
            self.testen(),
            self.testsel(),
            self.overstress(),
            self.test_buf_en(),
            self.test_buf_flip(),
            self.clk_valid(),
            self.start_valid()
        )
    }
}
#[doc = "FIRC Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctrim(pub u32);
impl Firctrim {
    #[doc = "Trim Fine."]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine."]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse."]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse."]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temperature."]
    #[must_use]
    #[inline(always)]
    pub const fn trimtemp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Trim Temperature."]
    #[inline(always)]
    pub const fn set_trimtemp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Trim Start."]
    #[must_use]
    #[inline(always)]
    pub const fn trimstart(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Start."]
    #[inline(always)]
    pub const fn set_trimstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Firctrim {
    #[inline(always)]
    fn default() -> Firctrim {
        Firctrim(0)
    }
}
impl core::fmt::Debug for Firctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctrim")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .field("trimtemp", &self.trimtemp())
            .field("trimstart", &self.trimstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctrim {{ trimfine: {=u8:?}, trimcoar: {=u8:?}, trimtemp: {=u8:?}, trimstart: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar(),
            self.trimtemp(),
            self.trimstart()
        )
    }
}
#[doc = "LDO Control and Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldocsr(pub u32);
impl Ldocsr {
    #[doc = "LDO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ldoen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Enable."]
    #[inline(always)]
    pub const fn set_ldoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO output voltage select."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_sel(&self) -> VoutSel {
        let val = (self.0 >> 1usize) & 0x07;
        VoutSel::from_bits(val as u8)
    }
    #[doc = "LDO output voltage select."]
    #[inline(always)]
    pub const fn set_vout_sel(&mut self, val: VoutSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "LDO Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn ldobypass(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Bypass."]
    #[inline(always)]
    pub const fn set_ldobypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LDO VOUT OK Inform."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_ok(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LDO VOUT OK Inform."]
    #[inline(always)]
    pub const fn set_vout_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldocsr {
    #[inline(always)]
    fn default() -> Ldocsr {
        Ldocsr(0)
    }
}
impl core::fmt::Debug for Ldocsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldocsr")
            .field("ldoen", &self.ldoen())
            .field("vout_sel", &self.vout_sel())
            .field("ldobypass", &self.ldobypass())
            .field("vout_ok", &self.vout_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldocsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldocsr {{ ldoen: {=bool:?}, vout_sel: {:?}, ldobypass: {=bool:?}, vout_ok: {=bool:?} }}",
            self.ldoen(),
            self.vout_sel(),
            self.ldobypass(),
            self.vout_ok()
        )
    }
}
#[doc = "LDO Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotest(pub u32);
impl Ldotest {
    #[doc = "Select analog test bus."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_atx(&self) -> SelAtx {
        let val = (self.0 >> 0usize) & 0x03;
        SelAtx::from_bits(val as u8)
    }
    #[doc = "Select analog test bus."]
    #[inline(always)]
    pub const fn set_sel_atx(&mut self, val: SelAtx) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Ldotest {
    #[inline(always)]
    fn default() -> Ldotest {
        Ldotest(0)
    }
}
impl core::fmt::Debug for Ldotest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotest")
            .field("sel_atx", &self.sel_atx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldotest {{ sel_atx: {:?} }}", self.sel_atx())
    }
}
#[doc = "Parameter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "SOSC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn soscclkpres(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Present."]
    #[inline(always)]
    pub const fn set_soscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn sircclkpres(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock Present."]
    #[inline(always)]
    pub const fn set_sircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIRC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn fircclkpres(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Clock Present."]
    #[inline(always)]
    pub const fn set_fircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ROSC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn roscclkpres(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Clock Present."]
    #[inline(always)]
    pub const fn set_roscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "APLL Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn apllclkpres(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "APLL Clock Present."]
    #[inline(always)]
    pub const fn set_apllclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SPLL Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn spllclkpres(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Present."]
    #[inline(always)]
    pub const fn set_spllclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "UPLL Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn upllclkpres(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "UPLL Clock Present."]
    #[inline(always)]
    pub const fn set_upllclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("soscclkpres", &self.soscclkpres())
            .field("sircclkpres", &self.sircclkpres())
            .field("fircclkpres", &self.fircclkpres())
            .field("roscclkpres", &self.roscclkpres())
            .field("apllclkpres", &self.apllclkpres())
            .field("spllclkpres", &self.spllclkpres())
            .field("upllclkpres", &self.upllclkpres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ soscclkpres: {=bool:?}, sircclkpres: {=bool:?}, fircclkpres: {=bool:?}, roscclkpres: {=bool:?}, apllclkpres: {=bool:?}, spllclkpres: {=bool:?}, upllclkpres: {=bool:?} }}",
            self.soscclkpres(),
            self.sircclkpres(),
            self.fircclkpres(),
            self.roscclkpres(),
            self.apllclkpres(),
            self.spllclkpres(),
            self.upllclkpres()
        )
    }
}
#[doc = "Run Clock Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rccr(pub u32);
impl Rccr {
    #[doc = "System Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> Scs {
        let val = (self.0 >> 24usize) & 0x0f;
        Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: Scs) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Rccr {
    #[inline(always)]
    fn default() -> Rccr {
        Rccr(0)
    }
}
impl core::fmt::Debug for Rccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rccr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rccr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "ROSC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rosccsr(pub u32);
impl Rosccsr {
    #[doc = "ROSC Clock Monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn rosccm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Clock Monitor."]
    #[inline(always)]
    pub const fn set_rosccm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ROSC Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rosccmre(&self) -> Rosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        Rosccmre::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_rosccmre(&mut self, val: Rosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> RosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        RosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: RosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "ROSC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn roscvld(&self) -> Roscvld {
        let val = (self.0 >> 24usize) & 0x01;
        Roscvld::from_bits(val as u8)
    }
    #[doc = "ROSC Valid."]
    #[inline(always)]
    pub const fn set_roscvld(&mut self, val: Roscvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "ROSC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn roscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Selected."]
    #[inline(always)]
    pub const fn set_roscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ROSC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn roscerr(&self) -> Roscerr {
        let val = (self.0 >> 26usize) & 0x01;
        Roscerr::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Error."]
    #[inline(always)]
    pub const fn set_roscerr(&mut self, val: Roscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Rosccsr {
    #[inline(always)]
    fn default() -> Rosccsr {
        Rosccsr(0)
    }
}
impl core::fmt::Debug for Rosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rosccsr")
            .field("rosccm", &self.rosccm())
            .field("rosccmre", &self.rosccmre())
            .field("lk", &self.lk())
            .field("roscvld", &self.roscvld())
            .field("roscsel", &self.roscsel())
            .field("roscerr", &self.roscerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rosccsr {{ rosccm: {=bool:?}, rosccmre: {:?}, lk: {:?}, roscvld: {:?}, roscsel: {=bool:?}, roscerr: {:?} }}",
            self.rosccm(),
            self.rosccmre(),
            self.lk(),
            self.roscvld(),
            self.roscsel(),
            self.roscerr()
        )
    }
}
#[doc = "SIRC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirccsr(pub u32);
impl Sirccsr {
    #[doc = "SIRC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sircsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Stop Enable."]
    #[inline(always)]
    pub const fn set_sircsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock to Peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sirc_clk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock to Peripherals Enable."]
    #[inline(always)]
    pub const fn set_sirc_clk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)."]
    #[must_use]
    #[inline(always)]
    pub const fn sirctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)."]
    #[inline(always)]
    pub const fn set_sirctren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SIRC Trim Update."]
    #[must_use]
    #[inline(always)]
    pub const fn sirctrup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Trim Update."]
    #[inline(always)]
    pub const fn set_sirctrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SIRC TRIM LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> SirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        SirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "SIRC TRIM LOCK."]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: SirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> SirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        SirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: SirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SIRC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn sircvld(&self) -> Sircvld {
        let val = (self.0 >> 24usize) & 0x01;
        Sircvld::from_bits(val as u8)
    }
    #[doc = "SIRC Valid."]
    #[inline(always)]
    pub const fn set_sircvld(&mut self, val: Sircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SIRC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn sircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Selected."]
    #[inline(always)]
    pub const fn set_sircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SIRC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr(&self) -> Sircerr {
        let val = (self.0 >> 26usize) & 0x01;
        Sircerr::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error."]
    #[inline(always)]
    pub const fn set_sircerr(&mut self, val: Sircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SIRC Clock Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr_ie(&self) -> SircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        SircerrIe::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sircerr_ie(&mut self, val: SircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Sirccsr {
    #[inline(always)]
    fn default() -> Sirccsr {
        Sirccsr(0)
    }
}
impl core::fmt::Debug for Sirccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirccsr")
            .field("sircsten", &self.sircsten())
            .field("sirc_clk_periph_en", &self.sirc_clk_periph_en())
            .field("sirctren", &self.sirctren())
            .field("sirctrup", &self.sirctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("sircvld", &self.sircvld())
            .field("sircsel", &self.sircsel())
            .field("sircerr", &self.sircerr())
            .field("sircerr_ie", &self.sircerr_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirccsr {{ sircsten: {=bool:?}, sirc_clk_periph_en: {=bool:?}, sirctren: {=bool:?}, sirctrup: {=bool:?}, trim_lock: {:?}, coarse_trim_bypass: {=bool:?}, lk: {:?}, sircvld: {:?}, sircsel: {=bool:?}, sircerr: {:?}, sircerr_ie: {:?} }}",
            self.sircsten(),
            self.sirc_clk_periph_en(),
            self.sirctren(),
            self.sirctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.sircvld(),
            self.sircsel(),
            self.sircerr(),
            self.sircerr_ie()
        )
    }
}
#[doc = "SIRC Auto-trimming Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sircstat(pub u32);
impl Sircstat {
    #[doc = "CCO Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim."]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim."]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Sircstat {
    #[inline(always)]
    fn default() -> Sircstat {
        Sircstat(0)
    }
}
impl core::fmt::Debug for Sircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sircstat")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sircstat {{ ccotrim: {=u8:?}, cltrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim()
        )
    }
}
#[doc = "SIRC Trim Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctcfg(pub u32);
impl Sirctcfg {
    #[doc = "Trim Source."]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> SirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        SirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source."]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: SirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SIRC Trim Predivider."]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "SIRC Trim Predivider."]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Sirctcfg {
    #[inline(always)]
    fn default() -> Sirctcfg {
        Sirctcfg(0)
    }
}
impl core::fmt::Debug for Sirctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "SIRC Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctest(pub u32);
impl Sirctest {
    #[doc = "SIRC ATX Test Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC ATX Test Enable."]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SIRC Div-by-16 Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn div16en(&self) -> Div16en {
        let val = (self.0 >> 4usize) & 0x01;
        Div16en::from_bits(val as u8)
    }
    #[doc = "SIRC Div-by-16 Output Enable."]
    #[inline(always)]
    pub const fn set_div16en(&mut self, val: Div16en) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SIRC Flip Buffer Inputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn flip_buf_in(&self) -> FlipBufIn {
        let val = (self.0 >> 5usize) & 0x01;
        FlipBufIn::from_bits(val as u8)
    }
    #[doc = "SIRC Flip Buffer Inputs Enable."]
    #[inline(always)]
    pub const fn set_flip_buf_in(&mut self, val: FlipBufIn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SIRC Buffer Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn test_buf_bypass(&self) -> TestBufBypass {
        let val = (self.0 >> 6usize) & 0x01;
        TestBufBypass::from_bits(val as u8)
    }
    #[doc = "SIRC Buffer Bypass."]
    #[inline(always)]
    pub const fn set_test_buf_bypass(&mut self, val: TestBufBypass) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SIRC Test Select."]
    #[must_use]
    #[inline(always)]
    pub const fn test_sel(&self) -> TestSel {
        let val = (self.0 >> 8usize) & 0x03;
        TestSel::from_bits(val as u8)
    }
    #[doc = "SIRC Test Select."]
    #[inline(always)]
    pub const fn set_test_sel(&mut self, val: TestSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Sirctest {
    #[inline(always)]
    fn default() -> Sirctest {
        Sirctest(0)
    }
}
impl core::fmt::Debug for Sirctest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctest")
            .field("test", &self.test())
            .field("div16en", &self.div16en())
            .field("flip_buf_in", &self.flip_buf_in())
            .field("test_buf_bypass", &self.test_buf_bypass())
            .field("test_sel", &self.test_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctest {{ test: {=bool:?}, div16en: {:?}, flip_buf_in: {:?}, test_buf_bypass: {:?}, test_sel: {:?} }}",
            self.test(),
            self.div16en(),
            self.flip_buf_in(),
            self.test_buf_bypass(),
            self.test_sel()
        )
    }
}
#[doc = "SIRC Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctrim(pub u32);
impl Sirctrim {
    #[doc = "CCO Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim."]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim."]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temp."]
    #[must_use]
    #[inline(always)]
    pub const fn tctrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim Temp."]
    #[inline(always)]
    pub const fn set_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period."]
    #[must_use]
    #[inline(always)]
    pub const fn fvchtrim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period."]
    #[inline(always)]
    pub const fn set_fvchtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sirctrim {
    #[inline(always)]
    fn default() -> Sirctrim {
        Sirctrim(0)
    }
}
impl core::fmt::Debug for Sirctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctrim")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .field("tctrim", &self.tctrim())
            .field("fvchtrim", &self.fvchtrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctrim {{ ccotrim: {=u8:?}, cltrim: {=u8:?}, tctrim: {=u8:?}, fvchtrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim(),
            self.tctrim(),
            self.fvchtrim()
        )
    }
}
#[doc = "SOSC Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccfg(pub u32);
impl Sosccfg {
    #[doc = "External Reference Select(connect to analog pin ~en_byp_lv)."]
    #[must_use]
    #[inline(always)]
    pub const fn erefs(&self) -> Erefs {
        let val = (self.0 >> 2usize) & 0x01;
        Erefs::from_bits(val as u8)
    }
    #[doc = "External Reference Select(connect to analog pin ~en_byp_lv)."]
    #[inline(always)]
    pub const fn set_erefs(&mut self, val: Erefs) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SOSC Range Select(connect to analog pin gm_lv\\[1:0\\])."]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> Range {
        let val = (self.0 >> 4usize) & 0x03;
        Range::from_bits(val as u8)
    }
    #[doc = "SOSC Range Select(connect to analog pin gm_lv\\[1:0\\])."]
    #[inline(always)]
    pub const fn set_range(&mut self, val: Range) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sosccfg {
    #[inline(always)]
    fn default() -> Sosccfg {
        Sosccfg(0)
    }
}
impl core::fmt::Debug for Sosccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccfg")
            .field("erefs", &self.erefs())
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccfg {{ erefs: {:?}, range: {:?} }}",
            self.erefs(),
            self.range()
        )
    }
}
#[doc = "SOSC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccsr(pub u32);
impl Sosccsr {
    #[doc = "SOSC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Enable."]
    #[inline(always)]
    pub const fn set_soscen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SOSC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Stop Enable."]
    #[inline(always)]
    pub const fn set_soscsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SOSC Clock Monitor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosccm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Monitor Enable."]
    #[inline(always)]
    pub const fn set_sosccm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SOSC Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosccmre(&self) -> Sosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        Sosccmre::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_sosccmre(&mut self, val: Sosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> SosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        SosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: SosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SOSC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid."]
    #[inline(always)]
    pub const fn set_soscvld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SOSC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn soscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Selected."]
    #[inline(always)]
    pub const fn set_soscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SOSC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soscerr(&self) -> Soscerr {
        let val = (self.0 >> 26usize) & 0x01;
        Soscerr::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Error."]
    #[inline(always)]
    pub const fn set_soscerr(&mut self, val: Soscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SOSC Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_soscvld_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Sosccsr {
    #[inline(always)]
    fn default() -> Sosccsr {
        Sosccsr(0)
    }
}
impl core::fmt::Debug for Sosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccsr")
            .field("soscen", &self.soscen())
            .field("soscsten", &self.soscsten())
            .field("sosccm", &self.sosccm())
            .field("sosccmre", &self.sosccmre())
            .field("lk", &self.lk())
            .field("soscvld", &self.soscvld())
            .field("soscsel", &self.soscsel())
            .field("soscerr", &self.soscerr())
            .field("soscvld_ie", &self.soscvld_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccsr {{ soscen: {=bool:?}, soscsten: {=bool:?}, sosccm: {=bool:?}, sosccmre: {:?}, lk: {:?}, soscvld: {=bool:?}, soscsel: {=bool:?}, soscerr: {:?}, soscvld_ie: {=bool:?} }}",
            self.soscen(),
            self.soscsten(),
            self.sosccm(),
            self.sosccmre(),
            self.lk(),
            self.soscvld(),
            self.soscsel(),
            self.soscerr(),
            self.soscvld_ie()
        )
    }
}
#[doc = "SOSC Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosctest(pub u32);
impl Sosctest {
    #[doc = "SOSC XTM Test Mode Enable (connect to analog pin xtm_lv)."]
    #[must_use]
    #[inline(always)]
    pub const fn sosc_xtm_test_mode_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC XTM Test Mode Enable (connect to analog pin xtm_lv)."]
    #[inline(always)]
    pub const fn set_sosc_xtm_test_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC Test Select (connect to analog pin tst_md_lv\\[1:0\\])."]
    #[must_use]
    #[inline(always)]
    pub const fn tstmd(&self) -> Tstmd {
        let val = (self.0 >> 8usize) & 0x03;
        Tstmd::from_bits(val as u8)
    }
    #[doc = "OSC Test Select (connect to analog pin tst_md_lv\\[1:0\\])."]
    #[inline(always)]
    pub const fn set_tstmd(&mut self, val: Tstmd) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "The analog output xo_ok_lv."]
    #[must_use]
    #[inline(always)]
    pub const fn xo_ok_lv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "The analog output xo_ok_lv."]
    #[inline(always)]
    pub const fn set_xo_ok_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Sosctest {
    #[inline(always)]
    fn default() -> Sosctest {
        Sosctest(0)
    }
}
impl core::fmt::Debug for Sosctest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosctest")
            .field("sosc_xtm_test_mode_en", &self.sosc_xtm_test_mode_en())
            .field("tstmd", &self.tstmd())
            .field("xo_ok_lv", &self.xo_ok_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosctest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosctest {{ sosc_xtm_test_mode_en: {=bool:?}, tstmd: {:?}, xo_ok_lv: {=bool:?} }}",
            self.sosc_xtm_test_mode_en(),
            self.tstmd(),
            self.xo_ok_lv()
        )
    }
}
#[doc = "SPLL Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllcsr(pub u32);
impl Spllcsr {
    #[doc = "SPLL Power Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllpwren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Power Enable."]
    #[inline(always)]
    pub const fn set_spllpwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPLL Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllclken(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Enable."]
    #[inline(always)]
    pub const fn set_spllclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPLL Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllsten(&self) -> Spllsten {
        let val = (self.0 >> 2usize) & 0x01;
        Spllsten::from_bits(val as u8)
    }
    #[doc = "SPLL Stop Enable."]
    #[inline(always)]
    pub const fn set_spllsten(&mut self, val: Spllsten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Free running mode clock stable."]
    #[must_use]
    #[inline(always)]
    pub const fn frm_clockstable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Free running mode clock stable."]
    #[inline(always)]
    pub const fn set_frm_clockstable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SPLL Clock Monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn spllcm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Monitor."]
    #[inline(always)]
    pub const fn set_spllcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SPLL Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllcmre(&self) -> Spllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        Spllcmre::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_spllcmre(&mut self, val: Spllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> SpllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        SpllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: SpllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SPLL LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock(&self) -> SpllLock {
        let val = (self.0 >> 24usize) & 0x01;
        SpllLock::from_bits(val as u8)
    }
    #[doc = "SPLL LOCK."]
    #[inline(always)]
    pub const fn set_spll_lock(&mut self, val: SpllLock) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SPLL Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn spllsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Selected."]
    #[inline(always)]
    pub const fn set_spllsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SPLL Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn spllerr(&self) -> Spllerr {
        let val = (self.0 >> 26usize) & 0x01;
        Spllerr::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Error."]
    #[inline(always)]
    pub const fn set_spllerr(&mut self, val: Spllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SPLL LOCK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL LOCK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_spll_lock_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Spllcsr {
    #[inline(always)]
    fn default() -> Spllcsr {
        Spllcsr(0)
    }
}
impl core::fmt::Debug for Spllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllcsr")
            .field("spllpwren", &self.spllpwren())
            .field("spllclken", &self.spllclken())
            .field("spllsten", &self.spllsten())
            .field("frm_clockstable", &self.frm_clockstable())
            .field("spllcm", &self.spllcm())
            .field("spllcmre", &self.spllcmre())
            .field("lk", &self.lk())
            .field("spll_lock", &self.spll_lock())
            .field("spllsel", &self.spllsel())
            .field("spllerr", &self.spllerr())
            .field("spll_lock_ie", &self.spll_lock_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllcsr {{ spllpwren: {=bool:?}, spllclken: {=bool:?}, spllsten: {:?}, frm_clockstable: {=bool:?}, spllcm: {=bool:?}, spllcmre: {:?}, lk: {:?}, spll_lock: {:?}, spllsel: {=bool:?}, spllerr: {:?}, spll_lock_ie: {=bool:?} }}",
            self.spllpwren(),
            self.spllclken(),
            self.spllsten(),
            self.frm_clockstable(),
            self.spllcm(),
            self.spllcmre(),
            self.lk(),
            self.spll_lock(),
            self.spllsel(),
            self.spllerr(),
            self.spll_lock_ie()
        )
    }
}
#[doc = "SPLL Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllctrl(pub u32);
impl Spllctrl {
    #[doc = "Bandwidth select R (resistor) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R (resistor) value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I (integration) value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass of Divide-by-2 Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of Divide-by-2 Divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Up Limiter."]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Up Limiter."]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn banddirect(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_banddirect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bypass of the predivider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the predivider."]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bypass of the postdivider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the postdivider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Free Running Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Free Running Mode Enable."]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Skew mode."]
    #[must_use]
    #[inline(always)]
    pub const fn skew_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Skew mode."]
    #[inline(always)]
    pub const fn set_skew_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> Source {
        let val = (self.0 >> 25usize) & 0x03;
        Source::from_bits(val as u8)
    }
    #[doc = "Clock Source."]
    #[inline(always)]
    pub const fn set_source(&mut self, val: Source) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for Spllctrl {
    #[inline(always)]
    fn default() -> Spllctrl {
        Spllctrl(0)
    }
}
impl core::fmt::Debug for Spllctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("banddirect", &self.banddirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("frm", &self.frm())
            .field("skew_en", &self.skew_en())
            .field("source", &self.source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspostdiv2: {=bool:?}, limupoff: {=bool:?}, banddirect: {=bool:?}, bypassprediv: {=bool:?}, bypasspostdiv: {=bool:?}, frm: {=bool:?}, skew_en: {=bool:?}, source: {:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.banddirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.frm(),
            self.skew_en(),
            self.source()
        )
    }
}
#[doc = "SPLL LOCK Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SplllockCnfg(pub u32);
impl SplllockCnfg {
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[inline(always)]
    pub const fn set_lock_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for SplllockCnfg {
    #[inline(always)]
    fn default() -> SplllockCnfg {
        SplllockCnfg(0)
    }
}
impl core::fmt::Debug for SplllockCnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SplllockCnfg")
            .field("lock_time", &self.lock_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SplllockCnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SplllockCnfg {{ lock_time: {=u32:?} }}",
            self.lock_time()
        )
    }
}
#[doc = "SPLL M Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllmdiv(pub u32);
impl Spllmdiv {
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_mdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback ratio change request."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllmdiv {
    #[inline(always)]
    fn default() -> Spllmdiv {
        Spllmdiv(0)
    }
}
impl core::fmt::Debug for Spllmdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllmdiv")
            .field("mdiv", &self.mdiv())
            .field("mreq", &self.mreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllmdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllmdiv {{ mdiv: {=u16:?}, mreq: {=bool:?} }}",
            self.mdiv(),
            self.mreq()
        )
    }
}
#[doc = "SPLL N Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllndiv(pub u32);
impl Spllndiv {
    #[doc = "Predivider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Predivider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Predivider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllndiv {
    #[inline(always)]
    fn default() -> Spllndiv {
        Spllndiv(0)
    }
}
impl core::fmt::Debug for Spllndiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllndiv")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllndiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllndiv {{ ndiv: {=u8:?}, nreq: {=bool:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "SPLL P Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllpdiv(pub u32);
impl Spllpdiv {
    #[doc = "Postdivider divider ratio (P-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Postdivider divider ratio (P-divider)."]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Postdivider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Postdivider ratio change request."]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllpdiv {
    #[inline(always)]
    fn default() -> Spllpdiv {
        Spllpdiv(0)
    }
}
impl core::fmt::Debug for Spllpdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllpdiv")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllpdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllpdiv {{ pdiv: {=u8:?}, preq: {=bool:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg0(pub u32);
impl Spllsscg0 {
    #[doc = "SS_MDIV\\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_lsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SS_MDIV\\[31:0\\]."]
    #[inline(always)]
    pub const fn set_ss_mdiv_lsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Spllsscg0 {
    #[inline(always)]
    fn default() -> Spllsscg0 {
        Spllsscg0(0)
    }
}
impl core::fmt::Debug for Spllsscg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg0")
            .field("ss_mdiv_lsb", &self.ss_mdiv_lsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg0 {{ ss_mdiv_lsb: {=u32:?} }}",
            self.ss_mdiv_lsb()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg1(pub u32);
impl Spllsscg1 {
    #[doc = "SS_MDIV\\[32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32\\]."]
    #[inline(always)]
    pub const fn set_ss_mdiv_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[inline(always)]
    pub const fn set_ss_mdiv_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Modulation Frequency Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Frequency Control."]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Modulation Depth Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Depth Control."]
    #[inline(always)]
    pub const fn set_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Modulation Waveform Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> Spllsscg1Mc {
        let val = (self.0 >> 8usize) & 0x03;
        Spllsscg1Mc::from_bits(val as u8)
    }
    #[doc = "Modulation Waveform Control."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: Spllsscg1Mc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Dither Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Dither Enable."]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SS_MDIV select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ss_mdiv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV select."]
    #[inline(always)]
    pub const fn set_sel_ss_mdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SSCG Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_pd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SSCG Power Down."]
    #[inline(always)]
    pub const fn set_ss_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllsscg1 {
    #[inline(always)]
    fn default() -> Spllsscg1 {
        Spllsscg1(0)
    }
}
impl core::fmt::Debug for Spllsscg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg1")
            .field("ss_mdiv_msb", &self.ss_mdiv_msb())
            .field("ss_mdiv_req", &self.ss_mdiv_req())
            .field("mf", &self.mf())
            .field("mr", &self.mr())
            .field("mc", &self.mc())
            .field("dither", &self.dither())
            .field("sel_ss_mdiv", &self.sel_ss_mdiv())
            .field("ss_pd", &self.ss_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg1 {{ ss_mdiv_msb: {=bool:?}, ss_mdiv_req: {=bool:?}, mf: {=u8:?}, mr: {=u8:?}, mc: {:?}, dither: {=bool:?}, sel_ss_mdiv: {=bool:?}, ss_pd: {=bool:?} }}",
            self.ss_mdiv_msb(),
            self.ss_mdiv_req(),
            self.mf(),
            self.mr(),
            self.mc(),
            self.dither(),
            self.sel_ss_mdiv(),
            self.ss_pd()
        )
    }
}
#[doc = "SPLL SSCG Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscgstat(pub u32);
impl Spllsscgstat {
    #[doc = "SS_MDIV change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV change acknowledge."]
    #[inline(always)]
    pub const fn set_ss_mdiv_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Spllsscgstat {
    #[inline(always)]
    fn default() -> Spllsscgstat {
        Spllsscgstat(0)
    }
}
impl core::fmt::Debug for Spllsscgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscgstat")
            .field("ss_mdiv_ack", &self.ss_mdiv_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscgstat {{ ss_mdiv_ack: {=bool:?} }}",
            self.ss_mdiv_ack()
        )
    }
}
#[doc = "SPLL Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllstat(pub u32);
impl Spllstat {
    #[doc = "Lock detector output (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock detector output (active high)."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Predivider (N) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ndivack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Predivider (N) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_ndivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Feedback (M) divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn mdivack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback (M) divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_mdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Postdivider (P) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn pdivack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Postdivider (P) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_pdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Free running detector (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn frmdet(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Free running detector (active high)."]
    #[inline(always)]
    pub const fn set_frmdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Spllstat {
    #[inline(always)]
    fn default() -> Spllstat {
        Spllstat(0)
    }
}
impl core::fmt::Debug for Spllstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllstat")
            .field("lock", &self.lock())
            .field("ndivack", &self.ndivack())
            .field("mdivack", &self.mdivack())
            .field("pdivack", &self.pdivack())
            .field("frmdet", &self.frmdet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllstat {{ lock: {=bool:?}, ndivack: {=bool:?}, mdivack: {=bool:?}, pdivack: {=bool:?}, frmdet: {=bool:?} }}",
            self.lock(),
            self.ndivack(),
            self.mdivack(),
            self.pdivack(),
            self.frmdet()
        )
    }
}
#[doc = "SPLL Test Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Splltestctrl(pub u32);
impl Splltestctrl {
    #[doc = "Input to functional test the predivider (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn nfunctest(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Input to functional test the predivider (N-divider)."]
    #[inline(always)]
    pub const fn set_nfunctest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Input to functional test the feedback divider (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mfunctest(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Input to functional test the feedback divider (M-divider)."]
    #[inline(always)]
    pub const fn set_mfunctest(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "Input to functional test the postdivider (P-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn pfunctest(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "Input to functional test the postdivider (P-divider)."]
    #[inline(always)]
    pub const fn set_pfunctest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[doc = "Enable test mux."]
    #[must_use]
    #[inline(always)]
    pub const fn testv_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable test mux."]
    #[inline(always)]
    pub const fn set_testv_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Select analog signal channel that need to be test."]
    #[must_use]
    #[inline(always)]
    pub const fn testv_sel(&self) -> SplltestctrlTestvSel {
        let val = (self.0 >> 27usize) & 0x07;
        SplltestctrlTestvSel::from_bits(val as u8)
    }
    #[doc = "Select analog signal channel that need to be test."]
    #[inline(always)]
    pub const fn set_testv_sel(&mut self, val: SplltestctrlTestvSel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Bypass PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspll(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass PLL."]
    #[inline(always)]
    pub const fn set_bypasspll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Splltestctrl {
    #[inline(always)]
    fn default() -> Splltestctrl {
        Splltestctrl(0)
    }
}
impl core::fmt::Debug for Splltestctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Splltestctrl")
            .field("nfunctest", &self.nfunctest())
            .field("mfunctest", &self.mfunctest())
            .field("pfunctest", &self.pfunctest())
            .field("testv_en", &self.testv_en())
            .field("testv_sel", &self.testv_sel())
            .field("bypasspll", &self.bypasspll())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Splltestctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Splltestctrl {{ nfunctest: {=u8:?}, mfunctest: {=u16:?}, pfunctest: {=u8:?}, testv_en: {=bool:?}, testv_sel: {:?}, bypasspll: {=bool:?} }}",
            self.nfunctest(),
            self.mfunctest(),
            self.pfunctest(),
            self.testv_en(),
            self.testv_sel(),
            self.bypasspll()
        )
    }
}
#[doc = "SPLL Test Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllteststat(pub u32);
impl Spllteststat {
    #[doc = "Output to observe the functional predivider test."]
    #[must_use]
    #[inline(always)]
    pub const fn nmotest(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Output to observe the functional predivider test."]
    #[inline(always)]
    pub const fn set_nmotest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output to observe the functional feedback divider test."]
    #[must_use]
    #[inline(always)]
    pub const fn mmotest(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Output to observe the functional feedback divider test."]
    #[inline(always)]
    pub const fn set_mmotest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Output to observe the functional postdivider test."]
    #[must_use]
    #[inline(always)]
    pub const fn pmotest(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Output to observe the functional postdivider test."]
    #[inline(always)]
    pub const fn set_pmotest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "PLL functional divider test clock pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_xfunc_test_pulse(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "PLL functional divider test clock pulse."]
    #[inline(always)]
    pub const fn set_pll_xfunc_test_pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable PLL functional divider test clock pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_xfunc_test_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL functional divider test clock pulse."]
    #[inline(always)]
    pub const fn set_pll_xfunc_test_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable to observe the xMOTEST flags."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_xmo_test_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable to observe the xMOTEST flags."]
    #[inline(always)]
    pub const fn set_pll_xmo_test_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllteststat {
    #[inline(always)]
    fn default() -> Spllteststat {
        Spllteststat(0)
    }
}
impl core::fmt::Debug for Spllteststat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllteststat")
            .field("nmotest", &self.nmotest())
            .field("mmotest", &self.mmotest())
            .field("pmotest", &self.pmotest())
            .field("pll_xfunc_test_pulse", &self.pll_xfunc_test_pulse())
            .field("pll_xfunc_test_en", &self.pll_xfunc_test_en())
            .field("pll_xmo_test_en", &self.pll_xmo_test_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllteststat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllteststat {{ nmotest: {=u8:?}, mmotest: {=u8:?}, pmotest: {=u8:?}, pll_xfunc_test_pulse: {=bool:?}, pll_xfunc_test_en: {=bool:?}, pll_xmo_test_en: {=bool:?} }}",
            self.nmotest(),
            self.mmotest(),
            self.pmotest(),
            self.pll_xfunc_test_pulse(),
            self.pll_xfunc_test_en(),
            self.pll_xmo_test_en()
        )
    }
}
#[doc = "Trim Lock register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimLock(pub u32);
impl TrimLock {
    #[doc = "TRIM_UNLOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_unlock(&self) -> TrimUnlock {
        let val = (self.0 >> 0usize) & 0x01;
        TrimUnlock::from_bits(val as u8)
    }
    #[doc = "TRIM_UNLOCK."]
    #[inline(always)]
    pub const fn set_trim_unlock(&mut self, val: TrimUnlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IFR_DISABLE."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_disable(&self) -> IfrDisable {
        let val = (self.0 >> 1usize) & 0x01;
        IfrDisable::from_bits(val as u8)
    }
    #[doc = "IFR_DISABLE."]
    #[inline(always)]
    pub const fn set_ifr_disable(&mut self, val: IfrDisable) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIM_LOCK_KEY."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock_key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIM_LOCK_KEY."]
    #[inline(always)]
    pub const fn set_trim_lock_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TrimLock {
    #[inline(always)]
    fn default() -> TrimLock {
        TrimLock(0)
    }
}
impl core::fmt::Debug for TrimLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimLock")
            .field("trim_unlock", &self.trim_unlock())
            .field("ifr_disable", &self.ifr_disable())
            .field("trim_lock_key", &self.trim_lock_key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimLock {{ trim_unlock: {:?}, ifr_disable: {:?}, trim_lock_key: {=u16:?} }}",
            self.trim_unlock(),
            self.ifr_disable(),
            self.trim_lock_key()
        )
    }
}
#[doc = "UPLL Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Upllcsr(pub u32);
impl Upllcsr {
    #[doc = "UPLL Clock Monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn upllcm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "UPLL Clock Monitor."]
    #[inline(always)]
    pub const fn set_upllcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "UPLL Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn upllcmre(&self) -> Upllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        Upllcmre::from_bits(val as u8)
    }
    #[doc = "UPLL Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_upllcmre(&mut self, val: Upllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> UpllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        UpllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: UpllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "UPLL Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn upllvld(&self) -> Upllvld {
        let val = (self.0 >> 24usize) & 0x01;
        Upllvld::from_bits(val as u8)
    }
    #[doc = "UPLL Valid."]
    #[inline(always)]
    pub const fn set_upllvld(&mut self, val: Upllvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "UPLL Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn upllsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "UPLL Selected."]
    #[inline(always)]
    pub const fn set_upllsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "UPLL Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn upllerr(&self) -> Upllerr {
        let val = (self.0 >> 26usize) & 0x01;
        Upllerr::from_bits(val as u8)
    }
    #[doc = "UPLL Clock Error."]
    #[inline(always)]
    pub const fn set_upllerr(&mut self, val: Upllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Upllcsr {
    #[inline(always)]
    fn default() -> Upllcsr {
        Upllcsr(0)
    }
}
impl core::fmt::Debug for Upllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Upllcsr")
            .field("upllcm", &self.upllcm())
            .field("upllcmre", &self.upllcmre())
            .field("lk", &self.lk())
            .field("upllvld", &self.upllvld())
            .field("upllsel", &self.upllsel())
            .field("upllerr", &self.upllerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Upllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Upllcsr {{ upllcm: {=bool:?}, upllcmre: {:?}, lk: {:?}, upllvld: {:?}, upllsel: {=bool:?}, upllerr: {:?} }}",
            self.upllcm(),
            self.upllcmre(),
            self.lk(),
            self.upllvld(),
            self.upllsel(),
            self.upllerr()
        )
    }
}
#[doc = "Version ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "SCG Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SCG Version Number."]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .field("version", &self.version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Verid {{ version: {=u32:?} }}", self.version())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllLock {
    #[doc = "APLL is not powered on or not locked."]
    DisabledOrNotValid = 0x0,
    #[doc = "APLL is locked."]
    EnabledAndValid = 0x01,
}
impl ApllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllLock {
    #[inline(always)]
    fn from(val: u8) -> ApllLock {
        ApllLock::from_bits(val)
    }
}
impl From<ApllLock> for u8 {
    #[inline(always)]
    fn from(val: ApllLock) -> u8 {
        ApllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllcmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GenerateInterrupt = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GenerateReset = 0x01,
}
impl Apllcmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllcmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllcmre {
    #[inline(always)]
    fn from(val: u8) -> Apllcmre {
        Apllcmre::from_bits(val)
    }
}
impl From<Apllcmre> for u8 {
    #[inline(always)]
    fn from(val: Apllcmre) -> u8 {
        Apllcmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllcsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
}
impl ApllcsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllcsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllcsrLk {
    #[inline(always)]
    fn from(val: u8) -> ApllcsrLk {
        ApllcsrLk::from_bits(val)
    }
}
impl From<ApllcsrLk> for u8 {
    #[inline(always)]
    fn from(val: ApllcsrLk) -> u8 {
        ApllcsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApllctrlSource {
    #[doc = "SOSC."]
    Sosc = 0x0,
    #[doc = "FIRC 48 MHz clock. FIRC_SCLK_PERIPH_EN must be set to use FIRC 48 MHz clock."]
    Firc = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "No clock."]
    Rsvd = 0x03,
}
impl ApllctrlSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApllctrlSource {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApllctrlSource {
    #[inline(always)]
    fn from(val: u8) -> ApllctrlSource {
        ApllctrlSource::from_bits(val)
    }
}
impl From<ApllctrlSource> for u8 {
    #[inline(always)]
    fn from(val: ApllctrlSource) -> u8 {
        ApllctrlSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllerr {
    #[doc = "APLL Clock Monitor is disabled or has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "APLL Clock Monitor is enabled and detected an error."]
    EnabledAndError = 0x01,
}
impl Apllerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllerr {
    #[inline(always)]
    fn from(val: u8) -> Apllerr {
        Apllerr::from_bits(val)
    }
}
impl From<Apllerr> for u8 {
    #[inline(always)]
    fn from(val: Apllerr) -> u8 {
        Apllerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsscg1Mc {
    #[doc = "MC\\[1:0\\] no compensation."]
    NoComp = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "MC\\[1:0\\] maximum compensation."]
    MaxComp = 0x03,
}
impl Apllsscg1Mc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsscg1Mc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsscg1Mc {
    #[inline(always)]
    fn from(val: u8) -> Apllsscg1Mc {
        Apllsscg1Mc::from_bits(val)
    }
}
impl From<Apllsscg1Mc> for u8 {
    #[inline(always)]
    fn from(val: Apllsscg1Mc) -> u8 {
        Apllsscg1Mc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apllsten {
    #[doc = "APLL is disabled in Deep Sleep mode."]
    DisabledInStop = 0x0,
    #[doc = "APLL is enabled in Deep Sleep mode."]
    EnabledInStop = 0x01,
}
impl Apllsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Apllsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Apllsten {
    #[inline(always)]
    fn from(val: u8) -> Apllsten {
        Apllsten::from_bits(val)
    }
}
impl From<Apllsten> for u8 {
    #[inline(always)]
    fn from(val: Apllsten) -> u8 {
        Apllsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AplltestctrlTestvSel {
    #[doc = "pll_bias_source."]
    PllBias = 0x0,
    #[doc = "pll_cco_current."]
    PllCco = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "ldo_ldo1p1."]
    Ldo = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl AplltestctrlTestvSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AplltestctrlTestvSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AplltestctrlTestvSel {
    #[inline(always)]
    fn from(val: u8) -> AplltestctrlTestvSel {
        AplltestctrlTestvSel::from_bits(val)
    }
}
impl From<AplltestctrlTestvSel> for u8 {
    #[inline(always)]
    fn from(val: AplltestctrlTestvSel) -> u8 {
        AplltestctrlTestvSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkValid {
    #[doc = "clk_valid_lv is not set."]
    ClkValid0 = 0x0,
    #[doc = "clk_valid_lv is set."]
    ClkValid1 = 0x01,
}
impl ClkValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkValid {
    #[inline(always)]
    fn from(val: u8) -> ClkValid {
        ClkValid::from_bits(val)
    }
}
impl From<ClkValid> for u8 {
    #[inline(always)]
    fn from(val: ClkValid) -> u8 {
        ClkValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Div16en {
    #[doc = "Divide-by-16 disabled, 12 MHz clock output."]
    Div16Disabled = 0x0,
    #[doc = "Divide-by-16 enabled, 750 kHz clock output."]
    Div16Enabled = 0x01,
}
impl Div16en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Div16en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Div16en {
    #[inline(always)]
    fn from(val: u8) -> Div16en {
        Div16en::from_bits(val)
    }
}
impl From<Div16en> for u8 {
    #[inline(always)]
    fn from(val: Div16en) -> u8 {
        Div16en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erefs {
    #[doc = "External reference clock from PAD pin selected. LDO can be disabled in this case."]
    External = 0x0,
    #[doc = "Internal crystal oscillator of OSC selected."]
    Internal = 0x01,
}
impl Erefs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erefs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erefs {
    #[inline(always)]
    fn from(val: u8) -> Erefs {
        Erefs::from_bits(val)
    }
}
impl From<Erefs> for u8 {
    #[inline(always)]
    fn from(val: Erefs) -> u8 {
        Erefs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircacc {
    #[doc = "FIRC is not enabled or clock is not accurate."]
    NotEnabledOrNotValid = 0x0,
    #[doc = "FIRC is enabled and output clock is accurate. The clock is accurate after 4096 clock cycles of 144 MHz (RANGE=1) or 1365 clock cycles of 48 MHz(RANGE=0) from the FIRC analog."]
    EnabledAndValid = 0x01,
}
impl Fircacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircacc {
    #[inline(always)]
    fn from(val: u8) -> Fircacc {
        Fircacc::from_bits(val)
    }
}
impl From<Fircacc> for u8 {
    #[inline(always)]
    fn from(val: Fircacc) -> u8 {
        Fircacc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FircaccIe {
    #[doc = "FIRCACC interrupt is not enabled."]
    Fircaccnot = 0x0,
    #[doc = "FIRCACC interrupt is enabled."]
    Fircaccyes = 0x01,
}
impl FircaccIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircaccIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircaccIe {
    #[inline(always)]
    fn from(val: u8) -> FircaccIe {
        FircaccIe::from_bits(val)
    }
}
impl From<FircaccIe> for u8 {
    #[inline(always)]
    fn from(val: FircaccIe) -> u8 {
        FircaccIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FirccsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
}
impl FirccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirccsrLk {
    #[inline(always)]
    fn from(val: u8) -> FirccsrLk {
        FirccsrLk::from_bits(val)
    }
}
impl From<FirccsrLk> for u8 {
    #[inline(always)]
    fn from(val: FirccsrLk) -> u8 {
        FirccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FirccsrTrimLock {
    #[doc = "FIRC auto trim not locked to target frequency range."]
    FircNotLocked = 0x0,
    #[doc = "FIRC auto trim locked to target frequency range."]
    FircLocked = 0x01,
}
impl FirccsrTrimLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirccsrTrimLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirccsrTrimLock {
    #[inline(always)]
    fn from(val: u8) -> FirccsrTrimLock {
        FirccsrTrimLock::from_bits(val)
    }
}
impl From<FirccsrTrimLock> for u8 {
    #[inline(always)]
    fn from(val: FirccsrTrimLock) -> u8 {
        FirccsrTrimLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircerr {
    #[doc = "Error not detected with the FIRC trimming."]
    ErrorNotDetected = 0x0,
    #[doc = "Error detected with the FIRC trimming."]
    ErrorDetected = 0x01,
}
impl Fircerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircerr {
    #[inline(always)]
    fn from(val: u8) -> Fircerr {
        Fircerr::from_bits(val)
    }
}
impl From<Fircerr> for u8 {
    #[inline(always)]
    fn from(val: Fircerr) -> u8 {
        Fircerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FircerrIe {
    #[doc = "FIRCERR interrupt is not enabled."]
    ErrorNotDetected = 0x0,
    #[doc = "FIRCERR interrupt is enabled."]
    ErrorDetected = 0x01,
}
impl FircerrIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircerrIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircerrIe {
    #[inline(always)]
    fn from(val: u8) -> FircerrIe {
        FircerrIe::from_bits(val)
    }
}
impl From<FircerrIe> for u8 {
    #[inline(always)]
    fn from(val: FircerrIe) -> u8 {
        FircerrIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircsten {
    #[doc = "FIRC is disabled in Deep Sleep mode."]
    DisabledInStopModes = 0x0,
    #[doc = "FIRC is enabled in Deep Sleep mode."]
    EnabledInStopModes = 0x01,
}
impl Fircsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircsten {
    #[inline(always)]
    fn from(val: u8) -> Fircsten {
        Fircsten::from_bits(val)
    }
}
impl From<Fircsten> for u8 {
    #[inline(always)]
    fn from(val: Fircsten) -> u8 {
        Fircsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FirctcfgTrimsrc {
    #[doc = "USB0 Start of Frame (1 kHz). This option does not use TRIMDIV because the clock came externally from USB."]
    Usb0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC."]
    Sosc = 0x02,
    #[doc = "ROSC."]
    Rosc = 0x03,
}
impl FirctcfgTrimsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirctcfgTrimsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirctcfgTrimsrc {
    #[inline(always)]
    fn from(val: u8) -> FirctcfgTrimsrc {
        FirctcfgTrimsrc::from_bits(val)
    }
}
impl From<FirctcfgTrimsrc> for u8 {
    #[inline(always)]
    fn from(val: FirctcfgTrimsrc) -> u8 {
        FirctcfgTrimsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircvld {
    #[doc = "FIRC is not enabled or clock is not valid."]
    NotEnabledOrNotValid = 0x0,
    #[doc = "FIRC is enabled and output clock is valid. The clock is valid after there is an output clock from the FIRC analog."]
    EnabledAndValid = 0x01,
}
impl Fircvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircvld {
    #[inline(always)]
    fn from(val: u8) -> Fircvld {
        Fircvld::from_bits(val)
    }
}
impl From<Fircvld> for u8 {
    #[inline(always)]
    fn from(val: Fircvld) -> u8 {
        Fircvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlipBufIn {
    #[doc = "Flip Buffer Inputs disabled."]
    FlipBufInDisabled = 0x0,
    #[doc = "Flip Buffer Inputs enabled."]
    FlipBufInEnabled = 0x01,
}
impl FlipBufIn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlipBufIn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlipBufIn {
    #[inline(always)]
    fn from(val: u8) -> FlipBufIn {
        FlipBufIn::from_bits(val)
    }
}
impl From<FlipBufIn> for u8 {
    #[inline(always)]
    fn from(val: FlipBufIn) -> u8 {
        FlipBufIn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqSel {
    _RESERVED_0 = 0x0,
    Firc45_48mhz = 0x01,
    _RESERVED_2 = 0x02,
    Firc60_64mhz = 0x03,
    _RESERVED_4 = 0x04,
    Firc90_96mhz = 0x05,
    _RESERVED_6 = 0x06,
    Firc180_192mhz = 0x07,
}
impl FreqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqSel {
    #[inline(always)]
    fn from(val: u8) -> FreqSel {
        FreqSel::from_bits(val)
    }
}
impl From<FreqSel> for u8 {
    #[inline(always)]
    fn from(val: FreqSel) -> u8 {
        FreqSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrDisable {
    #[doc = "IFR write access to SCG trim registers not disabled. The SCG Trim registers are reprogrammed with the IFR values after any system reset."]
    Enabled = 0x0,
    #[doc = "IFR write access to SCG trim registers during system reset is blocked."]
    Disabled = 0x01,
}
impl IfrDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrDisable {
    #[inline(always)]
    fn from(val: u8) -> IfrDisable {
        IfrDisable::from_bits(val)
    }
}
impl From<IfrDisable> for u8 {
    #[inline(always)]
    fn from(val: IfrDisable) -> u8 {
        IfrDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Overstress {
    #[doc = "Over stress test is disabled."]
    OverstressDis = 0x0,
    #[doc = "Over stress test is enabled."]
    OverstressEn = 0x01,
}
impl Overstress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Overstress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Overstress {
    #[inline(always)]
    fn from(val: u8) -> Overstress {
        Overstress::from_bits(val)
    }
}
impl From<Overstress> for u8 {
    #[inline(always)]
    fn from(val: Overstress) -> u8 {
        Overstress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    #[doc = "Frequency range select of 16-20 MHz."]
    Freq16to20mhz = 0x0,
    #[doc = "Frequency range select of 20-30 MHz."]
    LowFreq = 0x01,
    #[doc = "Frequency range select of 30-50 MHz."]
    MediumFreq = 0x02,
    #[doc = "Frequency range select of 50-66 MHz."]
    HighFreq = 0x03,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Rosccmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GenerateInterrupt = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GenerateReset = 0x01,
}
impl Rosccmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rosccmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rosccmre {
    #[inline(always)]
    fn from(val: u8) -> Rosccmre {
        Rosccmre::from_bits(val)
    }
}
impl From<Rosccmre> for u8 {
    #[inline(always)]
    fn from(val: Rosccmre) -> u8 {
        Rosccmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RosccsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
}
impl RosccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RosccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RosccsrLk {
    #[inline(always)]
    fn from(val: u8) -> RosccsrLk {
        RosccsrLk::from_bits(val)
    }
}
impl From<RosccsrLk> for u8 {
    #[inline(always)]
    fn from(val: RosccsrLk) -> u8 {
        RosccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roscerr {
    #[doc = "ROSC Clock Monitor is disabled or has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "ROSC Clock Monitor is enabled and detected an RTC loss of clock error."]
    EnabledAndError = 0x01,
}
impl Roscerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscerr {
    #[inline(always)]
    fn from(val: u8) -> Roscerr {
        Roscerr::from_bits(val)
    }
}
impl From<Roscerr> for u8 {
    #[inline(always)]
    fn from(val: Roscerr) -> u8 {
        Roscerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roscvld {
    #[doc = "ROSC is not enabled or clock is not valid."]
    DisabledOrNotValid = 0x0,
    #[doc = "ROSC is enabled and output clock is valid."]
    EnabledAndValid = 0x01,
}
impl Roscvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscvld {
    #[inline(always)]
    fn from(val: u8) -> Roscvld {
        Roscvld::from_bits(val)
    }
}
impl From<Roscvld> for u8 {
    #[inline(always)]
    fn from(val: Roscvld) -> u8 {
        Roscvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scs {
    _RESERVED_0 = 0x0,
    #[doc = "SOSC."]
    Sosc = 0x01,
    #[doc = "SIRC."]
    Sirc = 0x02,
    #[doc = "FIRC."]
    Firc = 0x03,
    #[doc = "ROSC."]
    Rosc = 0x04,
    #[doc = "APLL."]
    Apll = 0x05,
    #[doc = "SPLL."]
    Spll = 0x06,
    #[doc = "UPLL."]
    Upll = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scs {
    #[inline(always)]
    fn from(val: u8) -> Scs {
        Scs::from_bits(val)
    }
}
impl From<Scs> for u8 {
    #[inline(always)]
    fn from(val: Scs) -> u8 {
        Scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelAtx {
    #[doc = "ldo1p1v_atxout_ana_atx3v_a: floating; ldo1p1v_atxout_ana_atx3v_b: floating."]
    Floating = 0x0,
    #[doc = "ldo1p1v_atxout_ana_atx3v_a: ref_1v; ldo1p1v_atxout_ana_atx3v_b: vbiasn."]
    Vbiasn = 0x01,
    #[doc = "ldo1p1v_atxout_ana_atx3v_a: vea; ldo1p1v_atxout_ana_atx3v_b: vgate."]
    Vgate = 0x02,
    #[doc = "ldo1p1v_atxout_ana_atx3v_a: ldo1p1v_vout_ok_lv; ldo1p1v_atxout_ana_atx3v_b: ldo1p1v_vout_1p1v_ana_1p8v."]
    LdoP8v = 0x03,
}
impl SelAtx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelAtx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelAtx {
    #[inline(always)]
    fn from(val: u8) -> SelAtx {
        SelAtx::from_bits(val)
    }
}
impl From<SelAtx> for u8 {
    #[inline(always)]
    fn from(val: SelAtx) -> u8 {
        SelAtx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SirccsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
}
impl SirccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirccsrLk {
    #[inline(always)]
    fn from(val: u8) -> SirccsrLk {
        SirccsrLk::from_bits(val)
    }
}
impl From<SirccsrLk> for u8 {
    #[inline(always)]
    fn from(val: SirccsrLk) -> u8 {
        SirccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SirccsrTrimLock {
    #[doc = "SIRC auto trim not locked to target frequency range."]
    SircNotLocked = 0x0,
    #[doc = "SIRC auto trim locked to target frequency range."]
    SircLocked = 0x01,
}
impl SirccsrTrimLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirccsrTrimLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirccsrTrimLock {
    #[inline(always)]
    fn from(val: u8) -> SirccsrTrimLock {
        SirccsrTrimLock::from_bits(val)
    }
}
impl From<SirccsrTrimLock> for u8 {
    #[inline(always)]
    fn from(val: SirccsrTrimLock) -> u8 {
        SirccsrTrimLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircerr {
    #[doc = "Error not detected with the SIRC trimming."]
    ErrorNotDetected = 0x0,
    #[doc = "Error detected with the SIRC trimming."]
    ErrorDetected = 0x01,
}
impl Sircerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircerr {
    #[inline(always)]
    fn from(val: u8) -> Sircerr {
        Sircerr::from_bits(val)
    }
}
impl From<Sircerr> for u8 {
    #[inline(always)]
    fn from(val: Sircerr) -> u8 {
        Sircerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SircerrIe {
    #[doc = "SIRCERR interrupt is not enabled."]
    ErrorNotDetected = 0x0,
    #[doc = "SIRCERR interrupt is enabled."]
    ErrorDetected = 0x01,
}
impl SircerrIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SircerrIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SircerrIe {
    #[inline(always)]
    fn from(val: u8) -> SircerrIe {
        SircerrIe::from_bits(val)
    }
}
impl From<SircerrIe> for u8 {
    #[inline(always)]
    fn from(val: SircerrIe) -> u8 {
        SircerrIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SirctcfgTrimsrc {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC."]
    Sosc = 0x02,
    #[doc = "ROSC (32.768 kHz)."]
    Rosc = 0x03,
}
impl SirctcfgTrimsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirctcfgTrimsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirctcfgTrimsrc {
    #[inline(always)]
    fn from(val: u8) -> SirctcfgTrimsrc {
        SirctcfgTrimsrc::from_bits(val)
    }
}
impl From<SirctcfgTrimsrc> for u8 {
    #[inline(always)]
    fn from(val: SirctcfgTrimsrc) -> u8 {
        SirctcfgTrimsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircvld {
    #[doc = "SIRC is not enabled or clock is not valid."]
    DisabledOrNotValid = 0x0,
    #[doc = "SIRC is enabled and output clock is valid."]
    EnabledAndValid = 0x01,
}
impl Sircvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircvld {
    #[inline(always)]
    fn from(val: u8) -> Sircvld {
        Sircvld::from_bits(val)
    }
}
impl From<Sircvld> for u8 {
    #[inline(always)]
    fn from(val: Sircvld) -> u8 {
        Sircvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sosccmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GenerateInterrupt = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GenerateReset = 0x01,
}
impl Sosccmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sosccmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sosccmre {
    #[inline(always)]
    fn from(val: u8) -> Sosccmre {
        Sosccmre::from_bits(val)
    }
}
impl From<Sosccmre> for u8 {
    #[inline(always)]
    fn from(val: Sosccmre) -> u8 {
        Sosccmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SosccsrLk {
    #[doc = "This Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "This Control Status Register cannot be written."]
    WriteDisabled = 0x01,
}
impl SosccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SosccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SosccsrLk {
    #[inline(always)]
    fn from(val: u8) -> SosccsrLk {
        SosccsrLk::from_bits(val)
    }
}
impl From<SosccsrLk> for u8 {
    #[inline(always)]
    fn from(val: SosccsrLk) -> u8 {
        SosccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soscerr {
    #[doc = "SOSC Clock Monitor is disabled or has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "SOSC Clock Monitor is enabled and detected an error."]
    EnabledAndError = 0x01,
}
impl Soscerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscerr {
    #[inline(always)]
    fn from(val: u8) -> Soscerr {
        Soscerr::from_bits(val)
    }
}
impl From<Soscerr> for u8 {
    #[inline(always)]
    fn from(val: Soscerr) -> u8 {
        Soscerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Source {
    #[doc = "SOSC"]
    Sosc = 0x0,
    #[doc = "FIRC 45 MHz clock. FIRC_SCLK_PERIPH_EN needs to be set to use FIRC 45 MHz clock."]
    Firc = 0x01,
    #[doc = "ROSC"]
    Rosc = 0x02,
    #[doc = "SIRC 12 MHz clock"]
    Sirc = 0x03,
}
impl Source {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Source {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Source {
    #[inline(always)]
    fn from(val: u8) -> Source {
        Source::from_bits(val)
    }
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(val: Source) -> u8 {
        Source::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllLock {
    #[doc = "SPLL is not powered on or not locked."]
    DisabledOrNotValid = 0x0,
    #[doc = "SPLL is locked."]
    EnabledAndValid = 0x01,
}
impl SpllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllLock {
    #[inline(always)]
    fn from(val: u8) -> SpllLock {
        SpllLock::from_bits(val)
    }
}
impl From<SpllLock> for u8 {
    #[inline(always)]
    fn from(val: SpllLock) -> u8 {
        SpllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllcmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GenerateInterrupt = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GenerateReset = 0x01,
}
impl Spllcmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllcmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllcmre {
    #[inline(always)]
    fn from(val: u8) -> Spllcmre {
        Spllcmre::from_bits(val)
    }
}
impl From<Spllcmre> for u8 {
    #[inline(always)]
    fn from(val: Spllcmre) -> u8 {
        Spllcmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllcsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
}
impl SpllcsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllcsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllcsrLk {
    #[inline(always)]
    fn from(val: u8) -> SpllcsrLk {
        SpllcsrLk::from_bits(val)
    }
}
impl From<SpllcsrLk> for u8 {
    #[inline(always)]
    fn from(val: SpllcsrLk) -> u8 {
        SpllcsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllerr {
    #[doc = "SPLL Clock Monitor is disabled or has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "SPLL Clock Monitor is enabled and detected an error."]
    EnabledAndError = 0x01,
}
impl Spllerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllerr {
    #[inline(always)]
    fn from(val: u8) -> Spllerr {
        Spllerr::from_bits(val)
    }
}
impl From<Spllerr> for u8 {
    #[inline(always)]
    fn from(val: Spllerr) -> u8 {
        Spllerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsscg1Mc {
    #[doc = "MC\\[1:0\\] no compensation."]
    NoComp = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "MC\\[1:0\\] maximum compensation."]
    MaxComp = 0x03,
}
impl Spllsscg1Mc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsscg1Mc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsscg1Mc {
    #[inline(always)]
    fn from(val: u8) -> Spllsscg1Mc {
        Spllsscg1Mc::from_bits(val)
    }
}
impl From<Spllsscg1Mc> for u8 {
    #[inline(always)]
    fn from(val: Spllsscg1Mc) -> u8 {
        Spllsscg1Mc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsten {
    #[doc = "SPLL is disabled in Deep Sleep mode."]
    DisabledInStop = 0x0,
    #[doc = "SPLL is enabled in Deep Sleep mode."]
    EnabledInStop = 0x01,
}
impl Spllsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsten {
    #[inline(always)]
    fn from(val: u8) -> Spllsten {
        Spllsten::from_bits(val)
    }
}
impl From<Spllsten> for u8 {
    #[inline(always)]
    fn from(val: Spllsten) -> u8 {
        Spllsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SplltestctrlTestvSel {
    #[doc = "pll_bias_source."]
    PllBias = 0x0,
    #[doc = "pll_cco_current."]
    PllCco = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "ldo_ldo1p1."]
    Ldo = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SplltestctrlTestvSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SplltestctrlTestvSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SplltestctrlTestvSel {
    #[inline(always)]
    fn from(val: u8) -> SplltestctrlTestvSel {
        SplltestctrlTestvSel::from_bits(val)
    }
}
impl From<SplltestctrlTestvSel> for u8 {
    #[inline(always)]
    fn from(val: SplltestctrlTestvSel) -> u8 {
        SplltestctrlTestvSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StartValid {
    #[doc = "start_valid_lv is not set."]
    StartValid0 = 0x0,
    #[doc = "start_valid_lv is set."]
    StartValid1 = 0x01,
}
impl StartValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StartValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StartValid {
    #[inline(always)]
    fn from(val: u8) -> StartValid {
        StartValid::from_bits(val)
    }
}
impl From<StartValid> for u8 {
    #[inline(always)]
    fn from(val: StartValid) -> u8 {
        StartValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TestBufBypass {
    #[doc = "Not bypass buffer."]
    TestBufBypassDisabled = 0x0,
    #[doc = "Bypass buffer."]
    TestBufBypassEnabled = 0x01,
}
impl TestBufBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TestBufBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TestBufBypass {
    #[inline(always)]
    fn from(val: u8) -> TestBufBypass {
        TestBufBypass::from_bits(val)
    }
}
impl From<TestBufBypass> for u8 {
    #[inline(always)]
    fn from(val: TestBufBypass) -> u8 {
        TestBufBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TestBufEn {
    #[doc = "Test buffer is disabled."]
    TestBufEnDis = 0x0,
    #[doc = "Test buffer is enabled."]
    TestBufEnEn = 0x01,
}
impl TestBufEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TestBufEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TestBufEn {
    #[inline(always)]
    fn from(val: u8) -> TestBufEn {
        TestBufEn::from_bits(val)
    }
}
impl From<TestBufEn> for u8 {
    #[inline(always)]
    fn from(val: TestBufEn) -> u8 {
        TestBufEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TestBufFlip {
    #[doc = "Input of the buffer is not flipped."]
    TestBufFlipDis = 0x0,
    #[doc = "Input of the buffer is flipped."]
    TestBufFlipEn = 0x01,
}
impl TestBufFlip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TestBufFlip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TestBufFlip {
    #[inline(always)]
    fn from(val: u8) -> TestBufFlip {
        TestBufFlip::from_bits(val)
    }
}
impl From<TestBufFlip> for u8 {
    #[inline(always)]
    fn from(val: TestBufFlip) -> u8 {
        TestBufFlip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TestSel {
    #[doc = "VREF."]
    TestSel00 = 0x0,
    #[doc = "VFB."]
    TestSel01 = 0x01,
    #[doc = "BP."]
    TestSel10 = 0x02,
    #[doc = "VCCO."]
    TestSel11 = 0x03,
}
impl TestSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TestSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TestSel {
    #[inline(always)]
    fn from(val: u8) -> TestSel {
        TestSel::from_bits(val)
    }
}
impl From<TestSel> for u8 {
    #[inline(always)]
    fn from(val: TestSel) -> u8 {
        TestSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Testen {
    #[doc = "FIRC test is disabled."]
    FircTestDis = 0x0,
    #[doc = "FIRC test is enabled."]
    FircTestEn = 0x01,
}
impl Testen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Testen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Testen {
    #[inline(always)]
    fn from(val: u8) -> Testen {
        Testen::from_bits(val)
    }
}
impl From<Testen> for u8 {
    #[inline(always)]
    fn from(val: Testen) -> u8 {
        Testen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Testsel {
    _RESERVED_0 = 0x0,
    #[doc = "VREGOUT test."]
    Regout = 0x01,
    #[doc = "VSW test."]
    Vsw = 0x02,
    #[doc = "VREF test."]
    Vref = 0x03,
}
impl Testsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Testsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Testsel {
    #[inline(always)]
    fn from(val: u8) -> Testsel {
        Testsel::from_bits(val)
    }
}
impl From<Testsel> for u8 {
    #[inline(always)]
    fn from(val: Testsel) -> u8 {
        Testsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimUnlock {
    #[doc = "SCG Trim /Test registers are locked and not writable."]
    Locked = 0x0,
    #[doc = "SCG Trim /Test registers are unlocked and writable."]
    NotLocked = 0x01,
}
impl TrimUnlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimUnlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimUnlock {
    #[inline(always)]
    fn from(val: u8) -> TrimUnlock {
        TrimUnlock::from_bits(val)
    }
}
impl From<TrimUnlock> for u8 {
    #[inline(always)]
    fn from(val: TrimUnlock) -> u8 {
        TrimUnlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstmd {
    #[doc = "ipt_test_ana_atx3v_a=High-Z; ipt_test_ana_atx3v_b=High-Z."]
    Tstmd00 = 0x0,
    #[doc = "ipt_test_ana_atx3v_a=osc_in(EXTAL); ipt_test_ana_atx3v_b=High-Z."]
    Tstmd01 = 0x01,
    #[doc = "ipt_test_ana_atx3v_a=High-Z; ipt_test_ana_atx3v_b=osc_out(XTAL)."]
    Tstmd10 = 0x02,
    #[doc = "ipt_test_ana_atx3v_a=osc_in(EXTAL); ipt_test_ana_atx3v_b=osc_out(XTAL)."]
    Tstmd11 = 0x03,
}
impl Tstmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstmd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstmd {
    #[inline(always)]
    fn from(val: u8) -> Tstmd {
        Tstmd::from_bits(val)
    }
}
impl From<Tstmd> for u8 {
    #[inline(always)]
    fn from(val: Tstmd) -> u8 {
        Tstmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upllcmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GenerateInterrupt = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GenerateReset = 0x01,
}
impl Upllcmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllcmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllcmre {
    #[inline(always)]
    fn from(val: u8) -> Upllcmre {
        Upllcmre::from_bits(val)
    }
}
impl From<Upllcmre> for u8 {
    #[inline(always)]
    fn from(val: Upllcmre) -> u8 {
        Upllcmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UpllcsrLk {
    #[doc = "Control Status Register can be written."]
    WriteEnabled = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WriteDisabled = 0x01,
}
impl UpllcsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UpllcsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UpllcsrLk {
    #[inline(always)]
    fn from(val: u8) -> UpllcsrLk {
        UpllcsrLk::from_bits(val)
    }
}
impl From<UpllcsrLk> for u8 {
    #[inline(always)]
    fn from(val: UpllcsrLk) -> u8 {
        UpllcsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upllerr {
    #[doc = "UPLL Clock Monitor is disabled or has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "UPLL Clock Monitor is enabled and detected an error."]
    EnabledAndError = 0x01,
}
impl Upllerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllerr {
    #[inline(always)]
    fn from(val: u8) -> Upllerr {
        Upllerr::from_bits(val)
    }
}
impl From<Upllerr> for u8 {
    #[inline(always)]
    fn from(val: Upllerr) -> u8 {
        Upllerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upllvld {
    #[doc = "UPLL is not enabled or clock is not valid."]
    DisabledOrNotValid = 0x0,
    #[doc = "UPLL is enabled and output clock is valid."]
    EnabledAndValid = 0x01,
}
impl Upllvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upllvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upllvld {
    #[inline(always)]
    fn from(val: u8) -> Upllvld {
        Upllvld::from_bits(val)
    }
}
impl From<Upllvld> for u8 {
    #[inline(always)]
    fn from(val: Upllvld) -> u8 {
        Upllvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VoutSel {
    #[doc = "VOUT = 1V."]
    Vout1v1 = 0x0,
    #[doc = "VOUT = 1V."]
    Vout1v2 = 0x01,
    #[doc = "VOUT = 1V."]
    Vout1v3 = 0x02,
    #[doc = "VOUT = 1.05V."]
    Vout105v = 0x03,
    #[doc = "VOUT = 1.1V."]
    Vout11v = 0x04,
    #[doc = "VOUT = 1.15V."]
    Vout115v = 0x05,
    #[doc = "VOUT = 1.2V."]
    Vout12v = 0x06,
    #[doc = "VOUT = 1.25V."]
    Vout125v = 0x07,
}
impl VoutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VoutSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VoutSel {
    #[inline(always)]
    fn from(val: u8) -> VoutSel {
        VoutSel::from_bits(val)
    }
}
impl From<VoutSel> for u8 {
    #[inline(always)]
    fn from(val: VoutSel) -> u8 {
        VoutSel::to_bits(val)
    }
}
