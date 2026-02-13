#[doc = "Comparator Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0(pub u32);
impl Ccr0 {
    #[doc = "Comparator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Enable"]
    #[inline(always)]
    pub const fn set_cmp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator Deep Sleep Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_stop_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Deep Sleep Mode Enable"]
    #[inline(always)]
    pub const fn set_cmp_stop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Ccr0 {
    #[inline(always)]
    fn default() -> Ccr0 {
        Ccr0(0)
    }
}
impl core::fmt::Debug for Ccr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr0")
            .field("cmp_en", &self.cmp_en())
            .field("cmp_stop_en", &self.cmp_stop_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr0 {{ cmp_en: {=bool:?}, cmp_stop_en: {=bool:?} }}",
            self.cmp_en(),
            self.cmp_stop_en()
        )
    }
}
#[doc = "Comparator Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "Windowing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn window_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Windowing Enable"]
    #[inline(always)]
    pub const fn set_window_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sampling Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sampling Enable"]
    #[inline(always)]
    pub const fn set_sample_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator Invert"]
    #[must_use]
    #[inline(always)]
    pub const fn cout_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Invert"]
    #[inline(always)]
    pub const fn set_cout_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Comparator Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cout_sel(&self) -> super::vals::CoutSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CoutSel::from_bits(val as u8)
    }
    #[doc = "Comparator Output Select"]
    #[inline(always)]
    pub const fn set_cout_sel(&mut self, val: super::vals::CoutSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparator Output Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cout_pen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Output Pin Enable"]
    #[inline(always)]
    pub const fn set_cout_pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "COUTA_OW Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn couta_owen(&self) -> super::vals::CoutaOwen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CoutaOwen::from_bits(val as u8)
    }
    #[doc = "COUTA_OW Enable"]
    #[inline(always)]
    pub const fn set_couta_owen(&mut self, val: super::vals::CoutaOwen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "COUTA Output Level for Closed Window"]
    #[must_use]
    #[inline(always)]
    pub const fn couta_ow(&self) -> super::vals::CoutaOw {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CoutaOw::from_bits(val as u8)
    }
    #[doc = "COUTA Output Level for Closed Window"]
    #[inline(always)]
    pub const fn set_couta_ow(&mut self, val: super::vals::CoutaOw) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "WINDOW/SAMPLE Signal Invert"]
    #[must_use]
    #[inline(always)]
    pub const fn window_inv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "WINDOW/SAMPLE Signal Invert"]
    #[inline(always)]
    pub const fn set_window_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "COUT Event Window Close"]
    #[must_use]
    #[inline(always)]
    pub const fn window_cls(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "COUT Event Window Close"]
    #[inline(always)]
    pub const fn set_window_cls(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COUT Event Select"]
    #[must_use]
    #[inline(always)]
    pub const fn evt_sel(&self) -> super::vals::EvtSel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EvtSel::from_bits(val as u8)
    }
    #[doc = "COUT Event Select"]
    #[inline(always)]
    pub const fn set_evt_sel(&mut self, val: super::vals::EvtSel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Functional Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn func_clk_sel(&self) -> super::vals::FuncClkSel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::FuncClkSel::from_bits(val as u8)
    }
    #[doc = "Functional Clock Source Select"]
    #[inline(always)]
    pub const fn set_func_clk_sel(&mut self, val: super::vals::FuncClkSel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> super::vals::FiltCnt {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::FiltCnt::from_bits(val as u8)
    }
    #[doc = "Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: super::vals::FiltCnt) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
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
            .field("window_en", &self.window_en())
            .field("sample_en", &self.sample_en())
            .field("dma_en", &self.dma_en())
            .field("cout_inv", &self.cout_inv())
            .field("cout_sel", &self.cout_sel())
            .field("cout_pen", &self.cout_pen())
            .field("couta_owen", &self.couta_owen())
            .field("couta_ow", &self.couta_ow())
            .field("window_inv", &self.window_inv())
            .field("window_cls", &self.window_cls())
            .field("evt_sel", &self.evt_sel())
            .field("func_clk_sel", &self.func_clk_sel())
            .field("filt_cnt", &self.filt_cnt())
            .field("filt_per", &self.filt_per())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr1 {{ window_en: {=bool:?}, sample_en: {=bool:?}, dma_en: {=bool:?}, cout_inv: {=bool:?}, cout_sel: {:?}, cout_pen: {=bool:?}, couta_owen: {:?}, couta_ow: {:?}, window_inv: {=bool:?}, window_cls: {=bool:?}, evt_sel: {:?}, func_clk_sel: {:?}, filt_cnt: {:?}, filt_per: {=u8:?} }}",
            self.window_en(),
            self.sample_en(),
            self.dma_en(),
            self.cout_inv(),
            self.cout_sel(),
            self.cout_pen(),
            self.couta_owen(),
            self.couta_ow(),
            self.window_inv(),
            self.window_cls(),
            self.evt_sel(),
            self.func_clk_sel(),
            self.filt_cnt(),
            self.filt_per()
        )
    }
}
#[doc = "Comparator Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc = "CMP High Power Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_hpmd(&self) -> super::vals::CmpHpmd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CmpHpmd::from_bits(val as u8)
    }
    #[doc = "CMP High Power Mode Select"]
    #[inline(always)]
    pub const fn set_cmp_hpmd(&mut self, val: super::vals::CmpHpmd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CMP Nano Power Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_npmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Nano Power Mode Select"]
    #[inline(always)]
    pub const fn set_cmp_npmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator Hysteresis Control"]
    #[must_use]
    #[inline(always)]
    pub const fn hystctr(&self) -> super::vals::Hystctr {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Hystctr::from_bits(val as u8)
    }
    #[doc = "Comparator Hysteresis Control"]
    #[inline(always)]
    pub const fn set_hystctr(&mut self, val: super::vals::Hystctr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Plus Input MUX Select"]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::Psel {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Psel::from_bits(val as u8)
    }
    #[doc = "Plus Input MUX Select"]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: super::vals::Psel) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Minus Input MUX Select"]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> super::vals::Msel {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Msel::from_bits(val as u8)
    }
    #[doc = "Minus Input MUX Select"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: super::vals::Msel) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        Ccr2(0)
    }
}
impl core::fmt::Debug for Ccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr2")
            .field("cmp_hpmd", &self.cmp_hpmd())
            .field("cmp_npmd", &self.cmp_npmd())
            .field("hystctr", &self.hystctr())
            .field("psel", &self.psel())
            .field("msel", &self.msel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr2 {{ cmp_hpmd: {:?}, cmp_npmd: {=bool:?}, hystctr: {:?}, psel: {:?}, msel: {:?} }}",
            self.cmp_hpmd(),
            self.cmp_npmd(),
            self.hystctr(),
            self.psel(),
            self.msel()
        )
    }
}
#[doc = "Comparator Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Analog Comparator Flag Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Flag Rising"]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog Comparator Flag Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn cff(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Flag Falling"]
    #[inline(always)]
    pub const fn set_cff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Round-Robin Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Flag"]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Analog Comparator Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cout(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Output"]
    #[inline(always)]
    pub const fn set_cout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("cfr", &self.cfr())
            .field("cff", &self.cff())
            .field("rrf", &self.rrf())
            .field("cout", &self.cout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ cfr: {=bool:?}, cff: {=bool:?}, rrf: {=bool:?}, cout: {=bool:?} }}",
            self.cfr(),
            self.cff(),
            self.rrf(),
            self.cout()
        )
    }
}
#[doc = "DAC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc = "DAC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dac_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Enable"]
    #[inline(always)]
    pub const fn set_dac_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DAC High Power Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dac_hpmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DAC High Power Mode"]
    #[inline(always)]
    pub const fn set_dac_hpmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DAC Reference High Voltage Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vrsel(&self) -> super::vals::Vrsel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Vrsel::from_bits(val as u8)
    }
    #[doc = "DAC Reference High Voltage Source Select"]
    #[inline(always)]
    pub const fn set_vrsel(&mut self, val: super::vals::Vrsel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DAC Output Voltage Select"]
    #[must_use]
    #[inline(always)]
    pub const fn dac_data(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "DAC Output Voltage Select"]
    #[inline(always)]
    pub const fn set_dac_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        Dcr(0)
    }
}
impl core::fmt::Debug for Dcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcr")
            .field("dac_en", &self.dac_en())
            .field("dac_hpmd", &self.dac_hpmd())
            .field("vrsel", &self.vrsel())
            .field("dac_data", &self.dac_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcr {{ dac_en: {=bool:?}, dac_hpmd: {=bool:?}, vrsel: {:?}, dac_data: {=u8:?} }}",
            self.dac_en(),
            self.dac_hpmd(),
            self.vrsel(),
            self.dac_data()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Comparator Flag Rising Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cfr_ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Flag Rising Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cfr_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator Flag Falling Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cff_ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Flag Falling Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cff_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Round-Robin Flag Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rrf_ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Flag Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rrf_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("cfr_ie", &self.cfr_ie())
            .field("cff_ie", &self.cff_ie())
            .field("rrf_ie", &self.rrf_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ cfr_ie: {=bool:?}, cff_ie: {=bool:?}, rrf_ie: {=bool:?} }}",
            self.cfr_ie(),
            self.cff_ie(),
            self.rrf_ie()
        )
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "DAC Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn dac_res(&self) -> super::vals::DacRes {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DacRes::from_bits(val as u8)
    }
    #[doc = "DAC Resolution"]
    #[inline(always)]
    pub const fn set_dac_res(&mut self, val: super::vals::DacRes) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
            .field("dac_res", &self.dac_res())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ dac_res: {:?} }}", self.dac_res())
    }
}
#[doc = "Round Robin Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcr0(pub u32);
impl Rrcr0 {
    #[doc = "Round-Robin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Enable"]
    #[inline(always)]
    pub const fn set_rr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Round-Robin Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_trg_sel(&self) -> super::vals::RrTrgSel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RrTrgSel::from_bits(val as u8)
    }
    #[doc = "Round-Robin Trigger Select"]
    #[inline(always)]
    pub const fn set_rr_trg_sel(&mut self, val: super::vals::RrTrgSel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Number of Sample Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_nsam(&self) -> super::vals::RrNsam {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RrNsam::from_bits(val as u8)
    }
    #[doc = "Number of Sample Clocks"]
    #[inline(always)]
    pub const fn set_rr_nsam(&mut self, val: super::vals::RrNsam) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Round Robin Clock Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_clk_sel(&self) -> super::vals::RrClkSel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RrClkSel::from_bits(val as u8)
    }
    #[doc = "Round Robin Clock Source Select"]
    #[inline(always)]
    pub const fn set_rr_clk_sel(&mut self, val: super::vals::RrClkSel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Initialization Delay Modulus"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_initmod(&self) -> super::vals::RrInitmod {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::RrInitmod::from_bits(val as u8)
    }
    #[doc = "Initialization Delay Modulus"]
    #[inline(always)]
    pub const fn set_rr_initmod(&mut self, val: super::vals::RrInitmod) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "Number of Sample for One Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_sample_cnt(&self) -> super::vals::RrSampleCnt {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::RrSampleCnt::from_bits(val as u8)
    }
    #[doc = "Number of Sample for One Channel"]
    #[inline(always)]
    pub const fn set_rr_sample_cnt(&mut self, val: super::vals::RrSampleCnt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Sample Time Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_sample_threshold(&self) -> super::vals::RrSampleThreshold {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::RrSampleThreshold::from_bits(val as u8)
    }
    #[doc = "Sample Time Threshold"]
    #[inline(always)]
    pub const fn set_rr_sample_threshold(&mut self, val: super::vals::RrSampleThreshold) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Rrcr0 {
    #[inline(always)]
    fn default() -> Rrcr0 {
        Rrcr0(0)
    }
}
impl core::fmt::Debug for Rrcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rrcr0")
            .field("rr_en", &self.rr_en())
            .field("rr_trg_sel", &self.rr_trg_sel())
            .field("rr_nsam", &self.rr_nsam())
            .field("rr_clk_sel", &self.rr_clk_sel())
            .field("rr_initmod", &self.rr_initmod())
            .field("rr_sample_cnt", &self.rr_sample_cnt())
            .field("rr_sample_threshold", &self.rr_sample_threshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rrcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rrcr0 {{ rr_en: {=bool:?}, rr_trg_sel: {:?}, rr_nsam: {:?}, rr_clk_sel: {:?}, rr_initmod: {:?}, rr_sample_cnt: {:?}, rr_sample_threshold: {:?} }}",
            self.rr_en(),
            self.rr_trg_sel(),
            self.rr_nsam(),
            self.rr_clk_sel(),
            self.rr_initmod(),
            self.rr_sample_cnt(),
            self.rr_sample_threshold()
        )
    }
}
#[doc = "Round Robin Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcr1(pub u32);
impl Rrcr1 {
    #[doc = "Channel 0 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch2en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch3en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch4en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch5en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch5en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch6en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch6en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Input Enable in Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch7en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Input Enable in Trigger Mode"]
    #[inline(always)]
    pub const fn set_rr_ch7en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fixed Port"]
    #[must_use]
    #[inline(always)]
    pub const fn fixp(&self) -> super::vals::Fixp {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fixp::from_bits(val as u8)
    }
    #[doc = "Fixed Port"]
    #[inline(always)]
    pub const fn set_fixp(&mut self, val: super::vals::Fixp) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Fixed Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fixch(&self) -> super::vals::Fixch {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Fixch::from_bits(val as u8)
    }
    #[doc = "Fixed Channel Select"]
    #[inline(always)]
    pub const fn set_fixch(&mut self, val: super::vals::Fixch) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Rrcr1 {
    #[inline(always)]
    fn default() -> Rrcr1 {
        Rrcr1(0)
    }
}
impl core::fmt::Debug for Rrcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rrcr1")
            .field("rr_ch0en", &self.rr_ch0en())
            .field("rr_ch1en", &self.rr_ch1en())
            .field("rr_ch2en", &self.rr_ch2en())
            .field("rr_ch3en", &self.rr_ch3en())
            .field("rr_ch4en", &self.rr_ch4en())
            .field("rr_ch5en", &self.rr_ch5en())
            .field("rr_ch6en", &self.rr_ch6en())
            .field("rr_ch7en", &self.rr_ch7en())
            .field("fixp", &self.fixp())
            .field("fixch", &self.fixch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rrcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rrcr1 {{ rr_ch0en: {=bool:?}, rr_ch1en: {=bool:?}, rr_ch2en: {=bool:?}, rr_ch3en: {=bool:?}, rr_ch4en: {=bool:?}, rr_ch5en: {=bool:?}, rr_ch6en: {=bool:?}, rr_ch7en: {=bool:?}, fixp: {:?}, fixch: {:?} }}",
            self.rr_ch0en(),
            self.rr_ch1en(),
            self.rr_ch2en(),
            self.rr_ch3en(),
            self.rr_ch4en(),
            self.rr_ch5en(),
            self.rr_ch6en(),
            self.rr_ch7en(),
            self.fixp(),
            self.fixch()
        )
    }
}
#[doc = "Round Robin Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcr2(pub u32);
impl Rrcr2 {
    #[doc = "Number of Sample Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_timer_reload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Number of Sample Clocks"]
    #[inline(always)]
    pub const fn set_rr_timer_reload(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "Round-Robin Internal Timer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_timer_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Internal Timer Enable"]
    #[inline(always)]
    pub const fn set_rr_timer_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rrcr2 {
    #[inline(always)]
    fn default() -> Rrcr2 {
        Rrcr2(0)
    }
}
impl core::fmt::Debug for Rrcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rrcr2")
            .field("rr_timer_reload", &self.rr_timer_reload())
            .field("rr_timer_en", &self.rr_timer_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rrcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rrcr2 {{ rr_timer_reload: {=u32:?}, rr_timer_en: {=bool:?} }}",
            self.rr_timer_reload(),
            self.rr_timer_en()
        )
    }
}
#[doc = "Round Robin Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcsr(pub u32);
impl Rrcsr {
    #[doc = "Comparison Result for Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch0out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 0"]
    #[inline(always)]
    pub const fn set_rr_ch0out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparison Result for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch1out(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 1"]
    #[inline(always)]
    pub const fn set_rr_ch1out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparison Result for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch2out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 2"]
    #[inline(always)]
    pub const fn set_rr_ch2out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparison Result for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch3out(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 3"]
    #[inline(always)]
    pub const fn set_rr_ch3out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Comparison Result for Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch4out(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 4"]
    #[inline(always)]
    pub const fn set_rr_ch4out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparison Result for Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch5out(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 5"]
    #[inline(always)]
    pub const fn set_rr_ch5out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Comparison Result for Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch6out(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 6"]
    #[inline(always)]
    pub const fn set_rr_ch6out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Comparison Result for Channel 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch7out(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 7"]
    #[inline(always)]
    pub const fn set_rr_ch7out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Rrcsr {
    #[inline(always)]
    fn default() -> Rrcsr {
        Rrcsr(0)
    }
}
impl core::fmt::Debug for Rrcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rrcsr")
            .field("rr_ch0out", &self.rr_ch0out())
            .field("rr_ch1out", &self.rr_ch1out())
            .field("rr_ch2out", &self.rr_ch2out())
            .field("rr_ch3out", &self.rr_ch3out())
            .field("rr_ch4out", &self.rr_ch4out())
            .field("rr_ch5out", &self.rr_ch5out())
            .field("rr_ch6out", &self.rr_ch6out())
            .field("rr_ch7out", &self.rr_ch7out())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rrcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rrcsr {{ rr_ch0out: {=bool:?}, rr_ch1out: {=bool:?}, rr_ch2out: {=bool:?}, rr_ch3out: {=bool:?}, rr_ch4out: {=bool:?}, rr_ch5out: {=bool:?}, rr_ch6out: {=bool:?}, rr_ch7out: {=bool:?} }}",
            self.rr_ch0out(),
            self.rr_ch1out(),
            self.rr_ch2out(),
            self.rr_ch3out(),
            self.rr_ch4out(),
            self.rr_ch5out(),
            self.rr_ch6out(),
            self.rr_ch7out()
        )
    }
}
#[doc = "Round Robin Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrsr(pub u32);
impl Rrsr {
    #[doc = "Channel 0 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch0f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch1f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch2f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch3f(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch4f(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch4f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch5f(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch5f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch6f(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch6f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Input Changed Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch7f(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Input Changed Flag"]
    #[inline(always)]
    pub const fn set_rr_ch7f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Rrsr {
    #[inline(always)]
    fn default() -> Rrsr {
        Rrsr(0)
    }
}
impl core::fmt::Debug for Rrsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rrsr")
            .field("rr_ch0f", &self.rr_ch0f())
            .field("rr_ch1f", &self.rr_ch1f())
            .field("rr_ch2f", &self.rr_ch2f())
            .field("rr_ch3f", &self.rr_ch3f())
            .field("rr_ch4f", &self.rr_ch4f())
            .field("rr_ch5f", &self.rr_ch5f())
            .field("rr_ch6f", &self.rr_ch6f())
            .field("rr_ch7f", &self.rr_ch7f())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rrsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rrsr {{ rr_ch0f: {=bool:?}, rr_ch1f: {=bool:?}, rr_ch2f: {=bool:?}, rr_ch3f: {=bool:?}, rr_ch4f: {=bool:?}, rr_ch5f: {=bool:?}, rr_ch6f: {=bool:?}, rr_ch7f: {=bool:?} }}",
            self.rr_ch0f(),
            self.rr_ch1f(),
            self.rr_ch2f(),
            self.rr_ch3f(),
            self.rr_ch4f(),
            self.rr_ch5f(),
            self.rr_ch6f(),
            self.rr_ch7f()
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
