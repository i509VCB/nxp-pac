#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "LPCMP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp {
    ptr: *mut u8,
}
unsafe impl Send for Cmp {}
unsafe impl Sync for Cmp {}
impl Cmp {
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
    #[doc = "Comparator Control Register 0."]
    #[inline(always)]
    pub const fn ccr0(self) -> crate::pac::common::Reg<Ccr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Comparator Control Register 1."]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::pac::common::Reg<Ccr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Comparator Control Register 2."]
    #[inline(always)]
    pub const fn ccr2(self) -> crate::pac::common::Reg<Ccr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DAC Control."]
    #[inline(always)]
    pub const fn dcr(self) -> crate::pac::common::Reg<Dcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::pac::common::Reg<Ier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Comparator Status."]
    #[inline(always)]
    pub const fn csr(self) -> crate::pac::common::Reg<Csr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Round Robin Control Register 0."]
    #[inline(always)]
    pub const fn rrcr0(self) -> crate::pac::common::Reg<Rrcr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Round Robin Control Register 1."]
    #[inline(always)]
    pub const fn rrcr1(self) -> crate::pac::common::Reg<Rrcr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Round Robin Control and Status."]
    #[inline(always)]
    pub const fn rrcsr(self) -> crate::pac::common::Reg<Rrcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Round Robin Status."]
    #[inline(always)]
    pub const fn rrsr(self) -> crate::pac::common::Reg<Rrsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Round Robin Control Register 2."]
    #[inline(always)]
    pub const fn rrcr2(self) -> crate::pac::common::Reg<Rrcr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
#[doc = "Comparator Control Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0(pub u32);
impl Ccr0 {
    #[doc = "Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Enable."]
    #[inline(always)]
    pub const fn set_cmp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator Deep Sleep Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_stop_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Deep Sleep Mode Enable."]
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
#[doc = "Comparator Control Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "Windowing Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn window_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Windowing Enable."]
    #[inline(always)]
    pub const fn set_window_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sampling Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sample_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sampling Enable."]
    #[inline(always)]
    pub const fn set_sample_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn cout_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Invert."]
    #[inline(always)]
    pub const fn set_cout_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Comparator Output Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cout_sel(&self) -> CoutSel {
        let val = (self.0 >> 4usize) & 0x01;
        CoutSel::from_bits(val as u8)
    }
    #[doc = "Comparator Output Select."]
    #[inline(always)]
    pub const fn set_cout_sel(&mut self, val: CoutSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparator Output Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cout_pen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Output Pin Enable."]
    #[inline(always)]
    pub const fn set_cout_pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "COUTA_OW Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn couta_owen(&self) -> CoutaOwen {
        let val = (self.0 >> 6usize) & 0x01;
        CoutaOwen::from_bits(val as u8)
    }
    #[doc = "COUTA_OW Enable."]
    #[inline(always)]
    pub const fn set_couta_owen(&mut self, val: CoutaOwen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "COUTA Output Level for Closed Window."]
    #[must_use]
    #[inline(always)]
    pub const fn couta_ow(&self) -> CoutaOw {
        let val = (self.0 >> 7usize) & 0x01;
        CoutaOw::from_bits(val as u8)
    }
    #[doc = "COUTA Output Level for Closed Window."]
    #[inline(always)]
    pub const fn set_couta_ow(&mut self, val: CoutaOw) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "WINDOW/SAMPLE Signal Invert."]
    #[must_use]
    #[inline(always)]
    pub const fn window_inv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "WINDOW/SAMPLE Signal Invert."]
    #[inline(always)]
    pub const fn set_window_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "COUT Event Window Close."]
    #[must_use]
    #[inline(always)]
    pub const fn window_cls(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "COUT Event Window Close."]
    #[inline(always)]
    pub const fn set_window_cls(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COUT Event Select."]
    #[must_use]
    #[inline(always)]
    pub const fn evt_sel(&self) -> EvtSel {
        let val = (self.0 >> 10usize) & 0x03;
        EvtSel::from_bits(val as u8)
    }
    #[doc = "COUT Event Select."]
    #[inline(always)]
    pub const fn set_evt_sel(&mut self, val: EvtSel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Functional Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn func_clk_sel(&self) -> FuncClkSel {
        let val = (self.0 >> 12usize) & 0x03;
        FuncClkSel::from_bits(val as u8)
    }
    #[doc = "Functional Clock Source Select."]
    #[inline(always)]
    pub const fn set_func_clk_sel(&mut self, val: FuncClkSel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Filter Sample Count."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> FiltCnt {
        let val = (self.0 >> 16usize) & 0x07;
        FiltCnt::from_bits(val as u8)
    }
    #[doc = "Filter Sample Count."]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: FiltCnt) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Filter Sample Period."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Filter Sample Period."]
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
#[doc = "Comparator Control Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc = "CMP High Power Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_hpmd(&self) -> CmpHpmd {
        let val = (self.0 >> 0usize) & 0x01;
        CmpHpmd::from_bits(val as u8)
    }
    #[doc = "CMP High Power Mode Select."]
    #[inline(always)]
    pub const fn set_cmp_hpmd(&mut self, val: CmpHpmd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CMP Nano Power Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_npmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Nano Power Mode Select."]
    #[inline(always)]
    pub const fn set_cmp_npmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator Hysteresis Control."]
    #[must_use]
    #[inline(always)]
    pub const fn hystctr(&self) -> Hystctr {
        let val = (self.0 >> 4usize) & 0x03;
        Hystctr::from_bits(val as u8)
    }
    #[doc = "Comparator Hysteresis Control."]
    #[inline(always)]
    pub const fn set_hystctr(&mut self, val: Hystctr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Plus Input MUX Select."]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> Psel {
        let val = (self.0 >> 16usize) & 0x07;
        Psel::from_bits(val as u8)
    }
    #[doc = "Plus Input MUX Select."]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: Psel) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Minus Input MUX Select."]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> Msel {
        let val = (self.0 >> 20usize) & 0x07;
        Msel::from_bits(val as u8)
    }
    #[doc = "Minus Input MUX Select."]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: Msel) {
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
#[doc = "Comparator Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Analog Comparator Flag Rising."]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Flag Rising."]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog Comparator Flag Falling."]
    #[must_use]
    #[inline(always)]
    pub const fn cff(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Flag Falling."]
    #[inline(always)]
    pub const fn set_cff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Round-Robin Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Flag."]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Analog Comparator Output."]
    #[must_use]
    #[inline(always)]
    pub const fn cout(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Output."]
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
#[doc = "DAC Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc = "DAC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Enable."]
    #[inline(always)]
    pub const fn set_dac_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DAC High Power Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_hpmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DAC High Power Mode."]
    #[inline(always)]
    pub const fn set_dac_hpmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DAC Reference High Voltage Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn vrsel(&self) -> Vrsel {
        let val = (self.0 >> 8usize) & 0x01;
        Vrsel::from_bits(val as u8)
    }
    #[doc = "DAC Reference High Voltage Source Select."]
    #[inline(always)]
    pub const fn set_vrsel(&mut self, val: Vrsel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DAC Output Voltage Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_data(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "DAC Output Voltage Select."]
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
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Comparator Flag Rising Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cfr_ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Flag Rising Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cfr_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator Flag Falling Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cff_ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Flag Falling Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cff_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Round-Robin Flag Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rrf_ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Flag Interrupt Enable."]
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
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "DAC Resolution."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_res(&self) -> DacRes {
        let val = (self.0 >> 0usize) & 0x0f;
        DacRes::from_bits(val as u8)
    }
    #[doc = "DAC Resolution."]
    #[inline(always)]
    pub const fn set_dac_res(&mut self, val: DacRes) {
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
#[doc = "Round Robin Control Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcr0(pub u32);
impl Rrcr0 {
    #[doc = "Round-Robin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Enable."]
    #[inline(always)]
    pub const fn set_rr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Round-Robin Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_trg_sel(&self) -> RrTrgSel {
        let val = (self.0 >> 1usize) & 0x01;
        RrTrgSel::from_bits(val as u8)
    }
    #[doc = "Round-Robin Trigger Select."]
    #[inline(always)]
    pub const fn set_rr_trg_sel(&mut self, val: RrTrgSel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Number of Sample Clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_nsam(&self) -> RrNsam {
        let val = (self.0 >> 8usize) & 0x03;
        RrNsam::from_bits(val as u8)
    }
    #[doc = "Number of Sample Clocks."]
    #[inline(always)]
    pub const fn set_rr_nsam(&mut self, val: RrNsam) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Round Robin Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_clk_sel(&self) -> RrClkSel {
        let val = (self.0 >> 12usize) & 0x03;
        RrClkSel::from_bits(val as u8)
    }
    #[doc = "Round Robin Clock Source Select."]
    #[inline(always)]
    pub const fn set_rr_clk_sel(&mut self, val: RrClkSel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Initialization Delay Modulus."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_initmod(&self) -> RrInitmod {
        let val = (self.0 >> 16usize) & 0x3f;
        RrInitmod::from_bits(val as u8)
    }
    #[doc = "Initialization Delay Modulus."]
    #[inline(always)]
    pub const fn set_rr_initmod(&mut self, val: RrInitmod) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "Number of Sample for One Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_sample_cnt(&self) -> RrSampleCnt {
        let val = (self.0 >> 24usize) & 0x0f;
        RrSampleCnt::from_bits(val as u8)
    }
    #[doc = "Number of Sample for One Channel."]
    #[inline(always)]
    pub const fn set_rr_sample_cnt(&mut self, val: RrSampleCnt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Sample Time Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_sample_threshold(&self) -> RrSampleThreshold {
        let val = (self.0 >> 28usize) & 0x0f;
        RrSampleThreshold::from_bits(val as u8)
    }
    #[doc = "Sample Time Threshold."]
    #[inline(always)]
    pub const fn set_rr_sample_threshold(&mut self, val: RrSampleThreshold) {
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
#[doc = "Round Robin Control Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcr1(pub u32);
impl Rrcr1 {
    #[doc = "Channel 0 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch2en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch3en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch4en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch5en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch5en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch6en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch6en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Input Enable in Trigger Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch7en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Input Enable in Trigger Mode."]
    #[inline(always)]
    pub const fn set_rr_ch7en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Fixed Port."]
    #[must_use]
    #[inline(always)]
    pub const fn fixp(&self) -> Fixp {
        let val = (self.0 >> 16usize) & 0x01;
        Fixp::from_bits(val as u8)
    }
    #[doc = "Fixed Port."]
    #[inline(always)]
    pub const fn set_fixp(&mut self, val: Fixp) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Fixed Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn fixch(&self) -> Fixch {
        let val = (self.0 >> 20usize) & 0x07;
        Fixch::from_bits(val as u8)
    }
    #[doc = "Fixed Channel Select."]
    #[inline(always)]
    pub const fn set_fixch(&mut self, val: Fixch) {
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
#[doc = "Round Robin Control Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcr2(pub u32);
impl Rrcr2 {
    #[doc = "Number of Sample Clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_timer_reload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Number of Sample Clocks."]
    #[inline(always)]
    pub const fn set_rr_timer_reload(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "Round-Robin Internal Timer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_timer_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Round-Robin Internal Timer Enable."]
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
#[doc = "Round Robin Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrcsr(pub u32);
impl Rrcsr {
    #[doc = "Comparison Result for Channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch0out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 0."]
    #[inline(always)]
    pub const fn set_rr_ch0out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparison Result for Channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch1out(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 1."]
    #[inline(always)]
    pub const fn set_rr_ch1out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparison Result for Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch2out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 2."]
    #[inline(always)]
    pub const fn set_rr_ch2out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparison Result for Channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch3out(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 3."]
    #[inline(always)]
    pub const fn set_rr_ch3out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Comparison Result for Channel 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch4out(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 4."]
    #[inline(always)]
    pub const fn set_rr_ch4out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparison Result for Channel 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch5out(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 5."]
    #[inline(always)]
    pub const fn set_rr_ch5out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Comparison Result for Channel 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch6out(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 6."]
    #[inline(always)]
    pub const fn set_rr_ch6out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Comparison Result for Channel 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch7out(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Comparison Result for Channel 7."]
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
#[doc = "Round Robin Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rrsr(pub u32);
impl Rrsr {
    #[doc = "Channel 0 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch0f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Input Changed Flag."]
    #[inline(always)]
    pub const fn set_rr_ch0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch1f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Input Changed Flag."]
    #[inline(always)]
    pub const fn set_rr_ch1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch2f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Input Changed Flag."]
    #[inline(always)]
    pub const fn set_rr_ch2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch3f(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Input Changed Flag."]
    #[inline(always)]
    pub const fn set_rr_ch3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 4 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch4f(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 4 Input Changed Flag."]
    #[inline(always)]
    pub const fn set_rr_ch4f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel 5 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch5f(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 5 Input Changed Flag."]
    #[inline(always)]
    pub const fn set_rr_ch5f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel 6 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch6f(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 6 Input Changed Flag."]
    #[inline(always)]
    pub const fn set_rr_ch6f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel 7 Input Changed Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rr_ch7f(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 7 Input Changed Flag."]
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
pub enum CmpHpmd {
    #[doc = "Low power (speed) comparison mode."]
    Low = 0x0,
    #[doc = "High power (speed) comparison mode."]
    High = 0x01,
}
impl CmpHpmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpHpmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpHpmd {
    #[inline(always)]
    fn from(val: u8) -> CmpHpmd {
        CmpHpmd::from_bits(val)
    }
}
impl From<CmpHpmd> for u8 {
    #[inline(always)]
    fn from(val: CmpHpmd) -> u8 {
        CmpHpmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutSel {
    #[doc = "Use COUT (filtered)."]
    Cout = 0x0,
    #[doc = "Use COUTA (unfiltered)."]
    Couta = 0x01,
}
impl CoutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutSel {
    #[inline(always)]
    fn from(val: u8) -> CoutSel {
        CoutSel::from_bits(val)
    }
}
impl From<CoutSel> for u8 {
    #[inline(always)]
    fn from(val: CoutSel) -> u8 {
        CoutSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutaOw {
    #[doc = "COUTA is 0."]
    Couta0 = 0x0,
    #[doc = "COUTA is 1."]
    Couta1 = 0x01,
}
impl CoutaOw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutaOw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutaOw {
    #[inline(always)]
    fn from(val: u8) -> CoutaOw {
        CoutaOw::from_bits(val)
    }
}
impl From<CoutaOw> for u8 {
    #[inline(always)]
    fn from(val: CoutaOw) -> u8 {
        CoutaOw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoutaOwen {
    #[doc = "COUTA holds the last sampled value."]
    Sampled = 0x0,
    #[doc = "Enables the COUTA signal value to be defined by COUTA_OW."]
    CoutaOw = 0x01,
}
impl CoutaOwen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoutaOwen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoutaOwen {
    #[inline(always)]
    fn from(val: u8) -> CoutaOwen {
        CoutaOwen::from_bits(val)
    }
}
impl From<CoutaOwen> for u8 {
    #[inline(always)]
    fn from(val: CoutaOwen) -> u8 {
        CoutaOwen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacRes {
    #[doc = "4-bit DAC."]
    Reso4 = 0x0,
    #[doc = "6-bit DAC."]
    Reso6 = 0x01,
    #[doc = "8-bit DAC."]
    Reso8 = 0x02,
    #[doc = "10-bit DAC."]
    Reso10 = 0x03,
    #[doc = "12-bit DAC."]
    Reso12 = 0x04,
    #[doc = "14-bit DAC."]
    Reso14 = 0x05,
    #[doc = "16-bit DAC."]
    Reso16 = 0x06,
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
impl DacRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacRes {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacRes {
    #[inline(always)]
    fn from(val: u8) -> DacRes {
        DacRes::from_bits(val)
    }
}
impl From<DacRes> for u8 {
    #[inline(always)]
    fn from(val: DacRes) -> u8 {
        DacRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtSel {
    #[doc = "Rising edge."]
    Rising = 0x0,
    #[doc = "Falling edge."]
    Falling = 0x01,
    #[doc = "Both edges."]
    Both = 0x02,
    _RESERVED_3 = 0x03,
}
impl EvtSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtSel {
    #[inline(always)]
    fn from(val: u8) -> EvtSel {
        EvtSel::from_bits(val)
    }
}
impl From<EvtSel> for u8 {
    #[inline(always)]
    fn from(val: EvtSel) -> u8 {
        EvtSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Round robin feature."]
    pub const RoundRobin: Self = Self(0x01);
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
            0x01 => f.write_str("RoundRobin"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "RoundRobin"),
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
pub enum FiltCnt {
    #[doc = "Filter is bypassed: COUT = COUTA."]
    Bypassed = 0x0,
    #[doc = "1 consecutive sample (Comparator output is simply sampled.)."]
    Sample1 = 0x01,
    #[doc = "2 consecutive samples."]
    Sample2 = 0x02,
    #[doc = "3 consecutive samples."]
    Sample3 = 0x03,
    #[doc = "4 consecutive samples."]
    Sample4 = 0x04,
    #[doc = "5 consecutive samples."]
    Sample5 = 0x05,
    #[doc = "6 consecutive samples."]
    Sample6 = 0x06,
    #[doc = "7 consecutive samples."]
    Sample7 = 0x07,
}
impl FiltCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FiltCnt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FiltCnt {
    #[inline(always)]
    fn from(val: u8) -> FiltCnt {
        FiltCnt::from_bits(val)
    }
}
impl From<FiltCnt> for u8 {
    #[inline(always)]
    fn from(val: FiltCnt) -> u8 {
        FiltCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fixch {
    #[doc = "Channel 0."]
    FixCh0 = 0x0,
    #[doc = "Channel 1."]
    FixCh1 = 0x01,
    #[doc = "Channel 2."]
    FixCh2 = 0x02,
    #[doc = "Channel 3."]
    FixCh3 = 0x03,
    #[doc = "Channel 4."]
    FixCh4 = 0x04,
    #[doc = "Channel 5."]
    FixCh5 = 0x05,
    #[doc = "Channel 6."]
    FixCh6 = 0x06,
    #[doc = "Channel 7."]
    FixCh7 = 0x07,
}
impl Fixch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fixch {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fixch {
    #[inline(always)]
    fn from(val: u8) -> Fixch {
        Fixch::from_bits(val)
    }
}
impl From<Fixch> for u8 {
    #[inline(always)]
    fn from(val: Fixch) -> u8 {
        Fixch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fixp {
    #[doc = "Fix the plus port. Sweep only the inputs to the minus port."]
    FixPlus = 0x0,
    #[doc = "Fix the minus port. Sweep only the inputs to the plus port."]
    FixMinus = 0x01,
}
impl Fixp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fixp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fixp {
    #[inline(always)]
    fn from(val: u8) -> Fixp {
        Fixp::from_bits(val)
    }
}
impl From<Fixp> for u8 {
    #[inline(always)]
    fn from(val: Fixp) -> u8 {
        Fixp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FuncClkSel {
    #[doc = "Select functional clock source 0."]
    Func0 = 0x0,
    #[doc = "Select functional clock source 1."]
    Func1 = 0x01,
    #[doc = "Select functional clock source 2."]
    Func2 = 0x02,
    #[doc = "Select functional clock source 3."]
    Func3 = 0x03,
}
impl FuncClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FuncClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FuncClkSel {
    #[inline(always)]
    fn from(val: u8) -> FuncClkSel {
        FuncClkSel::from_bits(val)
    }
}
impl From<FuncClkSel> for u8 {
    #[inline(always)]
    fn from(val: FuncClkSel) -> u8 {
        FuncClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hystctr {
    #[doc = "Level 0: Analog comparator hysteresis 0 mV."]
    Level0 = 0x0,
    #[doc = "Level 1: Analog comparator hysteresis 10 mV."]
    Level1 = 0x01,
    #[doc = "Level 2: Analog comparator hysteresis 20 mV."]
    Level2 = 0x02,
    #[doc = "Level 3: Analog comparator hysteresis 30 mV."]
    Level3 = 0x03,
}
impl Hystctr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hystctr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hystctr {
    #[inline(always)]
    fn from(val: u8) -> Hystctr {
        Hystctr::from_bits(val)
    }
}
impl From<Hystctr> for u8 {
    #[inline(always)]
    fn from(val: Hystctr) -> u8 {
        Hystctr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msel {
    #[doc = "Input 0m."]
    Input0 = 0x0,
    #[doc = "Input 1m."]
    Input1 = 0x01,
    #[doc = "Input 2m."]
    Input2 = 0x02,
    #[doc = "Input 3m."]
    Input3 = 0x03,
    #[doc = "Input 4m."]
    Input4 = 0x04,
    #[doc = "Input 5m."]
    Input5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal DAC output."]
    Input7 = 0x07,
}
impl Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msel {
    #[inline(always)]
    fn from(val: u8) -> Msel {
        Msel::from_bits(val)
    }
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(val: Msel) -> u8 {
        Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psel {
    #[doc = "Input 0p."]
    Input0 = 0x0,
    #[doc = "Input 1p."]
    Input1 = 0x01,
    #[doc = "Input 2p."]
    Input2 = 0x02,
    #[doc = "Input 3p."]
    Input3 = 0x03,
    #[doc = "Input 4p."]
    Input4 = 0x04,
    #[doc = "Input 5p."]
    Input5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Internal DAC output."]
    Input7 = 0x07,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrClkSel {
    #[doc = "Select Round Robin clock Source 0."]
    Rr0 = 0x0,
    #[doc = "Select Round Robin clock Source 1."]
    Rr1 = 0x01,
    #[doc = "Select Round Robin clock Source 2."]
    Rr2 = 0x02,
    #[doc = "Select Round Robin clock Source 3."]
    Rr3 = 0x03,
}
impl RrClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrClkSel {
    #[inline(always)]
    fn from(val: u8) -> RrClkSel {
        RrClkSel::from_bits(val)
    }
}
impl From<RrClkSel> for u8 {
    #[inline(always)]
    fn from(val: RrClkSel) -> u8 {
        RrClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrInitmod {
    #[doc = "63 cycles (same as 111111b)."]
    Mod63 = 0x0,
    #[doc = "1 to 63 cycles."]
    Mod1631 = 0x01,
    #[doc = "1 to 63 cycles."]
    Mod1632 = 0x02,
    #[doc = "1 to 63 cycles."]
    Mod1633 = 0x03,
    #[doc = "1 to 63 cycles."]
    Mod1634 = 0x04,
    #[doc = "1 to 63 cycles."]
    Mod1635 = 0x05,
    #[doc = "1 to 63 cycles."]
    Mod1636 = 0x06,
    #[doc = "1 to 63 cycles."]
    Mod1637 = 0x07,
    #[doc = "1 to 63 cycles."]
    Mod1638 = 0x08,
    #[doc = "1 to 63 cycles."]
    Mod1639 = 0x09,
    #[doc = "1 to 63 cycles."]
    Mod16310 = 0x0a,
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
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    #[doc = "1 to 63 cycles."]
    Mod16358 = 0x3a,
    #[doc = "1 to 63 cycles."]
    Mod16359 = 0x3b,
    #[doc = "1 to 63 cycles."]
    Mod16360 = 0x3c,
    #[doc = "1 to 63 cycles."]
    Mod16361 = 0x3d,
    #[doc = "1 to 63 cycles."]
    Mod16362 = 0x3e,
    #[doc = "1 to 63 cycles."]
    Mod16363 = 0x3f,
}
impl RrInitmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrInitmod {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrInitmod {
    #[inline(always)]
    fn from(val: u8) -> RrInitmod {
        RrInitmod::from_bits(val)
    }
}
impl From<RrInitmod> for u8 {
    #[inline(always)]
    fn from(val: RrInitmod) -> u8 {
        RrInitmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrNsam {
    #[doc = "0 clock."]
    Wait0 = 0x0,
    #[doc = "1 clock."]
    Wait1 = 0x01,
    #[doc = "2 clocks."]
    Wait2 = 0x02,
    #[doc = "3 clocks."]
    Wait3 = 0x03,
}
impl RrNsam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrNsam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrNsam {
    #[inline(always)]
    fn from(val: u8) -> RrNsam {
        RrNsam::from_bits(val)
    }
}
impl From<RrNsam> for u8 {
    #[inline(always)]
    fn from(val: RrNsam) -> u8 {
        RrNsam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrSampleCnt {
    #[doc = "1 samples."]
    Sample0 = 0x0,
    #[doc = "2 samples."]
    Sample1 = 0x01,
    #[doc = "3 samples."]
    Sample2 = 0x02,
    #[doc = "4 samples."]
    Sample3 = 0x03,
    #[doc = "5 samples."]
    Sample4 = 0x04,
    #[doc = "6 samples."]
    Sample5 = 0x05,
    #[doc = "7 samples."]
    Sample6 = 0x06,
    #[doc = "8 samples."]
    Sample7 = 0x07,
    #[doc = "9 samples."]
    Sample8 = 0x08,
    #[doc = "10 samples."]
    Sample9 = 0x09,
    #[doc = "11 samples."]
    Sample10 = 0x0a,
    #[doc = "12 samples."]
    Sample11 = 0x0b,
    #[doc = "13 samples."]
    Sample12 = 0x0c,
    #[doc = "14 samples."]
    Sample13 = 0x0d,
    #[doc = "15 samples."]
    Sample14 = 0x0e,
    #[doc = "16 samples."]
    Sample15 = 0x0f,
}
impl RrSampleCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrSampleCnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrSampleCnt {
    #[inline(always)]
    fn from(val: u8) -> RrSampleCnt {
        RrSampleCnt::from_bits(val)
    }
}
impl From<RrSampleCnt> for u8 {
    #[inline(always)]
    fn from(val: RrSampleCnt) -> u8 {
        RrSampleCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrSampleThreshold {
    #[doc = "At least 1 sampled \"1\", the final result is \"1\"."]
    Sample0 = 0x0,
    #[doc = "At least 2 sampled \"1\", the final result is \"1\"."]
    Sample1 = 0x01,
    #[doc = "At least 3 sampled \"1\", the final result is \"1\"."]
    Sample2 = 0x02,
    #[doc = "At least 4 sampled \"1\", the final result is \"1\"."]
    Sample3 = 0x03,
    #[doc = "At least 5 sampled \"1\", the final result is \"1\"."]
    Sample4 = 0x04,
    #[doc = "At least 6 sampled \"1\", the final result is \"1\"."]
    Sample5 = 0x05,
    #[doc = "At least 7 sampled \"1\", the final result is \"1\"."]
    Sample6 = 0x06,
    #[doc = "At least 8 sampled \"1\", the final result is \"1\"."]
    Sample7 = 0x07,
    #[doc = "At least 9 sampled \"1\", the final result is \"1\"."]
    Sample8 = 0x08,
    #[doc = "At least 10 sampled \"1\", the final result is \"1\"."]
    Sample9 = 0x09,
    #[doc = "At least 11 sampled \"1\", the final result is \"1\"."]
    Sample10 = 0x0a,
    #[doc = "At least 12 sampled \"1\", the final result is \"1\"."]
    Sample11 = 0x0b,
    #[doc = "At least 13 sampled \"1\", the final result is \"1\"."]
    Sample12 = 0x0c,
    #[doc = "At least 14 sampled \"1\", the final result is \"1\"."]
    Sample13 = 0x0d,
    #[doc = "At least 15 sampled \"1\", the final result is \"1\"."]
    Sample14 = 0x0e,
    #[doc = "At least 16 sampled \"1\", the final result is \"1\"."]
    Sample15 = 0x0f,
}
impl RrSampleThreshold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrSampleThreshold {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrSampleThreshold {
    #[inline(always)]
    fn from(val: u8) -> RrSampleThreshold {
        RrSampleThreshold::from_bits(val)
    }
}
impl From<RrSampleThreshold> for u8 {
    #[inline(always)]
    fn from(val: RrSampleThreshold) -> u8 {
        RrSampleThreshold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrTrgSel {
    #[doc = "External trigger."]
    Enable = 0x0,
    #[doc = "Internal trigger."]
    Disable = 0x01,
}
impl RrTrgSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrTrgSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrTrgSel {
    #[inline(always)]
    fn from(val: u8) -> RrTrgSel {
        RrTrgSel::from_bits(val)
    }
}
impl From<RrTrgSel> for u8 {
    #[inline(always)]
    fn from(val: RrTrgSel) -> u8 {
        RrTrgSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrsel {
    #[doc = "VREFH0."]
    Vref0 = 0x0,
    #[doc = "VREFH1."]
    Vref1 = 0x01,
}
impl Vrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrsel {
    #[inline(always)]
    fn from(val: u8) -> Vrsel {
        Vrsel::from_bits(val)
    }
}
impl From<Vrsel> for u8 {
    #[inline(always)]
    fn from(val: Vrsel) -> u8 {
        Vrsel::to_bits(val)
    }
}
