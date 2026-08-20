#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Prefetch configuration array."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel n Control."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::pac::common::Reg<Ccr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel n Data Rate."]
    #[inline(always)]
    pub const fn cdr(self) -> crate::pac::common::Reg<Cdr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel n Configuration."]
    #[inline(always)]
    pub const fn ccfr(self) -> crate::pac::common::Reg<Ccfr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel n Protection."]
    #[inline(always)]
    pub const fn cprot(self) -> crate::pac::common::Reg<Cprot, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel n Bias."]
    #[inline(always)]
    pub const fn cbias(self) -> crate::pac::common::Reg<Cbias, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel n Low Limit."]
    #[inline(always)]
    pub const fn clolmt(self) -> crate::pac::common::Reg<Clolmt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel n High Limit."]
    #[inline(always)]
    pub const fn chilmt(self) -> crate::pac::common::Reg<Chilmt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Channel n Result Data."]
    #[inline(always)]
    pub const fn crdata(self) -> crate::pac::common::Reg<Crdata, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel n Multipurpose Data."]
    #[inline(always)]
    pub const fn cmpdata(self) -> crate::pac::common::Reg<Cmpdata, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel n Advanced Configuration."]
    #[inline(always)]
    pub const fn cacfr(self) -> crate::pac::common::Reg<Cacfr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel n Status."]
    #[inline(always)]
    pub const fn csr(self) -> crate::pac::common::Reg<Csr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel n Debug."]
    #[inline(always)]
    pub const fn cdbgr(self) -> crate::pac::common::Reg<Cdbgr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
#[doc = "SINC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc {
    ptr: *mut u8,
}
unsafe impl Send for Sinc {}
unsafe impl Sync for Sinc {}
impl Sinc {
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
    #[doc = "Parameters."]
    #[inline(always)]
    pub const fn parameter(self) -> crate::pac::common::Reg<Parameter, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Main Control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::pac::common::Reg<Mcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Normal Interrupt Enable."]
    #[inline(always)]
    pub const fn nie(self) -> crate::pac::common::Reg<Nie, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Error Interrupt Enable."]
    #[inline(always)]
    pub const fn eie(self) -> crate::pac::common::Reg<Eie, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "FIFO And CAD Error Interrupt Enable."]
    #[inline(always)]
    pub const fn fifoie(self) -> crate::pac::common::Reg<Fifoie, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Normal Interrupt Status."]
    #[inline(always)]
    pub const fn nis(self) -> crate::pac::common::Reg<Nis, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Error Interrupt Status."]
    #[inline(always)]
    pub const fn eis(self) -> crate::pac::common::Reg<Eis, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "FIFO And CAD Error Interrupt Status."]
    #[inline(always)]
    pub const fn fifois(self) -> crate::pac::common::Reg<Fifois, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn sr(self) -> crate::pac::common::Reg<Sr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Prefetch configuration array."]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 5usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x38usize + n * 48usize) as _) }
    }
}
#[doc = "Channel n Advanced Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacfr(pub u32);
impl Cacfr {
    #[doc = "Alternate DMA Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn admasel(&self) -> Admasel {
        let val = (self.0 >> 12usize) & 0x0f;
        Admasel::from_bits(val as u8)
    }
    #[doc = "Alternate DMA Source Selection."]
    #[inline(always)]
    pub const fn set_admasel(&mut self, val: Admasel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "HPF DC Remover Alpha Coefficient."]
    #[must_use]
    #[inline(always)]
    pub const fn hpfa(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "HPF DC Remover Alpha Coefficient."]
    #[inline(always)]
    pub const fn set_hpfa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Input Modulator Bitstream Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn ibdly(&self) -> Ibdly {
        let val = (self.0 >> 20usize) & 0x0f;
        Ibdly::from_bits(val as u8)
    }
    #[doc = "Input Modulator Bitstream Delay."]
    #[inline(always)]
    pub const fn set_ibdly(&mut self, val: Ibdly) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
}
impl Default for Cacfr {
    #[inline(always)]
    fn default() -> Cacfr {
        Cacfr(0)
    }
}
impl core::fmt::Debug for Cacfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacfr")
            .field("admasel", &self.admasel())
            .field("hpfa", &self.hpfa())
            .field("ibdly", &self.ibdly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cacfr {{ admasel: {:?}, hpfa: {=u8:?}, ibdly: {:?} }}",
            self.admasel(),
            self.hpfa(),
            self.ibdly()
        )
    }
}
#[doc = "Channel n Bias."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbias(pub u32);
impl Cbias {
    #[doc = "Bias Value."]
    #[must_use]
    #[inline(always)]
    pub const fn bias(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bias Value."]
    #[inline(always)]
    pub const fn set_bias(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Cbias {
    #[inline(always)]
    fn default() -> Cbias {
        Cbias(0)
    }
}
impl core::fmt::Debug for Cbias {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbias").field("bias", &self.bias()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbias {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cbias {{ bias: {=u32:?} }}", self.bias())
    }
}
#[doc = "Channel n Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccfr(pub u32);
impl Ccfr {
    #[doc = "PF Shift."]
    #[must_use]
    #[inline(always)]
    pub const fn pfsft(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "PF Shift."]
    #[inline(always)]
    pub const fn set_pfsft(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Result Data Format."]
    #[must_use]
    #[inline(always)]
    pub const fn rdfmt(&self) -> Rdfmt {
        let val = (self.0 >> 6usize) & 0x01;
        Rdfmt::from_bits(val as u8)
    }
    #[doc = "Result Data Format."]
    #[inline(always)]
    pub const fn set_rdfmt(&mut self, val: Rdfmt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn fifowmk(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "FIFO Watermark."]
    #[inline(always)]
    pub const fn set_fifowmk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Input Bit Format."]
    #[must_use]
    #[inline(always)]
    pub const fn ibfmt(&self) -> Ibfmt {
        let val = (self.0 >> 16usize) & 0x03;
        Ibfmt::from_bits(val as u8)
    }
    #[doc = "Input Bit Format."]
    #[inline(always)]
    pub const fn set_ibfmt(&mut self, val: Ibfmt) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Input Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn icsel(&self) -> Icsel {
        let val = (self.0 >> 18usize) & 0x07;
        Icsel::from_bits(val as u8)
    }
    #[doc = "Input Clock Select."]
    #[inline(always)]
    pub const fn set_icsel(&mut self, val: Icsel) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Input Clock Edge Select."]
    #[must_use]
    #[inline(always)]
    pub const fn icesel(&self) -> Icesel {
        let val = (self.0 >> 21usize) & 0x07;
        Icesel::from_bits(val as u8)
    }
    #[doc = "Input Clock Edge Select."]
    #[inline(always)]
    pub const fn set_icesel(&mut self, val: Icesel) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
    #[doc = "Input Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn itsel(&self) -> Itsel {
        let val = (self.0 >> 24usize) & 0x03;
        Itsel::from_bits(val as u8)
    }
    #[doc = "Input Trigger Select."]
    #[inline(always)]
    pub const fn set_itsel(&mut self, val: Itsel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Input Bit Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ibsel(&self) -> Ibsel {
        let val = (self.0 >> 26usize) & 0x03;
        Ibsel::from_bits(val as u8)
    }
    #[doc = "Input Bit Select."]
    #[inline(always)]
    pub const fn set_ibsel(&mut self, val: Ibsel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Input Trigger Level Type."]
    #[must_use]
    #[inline(always)]
    pub const fn itlvl(&self) -> Itlvl {
        let val = (self.0 >> 28usize) & 0x01;
        Itlvl::from_bits(val as u8)
    }
    #[doc = "Input Trigger Level Type."]
    #[inline(always)]
    pub const fn set_itlvl(&mut self, val: Itlvl) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Zero Cross Option."]
    #[must_use]
    #[inline(always)]
    pub const fn zcop(&self) -> Zcop {
        let val = (self.0 >> 30usize) & 0x03;
        Zcop::from_bits(val as u8)
    }
    #[doc = "Zero Cross Option."]
    #[inline(always)]
    pub const fn set_zcop(&mut self, val: Zcop) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccfr {
    #[inline(always)]
    fn default() -> Ccfr {
        Ccfr(0)
    }
}
impl core::fmt::Debug for Ccfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccfr")
            .field("pfsft", &self.pfsft())
            .field("rdfmt", &self.rdfmt())
            .field("fifowmk", &self.fifowmk())
            .field("ibfmt", &self.ibfmt())
            .field("icsel", &self.icsel())
            .field("icesel", &self.icesel())
            .field("itsel", &self.itsel())
            .field("ibsel", &self.ibsel())
            .field("itlvl", &self.itlvl())
            .field("zcop", &self.zcop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccfr {{ pfsft: {=u8:?}, rdfmt: {:?}, fifowmk: {=u8:?}, ibfmt: {:?}, icsel: {:?}, icesel: {:?}, itsel: {:?}, ibsel: {:?}, itlvl: {:?}, zcop: {:?} }}",
            self.pfsft(),
            self.rdfmt(),
            self.fifowmk(),
            self.ibfmt(),
            self.icsel(),
            self.icesel(),
            self.itsel(),
            self.ibsel(),
            self.itlvl(),
            self.zcop()
        )
    }
}
#[doc = "Channel n Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chen(&self) -> Chen {
        let val = (self.0 >> 0usize) & 0x01;
        Chen::from_bits(val as u8)
    }
    #[doc = "Channel Enable."]
    #[inline(always)]
    pub const fn set_chen(&mut self, val: Chen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PF Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfen(&self) -> Pfen {
        let val = (self.0 >> 1usize) & 0x01;
        Pfen::from_bits(val as u8)
    }
    #[doc = "PF Enable."]
    #[inline(always)]
    pub const fn set_pfen(&mut self, val: Pfen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> Dmaen {
        let val = (self.0 >> 3usize) & 0x01;
        Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: Dmaen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Short Circuit Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn scden(&self) -> Scden {
        let val = (self.0 >> 8usize) & 0x01;
        Scden::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detect Enable."]
    #[inline(always)]
    pub const fn set_scden(&mut self, val: Scden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock Absence Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn caden(&self) -> Caden {
        let val = (self.0 >> 9usize) & 0x01;
        Caden::from_bits(val as u8)
    }
    #[doc = "Clock Absence Detect Enable."]
    #[inline(always)]
    pub const fn set_caden(&mut self, val: Caden) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Zero Cross Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn zcden(&self) -> Zcden {
        let val = (self.0 >> 12usize) & 0x01;
        Zcden::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detect Enable."]
    #[inline(always)]
    pub const fn set_zcden(&mut self, val: Zcden) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Limit Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lmten(&self) -> Lmten {
        let val = (self.0 >> 13usize) & 0x01;
        Lmten::from_bits(val as u8)
    }
    #[doc = "Limit Enable."]
    #[inline(always)]
    pub const fn set_lmten(&mut self, val: Lmten) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoen(&self) -> Fifoen {
        let val = (self.0 >> 14usize) & 0x01;
        Fifoen::from_bits(val as u8)
    }
    #[doc = "FIFO Enable."]
    #[inline(always)]
    pub const fn set_fifoen(&mut self, val: Fifoen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Debug Output Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgsel(&self) -> Dbgsel {
        let val = (self.0 >> 20usize) & 0x0f;
        Dbgsel::from_bits(val as u8)
    }
    #[doc = "Debug Output Selection."]
    #[inline(always)]
    pub const fn set_dbgsel(&mut self, val: Dbgsel) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("chen", &self.chen())
            .field("pfen", &self.pfen())
            .field("dmaen", &self.dmaen())
            .field("scden", &self.scden())
            .field("caden", &self.caden())
            .field("zcden", &self.zcden())
            .field("lmten", &self.lmten())
            .field("fifoen", &self.fifoen())
            .field("dbgsel", &self.dbgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ chen: {:?}, pfen: {:?}, dmaen: {:?}, scden: {:?}, caden: {:?}, zcden: {:?}, lmten: {:?}, fifoen: {:?}, dbgsel: {:?} }}",
            self.chen(),
            self.pfen(),
            self.dmaen(),
            self.scden(),
            self.caden(),
            self.zcden(),
            self.lmten(),
            self.fifoen(),
            self.dbgsel()
        )
    }
}
#[doc = "Channel n Debug."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbgr(pub u32);
impl Cdbgr {
    #[doc = "Debug Data."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Debug Data."]
    #[inline(always)]
    pub const fn set_dbgdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cdbgr {
    #[inline(always)]
    fn default() -> Cdbgr {
        Cdbgr(0)
    }
}
impl core::fmt::Debug for Cdbgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdbgr")
            .field("dbgdata", &self.dbgdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdbgr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cdbgr {{ dbgdata: {=u32:?} }}", self.dbgdata())
    }
}
#[doc = "Channel n Data Rate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc = "PF OSR."]
    #[must_use]
    #[inline(always)]
    pub const fn pfosr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "PF OSR."]
    #[inline(always)]
    pub const fn set_pfosr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "PF Order."]
    #[must_use]
    #[inline(always)]
    pub const fn pford(&self) -> Pford {
        let val = (self.0 >> 11usize) & 0x03;
        Pford::from_bits(val as u8)
    }
    #[doc = "PF Order."]
    #[inline(always)]
    pub const fn set_pford(&mut self, val: Pford) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
    }
    #[doc = "PF Conversion Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn pfcm(&self) -> Pfcm {
        let val = (self.0 >> 14usize) & 0x03;
        Pfcm::from_bits(val as u8)
    }
    #[doc = "PF Conversion Mode."]
    #[inline(always)]
    pub const fn set_pfcm(&mut self, val: Pfcm) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        Cdr(0)
    }
}
impl core::fmt::Debug for Cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdr")
            .field("pfosr", &self.pfosr())
            .field("pford", &self.pford())
            .field("pfcm", &self.pfcm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdr {{ pfosr: {=u16:?}, pford: {:?}, pfcm: {:?} }}",
            self.pfosr(),
            self.pford(),
            self.pfcm()
        )
    }
}
#[doc = "Channel n High Limit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chilmt(pub u32);
impl Chilmt {
    #[doc = "High Limit Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn hilmt(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "High Limit Threshold."]
    #[inline(always)]
    pub const fn set_hilmt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Chilmt {
    #[inline(always)]
    fn default() -> Chilmt {
        Chilmt(0)
    }
}
impl core::fmt::Debug for Chilmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chilmt")
            .field("hilmt", &self.hilmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chilmt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Chilmt {{ hilmt: {=u32:?} }}", self.hilmt())
    }
}
#[doc = "Channel n Low Limit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clolmt(pub u32);
impl Clolmt {
    #[doc = "Low Limit Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn lolmt(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Low Limit Threshold."]
    #[inline(always)]
    pub const fn set_lolmt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Clolmt {
    #[inline(always)]
    fn default() -> Clolmt {
        Clolmt(0)
    }
}
impl core::fmt::Debug for Clolmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clolmt")
            .field("lolmt", &self.lolmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clolmt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clolmt {{ lolmt: {=u32:?} }}", self.lolmt())
    }
}
#[doc = "Channel n Multipurpose Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpdata(pub u32);
impl Cmpdata {
    #[doc = "Multipurpose Data."]
    #[must_use]
    #[inline(always)]
    pub const fn mpdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Multipurpose Data."]
    #[inline(always)]
    pub const fn set_mpdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmpdata {
    #[inline(always)]
    fn default() -> Cmpdata {
        Cmpdata(0)
    }
}
impl core::fmt::Debug for Cmpdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpdata")
            .field("mpdata", &self.mpdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmpdata {{ mpdata: {=u32:?} }}", self.mpdata())
    }
}
#[doc = "Channel n Protection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cprot(pub u32);
impl Cprot {
    #[doc = "SCD Limit Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn scdlmt(&self) -> Scdlmt {
        let val = (self.0 >> 0usize) & 0xff;
        Scdlmt::from_bits(val as u8)
    }
    #[doc = "SCD Limit Threshold."]
    #[inline(always)]
    pub const fn set_scdlmt(&mut self, val: Scdlmt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "SCD Conversion Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn scdcm(&self) -> Scdcm {
        let val = (self.0 >> 11usize) & 0x01;
        Scdcm::from_bits(val as u8)
    }
    #[doc = "SCD Conversion Mode."]
    #[inline(always)]
    pub const fn set_scdcm(&mut self, val: Scdcm) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SCD Option."]
    #[must_use]
    #[inline(always)]
    pub const fn scdop(&self) -> Scdop {
        let val = (self.0 >> 12usize) & 0x03;
        Scdop::from_bits(val as u8)
    }
    #[doc = "SCD Option."]
    #[inline(always)]
    pub const fn set_scdop(&mut self, val: Scdop) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Limit Detection Option."]
    #[must_use]
    #[inline(always)]
    pub const fn lmtop(&self) -> Lmtop {
        let val = (self.0 >> 14usize) & 0x03;
        Lmtop::from_bits(val as u8)
    }
    #[doc = "Limit Detection Option."]
    #[inline(always)]
    pub const fn set_lmtop(&mut self, val: Lmtop) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "CAD Limit Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn cadlmt(&self) -> Cadlmt {
        let val = (self.0 >> 16usize) & 0x0f;
        Cadlmt::from_bits(val as u8)
    }
    #[doc = "CAD Limit Threshold."]
    #[inline(always)]
    pub const fn set_cadlmt(&mut self, val: Cadlmt) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "CAD Break Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn cadbk(&self) -> Cadbk {
        let val = (self.0 >> 26usize) & 0x01;
        Cadbk::from_bits(val as u8)
    }
    #[doc = "CAD Break Signal."]
    #[inline(always)]
    pub const fn set_cadbk(&mut self, val: Cadbk) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SCD Break Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn scdbk(&self) -> Scdbk {
        let val = (self.0 >> 27usize) & 0x01;
        Scdbk::from_bits(val as u8)
    }
    #[doc = "SCD Break Signal."]
    #[inline(always)]
    pub const fn set_scdbk(&mut self, val: Scdbk) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Low Limit Break Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn llmtbk(&self) -> Llmtbk {
        let val = (self.0 >> 29usize) & 0x01;
        Llmtbk::from_bits(val as u8)
    }
    #[doc = "Low Limit Break Signal."]
    #[inline(always)]
    pub const fn set_llmtbk(&mut self, val: Llmtbk) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Window Limit Break Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn wlmtbk(&self) -> Wlmtbk {
        let val = (self.0 >> 30usize) & 0x01;
        Wlmtbk::from_bits(val as u8)
    }
    #[doc = "Window Limit Break Signal."]
    #[inline(always)]
    pub const fn set_wlmtbk(&mut self, val: Wlmtbk) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "High Limit Break Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn hlmtbk(&self) -> Hlmtbk {
        let val = (self.0 >> 31usize) & 0x01;
        Hlmtbk::from_bits(val as u8)
    }
    #[doc = "High Limit Break Signal."]
    #[inline(always)]
    pub const fn set_hlmtbk(&mut self, val: Hlmtbk) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cprot {
    #[inline(always)]
    fn default() -> Cprot {
        Cprot(0)
    }
}
impl core::fmt::Debug for Cprot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cprot")
            .field("scdlmt", &self.scdlmt())
            .field("scdcm", &self.scdcm())
            .field("scdop", &self.scdop())
            .field("lmtop", &self.lmtop())
            .field("cadlmt", &self.cadlmt())
            .field("cadbk", &self.cadbk())
            .field("scdbk", &self.scdbk())
            .field("llmtbk", &self.llmtbk())
            .field("wlmtbk", &self.wlmtbk())
            .field("hlmtbk", &self.hlmtbk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cprot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cprot {{ scdlmt: {:?}, scdcm: {:?}, scdop: {:?}, lmtop: {:?}, cadlmt: {:?}, cadbk: {:?}, scdbk: {:?}, llmtbk: {:?}, wlmtbk: {:?}, hlmtbk: {:?} }}",
            self.scdlmt(),
            self.scdcm(),
            self.scdop(),
            self.lmtop(),
            self.cadlmt(),
            self.cadbk(),
            self.scdbk(),
            self.llmtbk(),
            self.wlmtbk(),
            self.hlmtbk()
        )
    }
}
#[doc = "Channel n Result Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crdata(pub u32);
impl Crdata {
    #[doc = "Result Data."]
    #[must_use]
    #[inline(always)]
    pub const fn rdata(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Result Data."]
    #[inline(always)]
    pub const fn set_rdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Crdata {
    #[inline(always)]
    fn default() -> Crdata {
        Crdata(0)
    }
}
impl core::fmt::Debug for Crdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crdata")
            .field("rdata", &self.rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Crdata {{ rdata: {=u32:?} }}", self.rdata())
    }
}
#[doc = "Channel n Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "FIFO Available Data."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoavil(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "FIFO Available Data."]
    #[inline(always)]
    pub const fn set_fifoavil(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Parallel or Serial Data Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn psrdy(&self) -> Psrdy {
        let val = (self.0 >> 7usize) & 0x01;
        Psrdy::from_bits(val as u8)
    }
    #[doc = "Parallel or Serial Data Ready."]
    #[inline(always)]
    pub const fn set_psrdy(&mut self, val: Psrdy) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Primary CIC Saturation Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pfsat(&self) -> Pfsat {
        let val = (self.0 >> 8usize) & 0x01;
        Pfsat::from_bits(val as u8)
    }
    #[doc = "Primary CIC Saturation Flag."]
    #[inline(always)]
    pub const fn set_pfsat(&mut self, val: Pfsat) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "HPF Saturation Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hpfsat(&self) -> Hpfsat {
        let val = (self.0 >> 9usize) & 0x01;
        Hpfsat::from_bits(val as u8)
    }
    #[doc = "HPF Saturation Flag."]
    #[inline(always)]
    pub const fn set_hpfsat(&mut self, val: Hpfsat) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Shift Saturation Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sftsat(&self) -> Sftsat {
        let val = (self.0 >> 10usize) & 0x01;
        Sftsat::from_bits(val as u8)
    }
    #[doc = "Shift Saturation Flag."]
    #[inline(always)]
    pub const fn set_sftsat(&mut self, val: Sftsat) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bias Saturation Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn biassat(&self) -> Biassat {
        let val = (self.0 >> 11usize) & 0x01;
        Biassat::from_bits(val as u8)
    }
    #[doc = "Bias Saturation Flag."]
    #[inline(always)]
    pub const fn set_biassat(&mut self, val: Biassat) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Result Data Direct Read Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rdrs(&self) -> Rdrs {
        let val = (self.0 >> 12usize) & 0x01;
        Rdrs::from_bits(val as u8)
    }
    #[doc = "Result Data Direct Read Status."]
    #[inline(always)]
    pub const fn set_rdrs(&mut self, val: Rdrs) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Start Read Debug Data Sync."]
    #[must_use]
    #[inline(always)]
    pub const fn srds(&self) -> Srds {
        let val = (self.0 >> 13usize) & 0x01;
        Srds::from_bits(val as u8)
    }
    #[doc = "Start Read Debug Data Sync."]
    #[inline(always)]
    pub const fn set_srds(&mut self, val: Srds) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Debug Data Read Status."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgrs(&self) -> Dbgrs {
        let val = (self.0 >> 14usize) & 0x03;
        Dbgrs::from_bits(val as u8)
    }
    #[doc = "Debug Data Read Status."]
    #[inline(always)]
    pub const fn set_dbgrs(&mut self, val: Dbgrs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Number Of Conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn cnum(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number Of Conversions."]
    #[inline(always)]
    pub const fn set_cnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Overflow In Number Of Conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn cnum_ov(&self) -> CnumOv {
        let val = (self.0 >> 23usize) & 0x01;
        CnumOv::from_bits(val as u8)
    }
    #[doc = "Overflow In Number Of Conversions."]
    #[inline(always)]
    pub const fn set_cnum_ov(&mut self, val: CnumOv) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("fifoavil", &self.fifoavil())
            .field("psrdy", &self.psrdy())
            .field("pfsat", &self.pfsat())
            .field("hpfsat", &self.hpfsat())
            .field("sftsat", &self.sftsat())
            .field("biassat", &self.biassat())
            .field("rdrs", &self.rdrs())
            .field("srds", &self.srds())
            .field("dbgrs", &self.dbgrs())
            .field("cnum", &self.cnum())
            .field("cnum_ov", &self.cnum_ov())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ fifoavil: {=u8:?}, psrdy: {:?}, pfsat: {:?}, hpfsat: {:?}, sftsat: {:?}, biassat: {:?}, rdrs: {:?}, srds: {:?}, dbgrs: {:?}, cnum: {=u8:?}, cnum_ov: {:?} }}",
            self.fifoavil(),
            self.psrdy(),
            self.pfsat(),
            self.hpfsat(),
            self.sftsat(),
            self.biassat(),
            self.rdrs(),
            self.srds(),
            self.dbgrs(),
            self.cnum(),
            self.cnum_ov()
        )
    }
}
#[doc = "Error Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eie(pub u32);
impl Eie {
    #[doc = "Short Circuit Detected Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn scdie(&self, n: usize) -> Scdie {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Scdie::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Interrupt Enable."]
    #[inline(always)]
    pub const fn set_scdie(&mut self, n: usize, val: Scdie) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Window Limit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wlmtie(&self, n: usize) -> Wlmtie {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Wlmtie::from_bits(val as u8)
    }
    #[doc = "Window Limit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wlmtie(&mut self, n: usize, val: Wlmtie) {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Low Limit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn llmtie(&self, n: usize) -> Llmtie {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Llmtie::from_bits(val as u8)
    }
    #[doc = "Low Limit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_llmtie(&mut self, n: usize, val: Llmtie) {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "High Limit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hlmtie(&self, n: usize) -> Hlmtie {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Hlmtie::from_bits(val as u8)
    }
    #[doc = "High Limit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hlmtie(&mut self, n: usize, val: Hlmtie) {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Eie {
    #[inline(always)]
    fn default() -> Eie {
        Eie(0)
    }
}
impl core::fmt::Debug for Eie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eie")
            .field("scdie[0]", &self.scdie(0usize))
            .field("scdie[1]", &self.scdie(1usize))
            .field("scdie[2]", &self.scdie(2usize))
            .field("scdie[3]", &self.scdie(3usize))
            .field("scdie[4]", &self.scdie(4usize))
            .field("wlmtie[0]", &self.wlmtie(0usize))
            .field("wlmtie[1]", &self.wlmtie(1usize))
            .field("wlmtie[2]", &self.wlmtie(2usize))
            .field("wlmtie[3]", &self.wlmtie(3usize))
            .field("wlmtie[4]", &self.wlmtie(4usize))
            .field("llmtie[0]", &self.llmtie(0usize))
            .field("llmtie[1]", &self.llmtie(1usize))
            .field("llmtie[2]", &self.llmtie(2usize))
            .field("llmtie[3]", &self.llmtie(3usize))
            .field("llmtie[4]", &self.llmtie(4usize))
            .field("hlmtie[0]", &self.hlmtie(0usize))
            .field("hlmtie[1]", &self.hlmtie(1usize))
            .field("hlmtie[2]", &self.hlmtie(2usize))
            .field("hlmtie[3]", &self.hlmtie(3usize))
            .field("hlmtie[4]", &self.hlmtie(4usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eie {{ scdie[0]: {:?}, scdie[1]: {:?}, scdie[2]: {:?}, scdie[3]: {:?}, scdie[4]: {:?}, wlmtie[0]: {:?}, wlmtie[1]: {:?}, wlmtie[2]: {:?}, wlmtie[3]: {:?}, wlmtie[4]: {:?}, llmtie[0]: {:?}, llmtie[1]: {:?}, llmtie[2]: {:?}, llmtie[3]: {:?}, llmtie[4]: {:?}, hlmtie[0]: {:?}, hlmtie[1]: {:?}, hlmtie[2]: {:?}, hlmtie[3]: {:?}, hlmtie[4]: {:?} }}",
            self.scdie(0usize),
            self.scdie(1usize),
            self.scdie(2usize),
            self.scdie(3usize),
            self.scdie(4usize),
            self.wlmtie(0usize),
            self.wlmtie(1usize),
            self.wlmtie(2usize),
            self.wlmtie(3usize),
            self.wlmtie(4usize),
            self.llmtie(0usize),
            self.llmtie(1usize),
            self.llmtie(2usize),
            self.llmtie(3usize),
            self.llmtie(4usize),
            self.hlmtie(0usize),
            self.hlmtie(1usize),
            self.hlmtie(2usize),
            self.hlmtie(3usize),
            self.hlmtie(4usize)
        )
    }
}
#[doc = "Error Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eis(pub u32);
impl Eis {
    #[doc = "Short Circuit Detected Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn scd(&self, n: usize) -> Scd {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Scd::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Flag."]
    #[inline(always)]
    pub const fn set_scd(&mut self, n: usize, val: Scd) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Window Limit Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wlmt(&self, n: usize) -> Wlmt {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Wlmt::from_bits(val as u8)
    }
    #[doc = "Window Limit Flag."]
    #[inline(always)]
    pub const fn set_wlmt(&mut self, n: usize, val: Wlmt) {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Low Limit Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn llmt(&self, n: usize) -> Llmt {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Llmt::from_bits(val as u8)
    }
    #[doc = "Low Limit Flag."]
    #[inline(always)]
    pub const fn set_llmt(&mut self, n: usize, val: Llmt) {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "High Limit Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn hlmt(&self, n: usize) -> Hlmt {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Hlmt::from_bits(val as u8)
    }
    #[doc = "High Limit Flag."]
    #[inline(always)]
    pub const fn set_hlmt(&mut self, n: usize, val: Hlmt) {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Eis {
    #[inline(always)]
    fn default() -> Eis {
        Eis(0)
    }
}
impl core::fmt::Debug for Eis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eis")
            .field("scd[0]", &self.scd(0usize))
            .field("scd[1]", &self.scd(1usize))
            .field("scd[2]", &self.scd(2usize))
            .field("scd[3]", &self.scd(3usize))
            .field("scd[4]", &self.scd(4usize))
            .field("wlmt[0]", &self.wlmt(0usize))
            .field("wlmt[1]", &self.wlmt(1usize))
            .field("wlmt[2]", &self.wlmt(2usize))
            .field("wlmt[3]", &self.wlmt(3usize))
            .field("wlmt[4]", &self.wlmt(4usize))
            .field("llmt[0]", &self.llmt(0usize))
            .field("llmt[1]", &self.llmt(1usize))
            .field("llmt[2]", &self.llmt(2usize))
            .field("llmt[3]", &self.llmt(3usize))
            .field("llmt[4]", &self.llmt(4usize))
            .field("hlmt[0]", &self.hlmt(0usize))
            .field("hlmt[1]", &self.hlmt(1usize))
            .field("hlmt[2]", &self.hlmt(2usize))
            .field("hlmt[3]", &self.hlmt(3usize))
            .field("hlmt[4]", &self.hlmt(4usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eis {{ scd[0]: {:?}, scd[1]: {:?}, scd[2]: {:?}, scd[3]: {:?}, scd[4]: {:?}, wlmt[0]: {:?}, wlmt[1]: {:?}, wlmt[2]: {:?}, wlmt[3]: {:?}, wlmt[4]: {:?}, llmt[0]: {:?}, llmt[1]: {:?}, llmt[2]: {:?}, llmt[3]: {:?}, llmt[4]: {:?}, hlmt[0]: {:?}, hlmt[1]: {:?}, hlmt[2]: {:?}, hlmt[3]: {:?}, hlmt[4]: {:?} }}",
            self.scd(0usize),
            self.scd(1usize),
            self.scd(2usize),
            self.scd(3usize),
            self.scd(4usize),
            self.wlmt(0usize),
            self.wlmt(1usize),
            self.wlmt(2usize),
            self.wlmt(3usize),
            self.wlmt(4usize),
            self.llmt(0usize),
            self.llmt(1usize),
            self.llmt(2usize),
            self.llmt(3usize),
            self.llmt(4usize),
            self.hlmt(0usize),
            self.hlmt(1usize),
            self.hlmt(2usize),
            self.hlmt(3usize),
            self.hlmt(4usize)
        )
    }
}
#[doc = "FIFO And CAD Error Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoie(pub u32);
impl Fifoie {
    #[doc = "FIFO Underflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn funfie(&self, n: usize) -> Funfie {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Funfie::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_funfie(&mut self, n: usize, val: Funfie) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "FIFO Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfie(&self, n: usize) -> Fovfie {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Fovfie::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fovfie(&mut self, n: usize, val: Fovfie) {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Clock Absence Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cadie(&self, n: usize) -> Cadie {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Cadie::from_bits(val as u8)
    }
    #[doc = "Clock Absence Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cadie(&mut self, n: usize, val: Cadie) {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Saturation Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn satie(&self, n: usize) -> Satie {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Satie::from_bits(val as u8)
    }
    #[doc = "Saturation Interrupt Enable."]
    #[inline(always)]
    pub const fn set_satie(&mut self, n: usize, val: Satie) {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Fifoie {
    #[inline(always)]
    fn default() -> Fifoie {
        Fifoie(0)
    }
}
impl core::fmt::Debug for Fifoie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoie")
            .field("funfie[0]", &self.funfie(0usize))
            .field("funfie[1]", &self.funfie(1usize))
            .field("funfie[2]", &self.funfie(2usize))
            .field("funfie[3]", &self.funfie(3usize))
            .field("funfie[4]", &self.funfie(4usize))
            .field("fovfie[0]", &self.fovfie(0usize))
            .field("fovfie[1]", &self.fovfie(1usize))
            .field("fovfie[2]", &self.fovfie(2usize))
            .field("fovfie[3]", &self.fovfie(3usize))
            .field("fovfie[4]", &self.fovfie(4usize))
            .field("cadie[0]", &self.cadie(0usize))
            .field("cadie[1]", &self.cadie(1usize))
            .field("cadie[2]", &self.cadie(2usize))
            .field("cadie[3]", &self.cadie(3usize))
            .field("cadie[4]", &self.cadie(4usize))
            .field("satie[0]", &self.satie(0usize))
            .field("satie[1]", &self.satie(1usize))
            .field("satie[2]", &self.satie(2usize))
            .field("satie[3]", &self.satie(3usize))
            .field("satie[4]", &self.satie(4usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifoie {{ funfie[0]: {:?}, funfie[1]: {:?}, funfie[2]: {:?}, funfie[3]: {:?}, funfie[4]: {:?}, fovfie[0]: {:?}, fovfie[1]: {:?}, fovfie[2]: {:?}, fovfie[3]: {:?}, fovfie[4]: {:?}, cadie[0]: {:?}, cadie[1]: {:?}, cadie[2]: {:?}, cadie[3]: {:?}, cadie[4]: {:?}, satie[0]: {:?}, satie[1]: {:?}, satie[2]: {:?}, satie[3]: {:?}, satie[4]: {:?} }}",
            self.funfie(0usize),
            self.funfie(1usize),
            self.funfie(2usize),
            self.funfie(3usize),
            self.funfie(4usize),
            self.fovfie(0usize),
            self.fovfie(1usize),
            self.fovfie(2usize),
            self.fovfie(3usize),
            self.fovfie(4usize),
            self.cadie(0usize),
            self.cadie(1usize),
            self.cadie(2usize),
            self.cadie(3usize),
            self.cadie(4usize),
            self.satie(0usize),
            self.satie(1usize),
            self.satie(2usize),
            self.satie(3usize),
            self.satie(4usize)
        )
    }
}
#[doc = "FIFO And CAD Error Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifois(pub u32);
impl Fifois {
    #[doc = "FIFO Underflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn funf(&self, n: usize) -> Funf {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Funf::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Flag."]
    #[inline(always)]
    pub const fn set_funf(&mut self, n: usize, val: Funf) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "FIFO Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fovf(&self, n: usize) -> Fovf {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Fovf::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Flag."]
    #[inline(always)]
    pub const fn set_fovf(&mut self, n: usize, val: Fovf) {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Clock Absence Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cad(&self, n: usize) -> Cad {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Cad::from_bits(val as u8)
    }
    #[doc = "Clock Absence Flag."]
    #[inline(always)]
    pub const fn set_cad(&mut self, n: usize, val: Cad) {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Saturation Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sat(&self, n: usize) -> Sat {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Sat::from_bits(val as u8)
    }
    #[doc = "Saturation Flag."]
    #[inline(always)]
    pub const fn set_sat(&mut self, n: usize, val: Sat) {
        assert!(n < 5usize);
        let offs = 24usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Fifois {
    #[inline(always)]
    fn default() -> Fifois {
        Fifois(0)
    }
}
impl core::fmt::Debug for Fifois {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifois")
            .field("funf[0]", &self.funf(0usize))
            .field("funf[1]", &self.funf(1usize))
            .field("funf[2]", &self.funf(2usize))
            .field("funf[3]", &self.funf(3usize))
            .field("funf[4]", &self.funf(4usize))
            .field("fovf[0]", &self.fovf(0usize))
            .field("fovf[1]", &self.fovf(1usize))
            .field("fovf[2]", &self.fovf(2usize))
            .field("fovf[3]", &self.fovf(3usize))
            .field("fovf[4]", &self.fovf(4usize))
            .field("cad[0]", &self.cad(0usize))
            .field("cad[1]", &self.cad(1usize))
            .field("cad[2]", &self.cad(2usize))
            .field("cad[3]", &self.cad(3usize))
            .field("cad[4]", &self.cad(4usize))
            .field("sat[0]", &self.sat(0usize))
            .field("sat[1]", &self.sat(1usize))
            .field("sat[2]", &self.sat(2usize))
            .field("sat[3]", &self.sat(3usize))
            .field("sat[4]", &self.sat(4usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifois {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifois {{ funf[0]: {:?}, funf[1]: {:?}, funf[2]: {:?}, funf[3]: {:?}, funf[4]: {:?}, fovf[0]: {:?}, fovf[1]: {:?}, fovf[2]: {:?}, fovf[3]: {:?}, fovf[4]: {:?}, cad[0]: {:?}, cad[1]: {:?}, cad[2]: {:?}, cad[3]: {:?}, cad[4]: {:?}, sat[0]: {:?}, sat[1]: {:?}, sat[2]: {:?}, sat[3]: {:?}, sat[4]: {:?} }}",
            self.funf(0usize),
            self.funf(1usize),
            self.funf(2usize),
            self.funf(3usize),
            self.funf(4usize),
            self.fovf(0usize),
            self.fovf(1usize),
            self.fovf(2usize),
            self.fovf(3usize),
            self.fovf(4usize),
            self.cad(0usize),
            self.cad(1usize),
            self.cad(2usize),
            self.cad(3usize),
            self.cad(4usize),
            self.sat(0usize),
            self.sat(1usize),
            self.sat(2usize),
            self.sat(3usize),
            self.sat(4usize)
        )
    }
}
#[doc = "Main Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Software Trigger For Channel n."]
    #[must_use]
    #[inline(always)]
    pub const fn strig(&self, n: usize) -> Strig {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Strig::from_bits(val as u8)
    }
    #[doc = "Software Trigger For Channel n."]
    #[inline(always)]
    pub const fn set_strig(&mut self, n: usize, val: Strig) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Doze Or Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> Dozen {
        let val = (self.0 >> 10usize) & 0x01;
        Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Or Stop Enable."]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: Dozen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> Rst {
        let val = (self.0 >> 13usize) & 0x01;
        Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: Rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Master Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn men(&self) -> Men {
        let val = (self.0 >> 15usize) & 0x01;
        Men::from_bits(val as u8)
    }
    #[doc = "Master Enable."]
    #[inline(always)]
    pub const fn set_men(&mut self, val: Men) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Modulator Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn mclkdiv(&self) -> Mclkdiv {
        let val = (self.0 >> 16usize) & 0xff;
        Mclkdiv::from_bits(val as u8)
    }
    #[doc = "Modulator Clock Divider."]
    #[inline(always)]
    pub const fn set_mclkdiv(&mut self, val: Mclkdiv) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Prescale Before Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> Prescale {
        let val = (self.0 >> 25usize) & 0x03;
        Prescale::from_bits(val as u8)
    }
    #[doc = "Prescale Before Clock Divider."]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: Prescale) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Disable Modulator Clock n Output."]
    #[must_use]
    #[inline(always)]
    pub const fn mclkdis(&self, n: usize) -> Mclkdis {
        assert!(n < 3usize);
        let offs = 27usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Mclkdis::from_bits(val as u8)
    }
    #[doc = "Disable Modulator Clock n Output."]
    #[inline(always)]
    pub const fn set_mclkdis(&mut self, n: usize, val: Mclkdis) {
        assert!(n < 3usize);
        let offs = 27usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("strig[0]", &self.strig(0usize))
            .field("strig[1]", &self.strig(1usize))
            .field("strig[2]", &self.strig(2usize))
            .field("strig[3]", &self.strig(3usize))
            .field("strig[4]", &self.strig(4usize))
            .field("dozen", &self.dozen())
            .field("rst", &self.rst())
            .field("men", &self.men())
            .field("mclkdiv", &self.mclkdiv())
            .field("prescale", &self.prescale())
            .field("mclkdis[0]", &self.mclkdis(0usize))
            .field("mclkdis[1]", &self.mclkdis(1usize))
            .field("mclkdis[2]", &self.mclkdis(2usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ strig[0]: {:?}, strig[1]: {:?}, strig[2]: {:?}, strig[3]: {:?}, strig[4]: {:?}, dozen: {:?}, rst: {:?}, men: {:?}, mclkdiv: {:?}, prescale: {:?}, mclkdis[0]: {:?}, mclkdis[1]: {:?}, mclkdis[2]: {:?} }}",
            self.strig(0usize),
            self.strig(1usize),
            self.strig(2usize),
            self.strig(3usize),
            self.strig(4usize),
            self.dozen(),
            self.rst(),
            self.men(),
            self.mclkdiv(),
            self.prescale(),
            self.mclkdis(0usize),
            self.mclkdis(1usize),
            self.mclkdis(2usize)
        )
    }
}
#[doc = "Normal Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nie(pub u32);
impl Nie {
    #[doc = "Conversion Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cocie(&self, n: usize) -> Cocie {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Cocie::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cocie(&mut self, n: usize, val: Cocie) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Data Output Ready Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chfie(&self, n: usize) -> Chfie {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Chfie::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Interrupt Enable."]
    #[inline(always)]
    pub const fn set_chfie(&mut self, n: usize, val: Chfie) {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Zero Cross Detected Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn zcdie(&self, n: usize) -> Zcdie {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Zcdie::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Interrupt Enable."]
    #[inline(always)]
    pub const fn set_zcdie(&mut self, n: usize, val: Zcdie) {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Nie {
    #[inline(always)]
    fn default() -> Nie {
        Nie(0)
    }
}
impl core::fmt::Debug for Nie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nie")
            .field("cocie[0]", &self.cocie(0usize))
            .field("cocie[1]", &self.cocie(1usize))
            .field("cocie[2]", &self.cocie(2usize))
            .field("cocie[3]", &self.cocie(3usize))
            .field("cocie[4]", &self.cocie(4usize))
            .field("chfie[0]", &self.chfie(0usize))
            .field("chfie[1]", &self.chfie(1usize))
            .field("chfie[2]", &self.chfie(2usize))
            .field("chfie[3]", &self.chfie(3usize))
            .field("chfie[4]", &self.chfie(4usize))
            .field("zcdie[0]", &self.zcdie(0usize))
            .field("zcdie[1]", &self.zcdie(1usize))
            .field("zcdie[2]", &self.zcdie(2usize))
            .field("zcdie[3]", &self.zcdie(3usize))
            .field("zcdie[4]", &self.zcdie(4usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nie {{ cocie[0]: {:?}, cocie[1]: {:?}, cocie[2]: {:?}, cocie[3]: {:?}, cocie[4]: {:?}, chfie[0]: {:?}, chfie[1]: {:?}, chfie[2]: {:?}, chfie[3]: {:?}, chfie[4]: {:?}, zcdie[0]: {:?}, zcdie[1]: {:?}, zcdie[2]: {:?}, zcdie[3]: {:?}, zcdie[4]: {:?} }}",
            self.cocie(0usize),
            self.cocie(1usize),
            self.cocie(2usize),
            self.cocie(3usize),
            self.cocie(4usize),
            self.chfie(0usize),
            self.chfie(1usize),
            self.chfie(2usize),
            self.chfie(3usize),
            self.chfie(4usize),
            self.zcdie(0usize),
            self.zcdie(1usize),
            self.zcdie(2usize),
            self.zcdie(3usize),
            self.zcdie(4usize)
        )
    }
}
#[doc = "Normal Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nis(pub u32);
impl Nis {
    #[doc = "Conversion Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn coc(&self, n: usize) -> Coc {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Coc::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Flag."]
    #[inline(always)]
    pub const fn set_coc(&mut self, n: usize, val: Coc) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Data Output Ready Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn chf(&self, n: usize) -> Chf {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Chf::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Flag."]
    #[inline(always)]
    pub const fn set_chf(&mut self, n: usize, val: Chf) {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Zero Cross Detected Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn zcd(&self, n: usize) -> Zcd {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Zcd::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Flag."]
    #[inline(always)]
    pub const fn set_zcd(&mut self, n: usize, val: Zcd) {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Nis {
    #[inline(always)]
    fn default() -> Nis {
        Nis(0)
    }
}
impl core::fmt::Debug for Nis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nis")
            .field("coc[0]", &self.coc(0usize))
            .field("coc[1]", &self.coc(1usize))
            .field("coc[2]", &self.coc(2usize))
            .field("coc[3]", &self.coc(3usize))
            .field("coc[4]", &self.coc(4usize))
            .field("chf[0]", &self.chf(0usize))
            .field("chf[1]", &self.chf(1usize))
            .field("chf[2]", &self.chf(2usize))
            .field("chf[3]", &self.chf(3usize))
            .field("chf[4]", &self.chf(4usize))
            .field("zcd[0]", &self.zcd(0usize))
            .field("zcd[1]", &self.zcd(1usize))
            .field("zcd[2]", &self.zcd(2usize))
            .field("zcd[3]", &self.zcd(3usize))
            .field("zcd[4]", &self.zcd(4usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nis {{ coc[0]: {:?}, coc[1]: {:?}, coc[2]: {:?}, coc[3]: {:?}, coc[4]: {:?}, chf[0]: {:?}, chf[1]: {:?}, chf[2]: {:?}, chf[3]: {:?}, chf[4]: {:?}, zcd[0]: {:?}, zcd[1]: {:?}, zcd[2]: {:?}, zcd[3]: {:?}, zcd[4]: {:?} }}",
            self.coc(0usize),
            self.coc(1usize),
            self.coc(2usize),
            self.coc(3usize),
            self.coc(4usize),
            self.chf(0usize),
            self.chf(1usize),
            self.chf(2usize),
            self.chf(3usize),
            self.chf(4usize),
            self.zcd(0usize),
            self.zcd(1usize),
            self.zcd(2usize),
            self.zcd(3usize),
            self.zcd(4usize)
        )
    }
}
#[doc = "Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parameter(pub u32);
impl Parameter {
    #[doc = "FIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_depth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "FIFO Depth."]
    #[inline(always)]
    pub const fn set_fifo_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Filter Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_num(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filter Channel Number."]
    #[inline(always)]
    pub const fn set_flt_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "PF Order Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pf_ord_sel(&self) -> PfOrdSel {
        let val = (self.0 >> 19usize) & 0x03;
        PfOrdSel::from_bits(val as u8)
    }
    #[doc = "PF Order Select."]
    #[inline(always)]
    pub const fn set_pf_ord_sel(&mut self, val: PfOrdSel) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
}
impl Default for Parameter {
    #[inline(always)]
    fn default() -> Parameter {
        Parameter(0)
    }
}
impl core::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Parameter")
            .field("fifo_depth", &self.fifo_depth())
            .field("flt_num", &self.flt_num())
            .field("pf_ord_sel", &self.pf_ord_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Parameter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Parameter {{ fifo_depth: {=u8:?}, flt_num: {=u8:?}, pf_ord_sel: {:?} }}",
            self.fifo_depth(),
            self.flt_num(),
            self.pf_ord_sel()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Conversion In Progress."]
    #[must_use]
    #[inline(always)]
    pub const fn cip(&self, n: usize) -> Cip {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Cip::from_bits(val as u8)
    }
    #[doc = "Conversion In Progress."]
    #[inline(always)]
    pub const fn set_cip(&mut self, n: usize, val: Cip) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Channel Ready For Conversion."]
    #[must_use]
    #[inline(always)]
    pub const fn chrdy(&self, n: usize) -> Chrdy {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Chrdy::from_bits(val as u8)
    }
    #[doc = "Channel Ready For Conversion."]
    #[inline(always)]
    pub const fn set_chrdy(&mut self, n: usize, val: Chrdy) {
        assert!(n < 5usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoempty(&self, n: usize) -> Fifoempty {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Fifoempty::from_bits(val as u8)
    }
    #[doc = "FIFO Empty."]
    #[inline(always)]
    pub const fn set_fifoempty(&mut self, n: usize, val: Fifoempty) {
        assert!(n < 5usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Modulator Clock n Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn mclkrdy(&self, n: usize) -> Mclkrdy {
        assert!(n < 3usize);
        let offs = 24usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Mclkrdy::from_bits(val as u8)
    }
    #[doc = "Modulator Clock n Ready."]
    #[inline(always)]
    pub const fn set_mclkrdy(&mut self, n: usize, val: Mclkrdy) {
        assert!(n < 3usize);
        let offs = 24usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
            .field("cip[0]", &self.cip(0usize))
            .field("cip[1]", &self.cip(1usize))
            .field("cip[2]", &self.cip(2usize))
            .field("cip[3]", &self.cip(3usize))
            .field("cip[4]", &self.cip(4usize))
            .field("chrdy[0]", &self.chrdy(0usize))
            .field("chrdy[1]", &self.chrdy(1usize))
            .field("chrdy[2]", &self.chrdy(2usize))
            .field("chrdy[3]", &self.chrdy(3usize))
            .field("chrdy[4]", &self.chrdy(4usize))
            .field("fifoempty[0]", &self.fifoempty(0usize))
            .field("fifoempty[1]", &self.fifoempty(1usize))
            .field("fifoempty[2]", &self.fifoempty(2usize))
            .field("fifoempty[3]", &self.fifoempty(3usize))
            .field("fifoempty[4]", &self.fifoempty(4usize))
            .field("mclkrdy[0]", &self.mclkrdy(0usize))
            .field("mclkrdy[1]", &self.mclkrdy(1usize))
            .field("mclkrdy[2]", &self.mclkrdy(2usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ cip[0]: {:?}, cip[1]: {:?}, cip[2]: {:?}, cip[3]: {:?}, cip[4]: {:?}, chrdy[0]: {:?}, chrdy[1]: {:?}, chrdy[2]: {:?}, chrdy[3]: {:?}, chrdy[4]: {:?}, fifoempty[0]: {:?}, fifoempty[1]: {:?}, fifoempty[2]: {:?}, fifoempty[3]: {:?}, fifoempty[4]: {:?}, mclkrdy[0]: {:?}, mclkrdy[1]: {:?}, mclkrdy[2]: {:?} }}",
            self.cip(0usize),
            self.cip(1usize),
            self.cip(2usize),
            self.cip(3usize),
            self.cip(4usize),
            self.chrdy(0usize),
            self.chrdy(1usize),
            self.chrdy(2usize),
            self.chrdy(3usize),
            self.chrdy(4usize),
            self.fifoempty(0usize),
            self.fifoempty(1usize),
            self.fifoempty(2usize),
            self.fifoempty(3usize),
            self.fifoempty(4usize),
            self.mclkrdy(0usize),
            self.mclkrdy(1usize),
            self.mclkrdy(2usize)
        )
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Code."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Code."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> Minor {
        let val = (self.0 >> 16usize) & 0xff;
        Minor::from_bits(val as u8)
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: Minor) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> Major {
        let val = (self.0 >> 24usize) & 0xff;
        Major::from_bits(val as u8)
    }
    #[doc = "Major Version Number."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: Major) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
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
            "Verid {{ feature: {=u16:?}, minor: {:?}, major: {:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admasel {
    #[doc = "Alternate DMA disabled."]
    Disabled = 0x0,
    #[doc = "PF conversion complete."]
    PfConvComplete = 0x01,
    #[doc = "PF data output ready."]
    PfDataReady = 0x02,
    #[doc = "Zero crossing detected."]
    Zcd = 0x03,
    #[doc = "Short circuit detected."]
    Scd = 0x04,
    #[doc = "Window limit detected."]
    WindowLmt = 0x05,
    #[doc = "Low limit detected."]
    LowLmt = 0x06,
    #[doc = "High limit."]
    HighLmt = 0x07,
    #[doc = "FIFO underflow."]
    FifoUf = 0x08,
    #[doc = "FIFO overflow."]
    FifoOf = 0x09,
    #[doc = "Clock absence."]
    ClkAbs = 0x0a,
    #[doc = "Saturation."]
    Sat = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Admasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admasel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admasel {
    #[inline(always)]
    fn from(val: u8) -> Admasel {
        Admasel::from_bits(val)
    }
}
impl From<Admasel> for u8 {
    #[inline(always)]
    fn from(val: Admasel) -> u8 {
        Admasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Biassat {
    #[doc = "Did not occur."]
    SatNo = 0x0,
    #[doc = "Occurred."]
    SatYes = 0x01,
}
impl Biassat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Biassat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Biassat {
    #[inline(always)]
    fn from(val: u8) -> Biassat {
        Biassat::from_bits(val)
    }
}
impl From<Biassat> for u8 {
    #[inline(always)]
    fn from(val: Biassat) -> u8 {
        Biassat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cad {
    #[doc = "Clock present."]
    CadNo = 0x0,
    #[doc = "Clock absent."]
    CadYes = 0x01,
}
impl Cad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cad {
    #[inline(always)]
    fn from(val: u8) -> Cad {
        Cad::from_bits(val)
    }
}
impl From<Cad> for u8 {
    #[inline(always)]
    fn from(val: Cad) -> u8 {
        Cad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadbk {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Cadbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadbk {
    #[inline(always)]
    fn from(val: u8) -> Cadbk {
        Cadbk::from_bits(val)
    }
}
impl From<Cadbk> for u8 {
    #[inline(always)]
    fn from(val: Cadbk) -> u8 {
        Cadbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Caden {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Caden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Caden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Caden {
    #[inline(always)]
    fn from(val: u8) -> Caden {
        Caden::from_bits(val)
    }
}
impl From<Caden> for u8 {
    #[inline(always)]
    fn from(val: Caden) -> u8 {
        Caden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Cadie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadie {
    #[inline(always)]
    fn from(val: u8) -> Cadie {
        Cadie::from_bits(val)
    }
}
impl From<Cadie> for u8 {
    #[inline(always)]
    fn from(val: Cadie) -> u8 {
        Cadie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cadlmt {
    #[doc = "Disables CAD."]
    Disables = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cadlmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cadlmt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cadlmt {
    #[inline(always)]
    fn from(val: u8) -> Cadlmt {
        Cadlmt::from_bits(val)
    }
}
impl From<Cadlmt> for u8 {
    #[inline(always)]
    fn from(val: Cadlmt) -> u8 {
        Cadlmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chen {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Chen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chen {
    #[inline(always)]
    fn from(val: u8) -> Chen {
        Chen::from_bits(val)
    }
}
impl From<Chen> for u8 {
    #[inline(always)]
    fn from(val: Chen) -> u8 {
        Chen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chf {
    #[doc = "No overflow; data not available."]
    OvflwNo = 0x0,
    #[doc = "Overflow; data available."]
    OvflwYes = 0x01,
}
impl Chf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chf {
    #[inline(always)]
    fn from(val: u8) -> Chf {
        Chf::from_bits(val)
    }
}
impl From<Chf> for u8 {
    #[inline(always)]
    fn from(val: Chf) -> u8 {
        Chf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chfie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Chfie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chfie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chfie {
    #[inline(always)]
    fn from(val: u8) -> Chfie {
        Chfie::from_bits(val)
    }
}
impl From<Chfie> for u8 {
    #[inline(always)]
    fn from(val: Chfie) -> u8 {
        Chfie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chrdy {
    #[doc = "Not ready."]
    ReadyNo = 0x0,
    #[doc = "Ready."]
    ReadyYes = 0x01,
}
impl Chrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chrdy {
    #[inline(always)]
    fn from(val: u8) -> Chrdy {
        Chrdy::from_bits(val)
    }
}
impl From<Chrdy> for u8 {
    #[inline(always)]
    fn from(val: Chrdy) -> u8 {
        Chrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cip {
    #[doc = "Not in progress."]
    ConvNo = 0x0,
    #[doc = "In progress."]
    ConvYes = 0x01,
}
impl Cip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cip {
    #[inline(always)]
    fn from(val: u8) -> Cip {
        Cip::from_bits(val)
    }
}
impl From<Cip> for u8 {
    #[inline(always)]
    fn from(val: Cip) -> u8 {
        Cip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CnumOv {
    #[doc = "No overflow."]
    OflwNo = 0x0,
    #[doc = "Overflow."]
    OflwYes = 0x01,
}
impl CnumOv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CnumOv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CnumOv {
    #[inline(always)]
    fn from(val: u8) -> CnumOv {
        CnumOv::from_bits(val)
    }
}
impl From<CnumOv> for u8 {
    #[inline(always)]
    fn from(val: CnumOv) -> u8 {
        CnumOv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coc {
    #[doc = "Not finished; data not available."]
    CocNo = 0x0,
    #[doc = "Finished; data available."]
    CocYes = 0x01,
}
impl Coc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Coc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Coc {
    #[inline(always)]
    fn from(val: u8) -> Coc {
        Coc::from_bits(val)
    }
}
impl From<Coc> for u8 {
    #[inline(always)]
    fn from(val: Coc) -> u8 {
        Coc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cocie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Cocie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cocie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cocie {
    #[inline(always)]
    fn from(val: u8) -> Cocie {
        Cocie::from_bits(val)
    }
}
impl From<Cocie> for u8 {
    #[inline(always)]
    fn from(val: Cocie) -> u8 {
        Cocie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgrs {
    #[doc = "Valid."]
    Valid = 0x0,
    #[doc = "Invalid."]
    Invalid1 = 0x01,
    #[doc = "Invalid."]
    Invalid2 = 0x02,
    #[doc = "Invalid."]
    Invalid3 = 0x03,
}
impl Dbgrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgrs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgrs {
    #[inline(always)]
    fn from(val: u8) -> Dbgrs {
        Dbgrs::from_bits(val)
    }
}
impl From<Dbgrs> for u8 {
    #[inline(always)]
    fn from(val: Dbgrs) -> u8 {
        Dbgrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgsel {
    #[doc = "Final data from the PF (24 bits)."]
    Rslt = 0x0,
    #[doc = "Offset data (24 bits)."]
    Pfbis = 0x01,
    #[doc = "Shifted data from the PF (24 bits)."]
    Pfsft = 0x02,
    #[doc = "DC remover (HPF) data (32 bits)."]
    Hpf = 0x03,
    #[doc = "Raw data from the PF's CIC filter."]
    Pfcic = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Historical data from SCD."]
    Scd = 0x06,
    #[doc = "Data from the Manchester decoder."]
    Mm = 0x07,
    #[doc = "Data from CAD."]
    Cad = 0x08,
    #[doc = "Number of available entries in the FIFO."]
    Fifo = 0x09,
    #[doc = "Status of the parallel or serial data converter."]
    Ps = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Dbgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgsel {
    #[inline(always)]
    fn from(val: u8) -> Dbgsel {
        Dbgsel::from_bits(val)
    }
}
impl From<Dbgsel> for u8 {
    #[inline(always)]
    fn from(val: Dbgsel) -> u8 {
        Dbgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaen {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enables."]
    Enabled = 0x0,
    #[doc = "Disables."]
    Disabled = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoempty {
    #[doc = "Not empty."]
    EmptyNo = 0x0,
    #[doc = "Empty."]
    EmptyYes = 0x01,
}
impl Fifoempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoempty {
    #[inline(always)]
    fn from(val: u8) -> Fifoempty {
        Fifoempty::from_bits(val)
    }
}
impl From<Fifoempty> for u8 {
    #[inline(always)]
    fn from(val: Fifoempty) -> u8 {
        Fifoempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoen {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Fifoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoen {
    #[inline(always)]
    fn from(val: u8) -> Fifoen {
        Fifoen::from_bits(val)
    }
}
impl From<Fifoen> for u8 {
    #[inline(always)]
    fn from(val: Fifoen) -> u8 {
        Fifoen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovf {
    #[doc = "Did not occur."]
    FovfNo = 0x0,
    #[doc = "Occurred."]
    FovfYes = 0x01,
}
impl Fovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovf {
    #[inline(always)]
    fn from(val: u8) -> Fovf {
        Fovf::from_bits(val)
    }
}
impl From<Fovf> for u8 {
    #[inline(always)]
    fn from(val: Fovf) -> u8 {
        Fovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fovfie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Fovfie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fovfie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fovfie {
    #[inline(always)]
    fn from(val: u8) -> Fovfie {
        Fovfie::from_bits(val)
    }
}
impl From<Fovfie> for u8 {
    #[inline(always)]
    fn from(val: Fovfie) -> u8 {
        Fovfie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funf {
    #[doc = "Did not occur."]
    FunfNo = 0x0,
    #[doc = "Occurred."]
    FunfYes = 0x01,
}
impl Funf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funf {
    #[inline(always)]
    fn from(val: u8) -> Funf {
        Funf::from_bits(val)
    }
}
impl From<Funf> for u8 {
    #[inline(always)]
    fn from(val: Funf) -> u8 {
        Funf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Funfie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Funfie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Funfie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Funfie {
    #[inline(always)]
    fn from(val: u8) -> Funfie {
        Funfie::from_bits(val)
    }
}
impl From<Funfie> for u8 {
    #[inline(always)]
    fn from(val: Funfie) -> u8 {
        Funfie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmt {
    #[doc = "Not exceeded."]
    HlmtNo = 0x0,
    #[doc = "Exceeded."]
    HlmtYes = 0x01,
}
impl Hlmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmt {
    #[inline(always)]
    fn from(val: u8) -> Hlmt {
        Hlmt::from_bits(val)
    }
}
impl From<Hlmt> for u8 {
    #[inline(always)]
    fn from(val: Hlmt) -> u8 {
        Hlmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtbk {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Hlmtbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtbk {
    #[inline(always)]
    fn from(val: u8) -> Hlmtbk {
        Hlmtbk::from_bits(val)
    }
}
impl From<Hlmtbk> for u8 {
    #[inline(always)]
    fn from(val: Hlmtbk) -> u8 {
        Hlmtbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlmtie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Hlmtie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlmtie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlmtie {
    #[inline(always)]
    fn from(val: u8) -> Hlmtie {
        Hlmtie::from_bits(val)
    }
}
impl From<Hlmtie> for u8 {
    #[inline(always)]
    fn from(val: Hlmtie) -> u8 {
        Hlmtie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpfsat {
    #[doc = "Did not occur."]
    SatNo = 0x0,
    #[doc = "Occurred."]
    SatYes = 0x01,
}
impl Hpfsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpfsat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpfsat {
    #[inline(always)]
    fn from(val: u8) -> Hpfsat {
        Hpfsat::from_bits(val)
    }
}
impl From<Hpfsat> for u8 {
    #[inline(always)]
    fn from(val: Hpfsat) -> u8 {
        Hpfsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibdly {
    #[doc = "Disabled."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ibdly {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibdly {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibdly {
    #[inline(always)]
    fn from(val: u8) -> Ibdly {
        Ibdly::from_bits(val)
    }
}
impl From<Ibdly> for u8 {
    #[inline(always)]
    fn from(val: Ibdly) -> u8 {
        Ibdly::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibfmt {
    #[doc = "External bitstream from the MBIT\\[n\\] signal."]
    E1b = 0x0,
    #[doc = "External Manchester code; ICESEL selects the rise or fall decoder."]
    Emb = 0x01,
    #[doc = "Internal 16-bit parallel data from MPDATA."]
    Ipb = 0x02,
    #[doc = "Internal 32-bit serial data from MPDATA."]
    Isb = 0x03,
}
impl Ibfmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibfmt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibfmt {
    #[inline(always)]
    fn from(val: u8) -> Ibfmt {
        Ibfmt::from_bits(val)
    }
}
impl From<Ibfmt> for u8 {
    #[inline(always)]
    fn from(val: Ibfmt) -> u8 {
        Ibfmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibsel {
    #[doc = "External bitstream from the MBIT\\[n\\] signal."]
    Epb = 0x0,
    #[doc = "Alternate internal bitstream from the INP\\[n\\] signal."]
    Esb = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Grouped bitstream shared with an adjacent channel; the adjacent channel's IBSEL field determines the input."]
    Grp = 0x03,
}
impl Ibsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibsel {
    #[inline(always)]
    fn from(val: u8) -> Ibsel {
        Ibsel::from_bits(val)
    }
}
impl From<Ibsel> for u8 {
    #[inline(always)]
    fn from(val: Ibsel) -> u8 {
        Ibsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icesel {
    _RESERVED_0 = 0x0,
    #[doc = "Positive edge."]
    Pos = 0x01,
    #[doc = "Negative edge."]
    Neg = 0x02,
    #[doc = "Both edges."]
    Both = 0x03,
    #[doc = "Every other odd positive edge."]
    Opos = 0x04,
    #[doc = "Every other even positive edge."]
    Epos = 0x05,
    #[doc = "Every other odd negative edge."]
    Oneg = 0x06,
    #[doc = "Every other even negative edge."]
    Eneg = 0x07,
}
impl Icesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icesel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icesel {
    #[inline(always)]
    fn from(val: u8) -> Icesel {
        Icesel::from_bits(val)
    }
}
impl From<Icesel> for u8 {
    #[inline(always)]
    fn from(val: Icesel) -> u8 {
        Icesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icsel {
    #[doc = "MCLK_OUT0 with internal routeback."]
    MclkOut0 = 0x0,
    #[doc = "MCLK_OUT1 with internal routeback."]
    MclkOut1 = 0x01,
    #[doc = "MCLK_OUT2 with internal routeback."]
    MclkOut2 = 0x02,
    #[doc = "External modulator clock dedicated to this channel."]
    Ext = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Grouped clock shared with an adjacent channel; the adjacent channel's ICSEL field determines the input clock."]
    Grp = 0x07,
}
impl Icsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icsel {
    #[inline(always)]
    fn from(val: u8) -> Icsel {
        Icsel::from_bits(val)
    }
}
impl From<Icsel> for u8 {
    #[inline(always)]
    fn from(val: Icsel) -> u8 {
        Icsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itlvl {
    #[doc = "Edge."]
    Edge = 0x0,
    #[doc = "Level."]
    Level = 0x01,
}
impl Itlvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itlvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itlvl {
    #[inline(always)]
    fn from(val: u8) -> Itlvl {
        Itlvl::from_bits(val)
    }
}
impl From<Itlvl> for u8 {
    #[inline(always)]
    fn from(val: Itlvl) -> u8 {
        Itlvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itsel {
    #[doc = "Software."]
    Sw = 0x0,
    #[doc = "Hardware trigger dedicated to the channel."]
    Hw = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Grouped trigger shared with an adjacent channel; the adjacent channel's ITSEL field determines the trigger."]
    Gp = 0x03,
}
impl Itsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itsel {
    #[inline(always)]
    fn from(val: u8) -> Itsel {
        Itsel::from_bits(val)
    }
}
impl From<Itsel> for u8 {
    #[inline(always)]
    fn from(val: Itsel) -> u8 {
        Itsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmt {
    #[doc = "Not exceeded."]
    LlmtNo = 0x0,
    #[doc = "Exceeded."]
    LlmtYes = 0x01,
}
impl Llmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmt {
    #[inline(always)]
    fn from(val: u8) -> Llmt {
        Llmt::from_bits(val)
    }
}
impl From<Llmt> for u8 {
    #[inline(always)]
    fn from(val: Llmt) -> u8 {
        Llmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtbk {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Llmtbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtbk {
    #[inline(always)]
    fn from(val: u8) -> Llmtbk {
        Llmtbk::from_bits(val)
    }
}
impl From<Llmtbk> for u8 {
    #[inline(always)]
    fn from(val: Llmtbk) -> u8 {
        Llmtbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Llmtie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Llmtie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Llmtie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Llmtie {
    #[inline(always)]
    fn from(val: u8) -> Llmtie {
        Llmtie::from_bits(val)
    }
}
impl From<Llmtie> for u8 {
    #[inline(always)]
    fn from(val: Llmtie) -> u8 {
        Llmtie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lmten {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Lmten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lmten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lmten {
    #[inline(always)]
    fn from(val: u8) -> Lmten {
        Lmten::from_bits(val)
    }
}
impl From<Lmten> for u8 {
    #[inline(always)]
    fn from(val: Lmten) -> u8 {
        Lmten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lmtop {
    #[doc = "Both high and low limits."]
    Both = 0x0,
    #[doc = "High limit."]
    High = 0x01,
    #[doc = "Low limit."]
    Low = 0x02,
    #[doc = "Windowed value."]
    Window = 0x03,
}
impl Lmtop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lmtop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lmtop {
    #[inline(always)]
    fn from(val: u8) -> Lmtop {
        Lmtop::from_bits(val)
    }
}
impl From<Lmtop> for u8 {
    #[inline(always)]
    fn from(val: Lmtop) -> u8 {
        Lmtop::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Major(u8);
impl Major {
    #[doc = "1.x."]
    pub const Ver1: Self = Self(0x01);
    #[doc = "2.x."]
    pub const Ver2: Self = Self(0x02);
}
impl Major {
    pub const fn from_bits(val: u8) -> Major {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Major {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Ver1"),
            0x02 => f.write_str("Ver2"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Major {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Ver1"),
            0x02 => defmt::write!(f, "Ver2"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Major {
    #[inline(always)]
    fn from(val: u8) -> Major {
        Major::from_bits(val)
    }
}
impl From<Major> for u8 {
    #[inline(always)]
    fn from(val: Major) -> u8 {
        Major::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkdis {
    #[doc = "Enabled when MEN = 1."]
    Enabled = 0x0,
    #[doc = "Disabled regardless of MEN value."]
    Disabled = 0x01,
}
impl Mclkdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkdis {
    #[inline(always)]
    fn from(val: u8) -> Mclkdis {
        Mclkdis::from_bits(val)
    }
}
impl From<Mclkdis> for u8 {
    #[inline(always)]
    fn from(val: Mclkdis) -> u8 {
        Mclkdis::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mclkdiv(u8);
impl Mclkdiv {
    #[doc = "Prohibited."]
    pub const Prohibited: Self = Self(0x0);
}
impl Mclkdiv {
    pub const fn from_bits(val: u8) -> Mclkdiv {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Mclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Prohibited"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mclkdiv {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Prohibited"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Mclkdiv {
        Mclkdiv::from_bits(val)
    }
}
impl From<Mclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Mclkdiv) -> u8 {
        Mclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkrdy {
    #[doc = "Not ready."]
    ReadyNo = 0x0,
    #[doc = "Ready."]
    ReadyYes = 0x01,
}
impl Mclkrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkrdy {
    #[inline(always)]
    fn from(val: u8) -> Mclkrdy {
        Mclkrdy::from_bits(val)
    }
}
impl From<Mclkrdy> for u8 {
    #[inline(always)]
    fn from(val: Mclkrdy) -> u8 {
        Mclkrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Men {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Men {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Men {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Men {
    #[inline(always)]
    fn from(val: u8) -> Men {
        Men::from_bits(val)
    }
}
impl From<Men> for u8 {
    #[inline(always)]
    fn from(val: Men) -> u8 {
        Men::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Minor(u8);
impl Minor {
    #[doc = "x.0."]
    pub const Ver20: Self = Self(0x0);
}
impl Minor {
    pub const fn from_bits(val: u8) -> Minor {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Minor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Ver20"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Minor {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ver20"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Minor {
    #[inline(always)]
    fn from(val: u8) -> Minor {
        Minor::from_bits(val)
    }
}
impl From<Minor> for u8 {
    #[inline(always)]
    fn from(val: Minor) -> u8 {
        Minor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfOrdSel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "3."]
    Order3 = 0x02,
    #[doc = "2."]
    Order2 = 0x03,
}
impl PfOrdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfOrdSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfOrdSel {
    #[inline(always)]
    fn from(val: u8) -> PfOrdSel {
        PfOrdSel::from_bits(val)
    }
}
impl From<PfOrdSel> for u8 {
    #[inline(always)]
    fn from(val: PfOrdSel) -> u8 {
        PfOrdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfcm {
    #[doc = "Single."]
    Single = 0x0,
    #[doc = "Continuous."]
    Continuous = 0x01,
    #[doc = "Always."]
    Always = 0x02,
    #[doc = "Fixed number."]
    Fix = 0x03,
}
impl Pfcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfcm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfcm {
    #[inline(always)]
    fn from(val: u8) -> Pfcm {
        Pfcm::from_bits(val)
    }
}
impl From<Pfcm> for u8 {
    #[inline(always)]
    fn from(val: Pfcm) -> u8 {
        Pfcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfen {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Pfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfen {
    #[inline(always)]
    fn from(val: u8) -> Pfen {
        Pfen::from_bits(val)
    }
}
impl From<Pfen> for u8 {
    #[inline(always)]
    fn from(val: Pfen) -> u8 {
        Pfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pford {
    #[doc = "FastSinc."]
    Fastsinc = 0x0,
    #[doc = "First order."]
    Order1 = 0x01,
    #[doc = "Second order."]
    Order2 = 0x02,
    #[doc = "Third order."]
    Order3 = 0x03,
}
impl Pford {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pford {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pford {
    #[inline(always)]
    fn from(val: u8) -> Pford {
        Pford::from_bits(val)
    }
}
impl From<Pford> for u8 {
    #[inline(always)]
    fn from(val: Pford) -> u8 {
        Pford::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfsat {
    #[doc = "Did not occur."]
    SatNo = 0x0,
    #[doc = "Occurred."]
    SatYes = 0x01,
}
impl Pfsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfsat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfsat {
    #[inline(always)]
    fn from(val: u8) -> Pfsat {
        Pfsat::from_bits(val)
    }
}
impl From<Pfsat> for u8 {
    #[inline(always)]
    fn from(val: Pfsat) -> u8 {
        Pfsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "No prescale."]
    PrescaleNo = 0x0,
    #[doc = "2."]
    Prescale2 = 0x01,
    #[doc = "4."]
    Prescale4 = 0x02,
    #[doc = "8."]
    Prescale8 = 0x03,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psrdy {
    #[doc = "Not ready."]
    ReadyNo = 0x0,
    #[doc = "Ready."]
    ReadyYes = 0x01,
}
impl Psrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psrdy {
    #[inline(always)]
    fn from(val: u8) -> Psrdy {
        Psrdy::from_bits(val)
    }
}
impl From<Psrdy> for u8 {
    #[inline(always)]
    fn from(val: Psrdy) -> u8 {
        Psrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdfmt {
    #[doc = "Left justified, signed."]
    Signed = 0x0,
    #[doc = "Left justified, unsigned."]
    Unsigned = 0x01,
}
impl Rdfmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdfmt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdfmt {
    #[inline(always)]
    fn from(val: u8) -> Rdfmt {
        Rdfmt::from_bits(val)
    }
}
impl From<Rdfmt> for u8 {
    #[inline(always)]
    fn from(val: Rdfmt) -> u8 {
        Rdfmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdrs {
    #[doc = "Valid."]
    Valid = 0x0,
    #[doc = "Invalid."]
    Invalid = 0x01,
}
impl Rdrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdrs {
    #[inline(always)]
    fn from(val: u8) -> Rdrs {
        Rdrs::from_bits(val)
    }
}
impl From<Rdrs> for u8 {
    #[inline(always)]
    fn from(val: Rdrs) -> u8 {
        Rdrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "Do not reset."]
    ResetNo = 0x0,
    #[doc = "Reset."]
    ResetYes = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sat {
    #[doc = "Not saturated."]
    SatNo = 0x0,
    #[doc = "Saturated."]
    SatYes = 0x01,
}
impl Sat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sat {
    #[inline(always)]
    fn from(val: u8) -> Sat {
        Sat::from_bits(val)
    }
}
impl From<Sat> for u8 {
    #[inline(always)]
    fn from(val: Sat) -> u8 {
        Sat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Satie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Satie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Satie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Satie {
    #[inline(always)]
    fn from(val: u8) -> Satie {
        Satie::from_bits(val)
    }
}
impl From<Satie> for u8 {
    #[inline(always)]
    fn from(val: Satie) -> u8 {
        Satie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scd {
    #[doc = "Not detected."]
    ScNo = 0x0,
    #[doc = "Detected."]
    ScYes = 0x01,
}
impl Scd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scd {
    #[inline(always)]
    fn from(val: u8) -> Scd {
        Scd::from_bits(val)
    }
}
impl From<Scd> for u8 {
    #[inline(always)]
    fn from(val: Scd) -> u8 {
        Scd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdbk {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Scdbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdbk {
    #[inline(always)]
    fn from(val: u8) -> Scdbk {
        Scdbk::from_bits(val)
    }
}
impl From<Scdbk> for u8 {
    #[inline(always)]
    fn from(val: Scdbk) -> u8 {
        Scdbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdcm {
    #[doc = "Constantly when CnCR\\[CHEN\\] = MCR\\[MEN\\] = 1."]
    Always = 0x0,
    #[doc = "Only when the PF is performing a conversion."]
    DuringConv = 0x01,
}
impl Scdcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdcm {
    #[inline(always)]
    fn from(val: u8) -> Scdcm {
        Scdcm::from_bits(val)
    }
}
impl From<Scdcm> for u8 {
    #[inline(always)]
    fn from(val: Scdcm) -> u8 {
        Scdcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scden {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Scden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scden {
    #[inline(always)]
    fn from(val: u8) -> Scden {
        Scden::from_bits(val)
    }
}
impl From<Scden> for u8 {
    #[inline(always)]
    fn from(val: Scden) -> u8 {
        Scden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Scdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdie {
    #[inline(always)]
    fn from(val: u8) -> Scdie {
        Scdie::from_bits(val)
    }
}
impl From<Scdie> for u8 {
    #[inline(always)]
    fn from(val: Scdie) -> u8 {
        Scdie::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Scdlmt(u8);
impl Scdlmt {
    #[doc = "Disables SCD."]
    pub const Disables0: Self = Self(0x0);
    #[doc = "Disables SCD."]
    pub const Disables1: Self = Self(0x01);
}
impl Scdlmt {
    pub const fn from_bits(val: u8) -> Scdlmt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Scdlmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Disables0"),
            0x01 => f.write_str("Disables1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scdlmt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Disables0"),
            0x01 => defmt::write!(f, "Disables1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Scdlmt {
    #[inline(always)]
    fn from(val: u8) -> Scdlmt {
        Scdlmt::from_bits(val)
    }
}
impl From<Scdlmt> for u8 {
    #[inline(always)]
    fn from(val: Scdlmt) -> u8 {
        Scdlmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scdop {
    #[doc = "Both 0 and 1."]
    Both = 0x0,
    #[doc = "Only 1."]
    One = 0x01,
    #[doc = "Only 0."]
    Zero = 0x02,
    _RESERVED_3 = 0x03,
}
impl Scdop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scdop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scdop {
    #[inline(always)]
    fn from(val: u8) -> Scdop {
        Scdop::from_bits(val)
    }
}
impl From<Scdop> for u8 {
    #[inline(always)]
    fn from(val: Scdop) -> u8 {
        Scdop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sftsat {
    #[doc = "Did not occur."]
    SatNo = 0x0,
    #[doc = "Occurred."]
    SatYes = 0x01,
}
impl Sftsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sftsat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sftsat {
    #[inline(always)]
    fn from(val: u8) -> Sftsat {
        Sftsat::from_bits(val)
    }
}
impl From<Sftsat> for u8 {
    #[inline(always)]
    fn from(val: Sftsat) -> u8 {
        Sftsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srds {
    #[doc = "Data valid."]
    DataValid = 0x0,
    #[doc = "Procedure in progress."]
    InProgress = 0x01,
}
impl Srds {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srds {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srds {
    #[inline(always)]
    fn from(val: u8) -> Srds {
        Srds::from_bits(val)
    }
}
impl From<Srds> for u8 {
    #[inline(always)]
    fn from(val: Srds) -> u8 {
        Srds::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strig {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Trigger."]
    Trigger = 0x01,
}
impl Strig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strig {
    #[inline(always)]
    fn from(val: u8) -> Strig {
        Strig::from_bits(val)
    }
}
impl From<Strig> for u8 {
    #[inline(always)]
    fn from(val: Strig) -> u8 {
        Strig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmt {
    #[doc = "Not exceeded."]
    WlmtNo = 0x0,
    #[doc = "Exceeded."]
    WlmtYes = 0x01,
}
impl Wlmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmt {
    #[inline(always)]
    fn from(val: u8) -> Wlmt {
        Wlmt::from_bits(val)
    }
}
impl From<Wlmt> for u8 {
    #[inline(always)]
    fn from(val: Wlmt) -> u8 {
        Wlmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtbk {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Wlmtbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtbk {
    #[inline(always)]
    fn from(val: u8) -> Wlmtbk {
        Wlmtbk::from_bits(val)
    }
}
impl From<Wlmtbk> for u8 {
    #[inline(always)]
    fn from(val: Wlmtbk) -> u8 {
        Wlmtbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wlmtie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Wlmtie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wlmtie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wlmtie {
    #[inline(always)]
    fn from(val: u8) -> Wlmtie {
        Wlmtie::from_bits(val)
    }
}
impl From<Wlmtie> for u8 {
    #[inline(always)]
    fn from(val: Wlmtie) -> u8 {
        Wlmtie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcd {
    #[doc = "Not detected."]
    ZcNo = 0x0,
    #[doc = "Detected."]
    ZcYes = 0x01,
}
impl Zcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcd {
    #[inline(always)]
    fn from(val: u8) -> Zcd {
        Zcd::from_bits(val)
    }
}
impl From<Zcd> for u8 {
    #[inline(always)]
    fn from(val: Zcd) -> u8 {
        Zcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcden {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Zcden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcden {
    #[inline(always)]
    fn from(val: u8) -> Zcden {
        Zcden::from_bits(val)
    }
}
impl From<Zcden> for u8 {
    #[inline(always)]
    fn from(val: Zcden) -> u8 {
        Zcden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcdie {
    #[doc = "Disables."]
    Disables = 0x0,
    #[doc = "Enables."]
    Enables = 0x01,
}
impl Zcdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcdie {
    #[inline(always)]
    fn from(val: u8) -> Zcdie {
        Zcdie::from_bits(val)
    }
}
impl From<Zcdie> for u8 {
    #[inline(always)]
    fn from(val: Zcdie) -> u8 {
        Zcdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Zcop {
    #[doc = "Both rise and fall."]
    Both = 0x0,
    #[doc = "Fall."]
    Fall = 0x01,
    #[doc = "Rise."]
    Rise = 0x02,
    _RESERVED_3 = 0x03,
}
impl Zcop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Zcop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Zcop {
    #[inline(always)]
    fn from(val: u8) -> Zcop {
        Zcop::from_bits(val)
    }
}
impl From<Zcop> for u8 {
    #[inline(always)]
    fn from(val: Zcop) -> u8 {
        Zcop::to_bits(val)
    }
}
