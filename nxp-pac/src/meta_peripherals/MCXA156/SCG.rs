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
    #[doc = "FIRC Auto-trimming Counter 1."]
    #[inline(always)]
    pub const fn fircatc1(self) -> crate::pac::common::Reg<Fircatc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 2."]
    #[inline(always)]
    pub const fn fircatc2(self) -> crate::pac::common::Reg<Fircatc2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 3."]
    #[inline(always)]
    pub const fn fircatc3(self) -> crate::pac::common::Reg<Fircatc3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 4."]
    #[inline(always)]
    pub const fn fircatc4(self) -> crate::pac::common::Reg<Fircatc4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 5."]
    #[inline(always)]
    pub const fn fircatc5(self) -> crate::pac::common::Reg<Fircatc5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 6."]
    #[inline(always)]
    pub const fn fircatc6(self) -> crate::pac::common::Reg<Fircatc6, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 7."]
    #[inline(always)]
    pub const fn fircatc7(self) -> crate::pac::common::Reg<Fircatc7, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 8."]
    #[inline(always)]
    pub const fn fircatc8(self) -> crate::pac::common::Reg<Fircatc8, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 9."]
    #[inline(always)]
    pub const fn fircatc9(self) -> crate::pac::common::Reg<Fircatc9, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 10."]
    #[inline(always)]
    pub const fn fircatc10(self) -> crate::pac::common::Reg<Fircatc10, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 11."]
    #[inline(always)]
    pub const fn fircatc11(self) -> crate::pac::common::Reg<Fircatc11, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "ROSC Control Status Register."]
    #[inline(always)]
    pub const fn rosccsr(self) -> crate::pac::common::Reg<Rosccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
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
        let val = (self.0 >> 24usize) & 0x07;
        Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: Scs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
#[doc = "FIRC Auto-trimming Counter 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc1(pub u32);
impl Fircatc1 {
    #[doc = "Ideal Counter SOSC."]
    #[must_use]
    #[inline(always)]
    pub const fn idealc_sosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ideal Counter SOSC."]
    #[inline(always)]
    pub const fn set_idealc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sample Cycle SOSC."]
    #[must_use]
    #[inline(always)]
    pub const fn samcyc_sosc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sample Cycle SOSC."]
    #[inline(always)]
    pub const fn set_samcyc_sosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Setup Cycle SOSC."]
    #[must_use]
    #[inline(always)]
    pub const fn setcyc_sosc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Cycle SOSC."]
    #[inline(always)]
    pub const fn set_setcyc_sosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Fircatc1 {
    #[inline(always)]
    fn default() -> Fircatc1 {
        Fircatc1(0)
    }
}
impl core::fmt::Debug for Fircatc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc1")
            .field("idealc_sosc", &self.idealc_sosc())
            .field("samcyc_sosc", &self.samcyc_sosc())
            .field("setcyc_sosc", &self.setcyc_sosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc1 {{ idealc_sosc: {=u16:?}, samcyc_sosc: {=u8:?}, setcyc_sosc: {=u8:?} }}",
            self.idealc_sosc(),
            self.samcyc_sosc(),
            self.setcyc_sosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc10(pub u32);
impl Fircatc10 {
    #[doc = "Fine Trim Maximum Counter SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn finemaxc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Fine Trim Maximum Counter SOF."]
    #[inline(always)]
    pub const fn set_finemaxc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc10 {
    #[inline(always)]
    fn default() -> Fircatc10 {
        Fircatc10(0)
    }
}
impl core::fmt::Debug for Fircatc10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc10")
            .field("finemaxc_sof", &self.finemaxc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc10 {{ finemaxc_sof: {=u32:?} }}",
            self.finemaxc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc11(pub u32);
impl Fircatc11 {
    #[doc = "Fine Trim Minimum Counter SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn fineminc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Fine Trim Minimum Counter SOF."]
    #[inline(always)]
    pub const fn set_fineminc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc11 {
    #[inline(always)]
    fn default() -> Fircatc11 {
        Fircatc11(0)
    }
}
impl core::fmt::Debug for Fircatc11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc11")
            .field("fineminc_sof", &self.fineminc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc11 {{ fineminc_sof: {=u32:?} }}",
            self.fineminc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc2(pub u32);
impl Fircatc2 {
    #[doc = "Coarse Trim Minimum Counter SOSC."]
    #[must_use]
    #[inline(always)]
    pub const fn coarminc_sosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Minimum Counter SOSC."]
    #[inline(always)]
    pub const fn set_coarminc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Coarse Trim Maximum Counter SOSC."]
    #[must_use]
    #[inline(always)]
    pub const fn coarmaxc_sosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Maximum Counter SOSC."]
    #[inline(always)]
    pub const fn set_coarmaxc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc2 {
    #[inline(always)]
    fn default() -> Fircatc2 {
        Fircatc2(0)
    }
}
impl core::fmt::Debug for Fircatc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc2")
            .field("coarminc_sosc", &self.coarminc_sosc())
            .field("coarmaxc_sosc", &self.coarmaxc_sosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc2 {{ coarminc_sosc: {=u16:?}, coarmaxc_sosc: {=u16:?} }}",
            self.coarminc_sosc(),
            self.coarmaxc_sosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc3(pub u32);
impl Fircatc3 {
    #[doc = "Fine Trim Minimum Counter SOSC."]
    #[must_use]
    #[inline(always)]
    pub const fn fineminc_sosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Minimum Counter SOSC."]
    #[inline(always)]
    pub const fn set_fineminc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fine Trim Maximum Counter SOSC."]
    #[must_use]
    #[inline(always)]
    pub const fn finemaxc_sosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Maximum Counter SOSC."]
    #[inline(always)]
    pub const fn set_finemaxc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc3 {
    #[inline(always)]
    fn default() -> Fircatc3 {
        Fircatc3(0)
    }
}
impl core::fmt::Debug for Fircatc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc3")
            .field("fineminc_sosc", &self.fineminc_sosc())
            .field("finemaxc_sosc", &self.finemaxc_sosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc3 {{ fineminc_sosc: {=u16:?}, finemaxc_sosc: {=u16:?} }}",
            self.fineminc_sosc(),
            self.finemaxc_sosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc4(pub u32);
impl Fircatc4 {
    #[doc = "Ideal Counter ROSC."]
    #[must_use]
    #[inline(always)]
    pub const fn idealc_rosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ideal Counter ROSC."]
    #[inline(always)]
    pub const fn set_idealc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Setup Cycle ROSC."]
    #[must_use]
    #[inline(always)]
    pub const fn setcyc_rosc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Cycle ROSC."]
    #[inline(always)]
    pub const fn set_setcyc_rosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Sample Cycle ROSC."]
    #[must_use]
    #[inline(always)]
    pub const fn samcyc_rosc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sample Cycle ROSC."]
    #[inline(always)]
    pub const fn set_samcyc_rosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Fircatc4 {
    #[inline(always)]
    fn default() -> Fircatc4 {
        Fircatc4(0)
    }
}
impl core::fmt::Debug for Fircatc4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc4")
            .field("idealc_rosc", &self.idealc_rosc())
            .field("setcyc_rosc", &self.setcyc_rosc())
            .field("samcyc_rosc", &self.samcyc_rosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc4 {{ idealc_rosc: {=u16:?}, setcyc_rosc: {=u8:?}, samcyc_rosc: {=u8:?} }}",
            self.idealc_rosc(),
            self.setcyc_rosc(),
            self.samcyc_rosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc5(pub u32);
impl Fircatc5 {
    #[doc = "Coarse Trim Minimum Counter ROSC."]
    #[must_use]
    #[inline(always)]
    pub const fn coarminc_rosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Minimum Counter ROSC."]
    #[inline(always)]
    pub const fn set_coarminc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Coarse Trim Maximum Counter ROSC."]
    #[must_use]
    #[inline(always)]
    pub const fn coarmaxc_rosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Maximum Counter ROSC."]
    #[inline(always)]
    pub const fn set_coarmaxc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc5 {
    #[inline(always)]
    fn default() -> Fircatc5 {
        Fircatc5(0)
    }
}
impl core::fmt::Debug for Fircatc5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc5")
            .field("coarminc_rosc", &self.coarminc_rosc())
            .field("coarmaxc_rosc", &self.coarmaxc_rosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc5 {{ coarminc_rosc: {=u16:?}, coarmaxc_rosc: {=u16:?} }}",
            self.coarminc_rosc(),
            self.coarmaxc_rosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc6(pub u32);
impl Fircatc6 {
    #[doc = "Fine Trim Minimum Counter ROSC."]
    #[must_use]
    #[inline(always)]
    pub const fn fineminc_rosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Minimum Counter ROSC."]
    #[inline(always)]
    pub const fn set_fineminc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fine Trim Maximum Counter ROSC."]
    #[must_use]
    #[inline(always)]
    pub const fn finemaxc_rosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Maximum Counter ROSC."]
    #[inline(always)]
    pub const fn set_finemaxc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc6 {
    #[inline(always)]
    fn default() -> Fircatc6 {
        Fircatc6(0)
    }
}
impl core::fmt::Debug for Fircatc6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc6")
            .field("fineminc_rosc", &self.fineminc_rosc())
            .field("finemaxc_rosc", &self.finemaxc_rosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc6 {{ fineminc_rosc: {=u16:?}, finemaxc_rosc: {=u16:?} }}",
            self.fineminc_rosc(),
            self.finemaxc_rosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc7(pub u32);
impl Fircatc7 {
    #[doc = "Ideal Counter SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn idealc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Ideal Counter SOF."]
    #[inline(always)]
    pub const fn set_idealc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Setup Cycle SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn setcyc_sof(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Setup Cycle SOF."]
    #[inline(always)]
    pub const fn set_setcyc_sof(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Sample Cycle SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn samcyc_sof(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Sample Cycle SOF."]
    #[inline(always)]
    pub const fn set_samcyc_sof(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Fircatc7 {
    #[inline(always)]
    fn default() -> Fircatc7 {
        Fircatc7(0)
    }
}
impl core::fmt::Debug for Fircatc7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc7")
            .field("idealc_sof", &self.idealc_sof())
            .field("setcyc_sof", &self.setcyc_sof())
            .field("samcyc_sof", &self.samcyc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc7 {{ idealc_sof: {=u32:?}, setcyc_sof: {=u8:?}, samcyc_sof: {=u8:?} }}",
            self.idealc_sof(),
            self.setcyc_sof(),
            self.samcyc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc8(pub u32);
impl Fircatc8 {
    #[doc = "Coarse Trim Maximum Counter SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn coarmaxc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Coarse Trim Maximum Counter SOF."]
    #[inline(always)]
    pub const fn set_coarmaxc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc8 {
    #[inline(always)]
    fn default() -> Fircatc8 {
        Fircatc8(0)
    }
}
impl core::fmt::Debug for Fircatc8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc8")
            .field("coarmaxc_sof", &self.coarmaxc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc8 {{ coarmaxc_sof: {=u32:?} }}",
            self.coarmaxc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc9(pub u32);
impl Fircatc9 {
    #[doc = "Coarse Trim Minimum Counter SOF."]
    #[must_use]
    #[inline(always)]
    pub const fn coarminc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Coarse Trim Minimum Counter SOF."]
    #[inline(always)]
    pub const fn set_coarminc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc9 {
    #[inline(always)]
    fn default() -> Fircatc9 {
        Fircatc9(0)
    }
}
impl core::fmt::Debug for Fircatc9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc9")
            .field("coarminc_sof", &self.coarminc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc9 {{ coarminc_sof: {=u32:?} }}",
            self.coarminc_sof()
        )
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
    #[doc = "FRO_HF Clock to peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firc_fclk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FRO_HF Clock to peripherals Enable."]
    #[inline(always)]
    pub const fn set_firc_fclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "FRO_HF Trim Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FRO_HF Trim Enable."]
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
    #[doc = "FIRC Trim Pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "FIRC Trim Pre-divider."]
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
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trim Temperature."]
    #[inline(always)]
    pub const fn set_trimtemp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ soscclkpres: {=bool:?}, sircclkpres: {=bool:?}, fircclkpres: {=bool:?}, roscclkpres: {=bool:?} }}",
            self.soscclkpres(),
            self.sircclkpres(),
            self.fircclkpres(),
            self.roscclkpres()
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
        let val = (self.0 >> 24usize) & 0x07;
        Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: Scs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
            "Rosccsr {{ lk: {:?}, roscvld: {:?}, roscsel: {=bool:?}, roscerr: {:?} }}",
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
    #[doc = "SIRC Trim Pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "SIRC Trim Pre-divider."]
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
    #[doc = "External Reference Select."]
    #[must_use]
    #[inline(always)]
    pub const fn erefs(&self) -> Erefs {
        let val = (self.0 >> 2usize) & 0x01;
        Erefs::from_bits(val as u8)
    }
    #[doc = "External Reference Select."]
    #[inline(always)]
    pub const fn set_erefs(&mut self, val: Erefs) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SOSC Range Select."]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> Range {
        let val = (self.0 >> 4usize) & 0x03;
        Range::from_bits(val as u8)
    }
    #[doc = "SOSC Range Select."]
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
    #[doc = "SOSC clock safety enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosc_safe_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC clock safety enable."]
    #[inline(always)]
    pub const fn set_sosc_safe_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("sosc_safe_en", &self.sosc_safe_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccsr {{ soscen: {=bool:?}, soscsten: {=bool:?}, sosccm: {=bool:?}, sosccmre: {:?}, lk: {:?}, soscvld: {=bool:?}, soscsel: {=bool:?}, soscerr: {:?}, soscvld_ie: {=bool:?}, sosc_safe_en: {=bool:?} }}",
            self.soscen(),
            self.soscsten(),
            self.sosccm(),
            self.sosccmre(),
            self.lk(),
            self.soscvld(),
            self.soscsel(),
            self.soscerr(),
            self.soscvld_ie(),
            self.sosc_safe_en()
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
pub enum Erefs {
    #[doc = "External reference clock selected."]
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
    #[doc = "FIRC is enabled and output clock is accurate after some preparation time which is obtained by counting FRO_HF clock."]
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
    #[doc = "USB0 Start of Frame (1 KHz). This option does not use TRIMDIV."]
    Usb0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC. This option requires that SOSC be divided using the TRIMDIV field to get a frequency of 1 MHz."]
    Sosc = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum Range {
    #[doc = "Frequency range select of 8-16 MHz."]
    Freq16to20mHz = 0x0,
    #[doc = "Frequency range select of 16-25 MHz."]
    LowFreq = 0x01,
    #[doc = "Frequency range select of 25-40 MHz."]
    MediumFreq = 0x02,
    #[doc = "Frequency range select of 40-50 MHz."]
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
    #[doc = "ROSC Clock has not detected an error."]
    DisabledOrNoError = 0x0,
    #[doc = "ROSC Clock has detected an error."]
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
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scs {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "SOSC. This option requires that SOSC be divided using the TRIMDIV field to get a frequency of 1 MHz."]
    Sosc = 0x02,
    _RESERVED_3 = 0x03,
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
pub enum TrimUnlock {
    #[doc = "SCG Trim Registers locked and not writable."]
    Locked = 0x0,
    #[doc = "SCG Trim registers unlocked and writable."]
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
