#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Low-Power Serial Peripheral Interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi {
    ptr: *mut u8,
}
unsafe impl Send for Lpspi {}
unsafe impl Sync for Lpspi {}
impl Lpspi {
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
    #[doc = "Control."]
    #[inline(always)]
    pub const fn cr(self) -> crate::pac::common::Reg<Cr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn sr(self) -> crate::pac::common::Reg<Sr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::pac::common::Reg<Ier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn der(self) -> crate::pac::common::Reg<Der, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Configuration 0."]
    #[inline(always)]
    pub const fn cfgr0(self) -> crate::pac::common::Reg<Cfgr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Configuration 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::pac::common::Reg<Cfgr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Data Match 0."]
    #[inline(always)]
    pub const fn dmr0(self) -> crate::pac::common::Reg<Dmr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Data Match 1."]
    #[inline(always)]
    pub const fn dmr1(self) -> crate::pac::common::Reg<Dmr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Clock Configuration."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::pac::common::Reg<Ccr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Clock Configuration 1."]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::pac::common::Reg<Ccr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "FIFO Control."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::pac::common::Reg<Fcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "FIFO Status."]
    #[inline(always)]
    pub const fn fsr(self) -> crate::pac::common::Reg<Fsr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Transmit Command."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::pac::common::Reg<Tcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Transmit Data."]
    #[inline(always)]
    pub const fn tdr(self) -> crate::pac::common::Reg<Tdr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Receive Status."]
    #[inline(always)]
    pub const fn rsr(self) -> crate::pac::common::Reg<Rsr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn rdr(self) -> crate::pac::common::Reg<Rdr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Receive Data Read Only."]
    #[inline(always)]
    pub const fn rdror(self) -> crate::pac::common::Reg<Rdror, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Transmit Command Burst."]
    #[inline(always)]
    pub const fn tcbr(self) -> crate::pac::common::Reg<Tcbr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Transmit Data Burst."]
    #[inline(always)]
    pub const fn tdbr(self, n: usize) -> crate::pac::common::Reg<Tdbr, crate::pac::common::W> {
        assert!(n < 128usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Receive Data Burst."]
    #[inline(always)]
    pub const fn rdbr(self, n: usize) -> crate::pac::common::Reg<Rdbr, crate::pac::common::R> {
        assert!(n < 128usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 4usize) as _)
        }
    }
}
#[doc = "Clock Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "SCK Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn sckdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Divider."]
    #[inline(always)]
    pub const fn set_sckdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Delay Between Transfers."]
    #[must_use]
    #[inline(always)]
    pub const fn dbt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Delay Between Transfers."]
    #[inline(always)]
    pub const fn set_dbt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PCS-to-SCK Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn pcssck(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PCS-to-SCK Delay."]
    #[inline(always)]
    pub const fn set_pcssck(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "SCK-to-PCS Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn sckpcs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SCK-to-PCS Delay."]
    #[inline(always)]
    pub const fn set_sckpcs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            .field("sckdiv", &self.sckdiv())
            .field("dbt", &self.dbt())
            .field("pcssck", &self.pcssck())
            .field("sckpcs", &self.sckpcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ sckdiv: {=u8:?}, dbt: {=u8:?}, pcssck: {=u8:?}, sckpcs: {=u8:?} }}",
            self.sckdiv(),
            self.dbt(),
            self.pcssck(),
            self.sckpcs()
        )
    }
}
#[doc = "Clock Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "SCK Setup."]
    #[must_use]
    #[inline(always)]
    pub const fn sckset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Setup."]
    #[inline(always)]
    pub const fn set_sckset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SCK Hold."]
    #[must_use]
    #[inline(always)]
    pub const fn sckhld(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Hold."]
    #[inline(always)]
    pub const fn set_sckhld(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PCS to PCS Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn pcspcs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PCS to PCS Delay."]
    #[inline(always)]
    pub const fn set_pcspcs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "SCK Inter-Frame Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn scksck(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Inter-Frame Delay."]
    #[inline(always)]
    pub const fn set_scksck(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        Ccr1(0)
    }
}
impl core::fmt::Debug for Ccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr1")
            .field("sckset", &self.sckset())
            .field("sckhld", &self.sckhld())
            .field("pcspcs", &self.pcspcs())
            .field("scksck", &self.scksck())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr1 {{ sckset: {=u8:?}, sckhld: {=u8:?}, pcspcs: {=u8:?}, scksck: {=u8:?} }}",
            self.sckset(),
            self.sckhld(),
            self.pcspcs(),
            self.scksck()
        )
    }
}
#[doc = "Configuration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr0(pub u32);
impl Cfgr0 {
    #[doc = "Host Request Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Enable."]
    #[inline(always)]
    pub const fn set_hren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Request Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn hrpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Polarity."]
    #[inline(always)]
    pub const fn set_hrpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Host Request Select."]
    #[must_use]
    #[inline(always)]
    pub const fn hrsel(&self) -> Hrsel {
        let val = (self.0 >> 2usize) & 0x01;
        Hrsel::from_bits(val as u8)
    }
    #[doc = "Host Request Select."]
    #[inline(always)]
    pub const fn set_hrsel(&mut self, val: Hrsel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Request Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn hrdir(&self) -> Hrdir {
        let val = (self.0 >> 3usize) & 0x01;
        Hrdir::from_bits(val as u8)
    }
    #[doc = "Host Request Direction."]
    #[inline(always)]
    pub const fn set_hrdir(&mut self, val: Hrdir) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Circular FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cirfifo(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Circular FIFO Enable."]
    #[inline(always)]
    pub const fn set_cirfifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Data Match Only."]
    #[must_use]
    #[inline(always)]
    pub const fn rdmo(&self) -> Rdmo {
        let val = (self.0 >> 9usize) & 0x01;
        Rdmo::from_bits(val as u8)
    }
    #[doc = "Receive Data Match Only."]
    #[inline(always)]
    pub const fn set_rdmo(&mut self, val: Rdmo) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Cfgr0 {
    #[inline(always)]
    fn default() -> Cfgr0 {
        Cfgr0(0)
    }
}
impl core::fmt::Debug for Cfgr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr0")
            .field("hren", &self.hren())
            .field("hrpol", &self.hrpol())
            .field("hrsel", &self.hrsel())
            .field("hrdir", &self.hrdir())
            .field("cirfifo", &self.cirfifo())
            .field("rdmo", &self.rdmo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr0 {{ hren: {=bool:?}, hrpol: {=bool:?}, hrsel: {:?}, hrdir: {:?}, cirfifo: {=bool:?}, rdmo: {:?} }}",
            self.hren(),
            self.hrpol(),
            self.hrsel(),
            self.hrdir(),
            self.cirfifo(),
            self.rdmo()
        )
    }
}
#[doc = "Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc = "Controller Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> Master {
        let val = (self.0 >> 0usize) & 0x01;
        Master::from_bits(val as u8)
    }
    #[doc = "Controller Mode."]
    #[inline(always)]
    pub const fn set_master(&mut self, val: Master) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Sample Point."]
    #[must_use]
    #[inline(always)]
    pub const fn sample(&self) -> Sample {
        let val = (self.0 >> 1usize) & 0x01;
        Sample::from_bits(val as u8)
    }
    #[doc = "Sample Point."]
    #[inline(always)]
    pub const fn set_sample(&mut self, val: Sample) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Automatic PCS."]
    #[must_use]
    #[inline(always)]
    pub const fn autopcs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic PCS."]
    #[inline(always)]
    pub const fn set_autopcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "No Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn nostall(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "No Stall."]
    #[inline(always)]
    pub const fn set_nostall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Partial Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn partial(&self) -> Partial {
        let val = (self.0 >> 4usize) & 0x01;
        Partial::from_bits(val as u8)
    }
    #[doc = "Partial Enable."]
    #[inline(always)]
    pub const fn set_partial(&mut self, val: Partial) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Peripheral Chip Select Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pcspol(&self) -> Pcspol {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcspol::from_bits(val as u8)
    }
    #[doc = "Peripheral Chip Select Polarity."]
    #[inline(always)]
    pub const fn set_pcspol(&mut self, val: Pcspol) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Match Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn matcfg(&self) -> Matcfg {
        let val = (self.0 >> 16usize) & 0x07;
        Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration."]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: Matcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pin Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> Pincfg {
        let val = (self.0 >> 24usize) & 0x03;
        Pincfg::from_bits(val as u8)
    }
    #[doc = "Pin Configuration."]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: Pincfg) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Output Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn outcfg(&self) -> Outcfg {
        let val = (self.0 >> 26usize) & 0x01;
        Outcfg::from_bits(val as u8)
    }
    #[doc = "Output Configuration."]
    #[inline(always)]
    pub const fn set_outcfg(&mut self, val: Outcfg) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Peripheral Chip Select Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pcscfg(&self) -> Pcscfg {
        let val = (self.0 >> 27usize) & 0x01;
        Pcscfg::from_bits(val as u8)
    }
    #[doc = "Peripheral Chip Select Configuration."]
    #[inline(always)]
    pub const fn set_pcscfg(&mut self, val: Pcscfg) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Cfgr1 {
    #[inline(always)]
    fn default() -> Cfgr1 {
        Cfgr1(0)
    }
}
impl core::fmt::Debug for Cfgr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr1")
            .field("master", &self.master())
            .field("sample", &self.sample())
            .field("autopcs", &self.autopcs())
            .field("nostall", &self.nostall())
            .field("partial", &self.partial())
            .field("pcspol", &self.pcspol())
            .field("matcfg", &self.matcfg())
            .field("pincfg", &self.pincfg())
            .field("outcfg", &self.outcfg())
            .field("pcscfg", &self.pcscfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr1 {{ master: {:?}, sample: {:?}, autopcs: {=bool:?}, nostall: {=bool:?}, partial: {:?}, pcspol: {:?}, matcfg: {:?}, pincfg: {:?}, outcfg: {:?}, pcscfg: {:?} }}",
            self.master(),
            self.sample(),
            self.autopcs(),
            self.nostall(),
            self.partial(),
            self.pcspol(),
            self.matcfg(),
            self.pincfg(),
            self.outcfg(),
            self.pcscfg()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Module Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn men(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Module Enable."]
    #[inline(always)]
    pub const fn set_men(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Doze Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> Dozen {
        let val = (self.0 >> 2usize) & 0x01;
        Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Mode Enable."]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: Dozen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rtf(&self) -> Rtf {
        let val = (self.0 >> 8usize) & 0x01;
        Rtf::from_bits(val as u8)
    }
    #[doc = "Reset Transmit FIFO."]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: Rtf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> Rrf {
        let val = (self.0 >> 9usize) & 0x01;
        Rrf::from_bits(val as u8)
    }
    #[doc = "Reset Receive FIFO."]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: Rrf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
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
            .field("men", &self.men())
            .field("rst", &self.rst())
            .field("dozen", &self.dozen())
            .field("dbgen", &self.dbgen())
            .field("rtf", &self.rtf())
            .field("rrf", &self.rrf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ men: {=bool:?}, rst: {=bool:?}, dozen: {:?}, dbgen: {=bool:?}, rtf: {:?}, rrf: {:?} }}",
            self.men(),
            self.rst(),
            self.dozen(),
            self.dbgen(),
            self.rtf(),
            self.rrf()
        )
    }
}
#[doc = "DMA Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Der(pub u32);
impl Der {
    #[doc = "Transmit Data DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable."]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable."]
    #[inline(always)]
    pub const fn set_rdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame Complete DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fcde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Complete DMA Enable."]
    #[inline(always)]
    pub const fn set_fcde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Der {
    #[inline(always)]
    fn default() -> Der {
        Der(0)
    }
}
impl core::fmt::Debug for Der {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Der")
            .field("tdde", &self.tdde())
            .field("rdde", &self.rdde())
            .field("fcde", &self.fcde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Der {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Der {{ tdde: {=bool:?}, rdde: {=bool:?}, fcde: {=bool:?} }}",
            self.tdde(),
            self.rdde(),
            self.fcde()
        )
    }
}
#[doc = "Data Match 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmr0(pub u32);
impl Dmr0 {
    #[doc = "Match 0 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn match0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Match 0 Value."]
    #[inline(always)]
    pub const fn set_match0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmr0 {
    #[inline(always)]
    fn default() -> Dmr0 {
        Dmr0(0)
    }
}
impl core::fmt::Debug for Dmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmr0")
            .field("match0", &self.match0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmr0 {{ match0: {=u32:?} }}", self.match0())
    }
}
#[doc = "Data Match 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmr1(pub u32);
impl Dmr1 {
    #[doc = "Match 1 Value."]
    #[must_use]
    #[inline(always)]
    pub const fn match1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Match 1 Value."]
    #[inline(always)]
    pub const fn set_match1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmr1 {
    #[inline(always)]
    fn default() -> Dmr1 {
        Dmr1(0)
    }
}
impl core::fmt::Debug for Dmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmr1")
            .field("match1", &self.match1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmr1 {{ match1: {=u32:?} }}", self.match1())
    }
}
#[doc = "FIFO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc = "Transmit FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark."]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Receive FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive FIFO Watermark."]
    #[inline(always)]
    pub const fn set_rxwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        Fcr(0)
    }
}
impl core::fmt::Debug for Fcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcr")
            .field("txwater", &self.txwater())
            .field("rxwater", &self.rxwater())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcr {{ txwater: {=u8:?}, rxwater: {=u8:?} }}",
            self.txwater(),
            self.rxwater()
        )
    }
}
#[doc = "FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsr(pub u32);
impl Fsr {
    #[doc = "Transmit FIFO Count."]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit FIFO Count."]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Receive FIFO Count."]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Fsr {
    #[inline(always)]
    fn default() -> Fsr {
        Fsr(0)
    }
}
impl core::fmt::Debug for Fsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fsr")
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fsr {{ txcount: {=u8:?}, rxcount: {=u8:?} }}",
            self.txcount(),
            self.rxcount()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Transmit Data Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Word Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wcie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Word Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fcie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transfer Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Data Match Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("tdie", &self.tdie())
            .field("rdie", &self.rdie())
            .field("wcie", &self.wcie())
            .field("fcie", &self.fcie())
            .field("tcie", &self.tcie())
            .field("teie", &self.teie())
            .field("reie", &self.reie())
            .field("dmie", &self.dmie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ tdie: {=bool:?}, rdie: {=bool:?}, wcie: {=bool:?}, fcie: {=bool:?}, tcie: {=bool:?}, teie: {=bool:?}, reie: {=bool:?}, dmie: {=bool:?} }}",
            self.tdie(),
            self.rdie(),
            self.wcie(),
            self.fcie(),
            self.tcie(),
            self.teie(),
            self.reie(),
            self.dmie()
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Transmit FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Size."]
    #[inline(always)]
    pub const fn set_txfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Size."]
    #[inline(always)]
    pub const fn set_rxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PCS Number."]
    #[must_use]
    #[inline(always)]
    pub const fn pcsnum(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PCS Number."]
    #[inline(always)]
    pub const fn set_pcsnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
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
            .field("txfifo", &self.txfifo())
            .field("rxfifo", &self.rxfifo())
            .field("pcsnum", &self.pcsnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ txfifo: {=u8:?}, rxfifo: {=u8:?}, pcsnum: {=u8:?} }}",
            self.txfifo(),
            self.rxfifo(),
            self.pcsnum()
        )
    }
}
#[doc = "Receive Data Burst."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdbr(pub u32);
impl Rdbr {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdbr {
    #[inline(always)]
    fn default() -> Rdbr {
        Rdbr(0)
    }
}
impl core::fmt::Debug for Rdbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdbr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Receive Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc = "Receive Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        Rdr(0)
    }
}
impl core::fmt::Debug for Rdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Receive Data Read Only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdror(pub u32);
impl Rdror {
    #[doc = "Receive Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdror {
    #[inline(always)]
    fn default() -> Rdror {
        Rdror(0)
    }
}
impl core::fmt::Debug for Rdror {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdror").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdror {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdror {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Receive Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc = "Start of Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn sof(&self) -> Sof {
        let val = (self.0 >> 0usize) & 0x01;
        Sof::from_bits(val as u8)
    }
    #[doc = "Start of Frame."]
    #[inline(always)]
    pub const fn set_sof(&mut self, val: Sof) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO Empty."]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Rsr {
    #[inline(always)]
    fn default() -> Rsr {
        Rsr(0)
    }
}
impl core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsr")
            .field("sof", &self.sof())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsr {{ sof: {:?}, rxempty: {=bool:?} }}",
            self.sof(),
            self.rxempty()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Transmit Data Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tdf(&self) -> Tdf {
        let val = (self.0 >> 0usize) & 0x01;
        Tdf::from_bits(val as u8)
    }
    #[doc = "Transmit Data Flag."]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: Tdf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag."]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Word Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wcf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Word Complete Flag."]
    #[inline(always)]
    pub const fn set_wcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fcf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Complete Flag."]
    #[inline(always)]
    pub const fn set_fcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transfer Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete Flag."]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tef(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Error Flag."]
    #[inline(always)]
    pub const fn set_tef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Data Match Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dmf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Flag."]
    #[inline(always)]
    pub const fn set_dmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Module Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn mbf(&self) -> Mbf {
        let val = (self.0 >> 24usize) & 0x01;
        Mbf::from_bits(val as u8)
    }
    #[doc = "Module Busy Flag."]
    #[inline(always)]
    pub const fn set_mbf(&mut self, val: Mbf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
            .field("tdf", &self.tdf())
            .field("rdf", &self.rdf())
            .field("wcf", &self.wcf())
            .field("fcf", &self.fcf())
            .field("tcf", &self.tcf())
            .field("tef", &self.tef())
            .field("ref_", &self.ref_())
            .field("dmf", &self.dmf())
            .field("mbf", &self.mbf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ tdf: {:?}, rdf: {=bool:?}, wcf: {=bool:?}, fcf: {=bool:?}, tcf: {=bool:?}, tef: {=bool:?}, ref_: {=bool:?}, dmf: {=bool:?}, mbf: {:?} }}",
            self.tdf(),
            self.rdf(),
            self.wcf(),
            self.fcf(),
            self.tcf(),
            self.tef(),
            self.ref_(),
            self.dmf(),
            self.mbf()
        )
    }
}
#[doc = "Transmit Command Burst."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcbr(pub u32);
impl Tcbr {
    #[doc = "Command Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcbr {
    #[inline(always)]
    fn default() -> Tcbr {
        Tcbr(0)
    }
}
impl core::fmt::Debug for Tcbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcbr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Transmit Command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Frame Size."]
    #[must_use]
    #[inline(always)]
    pub const fn framesz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Frame Size."]
    #[inline(always)]
    pub const fn set_framesz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Transfer Width."]
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> Width {
        let val = (self.0 >> 16usize) & 0x03;
        Width::from_bits(val as u8)
    }
    #[doc = "Transfer Width."]
    #[inline(always)]
    pub const fn set_width(&mut self, val: Width) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Transmit Data Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn txmsk(&self) -> Txmsk {
        let val = (self.0 >> 18usize) & 0x01;
        Txmsk::from_bits(val as u8)
    }
    #[doc = "Transmit Data Mask."]
    #[inline(always)]
    pub const fn set_txmsk(&mut self, val: Txmsk) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Receive Data Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rxmsk(&self) -> Rxmsk {
        let val = (self.0 >> 19usize) & 0x01;
        Rxmsk::from_bits(val as u8)
    }
    #[doc = "Receive Data Mask."]
    #[inline(always)]
    pub const fn set_rxmsk(&mut self, val: Rxmsk) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Continuing Command."]
    #[must_use]
    #[inline(always)]
    pub const fn contc(&self) -> Contc {
        let val = (self.0 >> 20usize) & 0x01;
        Contc::from_bits(val as u8)
    }
    #[doc = "Continuing Command."]
    #[inline(always)]
    pub const fn set_contc(&mut self, val: Contc) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Continuous Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn cont(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Transfer."]
    #[inline(always)]
    pub const fn set_cont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Byte Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn bysw(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Swap."]
    #[inline(always)]
    pub const fn set_bysw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LSB First."]
    #[must_use]
    #[inline(always)]
    pub const fn lsbf(&self) -> Lsbf {
        let val = (self.0 >> 23usize) & 0x01;
        Lsbf::from_bits(val as u8)
    }
    #[doc = "LSB First."]
    #[inline(always)]
    pub const fn set_lsbf(&mut self, val: Lsbf) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Peripheral Chip Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> Pcs {
        let val = (self.0 >> 24usize) & 0x03;
        Pcs::from_bits(val as u8)
    }
    #[doc = "Peripheral Chip Select."]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: Pcs) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Prescaler Value."]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> Prescale {
        let val = (self.0 >> 27usize) & 0x07;
        Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler Value."]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: Prescale) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Clock Phase."]
    #[must_use]
    #[inline(always)]
    pub const fn cpha(&self) -> Cpha {
        let val = (self.0 >> 30usize) & 0x01;
        Cpha::from_bits(val as u8)
    }
    #[doc = "Clock Phase."]
    #[inline(always)]
    pub const fn set_cpha(&mut self, val: Cpha) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Clock Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cpol(&self) -> Cpol {
        let val = (self.0 >> 31usize) & 0x01;
        Cpol::from_bits(val as u8)
    }
    #[doc = "Clock Polarity."]
    #[inline(always)]
    pub const fn set_cpol(&mut self, val: Cpol) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("framesz", &self.framesz())
            .field("width", &self.width())
            .field("txmsk", &self.txmsk())
            .field("rxmsk", &self.rxmsk())
            .field("contc", &self.contc())
            .field("cont", &self.cont())
            .field("bysw", &self.bysw())
            .field("lsbf", &self.lsbf())
            .field("pcs", &self.pcs())
            .field("prescale", &self.prescale())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ framesz: {=u16:?}, width: {:?}, txmsk: {:?}, rxmsk: {:?}, contc: {:?}, cont: {=bool:?}, bysw: {=bool:?}, lsbf: {:?}, pcs: {:?}, prescale: {:?}, cpha: {:?}, cpol: {:?} }}",
            self.framesz(),
            self.width(),
            self.txmsk(),
            self.rxmsk(),
            self.contc(),
            self.cont(),
            self.bysw(),
            self.lsbf(),
            self.pcs(),
            self.prescale(),
            self.cpha(),
            self.cpol()
        )
    }
}
#[doc = "Transmit Data Burst."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdbr(pub u32);
impl Tdbr {
    #[doc = "Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdbr {
    #[inline(always)]
    fn default() -> Tdbr {
        Tdbr(0)
    }
}
impl core::fmt::Debug for Tdbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdbr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Transmit Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc = "Transmit Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        Tdr(0)
    }
}
impl core::fmt::Debug for Tdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Module Identification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Module Identification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Contc {
    #[doc = "Command word for start of new transfer."]
    Start = 0x0,
    #[doc = "Command word for continuing transfer."]
    Continue = 0x01,
}
impl Contc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Contc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Contc {
    #[inline(always)]
    fn from(val: u8) -> Contc {
        Contc::from_bits(val)
    }
}
impl From<Contc> for u8 {
    #[inline(always)]
    fn from(val: Contc) -> u8 {
        Contc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpha {
    #[doc = "Captured."]
    Captured = 0x0,
    #[doc = "Changed."]
    Changed = 0x01,
}
impl Cpha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpha {
    #[inline(always)]
    fn from(val: u8) -> Cpha {
        Cpha::from_bits(val)
    }
}
impl From<Cpha> for u8 {
    #[inline(always)]
    fn from(val: Cpha) -> u8 {
        Cpha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpol {
    #[doc = "Inactive low."]
    InactiveLow = 0x0,
    #[doc = "Inactive high."]
    InactiveHigh = 0x01,
}
impl Cpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpol {
    #[inline(always)]
    fn from(val: u8) -> Cpol {
        Cpol::from_bits(val)
    }
}
impl From<Cpol> for u8 {
    #[inline(always)]
    fn from(val: Cpol) -> u8 {
        Cpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enable."]
    Enabled = 0x0,
    #[doc = "Disable."]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set supporting a 32-bit shift register."]
    pub const Standard: Self = Self(0x04);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x04 => f.write_str("Standard"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x04 => defmt::write!(f, "Standard"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrdir {
    #[doc = "Input."]
    Input = 0x0,
    #[doc = "Output."]
    Output = 0x01,
}
impl Hrdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrdir {
    #[inline(always)]
    fn from(val: u8) -> Hrdir {
        Hrdir::from_bits(val)
    }
}
impl From<Hrdir> for u8 {
    #[inline(always)]
    fn from(val: Hrdir) -> u8 {
        Hrdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrsel {
    #[doc = "HREQ pin."]
    Hreqpin = 0x0,
    #[doc = "Input trigger."]
    InputTrigger = 0x01,
}
impl Hrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrsel {
    #[inline(always)]
    fn from(val: u8) -> Hrsel {
        Hrsel::from_bits(val)
    }
}
impl From<Hrsel> for u8 {
    #[inline(always)]
    fn from(val: Hrsel) -> u8 {
        Hrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsbf {
    #[doc = "MSB first."]
    MsbFirst = 0x0,
    #[doc = "LSB first."]
    LsbFirst = 0x01,
}
impl Lsbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsbf {
    #[inline(always)]
    fn from(val: u8) -> Lsbf {
        Lsbf::from_bits(val)
    }
}
impl From<Lsbf> for u8 {
    #[inline(always)]
    fn from(val: Lsbf) -> u8 {
        Lsbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Peripheral mode."]
    SlaveMode = 0x0,
    #[doc = "Controller mode."]
    MasterMode = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Match is disabled."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Match first data word with compare word."]
    EnabledFirstdatamatch = 0x02,
    #[doc = "Match any data word with compare word."]
    EnabledAnydatamatch = 0x03,
    #[doc = "Sequential match, first data word."]
    EnabledDatamatch100 = 0x04,
    #[doc = "Sequential match, any data word."]
    EnabledDatamatch101 = 0x05,
    #[doc = "Match first data word (masked) with compare word (masked)."]
    EnabledDatamatch110 = 0x06,
    #[doc = "Match any data word (masked) with compare word (masked)."]
    EnabledDatamatch111 = 0x07,
}
impl Matcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matcfg {
    #[inline(always)]
    fn from(val: u8) -> Matcfg {
        Matcfg::from_bits(val)
    }
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(val: Matcfg) -> u8 {
        Matcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbf {
    #[doc = "LPSPI is idle."]
    Idle = 0x0,
    #[doc = "LPSPI is busy."]
    Busy = 0x01,
}
impl Mbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbf {
    #[inline(always)]
    fn from(val: u8) -> Mbf {
        Mbf::from_bits(val)
    }
}
impl From<Mbf> for u8 {
    #[inline(always)]
    fn from(val: Mbf) -> u8 {
        Mbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outcfg {
    #[doc = "Retain last value."]
    RetainLastvalue = 0x0,
    #[doc = "3-stated."]
    Tristated = 0x01,
}
impl Outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outcfg {
    #[inline(always)]
    fn from(val: u8) -> Outcfg {
        Outcfg::from_bits(val)
    }
}
impl From<Outcfg> for u8 {
    #[inline(always)]
    fn from(val: Outcfg) -> u8 {
        Outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Partial {
    #[doc = "Discard."]
    Discarded = 0x0,
    #[doc = "Store."]
    Stored = 0x01,
}
impl Partial {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Partial {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Partial {
    #[inline(always)]
    fn from(val: u8) -> Partial {
        Partial::from_bits(val)
    }
}
impl From<Partial> for u8 {
    #[inline(always)]
    fn from(val: Partial) -> u8 {
        Partial::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcs {
    #[doc = "Transfer using PCS\\[0\\]."]
    TxPcs0 = 0x0,
    #[doc = "Transfer using PCS\\[1\\]."]
    TxPcs1 = 0x01,
    #[doc = "Transfer using PCS\\[2\\]."]
    TxPcs2 = 0x02,
    #[doc = "Transfer using PCS\\[3\\]."]
    TxPcs3 = 0x03,
}
impl Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcs {
    #[inline(always)]
    fn from(val: u8) -> Pcs {
        Pcs::from_bits(val)
    }
}
impl From<Pcs> for u8 {
    #[inline(always)]
    fn from(val: Pcs) -> u8 {
        Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcscfg {
    #[doc = "PCS\\[3:2\\] configured for chip select function."]
    ChipSelect = 0x0,
    #[doc = "PCS\\[3:2\\] configured for half-duplex 4-bit transfers (PCS\\[3:2\\] = DATA\\[3:2\\])."]
    Halfduplex4bit = 0x01,
}
impl Pcscfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcscfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcscfg {
    #[inline(always)]
    fn from(val: u8) -> Pcscfg {
        Pcscfg::from_bits(val)
    }
}
impl From<Pcscfg> for u8 {
    #[inline(always)]
    fn from(val: Pcscfg) -> u8 {
        Pcscfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcspol {
    #[doc = "Active low."]
    Discarded = 0x0,
    #[doc = "Active high."]
    Stored = 0x01,
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
impl Pcspol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcspol {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcspol {
    #[inline(always)]
    fn from(val: u8) -> Pcspol {
        Pcspol::from_bits(val)
    }
}
impl From<Pcspol> for u8 {
    #[inline(always)]
    fn from(val: Pcspol) -> u8 {
        Pcspol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pincfg {
    #[doc = "SIN is used for input data; SOUT is used for output data."]
    SinInSoutOut = 0x0,
    #[doc = "SIN is used for both input and output data; only half-duplex serial transfers are supported."]
    SinBothInOut = 0x01,
    #[doc = "SOUT is used for both input and output data; only half-duplex serial transfers are supported."]
    SoutBothInOut = 0x02,
    #[doc = "SOUT is used for input data; SIN is used for output data."]
    SoutInSinOut = 0x03,
}
impl Pincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pincfg {
    #[inline(always)]
    fn from(val: u8) -> Pincfg {
        Pincfg::from_bits(val)
    }
}
impl From<Pincfg> for u8 {
    #[inline(always)]
    fn from(val: Pincfg) -> u8 {
        Pincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "Divide by 1."]
    DivideBy1 = 0x0,
    #[doc = "Divide by 2."]
    DivideBy2 = 0x01,
    #[doc = "Divide by 4."]
    DivideBy4 = 0x02,
    #[doc = "Divide by 8."]
    DivideBy8 = 0x03,
    #[doc = "Divide by 16."]
    DivideBy16 = 0x04,
    #[doc = "Divide by 32."]
    DivideBy32 = 0x05,
    #[doc = "Divide by 64."]
    DivideBy64 = 0x06,
    #[doc = "Divide by 128."]
    DivideBy128 = 0x07,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum Rdmo {
    #[doc = "Disable."]
    Stored = 0x0,
    #[doc = "Enable."]
    Discarded = 0x01,
}
impl Rdmo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdmo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdmo {
    #[inline(always)]
    fn from(val: u8) -> Rdmo {
        Rdmo::from_bits(val)
    }
}
impl From<Rdmo> for u8 {
    #[inline(always)]
    fn from(val: Rdmo) -> u8 {
        Rdmo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset."]
    RxfifoRst = 0x01,
}
impl Rrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrf {
    #[inline(always)]
    fn from(val: u8) -> Rrf {
        Rrf::from_bits(val)
    }
}
impl From<Rrf> for u8 {
    #[inline(always)]
    fn from(val: Rrf) -> u8 {
        Rrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset."]
    TxfifoRst = 0x01,
}
impl Rtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtf {
    #[inline(always)]
    fn from(val: u8) -> Rtf {
        Rtf::from_bits(val)
    }
}
impl From<Rtf> for u8 {
    #[inline(always)]
    fn from(val: Rtf) -> u8 {
        Rtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxmsk {
    #[doc = "Normal transfer."]
    Normal = 0x0,
    #[doc = "Mask receive data."]
    Mask = 0x01,
}
impl Rxmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxmsk {
    #[inline(always)]
    fn from(val: u8) -> Rxmsk {
        Rxmsk::from_bits(val)
    }
}
impl From<Rxmsk> for u8 {
    #[inline(always)]
    fn from(val: Rxmsk) -> u8 {
        Rxmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sample {
    #[doc = "SCK edge."]
    OnSckEdge = 0x0,
    #[doc = "Delayed SCK edge."]
    OnDelayedSckEdge = 0x01,
}
impl Sample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sample {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sample {
    #[inline(always)]
    fn from(val: u8) -> Sample {
        Sample::from_bits(val)
    }
}
impl From<Sample> for u8 {
    #[inline(always)]
    fn from(val: Sample) -> u8 {
        Sample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sof {
    #[doc = "Subsequent data word or RX FIFO is empty (RXEMPTY=1)."]
    NextDataword = 0x0,
    #[doc = "First data word."]
    FirstDataword = 0x01,
}
impl Sof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sof {
    #[inline(always)]
    fn from(val: u8) -> Sof {
        Sof::from_bits(val)
    }
}
impl From<Sof> for u8 {
    #[inline(always)]
    fn from(val: Sof) -> u8 {
        Sof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdf {
    #[doc = "Transmit data not requested."]
    TxdataNotReqst = 0x0,
    #[doc = "Transmit data requested."]
    TxdataReqst = 0x01,
}
impl Tdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdf {
    #[inline(always)]
    fn from(val: u8) -> Tdf {
        Tdf::from_bits(val)
    }
}
impl From<Tdf> for u8 {
    #[inline(always)]
    fn from(val: Tdf) -> u8 {
        Tdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txmsk {
    #[doc = "Normal transfer."]
    Normal = 0x0,
    #[doc = "Mask transmit data."]
    Mask = 0x01,
}
impl Txmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txmsk {
    #[inline(always)]
    fn from(val: u8) -> Txmsk {
        Txmsk::from_bits(val)
    }
}
impl From<Txmsk> for u8 {
    #[inline(always)]
    fn from(val: Txmsk) -> u8 {
        Txmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Width {
    #[doc = "1-bit transfer."]
    Onebit = 0x0,
    #[doc = "2-bit transfer."]
    Twobit = 0x01,
    #[doc = "4-bit transfer."]
    Fourbit = 0x02,
    _RESERVED_3 = 0x03,
}
impl Width {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Width {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Width {
    #[inline(always)]
    fn from(val: u8) -> Width {
        Width::from_bits(val)
    }
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(val: Width) -> u8 {
        Width::to_bits(val)
    }
}
