#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "12-bit DAC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version Identifier."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn data(self) -> crate::pac::common::Reg<Data, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Global Control."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::pac::common::Reg<Gcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DAC FIFO Control."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::pac::common::Reg<Fcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DAC FIFO Pointer."]
    #[inline(always)]
    pub const fn fpr(self) -> crate::pac::common::Reg<Fpr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "FIFO Status."]
    #[inline(always)]
    pub const fn fsr(self) -> crate::pac::common::Reg<Fsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::pac::common::Reg<Ier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn der(self) -> crate::pac::common::Reg<Der, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Reset Control."]
    #[inline(always)]
    pub const fn rcr(self) -> crate::pac::common::Reg<Rcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Trigger Control."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::pac::common::Reg<Tcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Periodic Trigger Control."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::pac::common::Reg<Pcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Test Control Register."]
    #[inline(always)]
    pub const fn tst(self) -> crate::pac::common::Reg<Tst, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
#[doc = "Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "FIFO Entry or Buffer Entry."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "FIFO Entry or Buffer Entry."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Data {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "DMA Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Der(pub u32);
impl Der {
    #[doc = "FIFO Empty DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn empty_dmaen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Empty DMA Enable."]
    #[inline(always)]
    pub const fn set_empty_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Watermark DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wm_dmaen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Watermark DMA Enable."]
    #[inline(always)]
    pub const fn set_wm_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("empty_dmaen", &self.empty_dmaen())
            .field("wm_dmaen", &self.wm_dmaen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Der {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Der {{ empty_dmaen: {=bool:?}, wm_dmaen: {=bool:?} }}",
            self.empty_dmaen(),
            self.wm_dmaen()
        )
    }
}
#[doc = "DAC FIFO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc = "Watermark Level."]
    #[must_use]
    #[inline(always)]
    pub const fn wml(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Watermark Level."]
    #[inline(always)]
    pub const fn set_wml(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
        f.debug_struct("Fcr").field("wml", &self.wml()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fcr {{ wml: {=u8:?} }}", self.wml())
    }
}
#[doc = "DAC FIFO Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpr(pub u32);
impl Fpr {
    #[doc = "FIFO Read Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_rpt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Read Pointer."]
    #[inline(always)]
    pub const fn set_fifo_rpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "FIFO Write Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_wpt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Write Pointer."]
    #[inline(always)]
    pub const fn set_fifo_wpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fpr {
    #[inline(always)]
    fn default() -> Fpr {
        Fpr(0)
    }
}
impl core::fmt::Debug for Fpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fpr")
            .field("fifo_rpt", &self.fifo_rpt())
            .field("fifo_wpt", &self.fifo_wpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fpr {{ fifo_rpt: {=u8:?}, fifo_wpt: {=u8:?} }}",
            self.fifo_rpt(),
            self.fifo_wpt()
        )
    }
}
#[doc = "FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsr(pub u32);
impl Fsr {
    #[doc = "FIFO Full Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Full Flag."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Empty Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Empty Flag."]
    #[inline(always)]
    pub const fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Watermark Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wm(&self) -> Wm {
        let val = (self.0 >> 2usize) & 0x01;
        Wm::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark Status Flag."]
    #[inline(always)]
    pub const fn set_wm(&mut self, val: Wm) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Swing Back One Cycle Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn swbk(&self) -> Swbk {
        let val = (self.0 >> 3usize) & 0x01;
        Swbk::from_bits(val as u8)
    }
    #[doc = "Swing Back One Cycle Complete Flag."]
    #[inline(always)]
    pub const fn set_swbk(&mut self, val: Swbk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FIFO Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Flag."]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO Underflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Flag."]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Period Trigger Mode Conversion Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ptgcoco(&self) -> Ptgcoco {
        let val = (self.0 >> 8usize) & 0x01;
        Ptgcoco::from_bits(val as u8)
    }
    #[doc = "Period Trigger Mode Conversion Complete Flag."]
    #[inline(always)]
    pub const fn set_ptgcoco(&mut self, val: Ptgcoco) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
            .field("full", &self.full())
            .field("empty", &self.empty())
            .field("wm", &self.wm())
            .field("swbk", &self.swbk())
            .field("of", &self.of())
            .field("uf", &self.uf())
            .field("ptgcoco", &self.ptgcoco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fsr {{ full: {=bool:?}, empty: {=bool:?}, wm: {:?}, swbk: {:?}, of: {=bool:?}, uf: {=bool:?}, ptgcoco: {:?} }}",
            self.full(),
            self.empty(),
            self.wm(),
            self.swbk(),
            self.of(),
            self.uf(),
            self.ptgcoco()
        )
    }
}
#[doc = "Global Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc = "DAC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dacen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Enable."]
    #[inline(always)]
    pub const fn set_dacen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DAC Reference Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dacrfs(&self) -> Dacrfs {
        let val = (self.0 >> 1usize) & 0x03;
        Dacrfs::from_bits(val as u8)
    }
    #[doc = "DAC Reference Select."]
    #[inline(always)]
    pub const fn set_dacrfs(&mut self, val: Dacrfs) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoen(&self) -> Fifoen {
        let val = (self.0 >> 3usize) & 0x01;
        Fifoen::from_bits(val as u8)
    }
    #[doc = "FIFO Enable."]
    #[inline(always)]
    pub const fn set_fifoen(&mut self, val: Fifoen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Swing Back Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn swmd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Swing Back Mode."]
    #[inline(always)]
    pub const fn set_swmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> Trgsel {
        let val = (self.0 >> 5usize) & 0x01;
        Trgsel::from_bits(val as u8)
    }
    #[doc = "DAC Trigger Select."]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: Trgsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DAC Periodic Trigger Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ptgen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Periodic Trigger Mode Enable."]
    #[inline(always)]
    pub const fn set_ptgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RCLK Cycles Before Data Latch."]
    #[must_use]
    #[inline(always)]
    pub const fn latch_cyc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "RCLK Cycles Before Data Latch."]
    #[inline(always)]
    pub const fn set_latch_cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn buf_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Enable."]
    #[inline(always)]
    pub const fn set_buf_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "External On-Chip PTAT Current Reference Select."]
    #[must_use]
    #[inline(always)]
    pub const fn iref_ptat_ext_sel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "External On-Chip PTAT Current Reference Select."]
    #[inline(always)]
    pub const fn set_iref_ptat_ext_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "External On-Chip ZTC Current Reference Select."]
    #[must_use]
    #[inline(always)]
    pub const fn iref_ztc_ext_sel(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "External On-Chip ZTC Current Reference Select."]
    #[inline(always)]
    pub const fn set_iref_ztc_ext_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OPAMP as Buffer, Speed Control Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn buf_spd_ctrl(&self) -> BufSpdCtrl {
        let val = (self.0 >> 23usize) & 0x01;
        BufSpdCtrl::from_bits(val as u8)
    }
    #[doc = "OPAMP as Buffer, Speed Control Signal."]
    #[inline(always)]
    pub const fn set_buf_spd_ctrl(&mut self, val: BufSpdCtrl) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Gcr {
    #[inline(always)]
    fn default() -> Gcr {
        Gcr(0)
    }
}
impl core::fmt::Debug for Gcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcr")
            .field("dacen", &self.dacen())
            .field("dacrfs", &self.dacrfs())
            .field("fifoen", &self.fifoen())
            .field("swmd", &self.swmd())
            .field("trgsel", &self.trgsel())
            .field("ptgen", &self.ptgen())
            .field("latch_cyc", &self.latch_cyc())
            .field("buf_en", &self.buf_en())
            .field("iref_ptat_ext_sel", &self.iref_ptat_ext_sel())
            .field("iref_ztc_ext_sel", &self.iref_ztc_ext_sel())
            .field("buf_spd_ctrl", &self.buf_spd_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcr {{ dacen: {=bool:?}, dacrfs: {:?}, fifoen: {:?}, swmd: {=bool:?}, trgsel: {:?}, ptgen: {=bool:?}, latch_cyc: {=u8:?}, buf_en: {=bool:?}, iref_ptat_ext_sel: {=bool:?}, iref_ztc_ext_sel: {=bool:?}, buf_spd_ctrl: {:?} }}",
            self.dacen(),
            self.dacrfs(),
            self.fifoen(),
            self.swmd(),
            self.trgsel(),
            self.ptgen(),
            self.latch_cyc(),
            self.buf_en(),
            self.iref_ptat_ext_sel(),
            self.iref_ztc_ext_sel(),
            self.buf_spd_ctrl()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "FIFO Full Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn full_ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Full Interrupt Enable."]
    #[inline(always)]
    pub const fn set_full_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Empty Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn empty_ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Empty Interrupt Enable."]
    #[inline(always)]
    pub const fn set_empty_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Watermark Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wm_ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Watermark Interrupt Enable."]
    #[inline(always)]
    pub const fn set_wm_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Swing Back One Cycle Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swbk_ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Swing Back One Cycle Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_swbk_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FIFO Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn of_ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_of_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO Underflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uf_ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_uf_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PTG Mode Conversion Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ptgcoco_ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PTG Mode Conversion Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ptgcoco_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("full_ie", &self.full_ie())
            .field("empty_ie", &self.empty_ie())
            .field("wm_ie", &self.wm_ie())
            .field("swbk_ie", &self.swbk_ie())
            .field("of_ie", &self.of_ie())
            .field("uf_ie", &self.uf_ie())
            .field("ptgcoco_ie", &self.ptgcoco_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ full_ie: {=bool:?}, empty_ie: {=bool:?}, wm_ie: {=bool:?}, swbk_ie: {=bool:?}, of_ie: {=bool:?}, uf_ie: {=bool:?}, ptgcoco_ie: {=bool:?} }}",
            self.full_ie(),
            self.empty_ie(),
            self.wm_ie(),
            self.swbk_ie(),
            self.of_ie(),
            self.uf_ie(),
            self.ptgcoco_ie()
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn fifosz(&self) -> Fifosz {
        let val = (self.0 >> 0usize) & 0x07;
        Fifosz::from_bits(val as u8)
    }
    #[doc = "FIFO Size."]
    #[inline(always)]
    pub const fn set_fifosz(&mut self, val: Fifosz) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
            .field("fifosz", &self.fifosz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ fifosz: {:?} }}", self.fifosz())
    }
}
#[doc = "Periodic Trigger Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "Periodic Trigger Number."]
    #[must_use]
    #[inline(always)]
    pub const fn ptg_num(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Periodic Trigger Number."]
    #[inline(always)]
    pub const fn set_ptg_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Periodic Trigger Period Width."]
    #[must_use]
    #[inline(always)]
    pub const fn ptg_period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Periodic Trigger Period Width."]
    #[inline(always)]
    pub const fn set_ptg_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pcr {
    #[inline(always)]
    fn default() -> Pcr {
        Pcr(0)
    }
}
impl core::fmt::Debug for Pcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr")
            .field("ptg_num", &self.ptg_num())
            .field("ptg_period", &self.ptg_period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr {{ ptg_num: {=u16:?}, ptg_period: {=u16:?} }}",
            self.ptg_num(),
            self.ptg_period()
        )
    }
}
#[doc = "Reset Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swrst(&self) -> Swrst {
        let val = (self.0 >> 0usize) & 0x01;
        Swrst::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: Swrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fiforst(&self) -> Fiforst {
        let val = (self.0 >> 1usize) & 0x01;
        Fiforst::from_bits(val as u8)
    }
    #[doc = "FIFO Reset."]
    #[inline(always)]
    pub const fn set_fiforst(&mut self, val: Fiforst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0)
    }
}
impl core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr")
            .field("swrst", &self.swrst())
            .field("fiforst", &self.fiforst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr {{ swrst: {:?}, fiforst: {:?} }}",
            self.swrst(),
            self.fiforst()
        )
    }
}
#[doc = "Trigger Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Software Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn swtrg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Trigger."]
    #[inline(always)]
    pub const fn set_swtrg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        f.debug_struct("Tcr").field("swtrg", &self.swtrg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcr {{ swtrg: {=bool:?} }}", self.swtrg())
    }
}
#[doc = "Test Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tst(pub u32);
impl Tst {
    #[doc = "Option mux control for ATX bus function."]
    #[must_use]
    #[inline(always)]
    pub const fn opt_mux_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Option mux control for ATX bus function."]
    #[inline(always)]
    pub const fn set_opt_mux_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Tst {
    #[inline(always)]
    fn default() -> Tst {
        Tst(0)
    }
}
impl core::fmt::Debug for Tst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tst")
            .field("opt_mux_en", &self.opt_mux_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tst {{ opt_mux_en: {=bool:?} }}", self.opt_mux_en())
    }
}
#[doc = "Version Identifier."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Identification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Identification Number."]
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
pub enum BufSpdCtrl {
    #[doc = "Lower Low-Power mode."]
    LlpMode = 0x0,
    #[doc = "Low-Power mode."]
    LpMode = 0x01,
}
impl BufSpdCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BufSpdCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BufSpdCtrl {
    #[inline(always)]
    fn from(val: u8) -> BufSpdCtrl {
        BufSpdCtrl::from_bits(val)
    }
}
impl From<BufSpdCtrl> for u8 {
    #[inline(always)]
    fn from(val: BufSpdCtrl) -> u8 {
        BufSpdCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dacrfs {
    #[doc = "Selects VREFH0 as the reference voltage."]
    Vrefh0 = 0x0,
    #[doc = "Selects VREFH1 as the reference voltage."]
    Vrefh1 = 0x01,
    #[doc = "Selects VREFH2 as the reference voltage."]
    Vrefh2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dacrfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dacrfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dacrfs {
    #[inline(always)]
    fn from(val: u8) -> Dacrfs {
        Dacrfs::from_bits(val)
    }
}
impl From<Dacrfs> for u8 {
    #[inline(always)]
    fn from(val: Dacrfs) -> u8 {
        Dacrfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoen {
    #[doc = "Disables FIFO mode and enables Buffer mode. Any data written to DATA\\[DATA\\] goes to buffer then goes to conversion."]
    BufferMode = 0x0,
    #[doc = "Enables FIFO mode. Data will be first read from FIFO to buffer and then goes to conversion."]
    FifoMode = 0x01,
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
pub enum Fiforst {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "FIFO reset."]
    FifoReset = 0x01,
}
impl Fiforst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fiforst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fiforst {
    #[inline(always)]
    fn from(val: u8) -> Fiforst {
        Fiforst::from_bits(val)
    }
}
impl From<Fiforst> for u8 {
    #[inline(always)]
    fn from(val: Fiforst) -> u8 {
        Fiforst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifosz {
    _RESERVED_0 = 0x0,
    #[doc = "FIFO depth is 4."]
    Val1 = 0x01,
    #[doc = "FIFO depth is 8."]
    Val2 = 0x02,
    #[doc = "FIFO depth is 16."]
    Val3 = 0x03,
    #[doc = "FIFO depth is 32."]
    Val4 = 0x04,
    #[doc = "FIFO depth is 64."]
    Val5 = 0x05,
    #[doc = "FIFO depth is 128."]
    Val6 = 0x06,
    #[doc = "FIFO depth is 256."]
    Val7 = 0x07,
}
impl Fifosz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifosz {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifosz {
    #[inline(always)]
    fn from(val: u8) -> Fifosz {
        Fifosz::from_bits(val)
    }
}
impl From<Fifosz> for u8 {
    #[inline(always)]
    fn from(val: Fifosz) -> u8 {
        Fifosz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptgcoco {
    #[doc = "Not completed or not started."]
    NotStart = 0x0,
    #[doc = "Completed."]
    Completed = 0x01,
}
impl Ptgcoco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptgcoco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptgcoco {
    #[inline(always)]
    fn from(val: u8) -> Ptgcoco {
        Ptgcoco::from_bits(val)
    }
}
impl From<Ptgcoco> for u8 {
    #[inline(always)]
    fn from(val: Ptgcoco) -> u8 {
        Ptgcoco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swbk {
    #[doc = "No swing back cycle has completed since the last time the flag was cleared."]
    NoSwing = 0x0,
    #[doc = "At least one swing back cycle has occurred since the last time the flag was cleared."]
    SwingBack = 0x01,
}
impl Swbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swbk {
    #[inline(always)]
    fn from(val: u8) -> Swbk {
        Swbk::from_bits(val)
    }
}
impl From<Swbk> for u8 {
    #[inline(always)]
    fn from(val: Swbk) -> u8 {
        Swbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swrst {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Software reset."]
    SoftwareReset = 0x01,
}
impl Swrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swrst {
    #[inline(always)]
    fn from(val: u8) -> Swrst {
        Swrst::from_bits(val)
    }
}
impl From<Swrst> for u8 {
    #[inline(always)]
    fn from(val: Swrst) -> u8 {
        Swrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsel {
    #[doc = "Hardware trigger."]
    Hardware = 0x0,
    #[doc = "Software trigger."]
    Software = 0x01,
}
impl Trgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsel {
    #[inline(always)]
    fn from(val: u8) -> Trgsel {
        Trgsel::from_bits(val)
    }
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(val: Trgsel) -> u8 {
        Trgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wm {
    #[doc = "Data in FIFO is more than watermark level."]
    MoreThanWlevel = 0x0,
    #[doc = "Data in FIFO is less than or equal to watermark level."]
    LessThanWlevel = 0x01,
}
impl Wm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wm {
    #[inline(always)]
    fn from(val: u8) -> Wm {
        Wm::from_bits(val)
    }
}
impl From<Wm> for u8 {
    #[inline(always)]
    fn from(val: Wm) -> u8 {
        Wm::to_bits(val)
    }
}
