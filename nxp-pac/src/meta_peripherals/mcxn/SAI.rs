#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "SAI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai {
    ptr: *mut u8,
}
unsafe impl Send for Sai {}
unsafe impl Sync for Sai {}
impl Sai {
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
    #[doc = "Transmit Control."]
    #[inline(always)]
    pub const fn tcsr(self) -> crate::pac::common::Reg<Tcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Transmit Configuration 1."]
    #[inline(always)]
    pub const fn tcr1(self) -> crate::pac::common::Reg<Tcr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Transmit Configuration 2."]
    #[inline(always)]
    pub const fn tcr2(self) -> crate::pac::common::Reg<Tcr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Transmit Configuration 3."]
    #[inline(always)]
    pub const fn tcr3(self) -> crate::pac::common::Reg<Tcr3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Transmit Configuration 4."]
    #[inline(always)]
    pub const fn tcr4(self) -> crate::pac::common::Reg<Tcr4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Transmit Configuration 5."]
    #[inline(always)]
    pub const fn tcr5(self) -> crate::pac::common::Reg<Tcr5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Transmit Data."]
    #[inline(always)]
    pub const fn tdr(self, n: usize) -> crate::pac::common::Reg<Tdr, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
        }
    }
    #[doc = "Transmit FIFO."]
    #[inline(always)]
    pub const fn tfr(self, n: usize) -> crate::pac::common::Reg<Tfr, crate::pac::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "Transmit Mask."]
    #[inline(always)]
    pub const fn tmr(self) -> crate::pac::common::Reg<Tmr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Receive Control."]
    #[inline(always)]
    pub const fn rcsr(self) -> crate::pac::common::Reg<Rcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Receive Configuration 1."]
    #[inline(always)]
    pub const fn rcr1(self) -> crate::pac::common::Reg<Rcr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Receive Configuration 2."]
    #[inline(always)]
    pub const fn rcr2(self) -> crate::pac::common::Reg<Rcr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Receive Configuration 3."]
    #[inline(always)]
    pub const fn rcr3(self) -> crate::pac::common::Reg<Rcr3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Receive Configuration 4."]
    #[inline(always)]
    pub const fn rcr4(self) -> crate::pac::common::Reg<Rcr4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Receive Configuration 5."]
    #[inline(always)]
    pub const fn rcr5(self) -> crate::pac::common::Reg<Rcr5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn rdr(self, n: usize) -> crate::pac::common::Reg<Rdr, crate::pac::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "Receive FIFO."]
    #[inline(always)]
    pub const fn rfr(self, n: usize) -> crate::pac::common::Reg<Rfr, crate::pac::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _)
        }
    }
    #[doc = "Receive Mask."]
    #[inline(always)]
    pub const fn rmr(self) -> crate::pac::common::Reg<Rmr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "MCLK Control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::pac::common::Reg<Mcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
}
#[doc = "MCLK Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "MCLK Post Divide."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MCLK Post Divide."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "MCLK Post Divide Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn diven(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MCLK Post Divide Enable."]
    #[inline(always)]
    pub const fn set_diven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "MCLK Select."]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> McrMsel {
        let val = (self.0 >> 24usize) & 0x03;
        McrMsel::from_bits(val as u8)
    }
    #[doc = "MCLK Select."]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: McrMsel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "MCLK Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn moe(&self) -> Moe {
        let val = (self.0 >> 30usize) & 0x01;
        Moe::from_bits(val as u8)
    }
    #[doc = "MCLK Output Enable."]
    #[inline(always)]
    pub const fn set_moe(&mut self, val: Moe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
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
            .field("div", &self.div())
            .field("diven", &self.diven())
            .field("msel", &self.msel())
            .field("moe", &self.moe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ div: {=u8:?}, diven: {=bool:?}, msel: {:?}, moe: {:?} }}",
            self.div(),
            self.diven(),
            self.msel(),
            self.moe()
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Number of Data Lines."]
    #[must_use]
    #[inline(always)]
    pub const fn dataline(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Data Lines."]
    #[inline(always)]
    pub const fn set_dataline(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Size."]
    #[inline(always)]
    pub const fn set_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Frame Size."]
    #[must_use]
    #[inline(always)]
    pub const fn frame(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Frame Size."]
    #[inline(always)]
    pub const fn set_frame(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
            .field("dataline", &self.dataline())
            .field("fifo", &self.fifo())
            .field("frame", &self.frame())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ dataline: {=u8:?}, fifo: {=u8:?}, frame: {=u8:?} }}",
            self.dataline(),
            self.fifo(),
            self.frame()
        )
    }
}
#[doc = "Receive Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr1(pub u32);
impl Rcr1 {
    #[doc = "Receive FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn rfw(&self) -> Rfw {
        let val = (self.0 >> 0usize) & 0x07;
        Rfw::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Watermark."]
    #[inline(always)]
    pub const fn set_rfw(&mut self, val: Rfw) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        Rcr1(0)
    }
}
impl core::fmt::Debug for Rcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr1").field("rfw", &self.rfw()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rcr1 {{ rfw: {:?} }}", self.rfw())
    }
}
#[doc = "Receive Configuration 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2(pub u32);
impl Rcr2 {
    #[doc = "Bit Clock Divide."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit Clock Divide."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit Clock Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn byp(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Bypass."]
    #[inline(always)]
    pub const fn set_byp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Bit Clock Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn bcd(&self) -> Rcr2Bcd {
        let val = (self.0 >> 24usize) & 0x01;
        Rcr2Bcd::from_bits(val as u8)
    }
    #[doc = "Bit Clock Direction."]
    #[inline(always)]
    pub const fn set_bcd(&mut self, val: Rcr2Bcd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit Clock Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn bcp(&self) -> Rcr2Bcp {
        let val = (self.0 >> 25usize) & 0x01;
        Rcr2Bcp::from_bits(val as u8)
    }
    #[doc = "Bit Clock Polarity."]
    #[inline(always)]
    pub const fn set_bcp(&mut self, val: Rcr2Bcp) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "MCLK Select."]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> Rcr2Msel {
        let val = (self.0 >> 26usize) & 0x03;
        Rcr2Msel::from_bits(val as u8)
    }
    #[doc = "MCLK Select."]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: Rcr2Msel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Bit Clock Input."]
    #[must_use]
    #[inline(always)]
    pub const fn bci(&self) -> Rcr2Bci {
        let val = (self.0 >> 28usize) & 0x01;
        Rcr2Bci::from_bits(val as u8)
    }
    #[doc = "Bit Clock Input."]
    #[inline(always)]
    pub const fn set_bci(&mut self, val: Rcr2Bci) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit Clock Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn bcs(&self) -> Rcr2Bcs {
        let val = (self.0 >> 29usize) & 0x01;
        Rcr2Bcs::from_bits(val as u8)
    }
    #[doc = "Bit Clock Swap."]
    #[inline(always)]
    pub const fn set_bcs(&mut self, val: Rcr2Bcs) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Synchronous Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> Rcr2Sync {
        let val = (self.0 >> 30usize) & 0x03;
        Rcr2Sync::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode."]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: Rcr2Sync) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        Rcr2(0)
    }
}
impl core::fmt::Debug for Rcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr2")
            .field("div", &self.div())
            .field("byp", &self.byp())
            .field("bcd", &self.bcd())
            .field("bcp", &self.bcp())
            .field("msel", &self.msel())
            .field("bci", &self.bci())
            .field("bcs", &self.bcs())
            .field("sync", &self.sync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr2 {{ div: {=u8:?}, byp: {=bool:?}, bcd: {:?}, bcp: {:?}, msel: {:?}, bci: {:?}, bcs: {:?}, sync: {:?} }}",
            self.div(),
            self.byp(),
            self.bcd(),
            self.bcp(),
            self.msel(),
            self.bci(),
            self.bcs(),
            self.sync()
        )
    }
}
#[doc = "Receive Configuration 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr3(pub u32);
impl Rcr3 {
    #[doc = "Word Flag Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn wdfl(&self) -> Wdfl {
        let val = (self.0 >> 0usize) & 0x1f;
        Wdfl::from_bits(val as u8)
    }
    #[doc = "Word Flag Configuration."]
    #[inline(always)]
    pub const fn set_wdfl(&mut self, val: Wdfl) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Receive Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Channel Enable."]
    #[inline(always)]
    pub const fn set_rce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Channel FIFO Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Channel FIFO Reset."]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Rcr3 {
    #[inline(always)]
    fn default() -> Rcr3 {
        Rcr3(0)
    }
}
impl core::fmt::Debug for Rcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr3")
            .field("wdfl", &self.wdfl())
            .field("rce", &self.rce())
            .field("cfr", &self.cfr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr3 {{ wdfl: {:?}, rce: {=u8:?}, cfr: {=u8:?} }}",
            self.wdfl(),
            self.rce(),
            self.cfr()
        )
    }
}
#[doc = "Receive Configuration 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr4(pub u32);
impl Rcr4 {
    #[doc = "Frame Sync Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn fsd(&self) -> Rcr4Fsd {
        let val = (self.0 >> 0usize) & 0x01;
        Rcr4Fsd::from_bits(val as u8)
    }
    #[doc = "Frame Sync Direction."]
    #[inline(always)]
    pub const fn set_fsd(&mut self, val: Rcr4Fsd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame Sync Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn fsp(&self) -> Rcr4Fsp {
        let val = (self.0 >> 1usize) & 0x01;
        Rcr4Fsp::from_bits(val as u8)
    }
    #[doc = "Frame Sync Polarity."]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: Rcr4Fsp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "On-Demand Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ondem(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "On-Demand Mode."]
    #[inline(always)]
    pub const fn set_ondem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame Sync Early."]
    #[must_use]
    #[inline(always)]
    pub const fn fse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Early."]
    #[inline(always)]
    pub const fn set_fse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSB First."]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSB First."]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Sync Width."]
    #[must_use]
    #[inline(always)]
    pub const fn sywd(&self) -> Sywd {
        let val = (self.0 >> 8usize) & 0x1f;
        Sywd::from_bits(val as u8)
    }
    #[doc = "Sync Width."]
    #[inline(always)]
    pub const fn set_sywd(&mut self, val: Sywd) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Frame Size."]
    #[must_use]
    #[inline(always)]
    pub const fn frsz(&self) -> Frsz {
        let val = (self.0 >> 16usize) & 0x1f;
        Frsz::from_bits(val as u8)
    }
    #[doc = "Frame Size."]
    #[inline(always)]
    pub const fn set_frsz(&mut self, val: Frsz) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "FIFO Packing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn fpack(&self) -> Rcr4Fpack {
        let val = (self.0 >> 24usize) & 0x03;
        Rcr4Fpack::from_bits(val as u8)
    }
    #[doc = "FIFO Packing Mode."]
    #[inline(always)]
    pub const fn set_fpack(&mut self, val: Rcr4Fpack) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "FIFO Combine Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn fcomb(&self) -> Rcr4Fcomb {
        let val = (self.0 >> 26usize) & 0x03;
        Rcr4Fcomb::from_bits(val as u8)
    }
    #[doc = "FIFO Combine Mode."]
    #[inline(always)]
    pub const fn set_fcomb(&mut self, val: Rcr4Fcomb) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Continue on Error."]
    #[must_use]
    #[inline(always)]
    pub const fn fcont(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Continue on Error."]
    #[inline(always)]
    pub const fn set_fcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        Rcr4(0)
    }
}
impl core::fmt::Debug for Rcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr4")
            .field("fsd", &self.fsd())
            .field("fsp", &self.fsp())
            .field("ondem", &self.ondem())
            .field("fse", &self.fse())
            .field("mf", &self.mf())
            .field("sywd", &self.sywd())
            .field("frsz", &self.frsz())
            .field("fpack", &self.fpack())
            .field("fcomb", &self.fcomb())
            .field("fcont", &self.fcont())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr4 {{ fsd: {:?}, fsp: {:?}, ondem: {=bool:?}, fse: {=bool:?}, mf: {=bool:?}, sywd: {:?}, frsz: {:?}, fpack: {:?}, fcomb: {:?}, fcont: {=bool:?} }}",
            self.fsd(),
            self.fsp(),
            self.ondem(),
            self.fse(),
            self.mf(),
            self.sywd(),
            self.frsz(),
            self.fpack(),
            self.fcomb(),
            self.fcont()
        )
    }
}
#[doc = "Receive Configuration 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr5(pub u32);
impl Rcr5 {
    #[doc = "First Bit Shifted."]
    #[must_use]
    #[inline(always)]
    pub const fn fbt(&self) -> Rcr5Fbt {
        let val = (self.0 >> 8usize) & 0x1f;
        Rcr5Fbt::from_bits(val as u8)
    }
    #[doc = "First Bit Shifted."]
    #[inline(always)]
    pub const fn set_fbt(&mut self, val: Rcr5Fbt) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Word 0 Width."]
    #[must_use]
    #[inline(always)]
    pub const fn w0w(&self) -> Rcr5W0w {
        let val = (self.0 >> 16usize) & 0x1f;
        Rcr5W0w::from_bits(val as u8)
    }
    #[doc = "Word 0 Width."]
    #[inline(always)]
    pub const fn set_w0w(&mut self, val: Rcr5W0w) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Word N Width."]
    #[must_use]
    #[inline(always)]
    pub const fn wnw(&self) -> Rcr5Wnw {
        let val = (self.0 >> 24usize) & 0x1f;
        Rcr5Wnw::from_bits(val as u8)
    }
    #[doc = "Word N Width."]
    #[inline(always)]
    pub const fn set_wnw(&mut self, val: Rcr5Wnw) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
}
impl Default for Rcr5 {
    #[inline(always)]
    fn default() -> Rcr5 {
        Rcr5(0)
    }
}
impl core::fmt::Debug for Rcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr5")
            .field("fbt", &self.fbt())
            .field("w0w", &self.w0w())
            .field("wnw", &self.wnw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr5 {{ fbt: {:?}, w0w: {:?}, wnw: {:?} }}",
            self.fbt(),
            self.w0w(),
            self.wnw()
        )
    }
}
#[doc = "Receive Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcsr(pub u32);
impl Rcsr {
    #[doc = "FIFO Request DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request DMA Enable."]
    #[inline(always)]
    pub const fn set_frde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Warning DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning DMA Enable."]
    #[inline(always)]
    pub const fn set_fwde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Request Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Interrupt Enable."]
    #[inline(always)]
    pub const fn set_frie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Warning Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sync Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn seie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word Start Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FIFO Request Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn frf(&self) -> RcsrFrf {
        let val = (self.0 >> 16usize) & 0x01;
        RcsrFrf::from_bits(val as u8)
    }
    #[doc = "FIFO Request Flag."]
    #[inline(always)]
    pub const fn set_frf(&mut self, val: RcsrFrf) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "FIFO Warning Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fwf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Flag."]
    #[inline(always)]
    pub const fn set_fwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FIFO Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag."]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Sync Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Flag."]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Word Start Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wsf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Flag."]
    #[inline(always)]
    pub const fn set_wsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> RcsrSr {
        let val = (self.0 >> 24usize) & 0x01;
        RcsrSr::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: RcsrSr) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FIFO Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fr(&self) -> RcsrFr {
        let val = (self.0 >> 25usize) & 0x01;
        RcsrFr::from_bits(val as u8)
    }
    #[doc = "FIFO Reset."]
    #[inline(always)]
    pub const fn set_fr(&mut self, val: RcsrFr) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bce(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Enable."]
    #[inline(always)]
    pub const fn set_bce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stope(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable."]
    #[inline(always)]
    pub const fn set_stope(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receiver Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable."]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rcsr {
    #[inline(always)]
    fn default() -> Rcsr {
        Rcsr(0)
    }
}
impl core::fmt::Debug for Rcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcsr")
            .field("frde", &self.frde())
            .field("fwde", &self.fwde())
            .field("frie", &self.frie())
            .field("fwie", &self.fwie())
            .field("feie", &self.feie())
            .field("seie", &self.seie())
            .field("wsie", &self.wsie())
            .field("frf", &self.frf())
            .field("fwf", &self.fwf())
            .field("fef", &self.fef())
            .field("sef", &self.sef())
            .field("wsf", &self.wsf())
            .field("sr", &self.sr())
            .field("fr", &self.fr())
            .field("bce", &self.bce())
            .field("dbge", &self.dbge())
            .field("stope", &self.stope())
            .field("re", &self.re())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcsr {{ frde: {=bool:?}, fwde: {=bool:?}, frie: {=bool:?}, fwie: {=bool:?}, feie: {=bool:?}, seie: {=bool:?}, wsie: {=bool:?}, frf: {:?}, fwf: {=bool:?}, fef: {=bool:?}, sef: {=bool:?}, wsf: {=bool:?}, sr: {:?}, fr: {:?}, bce: {=bool:?}, dbge: {=bool:?}, stope: {=bool:?}, re: {=bool:?} }}",
            self.frde(),
            self.fwde(),
            self.frie(),
            self.fwie(),
            self.feie(),
            self.seie(),
            self.wsie(),
            self.frf(),
            self.fwf(),
            self.fef(),
            self.sef(),
            self.wsf(),
            self.sr(),
            self.fr(),
            self.bce(),
            self.dbge(),
            self.stope(),
            self.re()
        )
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
    pub const fn rdr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_rdr(&mut self, val: u32) {
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
        f.debug_struct("Rdr").field("rdr", &self.rdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdr {{ rdr: {=u32:?} }}", self.rdr())
    }
}
#[doc = "Receive FIFO."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfr(pub u32);
impl Rfr {
    #[doc = "Read FIFO Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn rfp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Read FIFO Pointer."]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Read Channel Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn rcp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Read Channel Pointer."]
    #[inline(always)]
    pub const fn set_rcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write FIFO Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn wfp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Write FIFO Pointer."]
    #[inline(always)]
    pub const fn set_wfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Rfr {
    #[inline(always)]
    fn default() -> Rfr {
        Rfr(0)
    }
}
impl core::fmt::Debug for Rfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfr")
            .field("rfp", &self.rfp())
            .field("rcp", &self.rcp())
            .field("wfp", &self.wfp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rfr {{ rfp: {=u8:?}, rcp: {=bool:?}, wfp: {=u8:?} }}",
            self.rfp(),
            self.rcp(),
            self.wfp()
        )
    }
}
#[doc = "Receive Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmr(pub u32);
impl Rmr {
    #[doc = "Receive Word Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn rwm(&self) -> Rwm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        Rwm::from_bits(val as u32)
    }
    #[doc = "Receive Word Mask."]
    #[inline(always)]
    pub const fn set_rwm(&mut self, val: Rwm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rmr {
    #[inline(always)]
    fn default() -> Rmr {
        Rmr(0)
    }
}
impl core::fmt::Debug for Rmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmr").field("rwm", &self.rwm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rmr {{ rwm: {:?} }}", self.rwm())
    }
}
#[doc = "Transmit Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr1(pub u32);
impl Tcr1 {
    #[doc = "Transmit FIFO Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn tfw(&self) -> Tfw {
        let val = (self.0 >> 0usize) & 0x07;
        Tfw::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Watermark."]
    #[inline(always)]
    pub const fn set_tfw(&mut self, val: Tfw) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Tcr1 {
    #[inline(always)]
    fn default() -> Tcr1 {
        Tcr1(0)
    }
}
impl core::fmt::Debug for Tcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr1").field("tfw", &self.tfw()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcr1 {{ tfw: {:?} }}", self.tfw())
    }
}
#[doc = "Transmit Configuration 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr2(pub u32);
impl Tcr2 {
    #[doc = "Bit Clock Divide."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit Clock Divide."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit Clock Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn byp(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Bypass."]
    #[inline(always)]
    pub const fn set_byp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Bit Clock Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn bcd(&self) -> Tcr2Bcd {
        let val = (self.0 >> 24usize) & 0x01;
        Tcr2Bcd::from_bits(val as u8)
    }
    #[doc = "Bit Clock Direction."]
    #[inline(always)]
    pub const fn set_bcd(&mut self, val: Tcr2Bcd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit Clock Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn bcp(&self) -> Tcr2Bcp {
        let val = (self.0 >> 25usize) & 0x01;
        Tcr2Bcp::from_bits(val as u8)
    }
    #[doc = "Bit Clock Polarity."]
    #[inline(always)]
    pub const fn set_bcp(&mut self, val: Tcr2Bcp) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "MCLK Select."]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> Tcr2Msel {
        let val = (self.0 >> 26usize) & 0x03;
        Tcr2Msel::from_bits(val as u8)
    }
    #[doc = "MCLK Select."]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: Tcr2Msel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Bit Clock Input."]
    #[must_use]
    #[inline(always)]
    pub const fn bci(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Input."]
    #[inline(always)]
    pub const fn set_bci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit Clock Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn bcs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Swap."]
    #[inline(always)]
    pub const fn set_bcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Synchronous Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> Tcr2Sync {
        let val = (self.0 >> 30usize) & 0x03;
        Tcr2Sync::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode."]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: Tcr2Sync) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Tcr2 {
    #[inline(always)]
    fn default() -> Tcr2 {
        Tcr2(0)
    }
}
impl core::fmt::Debug for Tcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr2")
            .field("div", &self.div())
            .field("byp", &self.byp())
            .field("bcd", &self.bcd())
            .field("bcp", &self.bcp())
            .field("msel", &self.msel())
            .field("bci", &self.bci())
            .field("bcs", &self.bcs())
            .field("sync", &self.sync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr2 {{ div: {=u8:?}, byp: {=bool:?}, bcd: {:?}, bcp: {:?}, msel: {:?}, bci: {=bool:?}, bcs: {=bool:?}, sync: {:?} }}",
            self.div(),
            self.byp(),
            self.bcd(),
            self.bcp(),
            self.msel(),
            self.bci(),
            self.bcs(),
            self.sync()
        )
    }
}
#[doc = "Transmit Configuration 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr3(pub u32);
impl Tcr3 {
    #[doc = "Word Flag Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn wdfl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Word Flag Configuration."]
    #[inline(always)]
    pub const fn set_wdfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Transmit Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit Channel Enable."]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Channel FIFO Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Channel FIFO Reset."]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Tcr3 {
    #[inline(always)]
    fn default() -> Tcr3 {
        Tcr3(0)
    }
}
impl core::fmt::Debug for Tcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr3")
            .field("wdfl", &self.wdfl())
            .field("tce", &self.tce())
            .field("cfr", &self.cfr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr3 {{ wdfl: {=u8:?}, tce: {=u8:?}, cfr: {=u8:?} }}",
            self.wdfl(),
            self.tce(),
            self.cfr()
        )
    }
}
#[doc = "Transmit Configuration 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr4(pub u32);
impl Tcr4 {
    #[doc = "Frame Sync Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn fsd(&self) -> Tcr4Fsd {
        let val = (self.0 >> 0usize) & 0x01;
        Tcr4Fsd::from_bits(val as u8)
    }
    #[doc = "Frame Sync Direction."]
    #[inline(always)]
    pub const fn set_fsd(&mut self, val: Tcr4Fsd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame Sync Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn fsp(&self) -> Tcr4Fsp {
        let val = (self.0 >> 1usize) & 0x01;
        Tcr4Fsp::from_bits(val as u8)
    }
    #[doc = "Frame Sync Polarity."]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: Tcr4Fsp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "On-Demand Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ondem(&self) -> Tcr4Ondem {
        let val = (self.0 >> 2usize) & 0x01;
        Tcr4Ondem::from_bits(val as u8)
    }
    #[doc = "On-Demand Mode."]
    #[inline(always)]
    pub const fn set_ondem(&mut self, val: Tcr4Ondem) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame Sync Early."]
    #[must_use]
    #[inline(always)]
    pub const fn fse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Early."]
    #[inline(always)]
    pub const fn set_fse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSB First."]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSB First."]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn chmod(&self) -> Chmod {
        let val = (self.0 >> 5usize) & 0x01;
        Chmod::from_bits(val as u8)
    }
    #[doc = "Channel Mode."]
    #[inline(always)]
    pub const fn set_chmod(&mut self, val: Chmod) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Sync Width."]
    #[must_use]
    #[inline(always)]
    pub const fn sywd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sync Width."]
    #[inline(always)]
    pub const fn set_sywd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Frame Size."]
    #[must_use]
    #[inline(always)]
    pub const fn frsz(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Frame Size."]
    #[inline(always)]
    pub const fn set_frsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "FIFO Packing Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn fpack(&self) -> Tcr4Fpack {
        let val = (self.0 >> 24usize) & 0x03;
        Tcr4Fpack::from_bits(val as u8)
    }
    #[doc = "FIFO Packing Mode."]
    #[inline(always)]
    pub const fn set_fpack(&mut self, val: Tcr4Fpack) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "FIFO Combine Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn fcomb(&self) -> Tcr4Fcomb {
        let val = (self.0 >> 26usize) & 0x03;
        Tcr4Fcomb::from_bits(val as u8)
    }
    #[doc = "FIFO Combine Mode."]
    #[inline(always)]
    pub const fn set_fcomb(&mut self, val: Tcr4Fcomb) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Continue on Error."]
    #[must_use]
    #[inline(always)]
    pub const fn fcont(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Continue on Error."]
    #[inline(always)]
    pub const fn set_fcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Tcr4 {
    #[inline(always)]
    fn default() -> Tcr4 {
        Tcr4(0)
    }
}
impl core::fmt::Debug for Tcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr4")
            .field("fsd", &self.fsd())
            .field("fsp", &self.fsp())
            .field("ondem", &self.ondem())
            .field("fse", &self.fse())
            .field("mf", &self.mf())
            .field("chmod", &self.chmod())
            .field("sywd", &self.sywd())
            .field("frsz", &self.frsz())
            .field("fpack", &self.fpack())
            .field("fcomb", &self.fcomb())
            .field("fcont", &self.fcont())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr4 {{ fsd: {:?}, fsp: {:?}, ondem: {:?}, fse: {=bool:?}, mf: {=bool:?}, chmod: {:?}, sywd: {=u8:?}, frsz: {=u8:?}, fpack: {:?}, fcomb: {:?}, fcont: {=bool:?} }}",
            self.fsd(),
            self.fsp(),
            self.ondem(),
            self.fse(),
            self.mf(),
            self.chmod(),
            self.sywd(),
            self.frsz(),
            self.fpack(),
            self.fcomb(),
            self.fcont()
        )
    }
}
#[doc = "Transmit Configuration 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr5(pub u32);
impl Tcr5 {
    #[doc = "First Bit Shifted."]
    #[must_use]
    #[inline(always)]
    pub const fn fbt(&self) -> Tcr5Fbt {
        let val = (self.0 >> 8usize) & 0x1f;
        Tcr5Fbt::from_bits(val as u8)
    }
    #[doc = "First Bit Shifted."]
    #[inline(always)]
    pub const fn set_fbt(&mut self, val: Tcr5Fbt) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Word 0 Width."]
    #[must_use]
    #[inline(always)]
    pub const fn w0w(&self) -> Tcr5W0w {
        let val = (self.0 >> 16usize) & 0x1f;
        Tcr5W0w::from_bits(val as u8)
    }
    #[doc = "Word 0 Width."]
    #[inline(always)]
    pub const fn set_w0w(&mut self, val: Tcr5W0w) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Word N Width."]
    #[must_use]
    #[inline(always)]
    pub const fn wnw(&self) -> Tcr5Wnw {
        let val = (self.0 >> 24usize) & 0x1f;
        Tcr5Wnw::from_bits(val as u8)
    }
    #[doc = "Word N Width."]
    #[inline(always)]
    pub const fn set_wnw(&mut self, val: Tcr5Wnw) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
}
impl Default for Tcr5 {
    #[inline(always)]
    fn default() -> Tcr5 {
        Tcr5(0)
    }
}
impl core::fmt::Debug for Tcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr5")
            .field("fbt", &self.fbt())
            .field("w0w", &self.w0w())
            .field("wnw", &self.wnw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr5 {{ fbt: {:?}, w0w: {:?}, wnw: {:?} }}",
            self.fbt(),
            self.w0w(),
            self.wnw()
        )
    }
}
#[doc = "Transmit Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr(pub u32);
impl Tcsr {
    #[doc = "FIFO Request DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request DMA Enable."]
    #[inline(always)]
    pub const fn set_frde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Warning DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning DMA Enable."]
    #[inline(always)]
    pub const fn set_fwde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Request Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Interrupt Enable."]
    #[inline(always)]
    pub const fn set_frie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Warning Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sync Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn seie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word Start Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FIFO Request Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn frf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Flag."]
    #[inline(always)]
    pub const fn set_frf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FIFO Warning Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fwf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Flag."]
    #[inline(always)]
    pub const fn set_fwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FIFO Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag."]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Sync Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Flag."]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Word Start Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wsf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Flag."]
    #[inline(always)]
    pub const fn set_wsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "FIFO Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fr(&self) -> TcsrFr {
        let val = (self.0 >> 25usize) & 0x01;
        TcsrFr::from_bits(val as u8)
    }
    #[doc = "FIFO Reset."]
    #[inline(always)]
    pub const fn set_fr(&mut self, val: TcsrFr) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bce(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Enable."]
    #[inline(always)]
    pub const fn set_bce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn stope(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable."]
    #[inline(always)]
    pub const fn set_stope(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Transmitter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable."]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcsr {
    #[inline(always)]
    fn default() -> Tcsr {
        Tcsr(0)
    }
}
impl core::fmt::Debug for Tcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr")
            .field("frde", &self.frde())
            .field("fwde", &self.fwde())
            .field("frie", &self.frie())
            .field("fwie", &self.fwie())
            .field("feie", &self.feie())
            .field("seie", &self.seie())
            .field("wsie", &self.wsie())
            .field("frf", &self.frf())
            .field("fwf", &self.fwf())
            .field("fef", &self.fef())
            .field("sef", &self.sef())
            .field("wsf", &self.wsf())
            .field("sr", &self.sr())
            .field("fr", &self.fr())
            .field("bce", &self.bce())
            .field("dbge", &self.dbge())
            .field("stope", &self.stope())
            .field("te", &self.te())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr {{ frde: {=bool:?}, fwde: {=bool:?}, frie: {=bool:?}, fwie: {=bool:?}, feie: {=bool:?}, seie: {=bool:?}, wsie: {=bool:?}, frf: {=bool:?}, fwf: {=bool:?}, fef: {=bool:?}, sef: {=bool:?}, wsf: {=bool:?}, sr: {=bool:?}, fr: {:?}, bce: {=bool:?}, dbge: {=bool:?}, stope: {=bool:?}, te: {=bool:?} }}",
            self.frde(),
            self.fwde(),
            self.frie(),
            self.fwie(),
            self.feie(),
            self.seie(),
            self.wsie(),
            self.frf(),
            self.fwf(),
            self.fef(),
            self.sef(),
            self.wsf(),
            self.sr(),
            self.fr(),
            self.bce(),
            self.dbge(),
            self.stope(),
            self.te()
        )
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
    pub const fn tdr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data."]
    #[inline(always)]
    pub const fn set_tdr(&mut self, val: u32) {
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
        f.debug_struct("Tdr").field("tdr", &self.tdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr {{ tdr: {=u32:?} }}", self.tdr())
    }
}
#[doc = "Transmit FIFO."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfr(pub u32);
impl Tfr {
    #[doc = "Read FIFO Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn rfp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Read FIFO Pointer."]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Write FIFO Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn wfp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Write FIFO Pointer."]
    #[inline(always)]
    pub const fn set_wfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Write Channel Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn wcp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write Channel Pointer."]
    #[inline(always)]
    pub const fn set_wcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tfr {
    #[inline(always)]
    fn default() -> Tfr {
        Tfr(0)
    }
}
impl core::fmt::Debug for Tfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfr")
            .field("rfp", &self.rfp())
            .field("wfp", &self.wfp())
            .field("wcp", &self.wcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tfr {{ rfp: {=u8:?}, wfp: {=u8:?}, wcp: {=bool:?} }}",
            self.rfp(),
            self.wfp(),
            self.wcp()
        )
    }
}
#[doc = "Transmit Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr(pub u32);
impl Tmr {
    #[doc = "Transmit Word Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn twm(&self) -> Twm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        Twm::from_bits(val as u32)
    }
    #[doc = "Transmit Word Mask."]
    #[inline(always)]
    pub const fn set_twm(&mut self, val: Twm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tmr {
    #[inline(always)]
    fn default() -> Tmr {
        Tmr(0)
    }
}
impl core::fmt::Debug for Tmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr").field("twm", &self.twm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tmr {{ twm: {:?} }}", self.twm())
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
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number."]
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
pub enum Chmod {
    #[doc = "TDM mode."]
    TdmMode = 0x0,
    #[doc = "Output mode."]
    OutputMode = 0x01,
}
impl Chmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chmod {
    #[inline(always)]
    fn from(val: u8) -> Chmod {
        Chmod::from_bits(val)
    }
}
impl From<Chmod> for u8 {
    #[inline(always)]
    fn from(val: Chmod) -> u8 {
        Chmod::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set."]
    pub const Std: Self = Self(0x0);
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
            0x0 => f.write_str("Std"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Std"),
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
pub enum Frsz {
    #[doc = "1."]
    OneWord = 0x0,
    #[doc = "2."]
    TwoWords = 0x01,
    #[doc = "(FRSZ value + 1)."]
    NWords2 = 0x02,
    #[doc = "(FRSZ value + 1)."]
    NWords3 = 0x03,
    #[doc = "(FRSZ value + 1)."]
    NWords4 = 0x04,
    #[doc = "(FRSZ value + 1)."]
    NWords5 = 0x05,
    #[doc = "(FRSZ value + 1)."]
    NWords6 = 0x06,
    #[doc = "(FRSZ value + 1)."]
    NWords7 = 0x07,
    #[doc = "(FRSZ value + 1)."]
    NWords8 = 0x08,
    #[doc = "(FRSZ value + 1)."]
    NWords9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "32."]
    MaxWords = 0x1f,
}
impl Frsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frsz {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frsz {
    #[inline(always)]
    fn from(val: u8) -> Frsz {
        Frsz::from_bits(val)
    }
}
impl From<Frsz> for u8 {
    #[inline(always)]
    fn from(val: Frsz) -> u8 {
        Frsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrMsel {
    #[doc = "Controller clock (MCLK) option 1."]
    Mclk1 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Controller clock (MCLK) option 2."]
    Mclk2 = 0x02,
    #[doc = "Controller clock (MCLK) option 3."]
    Mclk3 = 0x03,
}
impl McrMsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrMsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrMsel {
    #[inline(always)]
    fn from(val: u8) -> McrMsel {
        McrMsel::from_bits(val)
    }
}
impl From<McrMsel> for u8 {
    #[inline(always)]
    fn from(val: McrMsel) -> u8 {
        McrMsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moe {
    #[doc = "Input."]
    Input = 0x0,
    #[doc = "Output."]
    Output = 0x01,
}
impl Moe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moe {
    #[inline(always)]
    fn from(val: u8) -> Moe {
        Moe::from_bits(val)
    }
}
impl From<Moe> for u8 {
    #[inline(always)]
    fn from(val: Moe) -> u8 {
        Moe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bcd {
    #[doc = "Generated externally in Target mode."]
    ExtTargetMode = 0x0,
    #[doc = "Generated internally in Controller mode."]
    IntControllerMode = 0x01,
}
impl Rcr2Bcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bcd {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bcd {
        Rcr2Bcd::from_bits(val)
    }
}
impl From<Rcr2Bcd> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bcd) -> u8 {
        Rcr2Bcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bci {
    #[doc = "Disable."]
    NoEffect = 0x0,
    #[doc = "Enable."]
    ClockedAsIfExtGenerated = 0x01,
}
impl Rcr2Bci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bci {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bci {
        Rcr2Bci::from_bits(val)
    }
}
impl From<Rcr2Bci> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bci) -> u8 {
        Rcr2Bci::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bcp {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl Rcr2Bcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bcp {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bcp {
        Rcr2Bcp::from_bits(val)
    }
}
impl From<Rcr2Bcp> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bcp) -> u8 {
        Rcr2Bcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bcs {
    #[doc = "Use the normal bit clock source."]
    Normal = 0x0,
    #[doc = "Swap the bit clock source."]
    SwapBitClkSource = 0x01,
}
impl Rcr2Bcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bcs {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bcs {
        Rcr2Bcs::from_bits(val)
    }
}
impl From<Rcr2Bcs> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bcs) -> u8 {
        Rcr2Bcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Msel {
    #[doc = "Bus clock."]
    BusClock = 0x0,
    #[doc = "Controller clock (MCLK) option 1."]
    Mclk1 = 0x01,
    #[doc = "Controller clock (MCLK) option 2."]
    Mclk2 = 0x02,
    #[doc = "Controller clock (MCLK) option 3."]
    Mclk3 = 0x03,
}
impl Rcr2Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Msel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Msel {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Msel {
        Rcr2Msel::from_bits(val)
    }
}
impl From<Rcr2Msel> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Msel) -> u8 {
        Rcr2Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Sync {
    #[doc = "Asynchronous mode."]
    Async = 0x0,
    #[doc = "Synchronous with transmitter."]
    SyncWTx = 0x01,
    #[doc = "Synchronous with another SAI receiver."]
    SyncWAnotherSaiRx = 0x02,
    #[doc = "Synchronous with another SAI transmitter."]
    SyncWAnotherSaiTx = 0x03,
}
impl Rcr2Sync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Sync {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Sync {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Sync {
        Rcr2Sync::from_bits(val)
    }
}
impl From<Rcr2Sync> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Sync) -> u8 {
        Rcr2Sync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fcomb {
    #[doc = "Disable."]
    Disabled = 0x0,
    #[doc = "Enable on FIFO writes (from receive shift registers)."]
    EnaOnFifoWrites = 0x01,
    #[doc = "Enable on FIFO reads (by software)."]
    EnaOnFifoReads = 0x02,
    #[doc = "Enable on FIFO writes (from receive shift registers) and reads (by software)."]
    EnaOnFifoWritesReads = 0x03,
}
impl Rcr4Fcomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fcomb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fcomb {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fcomb {
        Rcr4Fcomb::from_bits(val)
    }
}
impl From<Rcr4Fcomb> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fcomb) -> u8 {
        Rcr4Fcomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fpack {
    #[doc = "Disable."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enable 8-bit FIFO packing."]
    EightBitPacking = 0x02,
    #[doc = "Enable 16-bit FIFO packing."]
    SixteenBitPacking = 0x03,
}
impl Rcr4Fpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fpack {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fpack {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fpack {
        Rcr4Fpack::from_bits(val)
    }
}
impl From<Rcr4Fpack> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fpack) -> u8 {
        Rcr4Fpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fsd {
    #[doc = "Generated externally in Target mode."]
    ExtTargetMode = 0x0,
    #[doc = "Generated internally in Controller mode."]
    IntControllerMode = 0x01,
}
impl Rcr4Fsd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fsd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fsd {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fsd {
        Rcr4Fsd::from_bits(val)
    }
}
impl From<Rcr4Fsd> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fsd) -> u8 {
        Rcr4Fsd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fsp {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl Rcr4Fsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fsp {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fsp {
        Rcr4Fsp::from_bits(val)
    }
}
impl From<Rcr4Fsp> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fsp) -> u8 {
        Rcr4Fsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr5Fbt {
    #[doc = "0."]
    Index0 = 0x0,
    #[doc = "FBT value."]
    Index1 = 0x01,
    #[doc = "FBT value."]
    Index2 = 0x02,
    #[doc = "FBT value."]
    Index3 = 0x03,
    #[doc = "FBT value."]
    Index4 = 0x04,
    #[doc = "FBT value."]
    Index5 = 0x05,
    #[doc = "FBT value."]
    Index6 = 0x06,
    #[doc = "FBT value."]
    Index7 = 0x07,
    #[doc = "FBT value."]
    Index8 = 0x08,
    #[doc = "FBT value."]
    Index9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "31."]
    Index31 = 0x1f,
}
impl Rcr5Fbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr5Fbt {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr5Fbt {
    #[inline(always)]
    fn from(val: u8) -> Rcr5Fbt {
        Rcr5Fbt::from_bits(val)
    }
}
impl From<Rcr5Fbt> for u8 {
    #[inline(always)]
    fn from(val: Rcr5Fbt) -> u8 {
        Rcr5Fbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr5W0w {
    #[doc = "1."]
    Min = 0x0,
    #[doc = "2."]
    Two = 0x01,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone2 = 0x02,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone3 = 0x03,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone4 = 0x04,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone5 = 0x05,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone6 = 0x06,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone7 = 0x07,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone8 = 0x08,
    #[doc = "(W0W value + 1)."]
    ThreeThirtyone9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "32."]
    Max = 0x1f,
}
impl Rcr5W0w {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr5W0w {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr5W0w {
    #[inline(always)]
    fn from(val: u8) -> Rcr5W0w {
        Rcr5W0w::from_bits(val)
    }
}
impl From<Rcr5W0w> for u8 {
    #[inline(always)]
    fn from(val: Rcr5W0w) -> u8 {
        Rcr5W0w::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr5Wnw {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8."]
    Eight = 0x07,
    #[doc = "9."]
    Nine = 0x08,
    #[doc = "(WNW value + 1)."]
    TenThirtyone9 = 0x09,
    #[doc = "(WNW value + 1)."]
    TenThirtyone10 = 0x0a,
    #[doc = "(WNW value + 1)."]
    TenThirtyone11 = 0x0b,
    #[doc = "(WNW value + 1)."]
    TenThirtyone12 = 0x0c,
    #[doc = "(WNW value + 1)."]
    TenThirtyone13 = 0x0d,
    #[doc = "(WNW value + 1)."]
    TenThirtyone14 = 0x0e,
    #[doc = "(WNW value + 1)."]
    TenThirtyone15 = 0x0f,
    #[doc = "(WNW value + 1)."]
    TenThirtyone16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "32."]
    Max = 0x1f,
}
impl Rcr5Wnw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr5Wnw {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr5Wnw {
    #[inline(always)]
    fn from(val: u8) -> Rcr5Wnw {
        Rcr5Wnw::from_bits(val)
    }
}
impl From<Rcr5Wnw> for u8 {
    #[inline(always)]
    fn from(val: Rcr5Wnw) -> u8 {
        Rcr5Wnw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset."]
    FifoReset = 0x01,
}
impl RcsrFr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFr {
    #[inline(always)]
    fn from(val: u8) -> RcsrFr {
        RcsrFr::from_bits(val)
    }
}
impl From<RcsrFr> for u8 {
    #[inline(always)]
    fn from(val: RcsrFr) -> u8 {
        RcsrFr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFrf {
    #[doc = "Watermark not reached."]
    BelowWatermark = 0x0,
    #[doc = "Watermark reached."]
    WatermarkReached = 0x01,
}
impl RcsrFrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFrf {
    #[inline(always)]
    fn from(val: u8) -> RcsrFrf {
        RcsrFrf::from_bits(val)
    }
}
impl From<RcsrFrf> for u8 {
    #[inline(always)]
    fn from(val: RcsrFrf) -> u8 {
        RcsrFrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrSr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Software reset."]
    SwReset = 0x01,
}
impl RcsrSr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrSr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrSr {
    #[inline(always)]
    fn from(val: u8) -> RcsrSr {
        RcsrSr::from_bits(val)
    }
}
impl From<RcsrSr> for u8 {
    #[inline(always)]
    fn from(val: RcsrSr) -> u8 {
        RcsrSr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfw {
    #[doc = "1."]
    Min = 0x0,
    #[doc = "2."]
    Two = 0x01,
    #[doc = "(RFW value + 1)."]
    Watermark2 = 0x02,
    #[doc = "(RFW value + 1)."]
    Watermark3 = 0x03,
    #[doc = "(RFW value + 1)."]
    Watermark4 = 0x04,
    #[doc = "(RFW value + 1)."]
    Watermark5 = 0x05,
    #[doc = "(RFW value + 1)."]
    Watermark6 = 0x06,
    #[doc = "8."]
    Max = 0x07,
}
impl Rfw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfw {
    #[inline(always)]
    fn from(val: u8) -> Rfw {
        Rfw::from_bits(val)
    }
}
impl From<Rfw> for u8 {
    #[inline(always)]
    fn from(val: Rfw) -> u8 {
        Rfw::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rwm(u32);
impl Rwm {
    #[doc = "Enable."]
    pub const WordNEnabled: Self = Self(0x0);
    #[doc = "Mask."]
    pub const WordNMasked: Self = Self(0x01);
}
impl Rwm {
    pub const fn from_bits(val: u32) -> Rwm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Rwm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WordNEnabled"),
            0x01 => f.write_str("WordNMasked"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WordNEnabled"),
            0x01 => defmt::write!(f, "WordNMasked"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Rwm {
    #[inline(always)]
    fn from(val: u32) -> Rwm {
        Rwm::from_bits(val)
    }
}
impl From<Rwm> for u32 {
    #[inline(always)]
    fn from(val: Rwm) -> u32 {
        Rwm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sywd {
    #[doc = "1."]
    Min = 0x0,
    #[doc = "2."]
    TwoClocks = 0x01,
    #[doc = "(SYWD value + 1)."]
    NClocks2 = 0x02,
    #[doc = "(SYWD value + 1)."]
    NClocks3 = 0x03,
    #[doc = "(SYWD value + 1)."]
    NClocks4 = 0x04,
    #[doc = "(SYWD value + 1)."]
    NClocks5 = 0x05,
    #[doc = "(SYWD value + 1)."]
    NClocks6 = 0x06,
    #[doc = "(SYWD value + 1)."]
    NClocks7 = 0x07,
    #[doc = "(SYWD value + 1)."]
    NClocks8 = 0x08,
    #[doc = "(SYWD value + 1)."]
    NClocks9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "32."]
    ThirtytwoClocks = 0x1f,
}
impl Sywd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sywd {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sywd {
    #[inline(always)]
    fn from(val: u8) -> Sywd {
        Sywd::from_bits(val)
    }
}
impl From<Sywd> for u8 {
    #[inline(always)]
    fn from(val: Sywd) -> u8 {
        Sywd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Bcd {
    #[doc = "Generate externally in Target mode."]
    ExtInTarget = 0x0,
    #[doc = "Generate internally in Controller mode."]
    IntInController = 0x01,
}
impl Tcr2Bcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Bcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Bcd {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Bcd {
        Tcr2Bcd::from_bits(val)
    }
}
impl From<Tcr2Bcd> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Bcd) -> u8 {
        Tcr2Bcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Bcp {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl Tcr2Bcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Bcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Bcp {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Bcp {
        Tcr2Bcp::from_bits(val)
    }
}
impl From<Tcr2Bcp> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Bcp) -> u8 {
        Tcr2Bcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Msel {
    #[doc = "Bus clock."]
    BusClock = 0x0,
    #[doc = "Controller clock (MCLK) option 1."]
    Mclk1 = 0x01,
    #[doc = "Controller clock (MCLK) option 2."]
    Mclk2 = 0x02,
    #[doc = "Controller clock (MCLK) option 3."]
    Mclk3 = 0x03,
}
impl Tcr2Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Msel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Msel {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Msel {
        Tcr2Msel::from_bits(val)
    }
}
impl From<Tcr2Msel> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Msel) -> u8 {
        Tcr2Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Sync {
    #[doc = "Asynchronous mode."]
    Async = 0x0,
    #[doc = "Synchronous with receiver."]
    SyncWRx = 0x01,
    #[doc = "Synchronous with another SAI transmitter."]
    SyncWTx = 0x02,
    #[doc = "Synchronous with another SAI receiver."]
    SyncWAnotherSaiRx = 0x03,
}
impl Tcr2Sync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Sync {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Sync {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Sync {
        Tcr2Sync::from_bits(val)
    }
}
impl From<Tcr2Sync> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Sync) -> u8 {
        Tcr2Sync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fcomb {
    #[doc = "Disable."]
    Disabled = 0x0,
    #[doc = "Enable on FIFO reads (from transmit shift registers)."]
    EnabledOnFifoReads = 0x01,
    #[doc = "Enable on FIFO writes (by software)."]
    EnabledOnFifoWrites = 0x02,
    #[doc = "Enable on FIFO reads (from transmit shift registers) and writes (by software)."]
    EnabledOnFifoReadsWrites = 0x03,
}
impl Tcr4Fcomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fcomb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fcomb {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fcomb {
        Tcr4Fcomb::from_bits(val)
    }
}
impl From<Tcr4Fcomb> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fcomb) -> u8 {
        Tcr4Fcomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fpack {
    #[doc = "Disable FIFO packing."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enable 8-bit FIFO packing."]
    EightBitFifoPacking = 0x02,
    #[doc = "Enable 16-bit FIFO packing."]
    SixteenBitFifoPacking = 0x03,
}
impl Tcr4Fpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fpack {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fpack {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fpack {
        Tcr4Fpack::from_bits(val)
    }
}
impl From<Tcr4Fpack> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fpack) -> u8 {
        Tcr4Fpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fsd {
    #[doc = "Generated externally in Target mode."]
    ExtInTargetMode = 0x0,
    #[doc = "Generated internally in Controller mode."]
    IntInControllerMode = 0x01,
}
impl Tcr4Fsd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fsd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fsd {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fsd {
        Tcr4Fsd::from_bits(val)
    }
}
impl From<Tcr4Fsd> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fsd) -> u8 {
        Tcr4Fsd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fsp {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl Tcr4Fsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fsp {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fsp {
        Tcr4Fsp::from_bits(val)
    }
}
impl From<Tcr4Fsp> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fsp) -> u8 {
        Tcr4Fsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Ondem {
    #[doc = "Generated continuously."]
    ContinuousFrameSync = 0x0,
    #[doc = "Generated after the FIFO warning flag is cleared."]
    OnDemandFrameSync = 0x01,
}
impl Tcr4Ondem {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Ondem {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Ondem {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Ondem {
        Tcr4Ondem::from_bits(val)
    }
}
impl From<Tcr4Ondem> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Ondem) -> u8 {
        Tcr4Ondem::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr5Fbt {
    #[doc = "0."]
    Index0 = 0x0,
    #[doc = "FBT."]
    Index1 = 0x01,
    #[doc = "FBT."]
    Index2 = 0x02,
    #[doc = "FBT."]
    Index3 = 0x03,
    #[doc = "FBT."]
    Index4 = 0x04,
    #[doc = "FBT."]
    Index5 = 0x05,
    #[doc = "FBT."]
    Index6 = 0x06,
    #[doc = "FBT."]
    Index7 = 0x07,
    #[doc = "FBT."]
    Index8 = 0x08,
    #[doc = "FBT."]
    Index9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "31."]
    Index31 = 0x1f,
}
impl Tcr5Fbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr5Fbt {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr5Fbt {
    #[inline(always)]
    fn from(val: u8) -> Tcr5Fbt {
        Tcr5Fbt::from_bits(val)
    }
}
impl From<Tcr5Fbt> for u8 {
    #[inline(always)]
    fn from(val: Tcr5Fbt) -> u8 {
        Tcr5Fbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr5W0w {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8."]
    Eight = 0x07,
    #[doc = "9."]
    Nine = 0x08,
    #[doc = "(W0W value + 1)."]
    TenThirtyone9 = 0x09,
    #[doc = "(W0W value + 1)."]
    TenThirtyone10 = 0x0a,
    #[doc = "(W0W value + 1)."]
    TenThirtyone11 = 0x0b,
    #[doc = "(W0W value + 1)."]
    TenThirtyone12 = 0x0c,
    #[doc = "(W0W value + 1)."]
    TenThirtyone13 = 0x0d,
    #[doc = "(W0W value + 1)."]
    TenThirtyone14 = 0x0e,
    #[doc = "(W0W value + 1)."]
    TenThirtyone15 = 0x0f,
    #[doc = "(W0W value + 1)."]
    TenThirtyone16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "32."]
    Max = 0x1f,
}
impl Tcr5W0w {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr5W0w {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr5W0w {
    #[inline(always)]
    fn from(val: u8) -> Tcr5W0w {
        Tcr5W0w::from_bits(val)
    }
}
impl From<Tcr5W0w> for u8 {
    #[inline(always)]
    fn from(val: Tcr5W0w) -> u8 {
        Tcr5W0w::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr5Wnw {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8."]
    Eight = 0x07,
    #[doc = "9."]
    Nine = 0x08,
    #[doc = "(WNW value + 1)."]
    TenThirtyone9 = 0x09,
    #[doc = "(WNW value + 1)."]
    TenThirtyone10 = 0x0a,
    #[doc = "(WNW value + 1)."]
    TenThirtyone11 = 0x0b,
    #[doc = "(WNW value + 1)."]
    TenThirtyone12 = 0x0c,
    #[doc = "(WNW value + 1)."]
    TenThirtyone13 = 0x0d,
    #[doc = "(WNW value + 1)."]
    TenThirtyone14 = 0x0e,
    #[doc = "(WNW value + 1)."]
    TenThirtyone15 = 0x0f,
    #[doc = "(WNW value + 1)."]
    TenThirtyone16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "32."]
    Max = 0x1f,
}
impl Tcr5Wnw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr5Wnw {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr5Wnw {
    #[inline(always)]
    fn from(val: u8) -> Tcr5Wnw {
        Tcr5Wnw::from_bits(val)
    }
}
impl From<Tcr5Wnw> for u8 {
    #[inline(always)]
    fn from(val: Tcr5Wnw) -> u8 {
        Tcr5Wnw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "FIFO reset."]
    Reset = 0x01,
}
impl TcsrFr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFr {
    #[inline(always)]
    fn from(val: u8) -> TcsrFr {
        TcsrFr::from_bits(val)
    }
}
impl From<TcsrFr> for u8 {
    #[inline(always)]
    fn from(val: TcsrFr) -> u8 {
        TcsrFr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfw {
    #[doc = "1."]
    Min = 0x0,
    #[doc = "2."]
    Two = 0x01,
    #[doc = "(TFW +1)."]
    WatermarkValue2 = 0x02,
    #[doc = "(TFW +1)."]
    WatermarkValue3 = 0x03,
    #[doc = "(TFW +1)."]
    WatermarkValue4 = 0x04,
    #[doc = "(TFW +1)."]
    WatermarkValue5 = 0x05,
    #[doc = "(TFW +1)."]
    WatermarkValue6 = 0x06,
    #[doc = "8."]
    Max = 0x07,
}
impl Tfw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfw {
    #[inline(always)]
    fn from(val: u8) -> Tfw {
        Tfw::from_bits(val)
    }
}
impl From<Tfw> for u8 {
    #[inline(always)]
    fn from(val: Tfw) -> u8 {
        Tfw::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Twm(u32);
impl Twm {
    #[doc = "Enable."]
    pub const WordNEnabled: Self = Self(0x0);
    #[doc = "Mask."]
    pub const WordNMasked: Self = Self(0x01);
}
impl Twm {
    pub const fn from_bits(val: u32) -> Twm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Twm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WordNEnabled"),
            0x01 => f.write_str("WordNMasked"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Twm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WordNEnabled"),
            0x01 => defmt::write!(f, "WordNMasked"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Twm {
    #[inline(always)]
    fn from(val: u32) -> Twm {
        Twm::from_bits(val)
    }
}
impl From<Twm> for u32 {
    #[inline(always)]
    fn from(val: Twm) -> u32 {
        Twm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdfl {
    #[doc = "Word 1."]
    Word1 = 0x0,
    #[doc = "Word 2."]
    Word2 = 0x01,
    #[doc = "Word (WDFL value + 1)."]
    WordN2 = 0x02,
    #[doc = "Word (WDFL value + 1)."]
    WordN3 = 0x03,
    #[doc = "Word (WDFL value + 1)."]
    WordN4 = 0x04,
    #[doc = "Word (WDFL value + 1)."]
    WordN5 = 0x05,
    #[doc = "Word (WDFL value + 1)."]
    WordN6 = 0x06,
    #[doc = "Word (WDFL value + 1)."]
    WordN7 = 0x07,
    #[doc = "Word (WDFL value + 1)."]
    WordN8 = 0x08,
    #[doc = "Word (WDFL value + 1)."]
    WordN9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Word 32."]
    WordMax = 0x1f,
}
impl Wdfl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdfl {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdfl {
    #[inline(always)]
    fn from(val: u8) -> Wdfl {
        Wdfl::from_bits(val)
    }
}
impl From<Wdfl> for u8 {
    #[inline(always)]
    fn from(val: Wdfl) -> u8 {
        Wdfl::to_bits(val)
    }
}
