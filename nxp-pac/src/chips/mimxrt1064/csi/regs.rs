#[doc = "CSI Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Pixel Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pixel_bit(&self) -> super::vals::PixelBit {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PixelBit::from_bits(val as u8)
    }
    #[doc = "Pixel Bit"]
    #[inline(always)]
    pub const fn set_pixel_bit(&mut self, val: super::vals::PixelBit) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Valid Pixel Clock Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn redge(&self) -> super::vals::Redge {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Redge::from_bits(val as u8)
    }
    #[doc = "Valid Pixel Clock Edge Select"]
    #[inline(always)]
    pub const fn set_redge(&mut self, val: super::vals::Redge) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert Pixel Clock Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv_pclk(&self) -> super::vals::InvPclk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::InvPclk::from_bits(val as u8)
    }
    #[doc = "Invert Pixel Clock Input"]
    #[inline(always)]
    pub const fn set_inv_pclk(&mut self, val: super::vals::InvPclk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert Data Input. This bit enables or disables internal inverters on the data lines."]
    #[must_use]
    #[inline(always)]
    pub const fn inv_data(&self) -> super::vals::InvData {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::InvData::from_bits(val as u8)
    }
    #[doc = "Invert Data Input. This bit enables or disables internal inverters on the data lines."]
    #[inline(always)]
    pub const fn set_inv_data(&mut self, val: super::vals::InvData) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Gated Clock Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gclk_mode(&self) -> super::vals::GclkMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::GclkMode::from_bits(val as u8)
    }
    #[doc = "Gated Clock Mode Enable"]
    #[inline(always)]
    pub const fn set_gclk_mode(&mut self, val: super::vals::GclkMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous RXFIFO Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_rxfifo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous RXFIFO Clear"]
    #[inline(always)]
    pub const fn set_clr_rxfifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Asynchronous STATFIFO Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_statfifo(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous STATFIFO Clear"]
    #[inline(always)]
    pub const fn set_clr_statfifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Packing Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn pack_dir(&self) -> super::vals::PackDir {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PackDir::from_bits(val as u8)
    }
    #[doc = "Data Packing Direction"]
    #[inline(always)]
    pub const fn set_pack_dir(&mut self, val: super::vals::PackDir) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FIFO Clear Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fcc(&self) -> super::vals::Fcc {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fcc::from_bits(val as u8)
    }
    #[doc = "FIFO Clear Control"]
    #[inline(always)]
    pub const fn set_fcc(&mut self, val: super::vals::Fcc) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "BT.656 Interface Enable. This bit selects the type of interface used."]
    #[must_use]
    #[inline(always)]
    pub const fn ccir_en(&self) -> super::vals::CcirEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CcirEn::from_bits(val as u8)
    }
    #[doc = "BT.656 Interface Enable. This bit selects the type of interface used."]
    #[inline(always)]
    pub const fn set_ccir_en(&mut self, val: super::vals::CcirEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "HSYNC Polarity Select"]
    #[must_use]
    #[inline(always)]
    pub const fn hsync_pol(&self) -> super::vals::HsyncPol {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::HsyncPol::from_bits(val as u8)
    }
    #[doc = "HSYNC Polarity Select"]
    #[inline(always)]
    pub const fn set_hsync_pol(&mut self, val: super::vals::HsyncPol) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sof_inten(&self) -> super::vals::SofInten {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SofInten::from_bits(val as u8)
    }
    #[doc = "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt."]
    #[inline(always)]
    pub const fn set_sof_inten(&mut self, val: super::vals::SofInten) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sof_pol(&self) -> super::vals::SofPol {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SofPol::from_bits(val as u8)
    }
    #[doc = "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt."]
    #[inline(always)]
    pub const fn set_sof_pol(&mut self, val: super::vals::SofPol) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxff_inten(&self) -> super::vals::RxffInten {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::RxffInten::from_bits(val as u8)
    }
    #[doc = "RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt."]
    #[inline(always)]
    pub const fn set_rxff_inten(&mut self, val: super::vals::RxffInten) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Frame Buffer1 DMA Transfer Done Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fb1_dma_done_inten(&self) -> super::vals::Fb1DmaDoneInten {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Fb1DmaDoneInten::from_bits(val as u8)
    }
    #[doc = "Frame Buffer1 DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fb1_dma_done_inten(&mut self, val: super::vals::Fb1DmaDoneInten) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Frame Buffer2 DMA Transfer Done Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fb2_dma_done_inten(&self) -> super::vals::Fb2DmaDoneInten {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Fb2DmaDoneInten::from_bits(val as u8)
    }
    #[doc = "Frame Buffer2 DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fb2_dma_done_inten(&mut self, val: super::vals::Fb2DmaDoneInten) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn statff_inten(&self) -> super::vals::StatffInten {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::StatffInten::from_bits(val as u8)
    }
    #[doc = "STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt."]
    #[inline(always)]
    pub const fn set_statff_inten(&mut self, val: super::vals::StatffInten) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "STATFIFO DMA Transfer Done Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sff_dma_done_inten(&self) -> super::vals::SffDmaDoneInten {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::SffDmaDoneInten::from_bits(val as u8)
    }
    #[doc = "STATFIFO DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sff_dma_done_inten(&mut self, val: super::vals::SffDmaDoneInten) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rf_or_inten(&self) -> super::vals::RfOrInten {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::RfOrInten::from_bits(val as u8)
    }
    #[doc = "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt."]
    #[inline(always)]
    pub const fn set_rf_or_inten(&mut self, val: super::vals::RfOrInten) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sf_or_inten(&self) -> super::vals::SfOrInten {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SfOrInten::from_bits(val as u8)
    }
    #[doc = "STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt."]
    #[inline(always)]
    pub const fn set_sf_or_inten(&mut self, val: super::vals::SfOrInten) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Change Of Image Field (COF) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cof_int_en(&self) -> super::vals::CofIntEn {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::CofIntEn::from_bits(val as u8)
    }
    #[doc = "Change Of Image Field (COF) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cof_int_en(&mut self, val: super::vals::CofIntEn) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "BT"]
    #[must_use]
    #[inline(always)]
    pub const fn ccir_mode(&self) -> super::vals::CcirMode {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::CcirMode::from_bits(val as u8)
    }
    #[doc = "BT"]
    #[inline(always)]
    pub const fn set_ccir_mode(&mut self, val: super::vals::CcirMode) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "CSI-PrP Interface Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pr_p_if_en(&self) -> super::vals::PrPIfEn {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::PrPIfEn::from_bits(val as u8)
    }
    #[doc = "CSI-PrP Interface Enable"]
    #[inline(always)]
    pub const fn set_pr_p_if_en(&mut self, val: super::vals::PrPIfEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn eof_int_en(&self) -> super::vals::EofIntEn {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::EofIntEn::from_bits(val as u8)
    }
    #[doc = "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt."]
    #[inline(always)]
    pub const fn set_eof_int_en(&mut self, val: super::vals::EofIntEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "External VSYNC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vsync(&self) -> super::vals::ExtVsync {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ExtVsync::from_bits(val as u8)
    }
    #[doc = "External VSYNC Enable"]
    #[inline(always)]
    pub const fn set_ext_vsync(&mut self, val: super::vals::ExtVsync) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "SWAP 16-Bit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn swap16_en(&self) -> super::vals::Swap16En {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Swap16En::from_bits(val as u8)
    }
    #[doc = "SWAP 16-Bit Enable"]
    #[inline(always)]
    pub const fn set_swap16_en(&mut self, val: super::vals::Swap16En) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("pixel_bit", &self.pixel_bit())
            .field("redge", &self.redge())
            .field("inv_pclk", &self.inv_pclk())
            .field("inv_data", &self.inv_data())
            .field("gclk_mode", &self.gclk_mode())
            .field("clr_rxfifo", &self.clr_rxfifo())
            .field("clr_statfifo", &self.clr_statfifo())
            .field("pack_dir", &self.pack_dir())
            .field("fcc", &self.fcc())
            .field("ccir_en", &self.ccir_en())
            .field("hsync_pol", &self.hsync_pol())
            .field("sof_inten", &self.sof_inten())
            .field("sof_pol", &self.sof_pol())
            .field("rxff_inten", &self.rxff_inten())
            .field("fb1_dma_done_inten", &self.fb1_dma_done_inten())
            .field("fb2_dma_done_inten", &self.fb2_dma_done_inten())
            .field("statff_inten", &self.statff_inten())
            .field("sff_dma_done_inten", &self.sff_dma_done_inten())
            .field("rf_or_inten", &self.rf_or_inten())
            .field("sf_or_inten", &self.sf_or_inten())
            .field("cof_int_en", &self.cof_int_en())
            .field("ccir_mode", &self.ccir_mode())
            .field("pr_p_if_en", &self.pr_p_if_en())
            .field("eof_int_en", &self.eof_int_en())
            .field("ext_vsync", &self.ext_vsync())
            .field("swap16_en", &self.swap16_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr1 {{ pixel_bit: {:?}, redge: {:?}, inv_pclk: {:?}, inv_data: {:?}, gclk_mode: {:?}, clr_rxfifo: {=bool:?}, clr_statfifo: {=bool:?}, pack_dir: {:?}, fcc: {:?}, ccir_en: {:?}, hsync_pol: {:?}, sof_inten: {:?}, sof_pol: {:?}, rxff_inten: {:?}, fb1_dma_done_inten: {:?}, fb2_dma_done_inten: {:?}, statff_inten: {:?}, sff_dma_done_inten: {:?}, rf_or_inten: {:?}, sf_or_inten: {:?}, cof_int_en: {:?}, ccir_mode: {:?}, pr_p_if_en: {:?}, eof_int_en: {:?}, ext_vsync: {:?}, swap16_en: {:?} }}",
            self.pixel_bit(),
            self.redge(),
            self.inv_pclk(),
            self.inv_data(),
            self.gclk_mode(),
            self.clr_rxfifo(),
            self.clr_statfifo(),
            self.pack_dir(),
            self.fcc(),
            self.ccir_en(),
            self.hsync_pol(),
            self.sof_inten(),
            self.sof_pol(),
            self.rxff_inten(),
            self.fb1_dma_done_inten(),
            self.fb2_dma_done_inten(),
            self.statff_inten(),
            self.sff_dma_done_inten(),
            self.rf_or_inten(),
            self.sf_or_inten(),
            self.cof_int_en(),
            self.ccir_mode(),
            self.pr_p_if_en(),
            self.eof_int_en(),
            self.ext_vsync(),
            self.swap16_en()
        )
    }
}
#[doc = "CSI Control Register 18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr18(pub u32);
impl Cr18 {
    #[doc = "This bit is used to select the output method When input is standard BT.656 video."]
    #[must_use]
    #[inline(always)]
    pub const fn deinterlace_en(&self) -> super::vals::DeinterlaceEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DeinterlaceEn::from_bits(val as u8)
    }
    #[doc = "This bit is used to select the output method When input is standard BT.656 video."]
    #[inline(always)]
    pub const fn set_deinterlace_en(&mut self, val: super::vals::DeinterlaceEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable bit for Parallel RGB888/YUV444 24bit input"]
    #[must_use]
    #[inline(always)]
    pub const fn parallel24_en(&self) -> super::vals::Parallel24En {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Parallel24En::from_bits(val as u8)
    }
    #[doc = "Enable bit for Parallel RGB888/YUV444 24bit input"]
    #[inline(always)]
    pub const fn set_parallel24_en(&mut self, val: super::vals::Parallel24En) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than automatically by DMA completed"]
    #[must_use]
    #[inline(always)]
    pub const fn baseaddr_switch_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than automatically by DMA completed"]
    #[inline(always)]
    pub const fn set_baseaddr_switch_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1."]
    #[must_use]
    #[inline(always)]
    pub const fn baseaddr_switch_sel(&self) -> super::vals::BaseaddrSwitchSel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::BaseaddrSwitchSel::from_bits(val as u8)
    }
    #[doc = "CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1."]
    #[inline(always)]
    pub const fn set_baseaddr_switch_sel(&mut self, val: super::vals::BaseaddrSwitchSel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "In interlace mode, field 0 means interrupt enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn field0_done_ie(&self) -> super::vals::Field0DoneIe {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Field0DoneIe::from_bits(val as u8)
    }
    #[doc = "In interlace mode, field 0 means interrupt enabled."]
    #[inline(always)]
    pub const fn set_field0_done_ie(&mut self, val: super::vals::Field0DoneIe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "When in interlace mode, field 1 done interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_field1_done_ie(&self) -> super::vals::DmaField1DoneIe {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DmaField1DoneIe::from_bits(val as u8)
    }
    #[doc = "When in interlace mode, field 1 done interrupt enable."]
    #[inline(always)]
    pub const fn set_dma_field1_done_ie(&mut self, val: super::vals::DmaField1DoneIe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Choosing the last DMA request condition"]
    #[must_use]
    #[inline(always)]
    pub const fn last_dma_req_sel(&self) -> super::vals::LastDmaReqSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::LastDmaReqSel::from_bits(val as u8)
    }
    #[doc = "Choosing the last DMA request condition"]
    #[inline(always)]
    pub const fn set_last_dma_req_sel(&mut self, val: super::vals::LastDmaReqSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Base address change error interrupt enable signal."]
    #[must_use]
    #[inline(always)]
    pub const fn baseaddr_change_error_ie(&self) -> super::vals::BaseaddrChangeErrorIe {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::BaseaddrChangeErrorIe::from_bits(val as u8)
    }
    #[doc = "Base address change error interrupt enable signal."]
    #[inline(always)]
    pub const fn set_baseaddr_change_error_ie(&mut self, val: super::vals::BaseaddrChangeErrorIe) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Output is 32-bit format."]
    #[must_use]
    #[inline(always)]
    pub const fn rgb888a_format_sel(&self) -> super::vals::Rgb888aFormatSel {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rgb888aFormatSel::from_bits(val as u8)
    }
    #[doc = "Output is 32-bit format."]
    #[inline(always)]
    pub const fn set_rgb888a_format_sel(&mut self, val: super::vals::Rgb888aFormatSel) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Hprot value in AHB bus protocol."]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_hprot(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Hprot value in AHB bus protocol."]
    #[inline(always)]
    pub const fn set_ahb_hprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "These bits used to choose the method to mask the CSI input."]
    #[must_use]
    #[inline(always)]
    pub const fn mask_option(&self) -> super::vals::MaskOption {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::MaskOption::from_bits(val as u8)
    }
    #[doc = "These bits used to choose the method to mask the CSI input."]
    #[inline(always)]
    pub const fn set_mask_option(&mut self, val: super::vals::MaskOption) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "CSI global enable signal"]
    #[must_use]
    #[inline(always)]
    pub const fn csi_enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "CSI global enable signal"]
    #[inline(always)]
    pub const fn set_csi_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr18 {
    #[inline(always)]
    fn default() -> Cr18 {
        Cr18(0)
    }
}
impl core::fmt::Debug for Cr18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr18")
            .field("deinterlace_en", &self.deinterlace_en())
            .field("parallel24_en", &self.parallel24_en())
            .field("baseaddr_switch_en", &self.baseaddr_switch_en())
            .field("baseaddr_switch_sel", &self.baseaddr_switch_sel())
            .field("field0_done_ie", &self.field0_done_ie())
            .field("dma_field1_done_ie", &self.dma_field1_done_ie())
            .field("last_dma_req_sel", &self.last_dma_req_sel())
            .field("baseaddr_change_error_ie", &self.baseaddr_change_error_ie())
            .field("rgb888a_format_sel", &self.rgb888a_format_sel())
            .field("ahb_hprot", &self.ahb_hprot())
            .field("mask_option", &self.mask_option())
            .field("csi_enable", &self.csi_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr18 {{ deinterlace_en: {:?}, parallel24_en: {:?}, baseaddr_switch_en: {=bool:?}, baseaddr_switch_sel: {:?}, field0_done_ie: {:?}, dma_field1_done_ie: {:?}, last_dma_req_sel: {:?}, baseaddr_change_error_ie: {:?}, rgb888a_format_sel: {:?}, ahb_hprot: {=u8:?}, mask_option: {:?}, csi_enable: {=bool:?} }}",
            self.deinterlace_en(),
            self.parallel24_en(),
            self.baseaddr_switch_en(),
            self.baseaddr_switch_sel(),
            self.field0_done_ie(),
            self.dma_field1_done_ie(),
            self.last_dma_req_sel(),
            self.baseaddr_change_error_ie(),
            self.rgb888a_format_sel(),
            self.ahb_hprot(),
            self.mask_option(),
            self.csi_enable()
        )
    }
}
#[doc = "CSI Control Register 19"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr19(pub u32);
impl Cr19 {
    #[doc = "This byte stores the highest FIFO level achieved by CSI FIFO timely and will be clear by writing 8'ff to it"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_rfifo_highest_fifo_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This byte stores the highest FIFO level achieved by CSI FIFO timely and will be clear by writing 8'ff to it"]
    #[inline(always)]
    pub const fn set_dma_rfifo_highest_fifo_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cr19 {
    #[inline(always)]
    fn default() -> Cr19 {
        Cr19(0)
    }
}
impl core::fmt::Debug for Cr19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr19")
            .field(
                "dma_rfifo_highest_fifo_level",
                &self.dma_rfifo_highest_fifo_level(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr19 {{ dma_rfifo_highest_fifo_level: {=u8:?} }}",
            self.dma_rfifo_highest_fifo_level()
        )
    }
}
#[doc = "CSI Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Horizontal Skip Count"]
    #[must_use]
    #[inline(always)]
    pub const fn hsc(&self) -> super::vals::Hsc {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Hsc::from_bits(val as u8)
    }
    #[doc = "Horizontal Skip Count"]
    #[inline(always)]
    pub const fn set_hsc(&mut self, val: super::vals::Hsc) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn vsc(&self) -> super::vals::Vsc {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Vsc::from_bits(val as u8)
    }
    #[doc = "Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored."]
    #[inline(always)]
    pub const fn set_vsc(&mut self, val: super::vals::Vsc) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Live View Resolution Mode. Selects the grid size used for live view resolution."]
    #[must_use]
    #[inline(always)]
    pub const fn lvrm(&self) -> super::vals::Lvrm {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Lvrm::from_bits(val as u8)
    }
    #[doc = "Live View Resolution Mode. Selects the grid size used for live view resolution."]
    #[inline(always)]
    pub const fn set_lvrm(&mut self, val: super::vals::Lvrm) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Bayer Tile Start. Controls the Bayer pattern starting point."]
    #[must_use]
    #[inline(always)]
    pub const fn bts(&self) -> super::vals::Bts {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::Bts::from_bits(val as u8)
    }
    #[doc = "Bayer Tile Start. Controls the Bayer pattern starting point."]
    #[inline(always)]
    pub const fn set_bts(&mut self, val: super::vals::Bts) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "Skip Count Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sce(&self) -> super::vals::Sce {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Sce::from_bits(val as u8)
    }
    #[doc = "Skip Count Enable"]
    #[inline(always)]
    pub const fn set_sce(&mut self, val: super::vals::Sce) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Auto Focus Spread. Selects which green pixels are used for auto-focus."]
    #[must_use]
    #[inline(always)]
    pub const fn afs(&self) -> super::vals::Afs {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Afs::from_bits(val as u8)
    }
    #[doc = "Auto Focus Spread. Selects which green pixels are used for auto-focus."]
    #[inline(always)]
    pub const fn set_afs(&mut self, val: super::vals::Afs) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Double Resolution Mode. Controls size of statistics grid."]
    #[must_use]
    #[inline(always)]
    pub const fn drm(&self) -> super::vals::Drm {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Drm::from_bits(val as u8)
    }
    #[doc = "Double Resolution Mode. Controls size of statistics grid."]
    #[inline(always)]
    pub const fn set_drm(&mut self, val: super::vals::Drm) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_burst_type_sff(&self) -> super::vals::DmaBurstTypeSff {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::DmaBurstTypeSff::from_bits(val as u8)
    }
    #[doc = "Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO."]
    #[inline(always)]
    pub const fn set_dma_burst_type_sff(&mut self, val: super::vals::DmaBurstTypeSff) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_burst_type_rff(&self) -> super::vals::DmaBurstTypeRff {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::DmaBurstTypeRff::from_bits(val as u8)
    }
    #[doc = "Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO."]
    #[inline(always)]
    pub const fn set_dma_burst_type_rff(&mut self, val: super::vals::DmaBurstTypeRff) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
impl core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr2")
            .field("hsc", &self.hsc())
            .field("vsc", &self.vsc())
            .field("lvrm", &self.lvrm())
            .field("bts", &self.bts())
            .field("sce", &self.sce())
            .field("afs", &self.afs())
            .field("drm", &self.drm())
            .field("dma_burst_type_sff", &self.dma_burst_type_sff())
            .field("dma_burst_type_rff", &self.dma_burst_type_rff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr2 {{ hsc: {:?}, vsc: {:?}, lvrm: {:?}, bts: {:?}, sce: {:?}, afs: {:?}, drm: {:?}, dma_burst_type_sff: {:?}, dma_burst_type_rff: {:?} }}",
            self.hsc(),
            self.vsc(),
            self.lvrm(),
            self.bts(),
            self.sce(),
            self.afs(),
            self.drm(),
            self.dma_burst_type_sff(),
            self.dma_burst_type_rff()
        )
    }
}
#[doc = "CSI Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc = "Automatic Error Correction Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_auto_en(&self) -> super::vals::EccAutoEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EccAutoEn::from_bits(val as u8)
    }
    #[doc = "Automatic Error Correction Enable"]
    #[inline(always)]
    pub const fn set_ecc_auto_en(&mut self, val: super::vals::EccAutoEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Error Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_int_en(&self) -> super::vals::EccIntEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EccIntEn::from_bits(val as u8)
    }
    #[doc = "Error Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ecc_int_en(&mut self, val: super::vals::EccIntEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Dummy Zero Packing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn zero_pack_en(&self) -> super::vals::ZeroPackEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ZeroPackEn::from_bits(val as u8)
    }
    #[doc = "Dummy Zero Packing Enable"]
    #[inline(always)]
    pub const fn set_zero_pack_en(&mut self, val: super::vals::ZeroPackEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "16-bit Sensor Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sensor_16bits(&self) -> super::vals::Sensor16bits {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sensor16bits::from_bits(val as u8)
    }
    #[doc = "16-bit Sensor Mode"]
    #[inline(always)]
    pub const fn set_sensor_16bits(&mut self, val: super::vals::Sensor16bits) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "RxFIFO Full Level"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_ff_level(&self) -> super::vals::RxFfLevel {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RxFfLevel::from_bits(val as u8)
    }
    #[doc = "RxFIFO Full Level"]
    #[inline(always)]
    pub const fn set_rx_ff_level(&mut self, val: super::vals::RxFfLevel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Hresponse Error Enable. This bit enables the hresponse (AHB protocol standard) error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hresp_err_en(&self) -> super::vals::HrespErrEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::HrespErrEn::from_bits(val as u8)
    }
    #[doc = "Hresponse Error Enable. This bit enables the hresponse (AHB protocol standard) error interrupt."]
    #[inline(always)]
    pub const fn set_hresp_err_en(&mut self, val: super::vals::HrespErrEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "STATFIFO Full Level"]
    #[must_use]
    #[inline(always)]
    pub const fn statff_level(&self) -> super::vals::StatffLevel {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::StatffLevel::from_bits(val as u8)
    }
    #[doc = "STATFIFO Full Level"]
    #[inline(always)]
    pub const fn set_statff_level(&mut self, val: super::vals::StatffLevel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "DMA Request Enable for STATFIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_req_en_sff(&self) -> super::vals::DmaReqEnSff {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::DmaReqEnSff::from_bits(val as u8)
    }
    #[doc = "DMA Request Enable for STATFIFO"]
    #[inline(always)]
    pub const fn set_dma_req_en_sff(&mut self, val: super::vals::DmaReqEnSff) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA Request Enable for RxFIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_req_en_rff(&self) -> super::vals::DmaReqEnRff {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DmaReqEnRff::from_bits(val as u8)
    }
    #[doc = "DMA Request Enable for RxFIFO"]
    #[inline(always)]
    pub const fn set_dma_req_en_rff(&mut self, val: super::vals::DmaReqEnRff) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Reflash DMA Controller for STATFIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_reflash_sff(&self) -> super::vals::DmaReflashSff {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::DmaReflashSff::from_bits(val as u8)
    }
    #[doc = "Reflash DMA Controller for STATFIFO"]
    #[inline(always)]
    pub const fn set_dma_reflash_sff(&mut self, val: super::vals::DmaReflashSff) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Reflash DMA Controller for RxFIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_reflash_rff(&self) -> super::vals::DmaReflashRff {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::DmaReflashRff::from_bits(val as u8)
    }
    #[doc = "Reflash DMA Controller for RxFIFO"]
    #[inline(always)]
    pub const fn set_dma_reflash_rff(&mut self, val: super::vals::DmaReflashRff) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)"]
    #[must_use]
    #[inline(always)]
    pub const fn frmcnt_rst(&self) -> super::vals::FrmcntRst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::FrmcntRst::from_bits(val as u8)
    }
    #[doc = "Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)"]
    #[inline(always)]
    pub const fn set_frmcnt_rst(&mut self, val: super::vals::FrmcntRst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Frame Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn frmcnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame Counter"]
    #[inline(always)]
    pub const fn set_frmcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cr3 {
    #[inline(always)]
    fn default() -> Cr3 {
        Cr3(0)
    }
}
impl core::fmt::Debug for Cr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr3")
            .field("ecc_auto_en", &self.ecc_auto_en())
            .field("ecc_int_en", &self.ecc_int_en())
            .field("zero_pack_en", &self.zero_pack_en())
            .field("sensor_16bits", &self.sensor_16bits())
            .field("rx_ff_level", &self.rx_ff_level())
            .field("hresp_err_en", &self.hresp_err_en())
            .field("statff_level", &self.statff_level())
            .field("dma_req_en_sff", &self.dma_req_en_sff())
            .field("dma_req_en_rff", &self.dma_req_en_rff())
            .field("dma_reflash_sff", &self.dma_reflash_sff())
            .field("dma_reflash_rff", &self.dma_reflash_rff())
            .field("frmcnt_rst", &self.frmcnt_rst())
            .field("frmcnt", &self.frmcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr3 {{ ecc_auto_en: {:?}, ecc_int_en: {:?}, zero_pack_en: {:?}, sensor_16bits: {:?}, rx_ff_level: {:?}, hresp_err_en: {:?}, statff_level: {:?}, dma_req_en_sff: {:?}, dma_req_en_rff: {:?}, dma_reflash_sff: {:?}, dma_reflash_rff: {:?}, frmcnt_rst: {:?}, frmcnt: {=u16:?} }}",
            self.ecc_auto_en(),
            self.ecc_int_en(),
            self.zero_pack_en(),
            self.sensor_16bits(),
            self.rx_ff_level(),
            self.hresp_err_en(),
            self.statff_level(),
            self.dma_req_en_sff(),
            self.dma_req_en_rff(),
            self.dma_reflash_sff(),
            self.dma_reflash_rff(),
            self.frmcnt_rst(),
            self.frmcnt()
        )
    }
}
#[doc = "CSI DMA Start Address Register - for Frame Buffer1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmasaFb1(pub u32);
impl DmasaFb1 {
    #[doc = "DMA Start Address in Frame Buffer1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_start_addr_fb1(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "DMA Start Address in Frame Buffer1"]
    #[inline(always)]
    pub const fn set_dma_start_addr_fb1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmasaFb1 {
    #[inline(always)]
    fn default() -> DmasaFb1 {
        DmasaFb1(0)
    }
}
impl core::fmt::Debug for DmasaFb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmasaFb1")
            .field("dma_start_addr_fb1", &self.dma_start_addr_fb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmasaFb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmasaFb1 {{ dma_start_addr_fb1: {=u32:?} }}",
            self.dma_start_addr_fb1()
        )
    }
}
#[doc = "CSI DMA Transfer Size Register - for Frame Buffer2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmasaFb2(pub u32);
impl DmasaFb2 {
    #[doc = "DMA Start Address in Frame Buffer2"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_start_addr_fb2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "DMA Start Address in Frame Buffer2"]
    #[inline(always)]
    pub const fn set_dma_start_addr_fb2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmasaFb2 {
    #[inline(always)]
    fn default() -> DmasaFb2 {
        DmasaFb2(0)
    }
}
impl core::fmt::Debug for DmasaFb2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmasaFb2")
            .field("dma_start_addr_fb2", &self.dma_start_addr_fb2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmasaFb2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmasaFb2 {{ dma_start_addr_fb2: {=u32:?} }}",
            self.dma_start_addr_fb2()
        )
    }
}
#[doc = "CSI DMA Start Address Register - for STATFIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmasaStatfifo(pub u32);
impl DmasaStatfifo {
    #[doc = "DMA Start Address for STATFIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_start_addr_sff(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "DMA Start Address for STATFIFO"]
    #[inline(always)]
    pub const fn set_dma_start_addr_sff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmasaStatfifo {
    #[inline(always)]
    fn default() -> DmasaStatfifo {
        DmasaStatfifo(0)
    }
}
impl core::fmt::Debug for DmasaStatfifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmasaStatfifo")
            .field("dma_start_addr_sff", &self.dma_start_addr_sff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmasaStatfifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmasaStatfifo {{ dma_start_addr_sff: {=u32:?} }}",
            self.dma_start_addr_sff()
        )
    }
}
#[doc = "CSI DMA Transfer Size Register - for STATFIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmatsStatfifo(pub u32);
impl DmatsStatfifo {
    #[doc = "DMA Transfer Size for STATFIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_tsf_size_sff(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DMA Transfer Size for STATFIFO"]
    #[inline(always)]
    pub const fn set_dma_tsf_size_sff(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmatsStatfifo {
    #[inline(always)]
    fn default() -> DmatsStatfifo {
        DmatsStatfifo(0)
    }
}
impl core::fmt::Debug for DmatsStatfifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmatsStatfifo")
            .field("dma_tsf_size_sff", &self.dma_tsf_size_sff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmatsStatfifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmatsStatfifo {{ dma_tsf_size_sff: {=u32:?} }}",
            self.dma_tsf_size_sff()
        )
    }
}
#[doc = "CSI Frame Buffer Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FbufPara(pub u32);
impl FbufPara {
    #[doc = "Frame Buffer Parameter"]
    #[must_use]
    #[inline(always)]
    pub const fn fbuf_stride(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame Buffer Parameter"]
    #[inline(always)]
    pub const fn set_fbuf_stride(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "DEINTERLACE_STRIDE is only used in the deinterlace mode"]
    #[must_use]
    #[inline(always)]
    pub const fn deinterlace_stride(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "DEINTERLACE_STRIDE is only used in the deinterlace mode"]
    #[inline(always)]
    pub const fn set_deinterlace_stride(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FbufPara {
    #[inline(always)]
    fn default() -> FbufPara {
        FbufPara(0)
    }
}
impl core::fmt::Debug for FbufPara {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FbufPara")
            .field("fbuf_stride", &self.fbuf_stride())
            .field("deinterlace_stride", &self.deinterlace_stride())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FbufPara {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FbufPara {{ fbuf_stride: {=u16:?}, deinterlace_stride: {=u16:?} }}",
            self.fbuf_stride(),
            self.deinterlace_stride()
        )
    }
}
#[doc = "CSI Image Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImagPara(pub u32);
impl ImagPara {
    #[doc = "Image Height. Indicates how many pixels in a column of the image from the sensor."]
    #[must_use]
    #[inline(always)]
    pub const fn image_height(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Image Height. Indicates how many pixels in a column of the image from the sensor."]
    #[inline(always)]
    pub const fn set_image_height(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "This field indicates the number of active pixel cycles per line"]
    #[must_use]
    #[inline(always)]
    pub const fn image_width(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the number of active pixel cycles per line"]
    #[inline(always)]
    pub const fn set_image_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ImagPara {
    #[inline(always)]
    fn default() -> ImagPara {
        ImagPara(0)
    }
}
impl core::fmt::Debug for ImagPara {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ImagPara")
            .field("image_height", &self.image_height())
            .field("image_width", &self.image_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ImagPara {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ImagPara {{ image_height: {=u16:?}, image_width: {=u16:?} }}",
            self.image_height(),
            self.image_width()
        )
    }
}
#[doc = "CSI RX FIFO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfifo(pub u32);
impl Rfifo {
    #[doc = "Received image data"]
    #[must_use]
    #[inline(always)]
    pub const fn image(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Received image data"]
    #[inline(always)]
    pub const fn set_image(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rfifo {
    #[inline(always)]
    fn default() -> Rfifo {
        Rfifo(0)
    }
}
impl core::fmt::Debug for Rfifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfifo")
            .field("image", &self.image())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rfifo {{ image: {=u32:?} }}", self.image())
    }
}
#[doc = "CSI RX Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxcnt(pub u32);
impl Rxcnt {
    #[doc = "RxFIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "RxFIFO Count"]
    #[inline(always)]
    pub const fn set_rxcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Rxcnt {
    #[inline(always)]
    fn default() -> Rxcnt {
        Rxcnt(0)
    }
}
impl core::fmt::Debug for Rxcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxcnt")
            .field("rxcnt", &self.rxcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxcnt {{ rxcnt: {=u32:?} }}", self.rxcnt())
    }
}
#[doc = "CSI Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "RXFIFO Data Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn drdy(&self) -> super::vals::Drdy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Drdy::from_bits(val as u8)
    }
    #[doc = "RXFIFO Data Ready"]
    #[inline(always)]
    pub const fn set_drdy(&mut self, val: super::vals::Drdy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "BT"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_int(&self) -> super::vals::EccInt {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EccInt::from_bits(val as u8)
    }
    #[doc = "BT"]
    #[inline(always)]
    pub const fn set_ecc_int(&mut self, val: super::vals::EccInt) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Hresponse Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn hresp_err_int(&self) -> super::vals::HrespErrInt {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::HrespErrInt::from_bits(val as u8)
    }
    #[doc = "Hresponse Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_hresp_err_int(&mut self, val: super::vals::HrespErrInt) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Change Of Field Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn cof_int(&self) -> super::vals::CofInt {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::CofInt::from_bits(val as u8)
    }
    #[doc = "Change Of Field Interrupt Status"]
    #[inline(always)]
    pub const fn set_cof_int(&mut self, val: super::vals::CofInt) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "BT"]
    #[must_use]
    #[inline(always)]
    pub const fn f1_int(&self) -> super::vals::F1Int {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::F1Int::from_bits(val as u8)
    }
    #[doc = "BT"]
    #[inline(always)]
    pub const fn set_f1_int(&mut self, val: super::vals::F1Int) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "BT"]
    #[must_use]
    #[inline(always)]
    pub const fn f2_int(&self) -> super::vals::F2Int {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::F2Int::from_bits(val as u8)
    }
    #[doc = "BT"]
    #[inline(always)]
    pub const fn set_f2_int(&mut self, val: super::vals::F2Int) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)"]
    #[must_use]
    #[inline(always)]
    pub const fn sof_int(&self) -> super::vals::SofInt {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SofInt::from_bits(val as u8)
    }
    #[doc = "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)"]
    #[inline(always)]
    pub const fn set_sof_int(&mut self, val: super::vals::SofInt) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)"]
    #[must_use]
    #[inline(always)]
    pub const fn eof_int(&self) -> super::vals::EofInt {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::EofInt::from_bits(val as u8)
    }
    #[doc = "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)"]
    #[inline(always)]
    pub const fn set_eof_int(&mut self, val: super::vals::EofInt) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "RXFIFO Full Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_ff_int(&self) -> super::vals::RxFfInt {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::RxFfInt::from_bits(val as u8)
    }
    #[doc = "RXFIFO Full Interrupt Status"]
    #[inline(always)]
    pub const fn set_rx_ff_int(&mut self, val: super::vals::RxFfInt) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA Transfer Done in Frame Buffer1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_tsf_done_fb1(&self) -> super::vals::DmaTsfDoneFb1 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::DmaTsfDoneFb1::from_bits(val as u8)
    }
    #[doc = "DMA Transfer Done in Frame Buffer1"]
    #[inline(always)]
    pub const fn set_dma_tsf_done_fb1(&mut self, val: super::vals::DmaTsfDoneFb1) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA Transfer Done in Frame Buffer2"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_tsf_done_fb2(&self) -> super::vals::DmaTsfDoneFb2 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::DmaTsfDoneFb2::from_bits(val as u8)
    }
    #[doc = "DMA Transfer Done in Frame Buffer2"]
    #[inline(always)]
    pub const fn set_dma_tsf_done_fb2(&mut self, val: super::vals::DmaTsfDoneFb2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "STATFIFO Full Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn statff_int(&self) -> super::vals::StatffInt {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::StatffInt::from_bits(val as u8)
    }
    #[doc = "STATFIFO Full Interrupt Status"]
    #[inline(always)]
    pub const fn set_statff_int(&mut self, val: super::vals::StatffInt) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA Transfer Done from StatFIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_tsf_done_sff(&self) -> super::vals::DmaTsfDoneSff {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::DmaTsfDoneSff::from_bits(val as u8)
    }
    #[doc = "DMA Transfer Done from StatFIFO"]
    #[inline(always)]
    pub const fn set_dma_tsf_done_sff(&mut self, val: super::vals::DmaTsfDoneSff) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "RxFIFO Overrun Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rf_or_int(&self) -> super::vals::RfOrInt {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::RfOrInt::from_bits(val as u8)
    }
    #[doc = "RxFIFO Overrun Interrupt Status"]
    #[inline(always)]
    pub const fn set_rf_or_int(&mut self, val: super::vals::RfOrInt) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "STATFIFO Overrun Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sf_or_int(&self) -> super::vals::SfOrInt {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SfOrInt::from_bits(val as u8)
    }
    #[doc = "STATFIFO Overrun Interrupt Status"]
    #[inline(always)]
    pub const fn set_sf_or_int(&mut self, val: super::vals::SfOrInt) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "When DMA field 1 is complete, this bit will be set to 1(clear by writing 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_field1_done(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "When DMA field 1 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline(always)]
    pub const fn set_dma_field1_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_field0_done(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline(always)]
    pub const fn set_dma_field0_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "When using base address switching enable, this bit will be 1 when switching occur before DMA complete"]
    #[must_use]
    #[inline(always)]
    pub const fn baseaddr_chhange_error(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When using base address switching enable, this bit will be 1 when switching occur before DMA complete"]
    #[inline(always)]
    pub const fn set_baseaddr_chhange_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("drdy", &self.drdy())
            .field("ecc_int", &self.ecc_int())
            .field("hresp_err_int", &self.hresp_err_int())
            .field("cof_int", &self.cof_int())
            .field("f1_int", &self.f1_int())
            .field("f2_int", &self.f2_int())
            .field("sof_int", &self.sof_int())
            .field("eof_int", &self.eof_int())
            .field("rx_ff_int", &self.rx_ff_int())
            .field("dma_tsf_done_fb1", &self.dma_tsf_done_fb1())
            .field("dma_tsf_done_fb2", &self.dma_tsf_done_fb2())
            .field("statff_int", &self.statff_int())
            .field("dma_tsf_done_sff", &self.dma_tsf_done_sff())
            .field("rf_or_int", &self.rf_or_int())
            .field("sf_or_int", &self.sf_or_int())
            .field("dma_field1_done", &self.dma_field1_done())
            .field("dma_field0_done", &self.dma_field0_done())
            .field("baseaddr_chhange_error", &self.baseaddr_chhange_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ drdy: {:?}, ecc_int: {:?}, hresp_err_int: {:?}, cof_int: {:?}, f1_int: {:?}, f2_int: {:?}, sof_int: {:?}, eof_int: {:?}, rx_ff_int: {:?}, dma_tsf_done_fb1: {:?}, dma_tsf_done_fb2: {:?}, statff_int: {:?}, dma_tsf_done_sff: {:?}, rf_or_int: {:?}, sf_or_int: {:?}, dma_field1_done: {=bool:?}, dma_field0_done: {=bool:?}, baseaddr_chhange_error: {=bool:?} }}",
            self.drdy(),
            self.ecc_int(),
            self.hresp_err_int(),
            self.cof_int(),
            self.f1_int(),
            self.f2_int(),
            self.sof_int(),
            self.eof_int(),
            self.rx_ff_int(),
            self.dma_tsf_done_fb1(),
            self.dma_tsf_done_fb2(),
            self.statff_int(),
            self.dma_tsf_done_sff(),
            self.rf_or_int(),
            self.sf_or_int(),
            self.dma_field1_done(),
            self.dma_field0_done(),
            self.baseaddr_chhange_error()
        )
    }
}
#[doc = "CSI Statistic FIFO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statfifo(pub u32);
impl Statfifo {
    #[doc = "Static data from sensor"]
    #[must_use]
    #[inline(always)]
    pub const fn stat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Static data from sensor"]
    #[inline(always)]
    pub const fn set_stat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Statfifo {
    #[inline(always)]
    fn default() -> Statfifo {
        Statfifo(0)
    }
}
impl core::fmt::Debug for Statfifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statfifo")
            .field("stat", &self.stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statfifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Statfifo {{ stat: {=u32:?} }}", self.stat())
    }
}
