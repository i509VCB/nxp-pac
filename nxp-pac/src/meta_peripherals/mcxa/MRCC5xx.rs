#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "MRCC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrcc {
    ptr: *mut u8,
}
unsafe impl Send for Mrcc {}
unsafe impl Sync for Mrcc {}
impl Mrcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Peripheral Reset Control 0."]
    #[inline(always)]
    pub const fn mrcc_glb_rst0(self) -> crate::pac::common::Reg<GlbRst0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 0."]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_set(
        self,
    ) -> crate::pac::common::Reg<GlbRstSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 0."]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_clr(
        self,
    ) -> crate::pac::common::Reg<GlbRstClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Peripheral Reset Control 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1(self) -> crate::pac::common::Reg<GlbRst1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_set(
        self,
    ) -> crate::pac::common::Reg<GlbRstSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_clr(
        self,
    ) -> crate::pac::common::Reg<GlbRstClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Peripheral Reset Control 2."]
    #[inline(always)]
    pub const fn mrcc_glb_rst2(self) -> crate::pac::common::Reg<GlbRst2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 2."]
    #[inline(always)]
    pub const fn mrcc_glb_rst2_set(
        self,
    ) -> crate::pac::common::Reg<GlbRstSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 2."]
    #[inline(always)]
    pub const fn mrcc_glb_rst2_clr(
        self,
    ) -> crate::pac::common::Reg<GlbRstClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Peripheral Reset Control 3."]
    #[inline(always)]
    pub const fn mrcc_glb_rst3(self) -> crate::pac::common::Reg<GlbRst3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 3."]
    #[inline(always)]
    pub const fn mrcc_glb_rst3_set(
        self,
    ) -> crate::pac::common::Reg<GlbRstSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 3."]
    #[inline(always)]
    pub const fn mrcc_glb_rst3_clr(
        self,
    ) -> crate::pac::common::Reg<GlbRstClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Peripheral Reset Control 4."]
    #[inline(always)]
    pub const fn mrcc_glb_rst4(self) -> crate::pac::common::Reg<GlbRst4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 4."]
    #[inline(always)]
    pub const fn mrcc_glb_rst4_set(
        self,
    ) -> crate::pac::common::Reg<GlbRstSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 4."]
    #[inline(always)]
    pub const fn mrcc_glb_rst4_clr(
        self,
    ) -> crate::pac::common::Reg<GlbRstClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "AHB Clock Control 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0(self) -> crate::pac::common::Reg<GlbCc0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "AHB Clock Control Set 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "AHB Clock Control 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1(self) -> crate::pac::common::Reg<GlbCc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "AHB Clock Control Set 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "AHB Clock Control 2."]
    #[inline(always)]
    pub const fn mrcc_glb_cc2(self) -> crate::pac::common::Reg<GlbCc2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "AHB Clock Control Set 2."]
    #[inline(always)]
    pub const fn mrcc_glb_cc2_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 2."]
    #[inline(always)]
    pub const fn mrcc_glb_cc2_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "AHB Clock Control 3."]
    #[inline(always)]
    pub const fn mrcc_glb_cc3(self) -> crate::pac::common::Reg<GlbCc3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "AHB Clock Control Set 3."]
    #[inline(always)]
    pub const fn mrcc_glb_cc3_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 3."]
    #[inline(always)]
    pub const fn mrcc_glb_cc3_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "AHB Clock Control 4."]
    #[inline(always)]
    pub const fn mrcc_glb_cc4(self) -> crate::pac::common::Reg<GlbCc4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "AHB Clock Control Set 4."]
    #[inline(always)]
    pub const fn mrcc_glb_cc4_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 4."]
    #[inline(always)]
    pub const fn mrcc_glb_cc4_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 0."]
    #[inline(always)]
    pub const fn mrcc_glb_acc0(self) -> crate::pac::common::Reg<GlbAcc0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 1."]
    #[inline(always)]
    pub const fn mrcc_glb_acc1(self) -> crate::pac::common::Reg<GlbAcc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 2."]
    #[inline(always)]
    pub const fn mrcc_glb_acc2(self) -> crate::pac::common::Reg<GlbAcc2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 3."]
    #[inline(always)]
    pub const fn mrcc_glb_acc3(self) -> crate::pac::common::Reg<GlbAcc3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 4."]
    #[inline(always)]
    pub const fn mrcc_glb_acc4(self) -> crate::pac::common::Reg<GlbAcc4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Peripheral Enable Configuration 0. Reset on POR only."]
    #[inline(always)]
    pub const fn mrcc_glb_pr0(self) -> crate::pac::common::Reg<GlbPr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Peripheral Enable Configuration 1. Reset on POR only."]
    #[inline(always)]
    pub const fn mrcc_glb_pr1(self) -> crate::pac::common::Reg<GlbPr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Peripheral Enable Configuration 2. Reset on POR only."]
    #[inline(always)]
    pub const fn mrcc_glb_pr2(self) -> crate::pac::common::Reg<GlbPr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Peripheral Enable Configuration 3. Reset on POR only."]
    #[inline(always)]
    pub const fn mrcc_glb_pr3(self) -> crate::pac::common::Reg<GlbPr3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Peripheral Enable Configuration 4. Reset on POR only."]
    #[inline(always)]
    pub const fn mrcc_glb_pr4(self) -> crate::pac::common::Reg<GlbPr4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "I3C0_FCLK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clksel(
        self,
    ) -> crate::pac::common::Reg<I3cFclkClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "I3C0_FCLK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "I3C1_FCLK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_i3c1_fclk_clksel(
        self,
    ) -> crate::pac::common::Reg<I3cFclkClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "I3C1_FCLK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_i3c1_fclk_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "CTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "CTIMER0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "CTIMER1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "CTIMER1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "CTIMER2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "CTIMER2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "CTIMER3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "CTIMER3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "CTIMER4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "CTIMER4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "WWDT0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_wwdt0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "WWDT1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_wwdt1_clksel(
        self,
    ) -> crate::pac::common::Reg<WwdtClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "WWDT1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_wwdt1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "E1588 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_e1588_clksel(
        self,
    ) -> crate::pac::common::Reg<E158Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "E1588 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_e1588_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "RMII clock selection control."]
    #[inline(always)]
    pub const fn mrcc_rmii_clksel(
        self,
    ) -> crate::pac::common::Reg<RmiiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "RMII clock divider control."]
    #[inline(always)]
    pub const fn mrcc_rmii_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "ESPI0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_espi0_clksel(
        self,
    ) -> crate::pac::common::Reg<EspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "ESPI0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_espi0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "FLEXSPI0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexspi0_clksel(
        self,
    ) -> crate::pac::common::Reg<FlexspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "FLEXSPI0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexspi0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "LPSPI2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi2_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "LPSPI2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "LPSPI3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi3_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "LPSPI3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "LPSPI4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi4_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "LPSPI4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "LPSPI5 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi5_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "LPSPI5 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi5_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "T1S0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_t1s0_clksel(
        self,
    ) -> crate::pac::common::Reg<T1sClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "T1S0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_t1s0_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "USB1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_usb1_clksel(
        self,
    ) -> crate::pac::common::Reg<UsbClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "USB1_PHY clock selection control."]
    #[inline(always)]
    pub const fn mrcc_usb1_phy_clksel(
        self,
    ) -> crate::pac::common::Reg<UsbPhyClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "USB1_PHY clock divider control."]
    #[inline(always)]
    pub const fn mrcc_usb1_phy_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "FLEXIO0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clksel(
        self,
    ) -> crate::pac::common::Reg<FlexioClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "FLEXIO0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "LPI2C0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "LPI2C0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "LPI2C1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "LPI2C1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "LPSPI0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "LPSPI0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "LPSPI1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "LPSPI1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "I3C2_FCLK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_i3c2_fclk_clksel(
        self,
    ) -> crate::pac::common::Reg<I3cFclkClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "I3C2_FCLK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_i3c2_fclk_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "LPUART0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "LPUART0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "LPUART1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "LPUART1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "LPUART2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "LPUART2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "LPUART3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "LPUART3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "LPUART4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "LPUART4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "LPTMR0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clksel(
        self,
    ) -> crate::pac::common::Reg<LptmrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "LPTMR0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "OSTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ostimer0_clksel(
        self,
    ) -> crate::pac::common::Reg<OstimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "ADCx clock selection control."]
    #[inline(always)]
    pub const fn mrcc_adc_clksel(
        self,
    ) -> crate::pac::common::Reg<AdcClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "ADCx clock divider control."]
    #[inline(always)]
    pub const fn mrcc_adc_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "CMP0_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_func_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "CMP0_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clksel(
        self,
    ) -> crate::pac::common::Reg<CmpRrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "CMP0_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "DAC0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clksel(
        self,
    ) -> crate::pac::common::Reg<DacClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "DAC0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "DAC1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dac1_clksel(
        self,
    ) -> crate::pac::common::Reg<DacClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "DAC1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dac1_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "TSI0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_tsi0_clksel(
        self,
    ) -> crate::pac::common::Reg<TsiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "TSI0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_tsi0_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "FLEXCAN0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clksel(
        self,
    ) -> crate::pac::common::Reg<FlexcanClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "FLEXCAN0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "FLEXCAN1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexcan1_clksel(
        self,
    ) -> crate::pac::common::Reg<FlexcanClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "FLEXCAN1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexcan1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "LPI2C2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "LPI2C2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "LPI2C3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "LPI2C3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "LPI2C4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c4_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "LPI2C4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "LPUART5 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart5_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "LPUART5 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart5_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "I3C3_FCLK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_i3c3_fclk_clksel(
        self,
    ) -> crate::pac::common::Reg<I3cFclkClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "I3C3_FCLK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_i3c3_fclk_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "DBG_TRACE clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clksel(
        self,
    ) -> crate::pac::common::Reg<DbgTraceClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "DBG_TRACE clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "CLKOUT clock selection control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clksel(
        self,
    ) -> crate::pac::common::Reg<ClkoutClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "CLKOUT clock divider control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "SYSTICK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_systick_clksel(
        self,
    ) -> crate::pac::common::Reg<SystickClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "SYSTICK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_systick_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
}
#[doc = "ADCx clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcClksel(pub u32);
impl AdcClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> AdcClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        AdcClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: AdcClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for AdcClksel {
    #[inline(always)]
    fn default() -> AdcClksel {
        AdcClksel(0)
    }
}
impl core::fmt::Debug for AdcClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "ADCx clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc = "Functional Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Functional Clock Divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reset divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Clkdiv {
    #[inline(always)]
    fn default() -> Clkdiv {
        Clkdiv(0)
    }
}
impl core::fmt::Debug for Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CLKOUT clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkoutClksel(pub u32);
impl ClkoutClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> ClkoutClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        ClkoutClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: ClkoutClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for ClkoutClksel {
    #[inline(always)]
    fn default() -> ClkoutClksel {
        ClkoutClksel(0)
    }
}
impl core::fmt::Debug for ClkoutClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkoutClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkoutClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ClkoutClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP0_RR clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpRrClksel(pub u32);
impl CmpRrClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> RrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        RrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: RrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CmpRrClksel {
    #[inline(always)]
    fn default() -> CmpRrClksel {
        CmpRrClksel(0)
    }
}
impl core::fmt::Debug for CmpRrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpRrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpRrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmpRrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtimerClksel(pub u32);
impl CtimerClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> CtimerClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        CtimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: CtimerClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CtimerClksel {
    #[inline(always)]
    fn default() -> CtimerClksel {
        CtimerClksel(0)
    }
}
impl core::fmt::Debug for CtimerClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtimerClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtimerClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CtimerClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DAC0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacClksel(pub u32);
impl DacClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> DacClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        DacClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: DacClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for DacClksel {
    #[inline(always)]
    fn default() -> DacClksel {
        DacClksel(0)
    }
}
impl core::fmt::Debug for DacClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DBG_TRACE clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgTraceClksel(pub u32);
impl DbgTraceClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> DbgTraceClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        DbgTraceClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: DbgTraceClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for DbgTraceClksel {
    #[inline(always)]
    fn default() -> DbgTraceClksel {
        DbgTraceClksel(0)
    }
}
impl core::fmt::Debug for DbgTraceClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgTraceClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgTraceClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DbgTraceClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "E1588 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E158Clksel(pub u32);
impl E158Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> E158ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        E158ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: E158ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for E158Clksel {
    #[inline(always)]
    fn default() -> E158Clksel {
        E158Clksel(0)
    }
}
impl core::fmt::Debug for E158Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E158Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E158Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "E158Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "ESPI0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EspiClksel(pub u32);
impl EspiClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> EspiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        EspiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: EspiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for EspiClksel {
    #[inline(always)]
    fn default() -> EspiClksel {
        EspiClksel(0)
    }
}
impl core::fmt::Debug for EspiClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EspiClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EspiClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EspiClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXCAN0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexcanClksel(pub u32);
impl FlexcanClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> FlexcanClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        FlexcanClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: FlexcanClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FlexcanClksel {
    #[inline(always)]
    fn default() -> FlexcanClksel {
        FlexcanClksel(0)
    }
}
impl core::fmt::Debug for FlexcanClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexcanClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexcanClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexcanClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXIO0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexioClksel(pub u32);
impl FlexioClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> FlexioClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        FlexioClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: FlexioClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FlexioClksel {
    #[inline(always)]
    fn default() -> FlexioClksel {
        FlexioClksel(0)
    }
}
impl core::fmt::Debug for FlexioClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexioClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexioClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexioClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXSPI0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiClksel(pub u32);
impl FlexspiClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> FlexspiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        FlexspiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: FlexspiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FlexspiClksel {
    #[inline(always)]
    fn default() -> FlexspiClksel {
        FlexspiClksel(0)
    }
}
impl core::fmt::Debug for FlexspiClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexspiClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "Control Automatic Clock Gating 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbAcc0(pub u32);
impl GlbAcc0 {
    #[doc = "INPUTMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0."]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPI2C4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C4."]
    #[inline(always)]
    pub const fn set_lpi2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbAcc0 {
    #[inline(always)]
    fn default() -> GlbAcc0 {
        GlbAcc0(0)
    }
}
impl core::fmt::Debug for GlbAcc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbAcc0")
            .field("inputmux0", &self.inputmux0())
            .field("freqme", &self.freqme())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpi2c4", &self.lpi2c4())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("lpuart5", &self.lpuart5())
            .field("ostimer0", &self.ostimer0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc0 {{ inputmux0: {=bool:?}, freqme: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, wwdt1: {=bool:?}, dma0: {=bool:?}, dma1: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpi2c4: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, lpuart5: {=bool:?}, ostimer0: {=bool:?} }}",
            self.inputmux0(),
            self.freqme(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.utick0(),
            self.wwdt0(),
            self.wwdt1(),
            self.dma0(),
            self.dma1(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpi2c4(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.lpuart5(),
            self.ostimer0()
        )
    }
}
#[doc = "Control Automatic Clock Gating 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbAcc1(pub u32);
impl GlbAcc1 {
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPSPI2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2."]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPSPI3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI3."]
    #[inline(always)]
    pub const fn set_lpspi3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LPSPI4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI4."]
    #[inline(always)]
    pub const fn set_lpspi4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPSPI5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi5(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI5."]
    #[inline(always)]
    pub const fn set_lpspi5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DAC1."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DAC1."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "VREF0."]
    #[must_use]
    #[inline(always)]
    pub const fn vref0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "VREF0."]
    #[inline(always)]
    pub const fn set_vref0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TSI0."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TSI0."]
    #[inline(always)]
    pub const fn set_tsi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbAcc1 {
    #[inline(always)]
    fn default() -> GlbAcc1 {
        GlbAcc1(0)
    }
}
impl core::fmt::Debug for GlbAcc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbAcc1")
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpspi2", &self.lpspi2())
            .field("lpspi3", &self.lpspi3())
            .field("lpspi4", &self.lpspi4())
            .field("lpspi5", &self.lpspi5())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .field("vref0", &self.vref0())
            .field("tsi0", &self.tsi0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc1 {{ lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpspi2: {=bool:?}, lpspi3: {=bool:?}, lpspi4: {=bool:?}, lpspi5: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, dac0: {=bool:?}, dac1: {=bool:?}, vref0: {=bool:?}, tsi0: {=bool:?} }}",
            self.lpspi0(),
            self.lpspi1(),
            self.lpspi2(),
            self.lpspi3(),
            self.lpspi4(),
            self.lpspi5(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.dac0(),
            self.dac1(),
            self.vref0(),
            self.tsi0()
        )
    }
}
#[doc = "Control Automatic Clock Gating 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbAcc2(pub u32);
impl GlbAcc2 {
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C2."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C2."]
    #[inline(always)]
    pub const fn set_i3c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I3C3."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I3C3."]
    #[inline(always)]
    pub const fn set_i3c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "E1588."]
    #[must_use]
    #[inline(always)]
    pub const fn e1588(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "E1588."]
    #[inline(always)]
    pub const fn set_e1588(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RMII."]
    #[must_use]
    #[inline(always)]
    pub const fn rmii(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RMII."]
    #[inline(always)]
    pub const fn set_rmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "T1S0."]
    #[must_use]
    #[inline(always)]
    pub const fn t1s0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "T1S0."]
    #[inline(always)]
    pub const fn set_t1s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_flexspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPI0_FILTER."]
    #[must_use]
    #[inline(always)]
    pub const fn spi0_filter(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPI0_FILTER."]
    #[inline(always)]
    pub const fn set_spi0_filter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ESPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn espi0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ESPI0."]
    #[inline(always)]
    pub const fn set_espi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "USB1_PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "USB1_PHY."]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for GlbAcc2 {
    #[inline(always)]
    fn default() -> GlbAcc2 {
        GlbAcc2(0)
    }
}
impl core::fmt::Debug for GlbAcc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbAcc2")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("i3c2", &self.i3c2())
            .field("i3c3", &self.i3c3())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("e1588", &self.e1588())
            .field("rmii", &self.rmii())
            .field("enet0", &self.enet0())
            .field("t1s0", &self.t1s0())
            .field("flexspi0", &self.flexspi0())
            .field("spi0_filter", &self.spi0_filter())
            .field("espi0", &self.espi0())
            .field("usb1", &self.usb1())
            .field("usb1_phy", &self.usb1_phy())
            .field("ewm0", &self.ewm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc2 {{ i3c0: {=bool:?}, i3c1: {=bool:?}, i3c2: {=bool:?}, i3c3: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, e1588: {=bool:?}, rmii: {=bool:?}, enet0: {=bool:?}, t1s0: {=bool:?}, flexspi0: {=bool:?}, spi0_filter: {=bool:?}, espi0: {=bool:?}, usb1: {=bool:?}, usb1_phy: {=bool:?}, ewm0: {=bool:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.i3c2(),
            self.i3c3(),
            self.flexcan0(),
            self.flexcan1(),
            self.e1588(),
            self.rmii(),
            self.enet0(),
            self.t1s0(),
            self.flexspi0(),
            self.spi0_filter(),
            self.espi0(),
            self.usb1(),
            self.usb1_phy(),
            self.ewm0()
        )
    }
}
#[doc = "Control Automatic Clock Gating 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbAcc3(pub u32);
impl GlbAcc3 {
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ROMC."]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "SMARTDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0."]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for GlbAcc3 {
    #[inline(always)]
    fn default() -> GlbAcc3 {
        GlbAcc3(0)
    }
}
impl core::fmt::Debug for GlbAcc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbAcc3")
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("romc", &self.romc())
            .field("smartdma0", &self.smartdma0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc3 {{ rama: {=bool:?}, ramb: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, romc: {=bool:?}, smartdma0: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.romc(),
            self.smartdma0()
        )
    }
}
#[doc = "Control Automatic Clock Gating 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbAcc4(pub u32);
impl GlbAcc4 {
    #[doc = "SECCON."]
    #[must_use]
    #[inline(always)]
    pub const fn seccon(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SECCON."]
    #[inline(always)]
    pub const fn set_seccon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GLIKEY0."]
    #[must_use]
    #[inline(always)]
    pub const fn glikey0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GLIKEY0."]
    #[inline(always)]
    pub const fn set_glikey0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "UDF0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DGDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn dgdet0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DGDET0."]
    #[inline(always)]
    pub const fn set_dgdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ATX0."]
    #[must_use]
    #[inline(always)]
    pub const fn atx0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ATX0."]
    #[inline(always)]
    pub const fn set_atx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for GlbAcc4 {
    #[inline(always)]
    fn default() -> GlbAcc4 {
        GlbAcc4(0)
    }
}
impl core::fmt::Debug for GlbAcc4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbAcc4")
            .field("seccon", &self.seccon())
            .field("glikey0", &self.glikey0())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("dgdet0", &self.dgdet0())
            .field("atx0", &self.atx0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc4 {{ seccon: {=bool:?}, glikey0: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, dgdet0: {=bool:?}, atx0: {=bool:?} }}",
            self.seccon(),
            self.glikey0(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.dgdet0(),
            self.atx0()
        )
    }
}
#[doc = "AHB Clock Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCc0(pub u32);
impl GlbCc0 {
    #[doc = "INPUTMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0."]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPI2C4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C4."]
    #[inline(always)]
    pub const fn set_lpi2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbCc0 {
    #[inline(always)]
    fn default() -> GlbCc0 {
        GlbCc0(0)
    }
}
impl core::fmt::Debug for GlbCc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCc0")
            .field("inputmux0", &self.inputmux0())
            .field("freqme", &self.freqme())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpi2c4", &self.lpi2c4())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("lpuart5", &self.lpuart5())
            .field("ostimer0", &self.ostimer0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc0 {{ inputmux0: {=bool:?}, freqme: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, wwdt1: {=bool:?}, dma0: {=bool:?}, dma1: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpi2c4: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, lpuart5: {=bool:?}, ostimer0: {=bool:?} }}",
            self.inputmux0(),
            self.freqme(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.utick0(),
            self.wwdt0(),
            self.wwdt1(),
            self.dma0(),
            self.dma1(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpi2c4(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.lpuart5(),
            self.ostimer0()
        )
    }
}
#[doc = "AHB Clock Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCc1(pub u32);
impl GlbCc1 {
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPSPI2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2."]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPSPI3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI3."]
    #[inline(always)]
    pub const fn set_lpspi3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LPSPI4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI4."]
    #[inline(always)]
    pub const fn set_lpspi4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPSPI5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi5(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI5."]
    #[inline(always)]
    pub const fn set_lpspi5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT5."]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT5."]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DAC1."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DAC1."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "VREF0."]
    #[must_use]
    #[inline(always)]
    pub const fn vref0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "VREF0."]
    #[inline(always)]
    pub const fn set_vref0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TSI0."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TSI0."]
    #[inline(always)]
    pub const fn set_tsi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbCc1 {
    #[inline(always)]
    fn default() -> GlbCc1 {
        GlbCc1(0)
    }
}
impl core::fmt::Debug for GlbCc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCc1")
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpspi2", &self.lpspi2())
            .field("lpspi3", &self.lpspi3())
            .field("lpspi4", &self.lpspi4())
            .field("lpspi5", &self.lpspi5())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("port5", &self.port5())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .field("vref0", &self.vref0())
            .field("tsi0", &self.tsi0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc1 {{ lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpspi2: {=bool:?}, lpspi3: {=bool:?}, lpspi4: {=bool:?}, lpspi5: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, port5: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, dac0: {=bool:?}, dac1: {=bool:?}, vref0: {=bool:?}, tsi0: {=bool:?} }}",
            self.lpspi0(),
            self.lpspi1(),
            self.lpspi2(),
            self.lpspi3(),
            self.lpspi4(),
            self.lpspi5(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.port5(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.dac0(),
            self.dac1(),
            self.vref0(),
            self.tsi0()
        )
    }
}
#[doc = "AHB Clock Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCc2(pub u32);
impl GlbCc2 {
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C2."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C2."]
    #[inline(always)]
    pub const fn set_i3c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I3C3."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I3C3."]
    #[inline(always)]
    pub const fn set_i3c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "E1588."]
    #[must_use]
    #[inline(always)]
    pub const fn e1588(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "E1588."]
    #[inline(always)]
    pub const fn set_e1588(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RMII."]
    #[must_use]
    #[inline(always)]
    pub const fn rmii(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RMII."]
    #[inline(always)]
    pub const fn set_rmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "T1S0."]
    #[must_use]
    #[inline(always)]
    pub const fn t1s0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "T1S0."]
    #[inline(always)]
    pub const fn set_t1s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_flexspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPI0_FILTER."]
    #[must_use]
    #[inline(always)]
    pub const fn spi0_filter(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPI0_FILTER."]
    #[inline(always)]
    pub const fn set_spi0_filter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ESPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn espi0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ESPI0."]
    #[inline(always)]
    pub const fn set_espi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "USB1_PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "USB1_PHY."]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for GlbCc2 {
    #[inline(always)]
    fn default() -> GlbCc2 {
        GlbCc2(0)
    }
}
impl core::fmt::Debug for GlbCc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCc2")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("i3c2", &self.i3c2())
            .field("i3c3", &self.i3c3())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("e1588", &self.e1588())
            .field("rmii", &self.rmii())
            .field("enet0", &self.enet0())
            .field("t1s0", &self.t1s0())
            .field("flexspi0", &self.flexspi0())
            .field("spi0_filter", &self.spi0_filter())
            .field("espi0", &self.espi0())
            .field("usb1", &self.usb1())
            .field("usb1_phy", &self.usb1_phy())
            .field("ewm0", &self.ewm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc2 {{ i3c0: {=bool:?}, i3c1: {=bool:?}, i3c2: {=bool:?}, i3c3: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, e1588: {=bool:?}, rmii: {=bool:?}, enet0: {=bool:?}, t1s0: {=bool:?}, flexspi0: {=bool:?}, spi0_filter: {=bool:?}, espi0: {=bool:?}, usb1: {=bool:?}, usb1_phy: {=bool:?}, ewm0: {=bool:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.i3c2(),
            self.i3c3(),
            self.flexcan0(),
            self.flexcan1(),
            self.e1588(),
            self.rmii(),
            self.enet0(),
            self.t1s0(),
            self.flexspi0(),
            self.spi0_filter(),
            self.espi0(),
            self.usb1(),
            self.usb1_phy(),
            self.ewm0()
        )
    }
}
#[doc = "AHB Clock Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCc3(pub u32);
impl GlbCc3 {
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO5."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO5."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ROMC."]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "SMARTDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0."]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for GlbCc3 {
    #[inline(always)]
    fn default() -> GlbCc3 {
        GlbCc3(0)
    }
}
impl core::fmt::Debug for GlbCc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCc3")
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("gpio5", &self.gpio5())
            .field("romc", &self.romc())
            .field("smartdma0", &self.smartdma0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc3 {{ rama: {=bool:?}, ramb: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, gpio5: {=bool:?}, romc: {=bool:?}, smartdma0: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.gpio5(),
            self.romc(),
            self.smartdma0()
        )
    }
}
#[doc = "AHB Clock Control 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCc4(pub u32);
impl GlbCc4 {
    #[doc = "SECCON."]
    #[must_use]
    #[inline(always)]
    pub const fn seccon(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SECCON."]
    #[inline(always)]
    pub const fn set_seccon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GLIKEY0."]
    #[must_use]
    #[inline(always)]
    pub const fn glikey0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GLIKEY0."]
    #[inline(always)]
    pub const fn set_glikey0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TDET0."]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "UDF0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DGDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn dgdet0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DGDET0."]
    #[inline(always)]
    pub const fn set_dgdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ITRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC0."]
    #[inline(always)]
    pub const fn set_itrc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ATX0."]
    #[must_use]
    #[inline(always)]
    pub const fn atx0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ATX0."]
    #[inline(always)]
    pub const fn set_atx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "MTR."]
    #[must_use]
    #[inline(always)]
    pub const fn mtr(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "MTR."]
    #[inline(always)]
    pub const fn set_mtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "TCU."]
    #[must_use]
    #[inline(always)]
    pub const fn tcu(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TCU."]
    #[inline(always)]
    pub const fn set_tcu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbCc4 {
    #[inline(always)]
    fn default() -> GlbCc4 {
        GlbCc4(0)
    }
}
impl core::fmt::Debug for GlbCc4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCc4")
            .field("seccon", &self.seccon())
            .field("glikey0", &self.glikey0())
            .field("tdet0", &self.tdet0())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("dgdet0", &self.dgdet0())
            .field("itrc0", &self.itrc0())
            .field("atx0", &self.atx0())
            .field("mtr", &self.mtr())
            .field("tcu", &self.tcu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc4 {{ seccon: {=bool:?}, glikey0: {=bool:?}, tdet0: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, dgdet0: {=bool:?}, itrc0: {=bool:?}, atx0: {=bool:?}, mtr: {=bool:?}, tcu: {=bool:?} }}",
            self.seccon(),
            self.glikey0(),
            self.tdet0(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.dgdet0(),
            self.itrc0(),
            self.atx0(),
            self.mtr(),
            self.tcu()
        )
    }
}
#[doc = "AHB Clock Control Clear 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCcClr(pub u32);
impl GlbCcClr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GlbCcClr {
    #[inline(always)]
    fn default() -> GlbCcClr {
        GlbCcClr(0)
    }
}
impl core::fmt::Debug for GlbCcClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCcClr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCcClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbCcClr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCcSet(pub u32);
impl GlbCcSet {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_CCn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GlbCcSet {
    #[inline(always)]
    fn default() -> GlbCcSet {
        GlbCcSet(0)
    }
}
impl core::fmt::Debug for GlbCcSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbCcSet")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCcSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbCcSet {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Enable Configuration 0. Reset on POR only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbPr0(pub u32);
impl GlbPr0 {
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPI2C4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C4."]
    #[inline(always)]
    pub const fn set_lpi2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbPr0 {
    #[inline(always)]
    fn default() -> GlbPr0 {
        GlbPr0(0)
    }
}
impl core::fmt::Debug for GlbPr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbPr0")
            .field("freqme", &self.freqme())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpi2c4", &self.lpi2c4())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("lpuart5", &self.lpuart5())
            .field("ostimer0", &self.ostimer0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbPr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbPr0 {{ freqme: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, wwdt1: {=bool:?}, dma0: {=bool:?}, dma1: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpi2c4: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, lpuart5: {=bool:?}, ostimer0: {=bool:?} }}",
            self.freqme(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.utick0(),
            self.wwdt0(),
            self.wwdt1(),
            self.dma0(),
            self.dma1(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpi2c4(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.lpuart5(),
            self.ostimer0()
        )
    }
}
#[doc = "Peripheral Enable Configuration 1. Reset on POR only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbPr1(pub u32);
impl GlbPr1 {
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPSPI2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2."]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPSPI3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI3."]
    #[inline(always)]
    pub const fn set_lpspi3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LPSPI4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI4."]
    #[inline(always)]
    pub const fn set_lpspi4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPSPI5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi5(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI5."]
    #[inline(always)]
    pub const fn set_lpspi5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT5."]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT5."]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DAC1."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DAC1."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "VREF0."]
    #[must_use]
    #[inline(always)]
    pub const fn vref0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "VREF0."]
    #[inline(always)]
    pub const fn set_vref0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TSI0."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TSI0."]
    #[inline(always)]
    pub const fn set_tsi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbPr1 {
    #[inline(always)]
    fn default() -> GlbPr1 {
        GlbPr1(0)
    }
}
impl core::fmt::Debug for GlbPr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbPr1")
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpspi2", &self.lpspi2())
            .field("lpspi3", &self.lpspi3())
            .field("lpspi4", &self.lpspi4())
            .field("lpspi5", &self.lpspi5())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("port5", &self.port5())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .field("vref0", &self.vref0())
            .field("tsi0", &self.tsi0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbPr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbPr1 {{ lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpspi2: {=bool:?}, lpspi3: {=bool:?}, lpspi4: {=bool:?}, lpspi5: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, port5: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, dac0: {=bool:?}, dac1: {=bool:?}, vref0: {=bool:?}, tsi0: {=bool:?} }}",
            self.lpspi0(),
            self.lpspi1(),
            self.lpspi2(),
            self.lpspi3(),
            self.lpspi4(),
            self.lpspi5(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.port5(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.dac0(),
            self.dac1(),
            self.vref0(),
            self.tsi0()
        )
    }
}
#[doc = "Peripheral Enable Configuration 2. Reset on POR only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbPr2(pub u32);
impl GlbPr2 {
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C2."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C2."]
    #[inline(always)]
    pub const fn set_i3c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I3C3."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I3C3."]
    #[inline(always)]
    pub const fn set_i3c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "E1588."]
    #[must_use]
    #[inline(always)]
    pub const fn e1588(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "E1588."]
    #[inline(always)]
    pub const fn set_e1588(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RMII."]
    #[must_use]
    #[inline(always)]
    pub const fn rmii(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RMII."]
    #[inline(always)]
    pub const fn set_rmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "T1S0."]
    #[must_use]
    #[inline(always)]
    pub const fn t1s0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "T1S0."]
    #[inline(always)]
    pub const fn set_t1s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_flexspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPI0_FILTER."]
    #[must_use]
    #[inline(always)]
    pub const fn spi0_filter(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPI0_FILTER."]
    #[inline(always)]
    pub const fn set_spi0_filter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ESPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn espi0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ESPI0."]
    #[inline(always)]
    pub const fn set_espi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "USB1_PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "USB1_PHY."]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for GlbPr2 {
    #[inline(always)]
    fn default() -> GlbPr2 {
        GlbPr2(0)
    }
}
impl core::fmt::Debug for GlbPr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbPr2")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("i3c2", &self.i3c2())
            .field("i3c3", &self.i3c3())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("e1588", &self.e1588())
            .field("rmii", &self.rmii())
            .field("enet0", &self.enet0())
            .field("t1s0", &self.t1s0())
            .field("flexspi0", &self.flexspi0())
            .field("spi0_filter", &self.spi0_filter())
            .field("espi0", &self.espi0())
            .field("usb1", &self.usb1())
            .field("usb1_phy", &self.usb1_phy())
            .field("ewm0", &self.ewm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbPr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbPr2 {{ i3c0: {=bool:?}, i3c1: {=bool:?}, i3c2: {=bool:?}, i3c3: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, e1588: {=bool:?}, rmii: {=bool:?}, enet0: {=bool:?}, t1s0: {=bool:?}, flexspi0: {=bool:?}, spi0_filter: {=bool:?}, espi0: {=bool:?}, usb1: {=bool:?}, usb1_phy: {=bool:?}, ewm0: {=bool:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.i3c2(),
            self.i3c3(),
            self.flexcan0(),
            self.flexcan1(),
            self.e1588(),
            self.rmii(),
            self.enet0(),
            self.t1s0(),
            self.flexspi0(),
            self.spi0_filter(),
            self.espi0(),
            self.usb1(),
            self.usb1_phy(),
            self.ewm0()
        )
    }
}
#[doc = "Peripheral Enable Configuration 3. Reset on POR only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbPr3(pub u32);
impl GlbPr3 {
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "GPIO5."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO5."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SMARTDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0."]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for GlbPr3 {
    #[inline(always)]
    fn default() -> GlbPr3 {
        GlbPr3(0)
    }
}
impl core::fmt::Debug for GlbPr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbPr3")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("gpio5", &self.gpio5())
            .field("smartdma0", &self.smartdma0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbPr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbPr3 {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, gpio5: {=bool:?}, smartdma0: {=bool:?} }}",
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.gpio5(),
            self.smartdma0()
        )
    }
}
#[doc = "Peripheral Enable Configuration 4. Reset on POR only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbPr4(pub u32);
impl GlbPr4 {
    #[doc = "SECCON."]
    #[must_use]
    #[inline(always)]
    pub const fn seccon(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SECCON."]
    #[inline(always)]
    pub const fn set_seccon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "GLIKEY0."]
    #[must_use]
    #[inline(always)]
    pub const fn glikey0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GLIKEY0."]
    #[inline(always)]
    pub const fn set_glikey0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TDET0."]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "UDF0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DGDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn dgdet0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DGDET0."]
    #[inline(always)]
    pub const fn set_dgdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ITRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC0."]
    #[inline(always)]
    pub const fn set_itrc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ATX0."]
    #[must_use]
    #[inline(always)]
    pub const fn atx0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ATX0."]
    #[inline(always)]
    pub const fn set_atx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "MTR."]
    #[must_use]
    #[inline(always)]
    pub const fn mtr(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "MTR."]
    #[inline(always)]
    pub const fn set_mtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "TCU."]
    #[must_use]
    #[inline(always)]
    pub const fn tcu(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TCU."]
    #[inline(always)]
    pub const fn set_tcu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbPr4 {
    #[inline(always)]
    fn default() -> GlbPr4 {
        GlbPr4(0)
    }
}
impl core::fmt::Debug for GlbPr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbPr4")
            .field("seccon", &self.seccon())
            .field("glikey0", &self.glikey0())
            .field("tdet0", &self.tdet0())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("dgdet0", &self.dgdet0())
            .field("itrc0", &self.itrc0())
            .field("atx0", &self.atx0())
            .field("mtr", &self.mtr())
            .field("tcu", &self.tcu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbPr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbPr4 {{ seccon: {=bool:?}, glikey0: {=bool:?}, tdet0: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, dgdet0: {=bool:?}, itrc0: {=bool:?}, atx0: {=bool:?}, mtr: {=bool:?}, tcu: {=bool:?} }}",
            self.seccon(),
            self.glikey0(),
            self.tdet0(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.dgdet0(),
            self.itrc0(),
            self.atx0(),
            self.mtr(),
            self.tcu()
        )
    }
}
#[doc = "Peripheral Reset Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRst0(pub u32);
impl GlbRst0 {
    #[doc = "INPUTMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "INPUTMUX0."]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPI2C4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C4."]
    #[inline(always)]
    pub const fn set_lpi2c4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GlbRst0 {
    #[inline(always)]
    fn default() -> GlbRst0 {
        GlbRst0(0)
    }
}
impl core::fmt::Debug for GlbRst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRst0")
            .field("inputmux0", &self.inputmux0())
            .field("freqme", &self.freqme())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("utick0", &self.utick0())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpi2c4", &self.lpi2c4())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("lpuart5", &self.lpuart5())
            .field("ostimer0", &self.ostimer0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst0 {{ inputmux0: {=bool:?}, freqme: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, utick0: {=bool:?}, dma0: {=bool:?}, dma1: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpi2c4: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, lpuart5: {=bool:?}, ostimer0: {=bool:?} }}",
            self.inputmux0(),
            self.freqme(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.utick0(),
            self.dma0(),
            self.dma1(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpi2c4(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.lpuart5(),
            self.ostimer0()
        )
    }
}
#[doc = "Peripheral Reset Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRst1(pub u32);
impl GlbRst1 {
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPSPI2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2."]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPSPI3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI3."]
    #[inline(always)]
    pub const fn set_lpspi3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LPSPI4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI4."]
    #[inline(always)]
    pub const fn set_lpspi4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPSPI5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi5(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI5."]
    #[inline(always)]
    pub const fn set_lpspi5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DAC1."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DAC1."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "VREF0."]
    #[must_use]
    #[inline(always)]
    pub const fn vref0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "VREF0."]
    #[inline(always)]
    pub const fn set_vref0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for GlbRst1 {
    #[inline(always)]
    fn default() -> GlbRst1 {
        GlbRst1(0)
    }
}
impl core::fmt::Debug for GlbRst1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRst1")
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpspi2", &self.lpspi2())
            .field("lpspi3", &self.lpspi3())
            .field("lpspi4", &self.lpspi4())
            .field("lpspi5", &self.lpspi5())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .field("vref0", &self.vref0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst1 {{ lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpspi2: {=bool:?}, lpspi3: {=bool:?}, lpspi4: {=bool:?}, lpspi5: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, dac0: {=bool:?}, dac1: {=bool:?}, vref0: {=bool:?} }}",
            self.lpspi0(),
            self.lpspi1(),
            self.lpspi2(),
            self.lpspi3(),
            self.lpspi4(),
            self.lpspi5(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.adc0(),
            self.adc1(),
            self.dac0(),
            self.dac1(),
            self.vref0()
        )
    }
}
#[doc = "Peripheral Reset Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRst2(pub u32);
impl GlbRst2 {
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C2."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C2."]
    #[inline(always)]
    pub const fn set_i3c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I3C3."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I3C3."]
    #[inline(always)]
    pub const fn set_i3c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "T1S0."]
    #[must_use]
    #[inline(always)]
    pub const fn t1s0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "T1S0."]
    #[inline(always)]
    pub const fn set_t1s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_flexspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPI0_FILTER."]
    #[must_use]
    #[inline(always)]
    pub const fn spi0_filter(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPI0_FILTER."]
    #[inline(always)]
    pub const fn set_spi0_filter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ESPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn espi0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ESPI0."]
    #[inline(always)]
    pub const fn set_espi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "USB1_PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "USB1_PHY."]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for GlbRst2 {
    #[inline(always)]
    fn default() -> GlbRst2 {
        GlbRst2(0)
    }
}
impl core::fmt::Debug for GlbRst2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRst2")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("i3c2", &self.i3c2())
            .field("i3c3", &self.i3c3())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("enet0", &self.enet0())
            .field("t1s0", &self.t1s0())
            .field("flexspi0", &self.flexspi0())
            .field("spi0_filter", &self.spi0_filter())
            .field("espi0", &self.espi0())
            .field("usb1", &self.usb1())
            .field("usb1_phy", &self.usb1_phy())
            .field("ewm0", &self.ewm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst2 {{ i3c0: {=bool:?}, i3c1: {=bool:?}, i3c2: {=bool:?}, i3c3: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, enet0: {=bool:?}, t1s0: {=bool:?}, flexspi0: {=bool:?}, spi0_filter: {=bool:?}, espi0: {=bool:?}, usb1: {=bool:?}, usb1_phy: {=bool:?}, ewm0: {=bool:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.i3c2(),
            self.i3c3(),
            self.flexcan0(),
            self.flexcan1(),
            self.enet0(),
            self.t1s0(),
            self.flexspi0(),
            self.spi0_filter(),
            self.espi0(),
            self.usb1(),
            self.usb1_phy(),
            self.ewm0()
        )
    }
}
#[doc = "Peripheral Reset Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRst3(pub u32);
impl GlbRst3 {
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SMARTDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0."]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for GlbRst3 {
    #[inline(always)]
    fn default() -> GlbRst3 {
        GlbRst3(0)
    }
}
impl core::fmt::Debug for GlbRst3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRst3")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("smartdma0", &self.smartdma0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst3 {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, smartdma0: {=bool:?} }}",
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.smartdma0()
        )
    }
}
#[doc = "Peripheral Reset Control 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRst4(pub u32);
impl GlbRst4 {
    #[doc = "GLIKEY0."]
    #[must_use]
    #[inline(always)]
    pub const fn glikey0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GLIKEY0."]
    #[inline(always)]
    pub const fn set_glikey0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DGDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn dgdet0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DGDET0."]
    #[inline(always)]
    pub const fn set_dgdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ATX0."]
    #[must_use]
    #[inline(always)]
    pub const fn atx0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ATX0."]
    #[inline(always)]
    pub const fn set_atx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for GlbRst4 {
    #[inline(always)]
    fn default() -> GlbRst4 {
        GlbRst4(0)
    }
}
impl core::fmt::Debug for GlbRst4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRst4")
            .field("glikey0", &self.glikey0())
            .field("pkc0", &self.pkc0())
            .field("trng0", &self.trng0())
            .field("dgdet0", &self.dgdet0())
            .field("atx0", &self.atx0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst4 {{ glikey0: {=bool:?}, pkc0: {=bool:?}, trng0: {=bool:?}, dgdet0: {=bool:?}, atx0: {=bool:?} }}",
            self.glikey0(),
            self.pkc0(),
            self.trng0(),
            self.dgdet0(),
            self.atx0()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRstClr(pub u32);
impl GlbRstClr {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GlbRstClr {
    #[inline(always)]
    fn default() -> GlbRstClr {
        GlbRstClr(0)
    }
}
impl core::fmt::Debug for GlbRstClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRstClr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRstClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbRstClr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRstSet(pub u32);
impl GlbRstSet {
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in MRCC_GLB_RSTn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GlbRstSet {
    #[inline(always)]
    fn default() -> GlbRstSet {
        GlbRstSet(0)
    }
}
impl core::fmt::Debug for GlbRstSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlbRstSet")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRstSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GlbRstSet {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "I3C0_FCLK clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3cFclkClksel(pub u32);
impl I3cFclkClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> FclkClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        FclkClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: FclkClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3cFclkClksel {
    #[inline(always)]
    fn default() -> I3cFclkClksel {
        I3cFclkClksel(0)
    }
}
impl core::fmt::Debug for I3cFclkClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3cFclkClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3cFclkClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3cFclkClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2cClksel(pub u32);
impl Lpi2cClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Lpi2cClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        Lpi2cClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Lpi2cClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Lpi2cClksel {
    #[inline(always)]
    fn default() -> Lpi2cClksel {
        Lpi2cClksel(0)
    }
}
impl core::fmt::Debug for Lpi2cClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2cClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2cClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2cClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpspiClksel(pub u32);
impl LpspiClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> LpspiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        LpspiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: LpspiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for LpspiClksel {
    #[inline(always)]
    fn default() -> LpspiClksel {
        LpspiClksel(0)
    }
}
impl core::fmt::Debug for LpspiClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpspiClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpspiClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpspiClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPTMR0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LptmrClksel(pub u32);
impl LptmrClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> LptmrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        LptmrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: LptmrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for LptmrClksel {
    #[inline(always)]
    fn default() -> LptmrClksel {
        LptmrClksel(0)
    }
}
impl core::fmt::Debug for LptmrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LptmrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LptmrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LptmrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpuartClksel(pub u32);
impl LpuartClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> LpuartClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        LpuartClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: LpuartClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for LpuartClksel {
    #[inline(always)]
    fn default() -> LpuartClksel {
        LpuartClksel(0)
    }
}
impl core::fmt::Debug for LpuartClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpuartClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpuartClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpuartClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "OSTIMER0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OstimerClksel(pub u32);
impl OstimerClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> OstimerClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        OstimerClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: OstimerClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for OstimerClksel {
    #[inline(always)]
    fn default() -> OstimerClksel {
        OstimerClksel(0)
    }
}
impl core::fmt::Debug for OstimerClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OstimerClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OstimerClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OstimerClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "RMII clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmiiClksel(pub u32);
impl RmiiClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> RmiiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        RmiiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: RmiiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for RmiiClksel {
    #[inline(always)]
    fn default() -> RmiiClksel {
        RmiiClksel(0)
    }
}
impl core::fmt::Debug for RmiiClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RmiiClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RmiiClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RmiiClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "SYSTICK clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystickClksel(pub u32);
impl SystickClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> SystickClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        SystickClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: SystickClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SystickClksel {
    #[inline(always)]
    fn default() -> SystickClksel {
        SystickClksel(0)
    }
}
impl core::fmt::Debug for SystickClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystickClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystickClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SystickClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "T1S0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1sClksel(pub u32);
impl T1sClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> T1sClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        T1sClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: T1sClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for T1sClksel {
    #[inline(always)]
    fn default() -> T1sClksel {
        T1sClksel(0)
    }
}
impl core::fmt::Debug for T1sClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1sClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T1sClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T1sClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "TSI0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsiClksel(pub u32);
impl TsiClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> TsiClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        TsiClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: TsiClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for TsiClksel {
    #[inline(always)]
    fn default() -> TsiClksel {
        TsiClksel(0)
    }
}
impl core::fmt::Debug for TsiClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TsiClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TsiClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TsiClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "USB1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbClksel(pub u32);
impl UsbClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> UsbClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        UsbClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: UsbClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for UsbClksel {
    #[inline(always)]
    fn default() -> UsbClksel {
        UsbClksel(0)
    }
}
impl core::fmt::Debug for UsbClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "USB1_PHY clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbPhyClksel(pub u32);
impl UsbPhyClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> PhyClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        PhyClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: PhyClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for UsbPhyClksel {
    #[inline(always)]
    fn default() -> UsbPhyClksel {
        UsbPhyClksel(0)
    }
}
impl core::fmt::Debug for UsbPhyClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbPhyClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbPhyClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbPhyClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "WWDT1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WwdtClksel(pub u32);
impl WwdtClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> WwdtClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        WwdtClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: WwdtClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for WwdtClksel {
    #[inline(always)]
    fn default() -> WwdtClksel {
        WwdtClksel(0)
    }
}
impl core::fmt::Debug for WwdtClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WwdtClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WwdtClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WwdtClksel {{ mux: {:?} }}", self.mux())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootSircDiv = 0x0,
    #[doc = "FRO_HF_GATED."]
    I1ClkrootFircGated = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    #[doc = "USB_PLL_CLK."]
    I4ClkrootUsbpll = 0x04,
    #[doc = "CLK_1M."]
    I5Clkroot1m = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootSpllDiv = 0x06,
    _RESERVED_7 = 0x07,
}
impl AdcClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcClkselMux {
    #[inline(always)]
    fn from(val: u8) -> AdcClkselMux {
        AdcClkselMux::from_bits(val)
    }
}
impl From<AdcClkselMux> for u8 {
    #[inline(always)]
    fn from(val: AdcClkselMux) -> u8 {
        AdcClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkdivHalt {
        ClkdivHalt::from_bits(val)
    }
}
impl From<ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkdivHalt) -> u8 {
        ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkdivReset {
        ClkdivReset::from_bits(val)
    }
}
impl From<ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkdivReset) -> u8 {
        ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> ClkdivUnstab {
        ClkdivUnstab::from_bits(val)
    }
}
impl From<ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: ClkdivUnstab) -> u8 {
        ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutClkselMux {
    #[doc = "FRO_12M."]
    I0Clkroot12m = 0x0,
    #[doc = "FRO_HF_DIV."]
    I1ClkrootFircDiv = 0x01,
    #[doc = "CLK_IN."]
    I2ClkrootSosc = 0x02,
    #[doc = "LP_OSC."]
    I3ClkrootLposc = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "PLL1_CLK_DIV."]
    I5ClkrootSpllDiv = 0x05,
    #[doc = "SLOW_CLK."]
    I6ClkrootSlow = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkoutClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutClkselMux {
    #[inline(always)]
    fn from(val: u8) -> ClkoutClkselMux {
        ClkoutClkselMux::from_bits(val)
    }
}
impl From<ClkoutClkselMux> for u8 {
    #[inline(always)]
    fn from(val: ClkoutClkselMux) -> u8 {
        ClkoutClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootSircDiv = 0x0,
    #[doc = "FRO_HF_GATED."]
    I1ClkrootFircGated = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    #[doc = "LP_OSC."]
    I4ClkrootLposc = 0x04,
    #[doc = "CLK_1M."]
    I5Clkroot1m = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootSpllDiv = 0x06,
    _RESERVED_7 = 0x07,
}
impl CtimerClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerClkselMux {
    #[inline(always)]
    fn from(val: u8) -> CtimerClkselMux {
        CtimerClkselMux::from_bits(val)
    }
}
impl From<CtimerClkselMux> for u8 {
    #[inline(always)]
    fn from(val: CtimerClkselMux) -> u8 {
        CtimerClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl DacClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacClkselMux {
    #[inline(always)]
    fn from(val: u8) -> DacClkselMux {
        DacClkselMux::from_bits(val)
    }
}
impl From<DacClkselMux> for u8 {
    #[inline(always)]
    fn from(val: DacClkselMux) -> u8 {
        DacClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgTraceClkselMux {
    #[doc = "CPU_CLK."]
    I0ClkrootCpuAlias = 0x0,
    #[doc = "CLK_1M."]
    I1Clkroot1m = 0x01,
    #[doc = "CLK_16K."]
    I2Clkroot16k = 0x02,
    _RESERVED_3 = 0x03,
}
impl DbgTraceClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgTraceClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgTraceClkselMux {
    #[inline(always)]
    fn from(val: u8) -> DbgTraceClkselMux {
        DbgTraceClkselMux::from_bits(val)
    }
}
impl From<DbgTraceClkselMux> for u8 {
    #[inline(always)]
    fn from(val: DbgTraceClkselMux) -> u8 {
        DbgTraceClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum E158ClkselMux {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    #[doc = "ENET0_TX_CLK."]
    I4IppEnet0ClkTxI = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK."]
    I6ClkrootSpll = 0x06,
    _RESERVED_7 = 0x07,
}
impl E158ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> E158ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for E158ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> E158ClkselMux {
        E158ClkselMux::from_bits(val)
    }
}
impl From<E158ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: E158ClkselMux) -> u8 {
        E158ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EspiClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    I1ClkrootFircGated = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    #[doc = "USB_PLL_CLK."]
    I4ClkrootUsbpll = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootSpllDiv = 0x06,
    _RESERVED_7 = 0x07,
}
impl EspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> EspiClkselMux {
        EspiClkselMux::from_bits(val)
    }
}
impl From<EspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: EspiClkselMux) -> u8 {
        EspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FclkClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl FclkClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FclkClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FclkClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FclkClkselMux {
        FclkClkselMux::from_bits(val)
    }
}
impl From<FclkClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FclkClkselMux) -> u8 {
        FclkClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcanClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    I1ClkrootFircGated = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFircDiv = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    #[doc = "USB_PLL_CLK."]
    I4ClkrootUsbpll = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK."]
    I6ClkrootSpll = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexcanClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcanClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcanClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexcanClkselMux {
        FlexcanClkselMux::from_bits(val)
    }
}
impl From<FlexcanClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexcanClkselMux) -> u8 {
        FlexcanClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    I1ClkrootFunc1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexioClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexioClkselMux {
        FlexioClkselMux::from_bits(val)
    }
}
impl From<FlexioClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexioClkselMux) -> u8 {
        FlexioClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    I1ClkrootFircGated = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "USB_PFD_CLK."]
    I4ClkrootUsbpfd = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK."]
    I6ClkrootSpll = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexspiClkselMux {
        FlexspiClkselMux::from_bits(val)
    }
}
impl From<FlexspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexspiClkselMux) -> u8 {
        FlexspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lpi2cClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkselMux {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkselMux {
        Lpi2cClkselMux::from_bits(val)
    }
}
impl From<Lpi2cClkselMux> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkselMux) -> u8 {
        Lpi2cClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LpspiClkselMux {
        LpspiClkselMux::from_bits(val)
    }
}
impl From<LpspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LpspiClkselMux) -> u8 {
        LpspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LptmrClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LptmrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LptmrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LptmrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LptmrClkselMux {
        LptmrClkselMux::from_bits(val)
    }
}
impl From<LptmrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LptmrClkselMux) -> u8 {
        LptmrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpuartClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootSircDiv = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFircDiv = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    #[doc = "LP_OSC."]
    I4ClkrootLposc = 0x04,
    #[doc = "CLK_1M."]
    I5Clkroot1m = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootSpllDiv = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpuartClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpuartClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpuartClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LpuartClkselMux {
        LpuartClkselMux::from_bits(val)
    }
}
impl From<LpuartClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LpuartClkselMux) -> u8 {
        LpuartClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerClkselMux {
    #[doc = "CLK_16K."]
    I0Clkroot16k = 0x0,
    #[doc = "CLK_32K."]
    I1Clkroot32k = 0x01,
    #[doc = "CLK_1M."]
    I2Clkroot1m = 0x02,
    _RESERVED_3 = 0x03,
}
impl OstimerClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerClkselMux {
    #[inline(always)]
    fn from(val: u8) -> OstimerClkselMux {
        OstimerClkselMux::from_bits(val)
    }
}
impl From<OstimerClkselMux> for u8 {
    #[inline(always)]
    fn from(val: OstimerClkselMux) -> u8 {
        OstimerClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    I1ClkrootFircGated = 0x01,
    #[doc = "CLK_IN."]
    I2ClkrootSosc = 0x02,
    _RESERVED_3 = 0x03,
}
impl PhyClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyClkselMux {
    #[inline(always)]
    fn from(val: u8) -> PhyClkselMux {
        PhyClkselMux::from_bits(val)
    }
}
impl From<PhyClkselMux> for u8 {
    #[inline(always)]
    fn from(val: PhyClkselMux) -> u8 {
        PhyClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiClkselMux {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK."]
    I6ClkrootSpll = 0x06,
    _RESERVED_7 = 0x07,
}
impl RmiiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmiiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmiiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> RmiiClkselMux {
        RmiiClkselMux::from_bits(val)
    }
}
impl From<RmiiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: RmiiClkselMux) -> u8 {
        RmiiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> RrClkselMux {
        RrClkselMux::from_bits(val)
    }
}
impl From<RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: RrClkselMux) -> u8 {
        RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystickClkselMux {
    #[doc = "CPU_CLK."]
    I0ClkrootCpu = 0x0,
    #[doc = "CLK_1M."]
    I1Clkroot1m = 0x01,
    #[doc = "CLK_16K."]
    I2Clkroot16k = 0x02,
    _RESERVED_3 = 0x03,
}
impl SystickClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystickClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystickClkselMux {
    #[inline(always)]
    fn from(val: u8) -> SystickClkselMux {
        SystickClkselMux::from_bits(val)
    }
}
impl From<SystickClkselMux> for u8 {
    #[inline(always)]
    fn from(val: SystickClkselMux) -> u8 {
        SystickClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum T1sClkselMux {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootSosc = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK."]
    I6ClkrootSpll = 0x06,
    _RESERVED_7 = 0x07,
}
impl T1sClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> T1sClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for T1sClkselMux {
    #[inline(always)]
    fn from(val: u8) -> T1sClkselMux {
        T1sClkselMux::from_bits(val)
    }
}
impl From<T1sClkselMux> for u8 {
    #[inline(always)]
    fn from(val: T1sClkselMux) -> u8 {
        T1sClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiClkselMux {
    #[doc = "FRO_LF_DIV."]
    I0ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    I2ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    I3ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    I5ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    I6ClkrootFunc6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TsiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> TsiClkselMux {
        TsiClkselMux::from_bits(val)
    }
}
impl From<TsiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: TsiClkselMux) -> u8 {
        TsiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbClkselMux {
    #[doc = "CLK_32K."]
    I0Clkroot32k = 0x0,
    #[doc = "CLK_1M."]
    I1Clkroot1m = 0x01,
    #[doc = "CLK__usbhs0_phy__clk_xtal."]
    I2ClkUsbhs0PhyClkXtal = 0x02,
    _RESERVED_3 = 0x03,
}
impl UsbClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbClkselMux {
    #[inline(always)]
    fn from(val: u8) -> UsbClkselMux {
        UsbClkselMux::from_bits(val)
    }
}
impl From<UsbClkselMux> for u8 {
    #[inline(always)]
    fn from(val: UsbClkselMux) -> u8 {
        UsbClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WwdtClkselMux {
    #[doc = "CLK_16K."]
    I0Clkroot16k = 0x0,
    #[doc = "FRO_HF_DIV."]
    I1ClkrootFircDiv = 0x01,
    #[doc = "CLK_1M."]
    I2Clkroot1m = 0x02,
    #[doc = "CLK_1M."]
    I3Clkroot1m = 0x03,
}
impl WwdtClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WwdtClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WwdtClkselMux {
    #[inline(always)]
    fn from(val: u8) -> WwdtClkselMux {
        WwdtClkselMux::from_bits(val)
    }
}
impl From<WwdtClkselMux> for u8 {
    #[inline(always)]
    fn from(val: WwdtClkselMux) -> u8 {
        WwdtClkselMux::to_bits(val)
    }
}
