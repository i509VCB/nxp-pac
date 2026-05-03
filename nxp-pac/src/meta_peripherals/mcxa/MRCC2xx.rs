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
    #[doc = "AHB Clock Control 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0(self) -> crate::pac::common::Reg<GlbCc0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "AHB Clock Control Set 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "AHB Clock Control 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1(self) -> crate::pac::common::Reg<GlbCc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "AHB Clock Control Set 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "AHB Clock Control 2."]
    #[inline(always)]
    pub const fn mrcc_glb_cc2(self) -> crate::pac::common::Reg<GlbCc2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "AHB Clock Control Set 2."]
    #[inline(always)]
    pub const fn mrcc_glb_cc2_set(
        self,
    ) -> crate::pac::common::Reg<GlbCcSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 2."]
    #[inline(always)]
    pub const fn mrcc_glb_cc2_clr(
        self,
    ) -> crate::pac::common::Reg<GlbCcClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 0."]
    #[inline(always)]
    pub const fn mrcc_glb_acc0(self) -> crate::pac::common::Reg<GlbAcc0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 1."]
    #[inline(always)]
    pub const fn mrcc_glb_acc1(self) -> crate::pac::common::Reg<GlbAcc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 2."]
    #[inline(always)]
    pub const fn mrcc_glb_acc2(self) -> crate::pac::common::Reg<GlbAcc2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "I3C0_FCLK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clksel(
        self,
    ) -> crate::pac::common::Reg<I3cFclkClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "I3C0_FCLK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "CTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "CTIMER0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "CTIMER1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "CTIMER1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "CTIMER2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "CTIMER2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "CTIMER3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "CTIMER3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "CTIMER4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clksel(
        self,
    ) -> crate::pac::common::Reg<CtimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "CTIMER4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "WWDT0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_wwdt0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "FLEXIO0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clksel(
        self,
    ) -> crate::pac::common::Reg<FlexioClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "FLEXIO0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "LPI2C0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "LPI2C0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "LPI2C1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "LPI2C1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "LPSPI0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "LPSPI0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "LPSPI1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clksel(
        self,
    ) -> crate::pac::common::Reg<LpspiClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "LPSPI1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "LPUART0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "LPUART0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "LPUART1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "LPUART1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "LPUART2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "LPUART2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "LPUART3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "LPUART3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "LPUART4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "LPUART4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "USB0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_usb0_clksel(
        self,
    ) -> crate::pac::common::Reg<UsbClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "USB0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_usb0_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "LPTMR0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clksel(
        self,
    ) -> crate::pac::common::Reg<LptmrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "LPTMR0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "OSTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ostimer0_clksel(
        self,
    ) -> crate::pac::common::Reg<OstimerClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ADCx clock selection control."]
    #[inline(always)]
    pub const fn mrcc_adc_clksel(
        self,
    ) -> crate::pac::common::Reg<AdcClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ADCx clock divider control."]
    #[inline(always)]
    pub const fn mrcc_adc_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "CMP0_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_func_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "CMP0_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clksel(
        self,
    ) -> crate::pac::common::Reg<CmpRrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "CMP0_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "CMP1_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_func_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "CMP1_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clksel(
        self,
    ) -> crate::pac::common::Reg<CmpRrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "CMP1_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "CMP2_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp2_func_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "CMP2_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp2_rr_clksel(
        self,
    ) -> crate::pac::common::Reg<CmpRrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "CMP2_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp2_rr_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "DAC0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clksel(
        self,
    ) -> crate::pac::common::Reg<DacClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "DAC0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clkdiv(self) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "FLEXCAN0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clksel(
        self,
    ) -> crate::pac::common::Reg<FlexcanClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "FLEXCAN0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "FLEXCAN1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexcan1_clksel(
        self,
    ) -> crate::pac::common::Reg<FlexcanClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "FLEXCAN1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexcan1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "LPI2C2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "LPI2C2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "LPI2C3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clksel(
        self,
    ) -> crate::pac::common::Reg<Lpi2cClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "LPI2C3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "LPUART5 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart5_clksel(
        self,
    ) -> crate::pac::common::Reg<LpuartClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "LPUART5 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart5_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "DBG_TRACE clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clksel(
        self,
    ) -> crate::pac::common::Reg<DbgTraceClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "DBG_TRACE clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "CLKOUT clock selection control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clksel(
        self,
    ) -> crate::pac::common::Reg<ClkoutClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "CLKOUT clock divider control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "SYSTICK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_systick_clksel(
        self,
    ) -> crate::pac::common::Reg<SystickClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "SYSTICK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_systick_clkdiv(
        self,
    ) -> crate::pac::common::Reg<Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
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
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
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
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SMARTDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0."]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
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
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
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
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("fmc", &self.fmc())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.wwdt0(),
            self.smartdma0(),
            self.dma0(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.fmc(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0()
        )
    }
}
#[doc = "Control Automatic Clock Gating 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbAcc1(pub u32);
impl GlbAcc1 {
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2."]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1."]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2."]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3."]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0."]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0."]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "UDF0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ADC2."]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2."]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3."]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3."]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "Control Automatic Clock Gating 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbAcc2(pub u32);
impl GlbAcc2 {
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMC."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC."]
    #[inline(always)]
    pub const fn set_ramc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0."]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0."]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ROMC."]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("ramc", &self.ramc())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbAcc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbAcc2 {{ rama: {=bool:?}, ramb: {=bool:?}, ramc: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?}, romc: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.ramc(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0(),
            self.romc()
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
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
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
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SMARTDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0."]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
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
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
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
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("fmc", &self.fmc())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.wwdt0(),
            self.smartdma0(),
            self.dma0(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.fmc(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0()
        )
    }
}
#[doc = "AHB Clock Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCc1(pub u32);
impl GlbCc1 {
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2."]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1."]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2."]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3."]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0."]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0."]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TDET0."]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "UDF0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "UDF0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ADC2."]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2."]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3."]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3."]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("tdet0", &self.tdet0())
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, tdet0: {=bool:?}, pkc0: {=bool:?}, sgi0: {=bool:?}, trng0: {=bool:?}, udf0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.tdet0(),
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "AHB Clock Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbCc2(pub u32);
impl GlbCc2 {
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMC."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC."]
    #[inline(always)]
    pub const fn set_ramc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0."]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0."]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ROMC."]
    #[must_use]
    #[inline(always)]
    pub const fn romc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("ramc", &self.ramc())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbCc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbCc2 {{ rama: {=bool:?}, ramb: {=bool:?}, ramc: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?}, romc: {=bool:?} }}",
            self.rama(),
            self.ramb(),
            self.ramc(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0(),
            self.romc()
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
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
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
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SMARTDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SMARTDMA0."]
    #[inline(always)]
    pub const fn set_smartdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
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
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO0."]
    #[inline(always)]
    pub const fn set_flexio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
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
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("smartdma0", &self.smartdma0())
            .field("dma0", &self.dma0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("eim0", &self.eim0())
            .field("erm0", &self.erm0())
            .field("aoi1", &self.aoi1())
            .field("flexio0", &self.flexio0())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpuart0", &self.lpuart0())
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb0", &self.usb0())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("flexpwm0", &self.flexpwm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, smartdma0: {=bool:?}, dma0: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.smartdma0(),
            self.dma0(),
            self.aoi0(),
            self.crc0(),
            self.eim0(),
            self.erm0(),
            self.aoi1(),
            self.flexio0(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpuart0(),
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb0(),
            self.qdc0(),
            self.qdc1(),
            self.flexpwm0()
        )
    }
}
#[doc = "Peripheral Reset Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRst1(pub u32);
impl GlbRst1 {
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CMP2."]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OPAMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP1."]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OPAMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP2."]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OPAMP3."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP3."]
    #[inline(always)]
    pub const fn set_opamp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SLCD0."]
    #[must_use]
    #[inline(always)]
    pub const fn slcd0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SLCD0."]
    #[inline(always)]
    pub const fn set_slcd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC2."]
    #[must_use]
    #[inline(always)]
    pub const fn adc2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC2."]
    #[inline(always)]
    pub const fn set_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ADC3."]
    #[must_use]
    #[inline(always)]
    pub const fn adc3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3."]
    #[inline(always)]
    pub const fn set_adc3(&mut self, val: bool) {
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
            .field("flexpwm1", &self.flexpwm1())
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("opamp3", &self.opamp3())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("slcd0", &self.slcd0())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpuart5", &self.lpuart5())
            .field("pkc0", &self.pkc0())
            .field("trng0", &self.trng0())
            .field("adc2", &self.adc2())
            .field("adc3", &self.adc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst1 {{ flexpwm1: {=bool:?}, ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, opamp3: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, slcd0: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, lpuart5: {=bool:?}, pkc0: {=bool:?}, trng0: {=bool:?}, adc2: {=bool:?}, adc3: {=bool:?} }}",
            self.flexpwm1(),
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.opamp3(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.slcd0(),
            self.flexcan0(),
            self.flexcan1(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpuart5(),
            self.pkc0(),
            self.trng0(),
            self.adc2(),
            self.adc3()
        )
    }
}
#[doc = "Peripheral Reset Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlbRst2(pub u32);
impl GlbRst2 {
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MAU0."]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MAU0."]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("mau0", &self.mau0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlbRst2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlbRst2 {{ gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, mau0: {=bool:?} }}",
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.mau0()
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
#[doc = "USB0 clock selection control."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClkselMux {
    #[doc = "FRO_LF_DIV."]
    ClkrootFunc0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    ClkrootFunc1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    Clkroot12m = 0x0,
    #[doc = "FRO_HF_DIV."]
    ClkrootFircDiv = 0x01,
    #[doc = "CLK_IN."]
    ClkrootSosc = 0x02,
    #[doc = "CLK_16K."]
    Clkroot16k = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "PLL1_CLK."]
    ClkrootSpll = 0x05,
    #[doc = "SLOW_CLK."]
    ClkrootSlow = 0x06,
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
    ClkrootFunc0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    ClkrootFunc1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    #[doc = "CLK_16K."]
    ClkrootFunc4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    ClkrootCpu = 0x0,
    #[doc = "CLK_1M."]
    Clkroot1m = 0x01,
    #[doc = "CLK_16K."]
    Clkroot16k = 0x02,
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
pub enum FclkClkselMux {
    #[doc = "FRO_LF_DIV."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    ClkrootFircGated = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFircDiv = 0x02,
    #[doc = "CLK_IN."]
    ClkrootSosc = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK."]
    ClkrootSpll = 0x06,
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
    ClkrootFunc0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    ClkrootFunc1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
pub enum Lpi2cClkselMux {
    #[doc = "FRO_LF_DIV."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    #[doc = "CLK_16K."]
    ClkrootFunc4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    Clkroot16k = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "CLK_1M."]
    Clkroot1m = 0x02,
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
pub enum RrClkselMux {
    #[doc = "FRO_LF_DIV."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    #[doc = "PLL1_CLK_DIV."]
    ClkrootFunc6 = 0x06,
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
    ClkrootCpu = 0x0,
    #[doc = "CLK_1M."]
    Clkroot1m = 0x01,
    #[doc = "CLK_16K."]
    Clkroot16k = 0x02,
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
pub enum UsbClkselMux {
    #[doc = "PLL1_CLK."]
    ClkrootSpll = 0x0,
    #[doc = "CLK_48M."]
    ScgScgFirc48mhzClk = 0x01,
    #[doc = "CLK_IN."]
    ClkrootSosc = 0x02,
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
