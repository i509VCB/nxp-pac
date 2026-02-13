#[doc = "Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "FIFO Entry or Buffer Entry"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "FIFO Entry or Buffer Entry"]
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
#[doc = "DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Der(pub u32);
impl Der {
    #[doc = "FIFO Empty DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn empty_dmaen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Empty DMA Enable"]
    #[inline(always)]
    pub const fn set_empty_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Watermark DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wm_dmaen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Watermark DMA Enable"]
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
#[doc = "DAC FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc = "Watermark Level"]
    #[must_use]
    #[inline(always)]
    pub const fn wml(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Watermark Level"]
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
#[doc = "DAC FIFO Pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpr(pub u32);
impl Fpr {
    #[doc = "FIFO Read Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_rpt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Read Pointer"]
    #[inline(always)]
    pub const fn set_fifo_rpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "FIFO Write Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_wpt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Write Pointer"]
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
#[doc = "FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsr(pub u32);
impl Fsr {
    #[doc = "FIFO Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Full Flag"]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Empty Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Empty Flag"]
    #[inline(always)]
    pub const fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Watermark Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wm(&self) -> super::vals::Wm {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wm::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark Status Flag"]
    #[inline(always)]
    pub const fn set_wm(&mut self, val: super::vals::Wm) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Swing Back One Cycle Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn swbk(&self) -> super::vals::Swbk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Swbk::from_bits(val as u8)
    }
    #[doc = "Swing Back One Cycle Complete Flag"]
    #[inline(always)]
    pub const fn set_swbk(&mut self, val: super::vals::Swbk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn of(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn uf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Period Trigger Mode Conversion Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ptgcoco(&self) -> super::vals::Ptgcoco {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ptgcoco::from_bits(val as u8)
    }
    #[doc = "Period Trigger Mode Conversion Complete Flag"]
    #[inline(always)]
    pub const fn set_ptgcoco(&mut self, val: super::vals::Ptgcoco) {
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
#[doc = "Global Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc = "DAC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dacen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Enable"]
    #[inline(always)]
    pub const fn set_dacen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DAC Reference Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dacrfs(&self) -> super::vals::Dacrfs {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Dacrfs::from_bits(val as u8)
    }
    #[doc = "DAC Reference Select"]
    #[inline(always)]
    pub const fn set_dacrfs(&mut self, val: super::vals::Dacrfs) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoen(&self) -> super::vals::Fifoen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Fifoen::from_bits(val as u8)
    }
    #[doc = "FIFO Enable"]
    #[inline(always)]
    pub const fn set_fifoen(&mut self, val: super::vals::Fifoen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Swing Back Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn swmd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Swing Back Mode"]
    #[inline(always)]
    pub const fn set_swmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> super::vals::Trgsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Trgsel::from_bits(val as u8)
    }
    #[doc = "DAC Trigger Select"]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: super::vals::Trgsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DAC Periodic Trigger Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ptgen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Periodic Trigger Mode Enable"]
    #[inline(always)]
    pub const fn set_ptgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RCLK Cycles Before Data Latch"]
    #[must_use]
    #[inline(always)]
    pub const fn latch_cyc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "RCLK Cycles Before Data Latch"]
    #[inline(always)]
    pub const fn set_latch_cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn buf_en(&self) -> super::vals::BufEn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::BufEn::from_bits(val as u8)
    }
    #[doc = "Buffer Enable"]
    #[inline(always)]
    pub const fn set_buf_en(&mut self, val: super::vals::BufEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "External On-Chip PTAT Current Reference Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iref_ptat_ext_sel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "External On-Chip PTAT Current Reference Select"]
    #[inline(always)]
    pub const fn set_iref_ptat_ext_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "External On-Chip ZTC Current Reference Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iref_ztc_ext_sel(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "External On-Chip ZTC Current Reference Select"]
    #[inline(always)]
    pub const fn set_iref_ztc_ext_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OPAMP as Buffer, Speed Control Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn buf_spd_ctrl(&self) -> super::vals::BufSpdCtrl {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::BufSpdCtrl::from_bits(val as u8)
    }
    #[doc = "OPAMP as Buffer, Speed Control Signal"]
    #[inline(always)]
    pub const fn set_buf_spd_ctrl(&mut self, val: super::vals::BufSpdCtrl) {
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
            "Gcr {{ dacen: {=bool:?}, dacrfs: {:?}, fifoen: {:?}, swmd: {=bool:?}, trgsel: {:?}, ptgen: {=bool:?}, latch_cyc: {=u8:?}, buf_en: {:?}, iref_ptat_ext_sel: {=bool:?}, iref_ztc_ext_sel: {=bool:?}, buf_spd_ctrl: {:?} }}",
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
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "FIFO Full Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn full_ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub const fn set_full_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Empty Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn empty_ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn set_empty_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Watermark Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wm_ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Watermark Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wm_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Swing Back One Cycle Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn swbk_ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Swing Back One Cycle Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_swbk_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn of_ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_of_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uf_ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_uf_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PTG Mode Conversion Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ptgcoco_ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PTG Mode Conversion Complete Interrupt Enable"]
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
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn fifosz(&self) -> super::vals::Fifosz {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Fifosz::from_bits(val as u8)
    }
    #[doc = "FIFO Size"]
    #[inline(always)]
    pub const fn set_fifosz(&mut self, val: super::vals::Fifosz) {
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
#[doc = "Periodic Trigger Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "Periodic Trigger Number"]
    #[must_use]
    #[inline(always)]
    pub const fn ptg_num(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Periodic Trigger Number"]
    #[inline(always)]
    pub const fn set_ptg_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Periodic Trigger Period Width"]
    #[must_use]
    #[inline(always)]
    pub const fn ptg_period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Periodic Trigger Period Width"]
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
#[doc = "Reset Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swrst(&self) -> super::vals::Swrst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swrst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: super::vals::Swrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn fiforst(&self) -> super::vals::Fiforst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fiforst::from_bits(val as u8)
    }
    #[doc = "FIFO Reset"]
    #[inline(always)]
    pub const fn set_fiforst(&mut self, val: super::vals::Fiforst) {
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
#[doc = "Trigger Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Software Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn swtrg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Trigger"]
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
#[doc = "Version Identifier"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Identification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Identification Number"]
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
