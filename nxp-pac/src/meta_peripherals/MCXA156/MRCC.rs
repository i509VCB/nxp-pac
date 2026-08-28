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
    pub const fn mrcc_glb_rst0(
        self,
    ) -> crate::pac::common::Reg<MrccGlbRst0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 0."]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_set(
        self,
    ) -> crate::pac::common::Reg<MrccGlbRst0Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 0."]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_clr(
        self,
    ) -> crate::pac::common::Reg<MrccGlbRst0Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Peripheral Reset Control 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1(
        self,
    ) -> crate::pac::common::Reg<MrccGlbRst1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_set(
        self,
    ) -> crate::pac::common::Reg<MrccGlbRst1Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_clr(
        self,
    ) -> crate::pac::common::Reg<MrccGlbRst1Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "AHB Clock Control 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0(self) -> crate::pac::common::Reg<MrccGlbCc0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "AHB Clock Control Set 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_set(
        self,
    ) -> crate::pac::common::Reg<MrccGlbCc0Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_clr(
        self,
    ) -> crate::pac::common::Reg<MrccGlbCc0Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "AHB Clock Control 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1(self) -> crate::pac::common::Reg<MrccGlbCc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "AHB Clock Control Set 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_set(
        self,
    ) -> crate::pac::common::Reg<MrccGlbCc1Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_clr(
        self,
    ) -> crate::pac::common::Reg<MrccGlbCc1Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 0."]
    #[inline(always)]
    pub const fn mrcc_glb_acc0(
        self,
    ) -> crate::pac::common::Reg<MrccGlbAcc0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 1."]
    #[inline(always)]
    pub const fn mrcc_glb_acc1(
        self,
    ) -> crate::pac::common::Reg<MrccGlbAcc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "I3C0_FCLK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccI3c0FclkClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "I3C0_FCLK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccI3c0FclkClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "CTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "CTIMER0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "CTIMER1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer1Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "CTIMER1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer1Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "CTIMER2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer2Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "CTIMER2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer2Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "CTIMER3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer3Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "CTIMER3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer3Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "CTIMER4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer4Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "CTIMER4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCtimer4Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "WWDT0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_wwdt0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccWwdt0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "FLEXIO0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccFlexio0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "FLEXIO0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccFlexio0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "LPI2C0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "LPI2C0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "LPI2C1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c1Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "LPI2C1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c1Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "LPSPI0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpspi0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "LPSPI0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpspi0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "LPSPI1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpspi1Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "LPSPI1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpspi1Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "LPUART0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "LPUART0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "LPUART1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart1Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "LPUART1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart1Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "LPUART2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart2Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "LPUART2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart2Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "LPUART3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart3Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "LPUART3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart3Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "LPUART4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart4Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "LPUART4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpuart4Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "USB0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_usb0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccUsb0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "LPTMR0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLptmr0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "LPTMR0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLptmr0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "OSTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ostimer0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccOstimer0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ADC0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_adc0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccAdc0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ADC0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_adc0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccAdc0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "ADC1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_adc1_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccAdc1Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "ADC1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_adc1_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccAdc1Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "CMP0_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_func_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCmp0FuncClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "CMP0_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccCmp0RrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "CMP0_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCmp0RrClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "CMP1_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_func_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCmp1FuncClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "CMP1_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccCmp1RrClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "CMP1_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccCmp1RrClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "DAC0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccDac0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "DAC0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccDac0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "FLEXCAN0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccFlexcan0Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "FLEXCAN0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccFlexcan0Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "LPI2C2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c2Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "LPI2C2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c2Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "LPI2C3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c3Clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "LPI2C3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccLpi2c3Clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "DBG_TRACE clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccDbgTraceClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "DBG_TRACE clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccDbgTraceClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "CLKOUT clock selection control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccClkoutClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "CLKOUT clock divider control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccClkoutClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "SYSTICK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_systick_clksel(
        self,
    ) -> crate::pac::common::Reg<MrccSystickClksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "SYSTICK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_systick_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccSystickClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "FRO_HF_DIV clock divider control."]
    #[inline(always)]
    pub const fn mrcc_fro_hf_div_clkdiv(
        self,
    ) -> crate::pac::common::Reg<MrccFroHfDivClkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
}
#[doc = "ADC0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc0Clkdiv(pub u32);
impl MrccAdc0Clkdiv {
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
    pub const fn reset(&self) -> MrccAdc0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccAdc0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccAdc0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccAdc0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccAdc0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccAdc0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccAdc0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccAdc0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccAdc0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccAdc0Clkdiv {
    #[inline(always)]
    fn default() -> MrccAdc0Clkdiv {
        MrccAdc0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccAdc0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccAdc0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc0Clksel(pub u32);
impl MrccAdc0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccAdc0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccAdc0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccAdc0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccAdc0Clksel {
    #[inline(always)]
    fn default() -> MrccAdc0Clksel {
        MrccAdc0Clksel(0)
    }
}
impl core::fmt::Debug for MrccAdc0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccAdc0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "ADC1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc1Clkdiv(pub u32);
impl MrccAdc1Clkdiv {
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
    pub const fn reset(&self) -> MrccAdc1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccAdc1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccAdc1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccAdc1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccAdc1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccAdc1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccAdc1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccAdc1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccAdc1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccAdc1Clkdiv {
    #[inline(always)]
    fn default() -> MrccAdc1Clkdiv {
        MrccAdc1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccAdc1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccAdc1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccAdc1Clksel(pub u32);
impl MrccAdc1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccAdc1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccAdc1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccAdc1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccAdc1Clksel {
    #[inline(always)]
    fn default() -> MrccAdc1Clksel {
        MrccAdc1Clksel(0)
    }
}
impl core::fmt::Debug for MrccAdc1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccAdc1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccAdc1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccAdc1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CLKOUT clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccClkoutClkdiv(pub u32);
impl MrccClkoutClkdiv {
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
    pub const fn reset(&self) -> MrccClkoutClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccClkoutClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccClkoutClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccClkoutClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccClkoutClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccClkoutClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccClkoutClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccClkoutClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccClkoutClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccClkoutClkdiv {
    #[inline(always)]
    fn default() -> MrccClkoutClkdiv {
        MrccClkoutClkdiv(0)
    }
}
impl core::fmt::Debug for MrccClkoutClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccClkoutClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccClkoutClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccClkoutClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
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
pub struct MrccClkoutClksel(pub u32);
impl MrccClkoutClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccClkoutClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccClkoutClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccClkoutClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccClkoutClksel {
    #[inline(always)]
    fn default() -> MrccClkoutClksel {
        MrccClkoutClksel(0)
    }
}
impl core::fmt::Debug for MrccClkoutClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccClkoutClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccClkoutClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccClkoutClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP0_FUNC clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0FuncClkdiv(pub u32);
impl MrccCmp0FuncClkdiv {
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
    pub const fn reset(&self) -> MrccCmp0FuncClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCmp0FuncClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCmp0FuncClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCmp0FuncClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCmp0FuncClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCmp0FuncClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCmp0FuncClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCmp0FuncClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCmp0FuncClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp0FuncClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp0FuncClkdiv {
        MrccCmp0FuncClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp0FuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0FuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0FuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp0FuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0RrClkdiv(pub u32);
impl MrccCmp0RrClkdiv {
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
    pub const fn reset(&self) -> MrccCmp0RrClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCmp0RrClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCmp0RrClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCmp0RrClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCmp0RrClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCmp0RrClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCmp0RrClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCmp0RrClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCmp0RrClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp0RrClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp0RrClkdiv {
        MrccCmp0RrClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp0RrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0RrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0RrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp0RrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0_RR clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp0RrClksel(pub u32);
impl MrccCmp0RrClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccCmp0RrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccCmp0RrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccCmp0RrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCmp0RrClksel {
    #[inline(always)]
    fn default() -> MrccCmp0RrClksel {
        MrccCmp0RrClksel(0)
    }
}
impl core::fmt::Debug for MrccCmp0RrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp0RrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp0RrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCmp0RrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CMP1_FUNC clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1FuncClkdiv(pub u32);
impl MrccCmp1FuncClkdiv {
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
    pub const fn reset(&self) -> MrccCmp1FuncClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCmp1FuncClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCmp1FuncClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCmp1FuncClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCmp1FuncClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCmp1FuncClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCmp1FuncClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCmp1FuncClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCmp1FuncClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp1FuncClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp1FuncClkdiv {
        MrccCmp1FuncClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp1FuncClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1FuncClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1FuncClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp1FuncClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1_RR clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1RrClkdiv(pub u32);
impl MrccCmp1RrClkdiv {
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
    pub const fn reset(&self) -> MrccCmp1RrClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCmp1RrClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCmp1RrClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCmp1RrClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCmp1RrClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCmp1RrClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCmp1RrClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCmp1RrClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCmp1RrClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCmp1RrClkdiv {
    #[inline(always)]
    fn default() -> MrccCmp1RrClkdiv {
        MrccCmp1RrClkdiv(0)
    }
}
impl core::fmt::Debug for MrccCmp1RrClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1RrClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1RrClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCmp1RrClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1_RR clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCmp1RrClksel(pub u32);
impl MrccCmp1RrClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccCmp1RrClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccCmp1RrClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccCmp1RrClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCmp1RrClksel {
    #[inline(always)]
    fn default() -> MrccCmp1RrClksel {
        MrccCmp1RrClksel(0)
    }
}
impl core::fmt::Debug for MrccCmp1RrClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCmp1RrClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCmp1RrClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCmp1RrClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer0Clkdiv(pub u32);
impl MrccCtimer0Clkdiv {
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
    pub const fn reset(&self) -> MrccCtimer0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCtimer0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCtimer0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCtimer0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCtimer0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCtimer0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCtimer0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCtimer0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCtimer0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer0Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer0Clkdiv {
        MrccCtimer0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer0Clksel(pub u32);
impl MrccCtimer0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccCtimer0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccCtimer0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccCtimer0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer0Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer0Clksel {
        MrccCtimer0Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer1Clkdiv(pub u32);
impl MrccCtimer1Clkdiv {
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
    pub const fn reset(&self) -> MrccCtimer1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCtimer1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCtimer1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCtimer1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCtimer1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCtimer1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCtimer1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCtimer1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCtimer1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer1Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer1Clkdiv {
        MrccCtimer1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer1Clksel(pub u32);
impl MrccCtimer1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccCtimer1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccCtimer1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccCtimer1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer1Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer1Clksel {
        MrccCtimer1Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER2 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer2Clkdiv(pub u32);
impl MrccCtimer2Clkdiv {
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
    pub const fn reset(&self) -> MrccCtimer2ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCtimer2ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCtimer2ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCtimer2ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCtimer2ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCtimer2ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCtimer2ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCtimer2ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCtimer2ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer2Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer2Clkdiv {
        MrccCtimer2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER2 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer2Clksel(pub u32);
impl MrccCtimer2Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccCtimer2ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccCtimer2ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccCtimer2ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer2Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer2Clksel {
        MrccCtimer2Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER3 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer3Clkdiv(pub u32);
impl MrccCtimer3Clkdiv {
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
    pub const fn reset(&self) -> MrccCtimer3ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCtimer3ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCtimer3ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCtimer3ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCtimer3ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCtimer3ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCtimer3ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCtimer3ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCtimer3ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer3Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer3Clkdiv {
        MrccCtimer3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER3 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer3Clksel(pub u32);
impl MrccCtimer3Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccCtimer3ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccCtimer3ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccCtimer3ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer3Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer3Clksel {
        MrccCtimer3Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "CTIMER4 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer4Clkdiv(pub u32);
impl MrccCtimer4Clkdiv {
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
    pub const fn reset(&self) -> MrccCtimer4ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccCtimer4ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccCtimer4ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccCtimer4ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccCtimer4ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccCtimer4ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccCtimer4ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccCtimer4ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccCtimer4ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccCtimer4Clkdiv {
    #[inline(always)]
    fn default() -> MrccCtimer4Clkdiv {
        MrccCtimer4Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccCtimer4Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer4Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer4Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccCtimer4Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER4 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccCtimer4Clksel(pub u32);
impl MrccCtimer4Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccCtimer4ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccCtimer4ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccCtimer4ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccCtimer4Clksel {
    #[inline(always)]
    fn default() -> MrccCtimer4Clksel {
        MrccCtimer4Clksel(0)
    }
}
impl core::fmt::Debug for MrccCtimer4Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccCtimer4Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccCtimer4Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccCtimer4Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DAC0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDac0Clkdiv(pub u32);
impl MrccDac0Clkdiv {
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
    pub const fn reset(&self) -> MrccDac0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccDac0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccDac0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccDac0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccDac0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccDac0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccDac0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccDac0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccDac0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccDac0Clkdiv {
    #[inline(always)]
    fn default() -> MrccDac0Clkdiv {
        MrccDac0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccDac0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDac0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDac0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccDac0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDac0Clksel(pub u32);
impl MrccDac0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccDac0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccDac0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccDac0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccDac0Clksel {
    #[inline(always)]
    fn default() -> MrccDac0Clksel {
        MrccDac0Clksel(0)
    }
}
impl core::fmt::Debug for MrccDac0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDac0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDac0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccDac0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "DBG_TRACE clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClkdiv(pub u32);
impl MrccDbgTraceClkdiv {
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
    pub const fn reset(&self) -> MrccDbgTraceClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccDbgTraceClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccDbgTraceClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccDbgTraceClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccDbgTraceClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccDbgTraceClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccDbgTraceClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccDbgTraceClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccDbgTraceClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccDbgTraceClkdiv {
    #[inline(always)]
    fn default() -> MrccDbgTraceClkdiv {
        MrccDbgTraceClkdiv(0)
    }
}
impl core::fmt::Debug for MrccDbgTraceClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDbgTraceClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDbgTraceClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccDbgTraceClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DBG_TRACE clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccDbgTraceClksel(pub u32);
impl MrccDbgTraceClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccDbgTraceClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        MrccDbgTraceClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccDbgTraceClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccDbgTraceClksel {
    #[inline(always)]
    fn default() -> MrccDbgTraceClksel {
        MrccDbgTraceClksel(0)
    }
}
impl core::fmt::Debug for MrccDbgTraceClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccDbgTraceClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccDbgTraceClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccDbgTraceClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXCAN0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan0Clkdiv(pub u32);
impl MrccFlexcan0Clkdiv {
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
    pub const fn reset(&self) -> MrccFlexcan0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccFlexcan0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccFlexcan0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccFlexcan0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccFlexcan0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccFlexcan0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccFlexcan0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccFlexcan0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccFlexcan0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFlexcan0Clkdiv {
    #[inline(always)]
    fn default() -> MrccFlexcan0Clkdiv {
        MrccFlexcan0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccFlexcan0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFlexcan0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexcan0Clksel(pub u32);
impl MrccFlexcan0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccFlexcan0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccFlexcan0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccFlexcan0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccFlexcan0Clksel {
    #[inline(always)]
    fn default() -> MrccFlexcan0Clksel {
        MrccFlexcan0Clksel(0)
    }
}
impl core::fmt::Debug for MrccFlexcan0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexcan0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexcan0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccFlexcan0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FLEXIO0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexio0Clkdiv(pub u32);
impl MrccFlexio0Clkdiv {
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
    pub const fn reset(&self) -> MrccFlexio0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccFlexio0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccFlexio0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccFlexio0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccFlexio0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccFlexio0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccFlexio0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccFlexio0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccFlexio0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFlexio0Clkdiv {
    #[inline(always)]
    fn default() -> MrccFlexio0Clkdiv {
        MrccFlexio0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccFlexio0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexio0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexio0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFlexio0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFlexio0Clksel(pub u32);
impl MrccFlexio0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccFlexio0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccFlexio0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccFlexio0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccFlexio0Clksel {
    #[inline(always)]
    fn default() -> MrccFlexio0Clksel {
        MrccFlexio0Clksel(0)
    }
}
impl core::fmt::Debug for MrccFlexio0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFlexio0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFlexio0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccFlexio0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "FRO_HF_DIV clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccFroHfDivClkdiv(pub u32);
impl MrccFroHfDivClkdiv {
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
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccFroHfDivClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccFroHfDivClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccFroHfDivClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccFroHfDivClkdiv {
    #[inline(always)]
    fn default() -> MrccFroHfDivClkdiv {
        MrccFroHfDivClkdiv(0)
    }
}
impl core::fmt::Debug for MrccFroHfDivClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccFroHfDivClkdiv")
            .field("div", &self.div())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccFroHfDivClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccFroHfDivClkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "Control Automatic Clock Gating 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc0(pub u32);
impl MrccGlbAcc0 {
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
    #[doc = "DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
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
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbAcc0 {
    #[inline(always)]
    fn default() -> MrccGlbAcc0 {
        MrccGlbAcc0(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc0")
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
            .field("dma", &self.dma())
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
            .field("flexpwm1", &self.flexpwm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, dma: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?}, flexpwm1: {=bool:?} }}",
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
            self.dma(),
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
            self.flexpwm0(),
            self.flexpwm1()
        )
    }
}
#[doc = "Control Automatic Clock Gating 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbAcc1(pub u32);
impl MrccGlbAcc1 {
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for MrccGlbAcc1 {
    #[inline(always)]
    fn default() -> MrccGlbAcc1 {
        MrccGlbAcc1(0)
    }
}
impl core::fmt::Debug for MrccGlbAcc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbAcc1")
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("flexcan0", &self.flexcan0())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbAcc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbAcc1 {{ ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, flexcan0: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, rama: {=bool:?}, ramb: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, romc: {=bool:?} }}",
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.dac0(),
            self.opamp0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.flexcan0(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.rama(),
            self.ramb(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.romc()
        )
    }
}
#[doc = "AHB Clock Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0(pub u32);
impl MrccGlbCc0 {
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
    #[doc = "DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
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
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbCc0 {
    #[inline(always)]
    fn default() -> MrccGlbCc0 {
        MrccGlbCc0(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0")
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
            .field("dma", &self.dma())
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
            .field("flexpwm1", &self.flexpwm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, wwdt0: {=bool:?}, dma: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, fmc: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?}, flexpwm1: {=bool:?} }}",
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
            self.dma(),
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
            self.flexpwm0(),
            self.flexpwm1()
        )
    }
}
#[doc = "AHB Clock Control Clear 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0Clr(pub u32);
impl MrccGlbCc0Clr {
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
impl Default for MrccGlbCc0Clr {
    #[inline(always)]
    fn default() -> MrccGlbCc0Clr {
        MrccGlbCc0Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc0Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc0Set(pub u32);
impl MrccGlbCc0Set {
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
impl Default for MrccGlbCc0Set {
    #[inline(always)]
    fn default() -> MrccGlbCc0Set {
        MrccGlbCc0Set(0)
    }
}
impl core::fmt::Debug for MrccGlbCc0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc0Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc0Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1(pub u32);
impl MrccGlbCc1 {
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RAMA."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAMB."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB."]
    #[inline(always)]
    pub const fn set_ramb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROMC."]
    #[inline(always)]
    pub const fn set_romc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for MrccGlbCc1 {
    #[inline(always)]
    fn default() -> MrccGlbCc1 {
        MrccGlbCc1(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1")
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("flexcan0", &self.flexcan0())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("rama", &self.rama())
            .field("ramb", &self.ramb())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("romc", &self.romc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbCc1 {{ ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, flexcan0: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, rama: {=bool:?}, ramb: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, romc: {=bool:?} }}",
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.dac0(),
            self.opamp0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.flexcan0(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.rama(),
            self.ramb(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.romc()
        )
    }
}
#[doc = "AHB Clock Control Clear 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1Clr(pub u32);
impl MrccGlbCc1Clr {
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
impl Default for MrccGlbCc1Clr {
    #[inline(always)]
    fn default() -> MrccGlbCc1Clr {
        MrccGlbCc1Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc1Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbCc1Set(pub u32);
impl MrccGlbCc1Set {
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
impl Default for MrccGlbCc1Set {
    #[inline(always)]
    fn default() -> MrccGlbCc1Set {
        MrccGlbCc1Set(0)
    }
}
impl core::fmt::Debug for MrccGlbCc1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbCc1Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbCc1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbCc1Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0(pub u32);
impl MrccGlbRst0 {
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
    #[doc = "DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA."]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn eim0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "EIM0."]
    #[inline(always)]
    pub const fn set_eim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "AOI1."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AOI1."]
    #[inline(always)]
    pub const fn set_aoi1(&mut self, val: bool) {
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
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXPWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXPWM1."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccGlbRst0 {
    #[inline(always)]
    fn default() -> MrccGlbRst0 {
        MrccGlbRst0(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0")
            .field("inputmux0", &self.inputmux0())
            .field("i3c0", &self.i3c0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick0", &self.utick0())
            .field("dma", &self.dma())
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
            .field("flexpwm1", &self.flexpwm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst0 {{ inputmux0: {=bool:?}, i3c0: {=bool:?}, ctimer0: {=bool:?}, ctimer1: {=bool:?}, ctimer2: {=bool:?}, ctimer3: {=bool:?}, ctimer4: {=bool:?}, freqme: {=bool:?}, utick0: {=bool:?}, dma: {=bool:?}, aoi0: {=bool:?}, crc0: {=bool:?}, eim0: {=bool:?}, erm0: {=bool:?}, aoi1: {=bool:?}, flexio0: {=bool:?}, lpi2c0: {=bool:?}, lpi2c1: {=bool:?}, lpspi0: {=bool:?}, lpspi1: {=bool:?}, lpuart0: {=bool:?}, lpuart1: {=bool:?}, lpuart2: {=bool:?}, lpuart3: {=bool:?}, lpuart4: {=bool:?}, usb0: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, flexpwm0: {=bool:?}, flexpwm1: {=bool:?} }}",
            self.inputmux0(),
            self.i3c0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3(),
            self.ctimer4(),
            self.freqme(),
            self.utick0(),
            self.dma(),
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
            self.flexpwm0(),
            self.flexpwm1()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0Clr(pub u32);
impl MrccGlbRst0Clr {
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
impl Default for MrccGlbRst0Clr {
    #[inline(always)]
    fn default() -> MrccGlbRst0Clr {
        MrccGlbRst0Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst0Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst0Set(pub u32);
impl MrccGlbRst0Set {
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
impl Default for MrccGlbRst0Set {
    #[inline(always)]
    fn default() -> MrccGlbRst0Set {
        MrccGlbRst0Set(0)
    }
}
impl core::fmt::Debug for MrccGlbRst0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst0Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst0Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1(pub u32);
impl MrccGlbRst1 {
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CMP1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMP1."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OPAMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP0."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
}
impl Default for MrccGlbRst1 {
    #[inline(always)]
    fn default() -> MrccGlbRst1 {
        MrccGlbRst1(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1")
            .field("ostimer0", &self.ostimer0())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("cmp1", &self.cmp1())
            .field("dac0", &self.dac0())
            .field("opamp0", &self.opamp0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("flexcan0", &self.flexcan0())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccGlbRst1 {{ ostimer0: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, cmp1: {=bool:?}, dac0: {=bool:?}, opamp0: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, flexcan0: {=bool:?}, lpi2c2: {=bool:?}, lpi2c3: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?} }}",
            self.ostimer0(),
            self.adc0(),
            self.adc1(),
            self.cmp1(),
            self.dac0(),
            self.opamp0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.flexcan0(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4()
        )
    }
}
#[doc = "Peripheral Reset Control Clear 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1Clr(pub u32);
impl MrccGlbRst1Clr {
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
impl Default for MrccGlbRst1Clr {
    #[inline(always)]
    fn default() -> MrccGlbRst1Clr {
        MrccGlbRst1Clr(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1Clr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst1Clr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccGlbRst1Set(pub u32);
impl MrccGlbRst1Set {
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
impl Default for MrccGlbRst1Set {
    #[inline(always)]
    fn default() -> MrccGlbRst1Set {
        MrccGlbRst1Set(0)
    }
}
impl core::fmt::Debug for MrccGlbRst1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccGlbRst1Set")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccGlbRst1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccGlbRst1Set {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "I3C0_FCLK clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccI3c0FclkClkdiv(pub u32);
impl MrccI3c0FclkClkdiv {
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
    pub const fn reset(&self) -> MrccI3c0FclkClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccI3c0FclkClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccI3c0FclkClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccI3c0FclkClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccI3c0FclkClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccI3c0FclkClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccI3c0FclkClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccI3c0FclkClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccI3c0FclkClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccI3c0FclkClkdiv {
    #[inline(always)]
    fn default() -> MrccI3c0FclkClkdiv {
        MrccI3c0FclkClkdiv(0)
    }
}
impl core::fmt::Debug for MrccI3c0FclkClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccI3c0FclkClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccI3c0FclkClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccI3c0FclkClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0_FCLK clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccI3c0FclkClksel(pub u32);
impl MrccI3c0FclkClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccI3c0FclkClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccI3c0FclkClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccI3c0FclkClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccI3c0FclkClksel {
    #[inline(always)]
    fn default() -> MrccI3c0FclkClksel {
        MrccI3c0FclkClksel(0)
    }
}
impl core::fmt::Debug for MrccI3c0FclkClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccI3c0FclkClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccI3c0FclkClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccI3c0FclkClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c0Clkdiv(pub u32);
impl MrccLpi2c0Clkdiv {
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
    pub const fn reset(&self) -> MrccLpi2c0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpi2c0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpi2c0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpi2c0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpi2c0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpi2c0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpi2c0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpi2c0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpi2c0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c0Clkdiv {
        MrccLpi2c0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c0Clksel(pub u32);
impl MrccLpi2c0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpi2c0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpi2c0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpi2c0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c0Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c0Clksel {
        MrccLpi2c0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c1Clkdiv(pub u32);
impl MrccLpi2c1Clkdiv {
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
    pub const fn reset(&self) -> MrccLpi2c1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpi2c1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpi2c1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpi2c1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpi2c1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpi2c1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpi2c1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpi2c1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpi2c1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c1Clkdiv {
        MrccLpi2c1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c1Clksel(pub u32);
impl MrccLpi2c1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpi2c1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpi2c1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpi2c1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c1Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c1Clksel {
        MrccLpi2c1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C2 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c2Clkdiv(pub u32);
impl MrccLpi2c2Clkdiv {
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
    pub const fn reset(&self) -> MrccLpi2c2ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpi2c2ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpi2c2ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpi2c2ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpi2c2ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpi2c2ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpi2c2ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpi2c2ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpi2c2ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c2Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c2Clkdiv {
        MrccLpi2c2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C2 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c2Clksel(pub u32);
impl MrccLpi2c2Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpi2c2ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpi2c2ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpi2c2ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c2Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c2Clksel {
        MrccLpi2c2Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPI2C3 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c3Clkdiv(pub u32);
impl MrccLpi2c3Clkdiv {
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
    pub const fn reset(&self) -> MrccLpi2c3ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpi2c3ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpi2c3ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpi2c3ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpi2c3ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpi2c3ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpi2c3ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpi2c3ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpi2c3ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpi2c3Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpi2c3Clkdiv {
        MrccLpi2c3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpi2c3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpi2c3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPI2C3 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpi2c3Clksel(pub u32);
impl MrccLpi2c3Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpi2c3ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpi2c3ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpi2c3ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpi2c3Clksel {
    #[inline(always)]
    fn default() -> MrccLpi2c3Clksel {
        MrccLpi2c3Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpi2c3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpi2c3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpi2c3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpi2c3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi0Clkdiv(pub u32);
impl MrccLpspi0Clkdiv {
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
    pub const fn reset(&self) -> MrccLpspi0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpspi0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpspi0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpspi0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpspi0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpspi0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpspi0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpspi0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpspi0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpspi0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpspi0Clkdiv {
        MrccLpspi0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpspi0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpspi0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPSPI0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi0Clksel(pub u32);
impl MrccLpspi0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpspi0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpspi0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpspi0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpspi0Clksel {
    #[inline(always)]
    fn default() -> MrccLpspi0Clksel {
        MrccLpspi0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpspi0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpspi0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPSPI1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi1Clkdiv(pub u32);
impl MrccLpspi1Clkdiv {
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
    pub const fn reset(&self) -> MrccLpspi1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpspi1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpspi1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpspi1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpspi1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpspi1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpspi1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpspi1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpspi1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpspi1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpspi1Clkdiv {
        MrccLpspi1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpspi1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpspi1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPSPI1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpspi1Clksel(pub u32);
impl MrccLpspi1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpspi1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpspi1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpspi1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpspi1Clksel {
    #[inline(always)]
    fn default() -> MrccLpspi1Clksel {
        MrccLpspi1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpspi1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpspi1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpspi1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpspi1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPTMR0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLptmr0Clkdiv(pub u32);
impl MrccLptmr0Clkdiv {
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
    pub const fn reset(&self) -> MrccLptmr0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLptmr0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLptmr0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLptmr0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLptmr0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLptmr0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLptmr0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLptmr0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLptmr0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLptmr0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLptmr0Clkdiv {
        MrccLptmr0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLptmr0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLptmr0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLptmr0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLptmr0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPTMR0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLptmr0Clksel(pub u32);
impl MrccLptmr0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLptmr0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLptmr0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLptmr0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLptmr0Clksel {
    #[inline(always)]
    fn default() -> MrccLptmr0Clksel {
        MrccLptmr0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLptmr0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLptmr0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLptmr0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLptmr0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart0Clkdiv(pub u32);
impl MrccLpuart0Clkdiv {
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
    pub const fn reset(&self) -> MrccLpuart0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpuart0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpuart0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpuart0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpuart0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpuart0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpuart0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpuart0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpuart0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart0Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart0Clkdiv {
        MrccLpuart0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart0Clksel(pub u32);
impl MrccLpuart0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpuart0ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpuart0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpuart0ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart0Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart0Clksel {
        MrccLpuart0Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART1 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart1Clkdiv(pub u32);
impl MrccLpuart1Clkdiv {
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
    pub const fn reset(&self) -> MrccLpuart1ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpuart1ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpuart1ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpuart1ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpuart1ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpuart1ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpuart1ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpuart1ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpuart1ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart1Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart1Clkdiv {
        MrccLpuart1Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart1Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart1Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart1Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart1Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART1 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart1Clksel(pub u32);
impl MrccLpuart1Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpuart1ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpuart1ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpuart1ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart1Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart1Clksel {
        MrccLpuart1Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart1Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart1Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart1Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart1Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART2 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart2Clkdiv(pub u32);
impl MrccLpuart2Clkdiv {
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
    pub const fn reset(&self) -> MrccLpuart2ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpuart2ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpuart2ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpuart2ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpuart2ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpuart2ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpuart2ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpuart2ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpuart2ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart2Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart2Clkdiv {
        MrccLpuart2Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart2Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart2Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart2Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart2Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART2 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart2Clksel(pub u32);
impl MrccLpuart2Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpuart2ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpuart2ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpuart2ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart2Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart2Clksel {
        MrccLpuart2Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart2Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart2Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart2Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart2Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART3 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart3Clkdiv(pub u32);
impl MrccLpuart3Clkdiv {
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
    pub const fn reset(&self) -> MrccLpuart3ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpuart3ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpuart3ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpuart3ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpuart3ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpuart3ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpuart3ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpuart3ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpuart3ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart3Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart3Clkdiv {
        MrccLpuart3Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart3Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart3Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart3Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart3Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART3 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart3Clksel(pub u32);
impl MrccLpuart3Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpuart3ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpuart3ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpuart3ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart3Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart3Clksel {
        MrccLpuart3Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart3Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart3Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart3Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart3Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "LPUART4 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart4Clkdiv(pub u32);
impl MrccLpuart4Clkdiv {
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
    pub const fn reset(&self) -> MrccLpuart4ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccLpuart4ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccLpuart4ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccLpuart4ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccLpuart4ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccLpuart4ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccLpuart4ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccLpuart4ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccLpuart4ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccLpuart4Clkdiv {
    #[inline(always)]
    fn default() -> MrccLpuart4Clkdiv {
        MrccLpuart4Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccLpuart4Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart4Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart4Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccLpuart4Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "LPUART4 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccLpuart4Clksel(pub u32);
impl MrccLpuart4Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccLpuart4ClkselMux {
        let val = (self.0 >> 0usize) & 0x07;
        MrccLpuart4ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccLpuart4ClkselMux) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MrccLpuart4Clksel {
    #[inline(always)]
    fn default() -> MrccLpuart4Clksel {
        MrccLpuart4Clksel(0)
    }
}
impl core::fmt::Debug for MrccLpuart4Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccLpuart4Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccLpuart4Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccLpuart4Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "OSTIMER0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccOstimer0Clksel(pub u32);
impl MrccOstimer0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccOstimer0ClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        MrccOstimer0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccOstimer0ClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccOstimer0Clksel {
    #[inline(always)]
    fn default() -> MrccOstimer0Clksel {
        MrccOstimer0Clksel(0)
    }
}
impl core::fmt::Debug for MrccOstimer0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccOstimer0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccOstimer0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccOstimer0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "SYSTICK clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClkdiv(pub u32);
impl MrccSystickClkdiv {
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
    pub const fn reset(&self) -> MrccSystickClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccSystickClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccSystickClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccSystickClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccSystickClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccSystickClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccSystickClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccSystickClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccSystickClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccSystickClkdiv {
    #[inline(always)]
    fn default() -> MrccSystickClkdiv {
        MrccSystickClkdiv(0)
    }
}
impl core::fmt::Debug for MrccSystickClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccSystickClkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccSystickClkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccSystickClkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SYSTICK clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccSystickClksel(pub u32);
impl MrccSystickClksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccSystickClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        MrccSystickClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccSystickClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccSystickClksel {
    #[inline(always)]
    fn default() -> MrccSystickClksel {
        MrccSystickClksel(0)
    }
}
impl core::fmt::Debug for MrccSystickClksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccSystickClksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccSystickClksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccSystickClksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "USB0 clock selection control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccUsb0Clksel(pub u32);
impl MrccUsb0Clksel {
    #[doc = "Functional Clock Mux Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> MrccUsb0ClkselMux {
        let val = (self.0 >> 0usize) & 0x03;
        MrccUsb0ClkselMux::from_bits(val as u8)
    }
    #[doc = "Functional Clock Mux Select."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: MrccUsb0ClkselMux) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MrccUsb0Clksel {
    #[inline(always)]
    fn default() -> MrccUsb0Clksel {
        MrccUsb0Clksel(0)
    }
}
impl core::fmt::Debug for MrccUsb0Clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccUsb0Clksel")
            .field("mux", &self.mux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccUsb0Clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrccUsb0Clksel {{ mux: {:?} }}", self.mux())
    }
}
#[doc = "WWDT0 clock divider control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrccWwdt0Clkdiv(pub u32);
impl MrccWwdt0Clkdiv {
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
    pub const fn reset(&self) -> MrccWwdt0ClkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MrccWwdt0ClkdivReset::from_bits(val as u8)
    }
    #[doc = "Reset divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MrccWwdt0ClkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halt divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MrccWwdt0ClkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MrccWwdt0ClkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halt divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MrccWwdt0ClkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MrccWwdt0ClkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MrccWwdt0ClkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MrccWwdt0ClkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MrccWwdt0Clkdiv {
    #[inline(always)]
    fn default() -> MrccWwdt0Clkdiv {
        MrccWwdt0Clkdiv(0)
    }
}
impl core::fmt::Debug for MrccWwdt0Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrccWwdt0Clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrccWwdt0Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MrccWwdt0Clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccAdc0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc0ClkdivHalt {
        MrccAdc0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccAdc0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc0ClkdivHalt) -> u8 {
        MrccAdc0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccAdc0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc0ClkdivReset {
        MrccAdc0ClkdivReset::from_bits(val)
    }
}
impl From<MrccAdc0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc0ClkdivReset) -> u8 {
        MrccAdc0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccAdc0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc0ClkdivUnstab {
        MrccAdc0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccAdc0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc0ClkdivUnstab) -> u8 {
        MrccAdc0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc0ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    ClkrootFunc1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccAdc0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc0ClkselMux {
        MrccAdc0ClkselMux::from_bits(val)
    }
}
impl From<MrccAdc0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc0ClkselMux) -> u8 {
        MrccAdc0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc1ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccAdc1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc1ClkdivHalt {
        MrccAdc1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccAdc1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc1ClkdivHalt) -> u8 {
        MrccAdc1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc1ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccAdc1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc1ClkdivReset {
        MrccAdc1ClkdivReset::from_bits(val)
    }
}
impl From<MrccAdc1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc1ClkdivReset) -> u8 {
        MrccAdc1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc1ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccAdc1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc1ClkdivUnstab {
        MrccAdc1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccAdc1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc1ClkdivUnstab) -> u8 {
        MrccAdc1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccAdc1ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    ClkrootFunc1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccAdc1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccAdc1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccAdc1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccAdc1ClkselMux {
        MrccAdc1ClkselMux::from_bits(val)
    }
}
impl From<MrccAdc1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccAdc1ClkselMux) -> u8 {
        MrccAdc1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccClkoutClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkdivHalt {
        MrccClkoutClkdivHalt::from_bits(val)
    }
}
impl From<MrccClkoutClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkdivHalt) -> u8 {
        MrccClkoutClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccClkoutClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkdivReset {
        MrccClkoutClkdivReset::from_bits(val)
    }
}
impl From<MrccClkoutClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkdivReset) -> u8 {
        MrccClkoutClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccClkoutClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkdivUnstab {
        MrccClkoutClkdivUnstab::from_bits(val)
    }
}
impl From<MrccClkoutClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkdivUnstab) -> u8 {
        MrccClkoutClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccClkoutClkselMux {
    #[doc = "FRO_12M."]
    Clkroot12m = 0x0,
    #[doc = "FRO_HF_DIV."]
    ClkrootFircDiv = 0x01,
    #[doc = "CLK_IN."]
    ClkrootSosc = 0x02,
    #[doc = "CLK_16K."]
    Clkroot16k = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "SLOW_CLK."]
    ClkrootSlow = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccClkoutClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccClkoutClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccClkoutClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccClkoutClkselMux {
        MrccClkoutClkselMux::from_bits(val)
    }
}
impl From<MrccClkoutClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccClkoutClkselMux) -> u8 {
        MrccClkoutClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0FuncClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCmp0FuncClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0FuncClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0FuncClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0FuncClkdivHalt {
        MrccCmp0FuncClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp0FuncClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0FuncClkdivHalt) -> u8 {
        MrccCmp0FuncClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0FuncClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCmp0FuncClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0FuncClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0FuncClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0FuncClkdivReset {
        MrccCmp0FuncClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp0FuncClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0FuncClkdivReset) -> u8 {
        MrccCmp0FuncClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0FuncClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCmp0FuncClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0FuncClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0FuncClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0FuncClkdivUnstab {
        MrccCmp0FuncClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp0FuncClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0FuncClkdivUnstab) -> u8 {
        MrccCmp0FuncClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCmp0RrClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkdivHalt {
        MrccCmp0RrClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp0RrClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkdivHalt) -> u8 {
        MrccCmp0RrClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCmp0RrClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkdivReset {
        MrccCmp0RrClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp0RrClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkdivReset) -> u8 {
        MrccCmp0RrClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCmp0RrClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkdivUnstab {
        MrccCmp0RrClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp0RrClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkdivUnstab) -> u8 {
        MrccCmp0RrClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp0RrClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCmp0RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp0RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp0RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp0RrClkselMux {
        MrccCmp0RrClkselMux::from_bits(val)
    }
}
impl From<MrccCmp0RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp0RrClkselMux) -> u8 {
        MrccCmp0RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1FuncClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCmp1FuncClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1FuncClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1FuncClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1FuncClkdivHalt {
        MrccCmp1FuncClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp1FuncClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1FuncClkdivHalt) -> u8 {
        MrccCmp1FuncClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1FuncClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCmp1FuncClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1FuncClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1FuncClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1FuncClkdivReset {
        MrccCmp1FuncClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp1FuncClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1FuncClkdivReset) -> u8 {
        MrccCmp1FuncClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1FuncClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCmp1FuncClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1FuncClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1FuncClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1FuncClkdivUnstab {
        MrccCmp1FuncClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp1FuncClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1FuncClkdivUnstab) -> u8 {
        MrccCmp1FuncClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCmp1RrClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkdivHalt {
        MrccCmp1RrClkdivHalt::from_bits(val)
    }
}
impl From<MrccCmp1RrClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkdivHalt) -> u8 {
        MrccCmp1RrClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCmp1RrClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkdivReset {
        MrccCmp1RrClkdivReset::from_bits(val)
    }
}
impl From<MrccCmp1RrClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkdivReset) -> u8 {
        MrccCmp1RrClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCmp1RrClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkdivUnstab {
        MrccCmp1RrClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCmp1RrClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkdivUnstab) -> u8 {
        MrccCmp1RrClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCmp1RrClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCmp1RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCmp1RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCmp1RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCmp1RrClkselMux {
        MrccCmp1RrClkselMux::from_bits(val)
    }
}
impl From<MrccCmp1RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCmp1RrClkselMux) -> u8 {
        MrccCmp1RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCtimer0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkdivHalt {
        MrccCtimer0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkdivHalt) -> u8 {
        MrccCtimer0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCtimer0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkdivReset {
        MrccCtimer0ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkdivReset) -> u8 {
        MrccCtimer0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCtimer0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkdivUnstab {
        MrccCtimer0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkdivUnstab) -> u8 {
        MrccCtimer0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer0ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCtimer0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer0ClkselMux {
        MrccCtimer0ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer0ClkselMux) -> u8 {
        MrccCtimer0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCtimer1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkdivHalt {
        MrccCtimer1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkdivHalt) -> u8 {
        MrccCtimer1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCtimer1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkdivReset {
        MrccCtimer1ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkdivReset) -> u8 {
        MrccCtimer1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCtimer1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkdivUnstab {
        MrccCtimer1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkdivUnstab) -> u8 {
        MrccCtimer1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer1ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCtimer1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer1ClkselMux {
        MrccCtimer1ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer1ClkselMux) -> u8 {
        MrccCtimer1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCtimer2ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkdivHalt {
        MrccCtimer2ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer2ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkdivHalt) -> u8 {
        MrccCtimer2ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCtimer2ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkdivReset {
        MrccCtimer2ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer2ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkdivReset) -> u8 {
        MrccCtimer2ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCtimer2ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkdivUnstab {
        MrccCtimer2ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer2ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkdivUnstab) -> u8 {
        MrccCtimer2ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer2ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCtimer2ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer2ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer2ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer2ClkselMux {
        MrccCtimer2ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer2ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer2ClkselMux) -> u8 {
        MrccCtimer2ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCtimer3ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkdivHalt {
        MrccCtimer3ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer3ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkdivHalt) -> u8 {
        MrccCtimer3ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCtimer3ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkdivReset {
        MrccCtimer3ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer3ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkdivReset) -> u8 {
        MrccCtimer3ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCtimer3ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkdivUnstab {
        MrccCtimer3ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer3ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkdivUnstab) -> u8 {
        MrccCtimer3ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer3ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCtimer3ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer3ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer3ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer3ClkselMux {
        MrccCtimer3ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer3ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer3ClkselMux) -> u8 {
        MrccCtimer3ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccCtimer4ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkdivHalt {
        MrccCtimer4ClkdivHalt::from_bits(val)
    }
}
impl From<MrccCtimer4ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkdivHalt) -> u8 {
        MrccCtimer4ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccCtimer4ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkdivReset {
        MrccCtimer4ClkdivReset::from_bits(val)
    }
}
impl From<MrccCtimer4ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkdivReset) -> u8 {
        MrccCtimer4ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccCtimer4ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkdivUnstab {
        MrccCtimer4ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccCtimer4ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkdivUnstab) -> u8 {
        MrccCtimer4ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccCtimer4ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccCtimer4ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccCtimer4ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccCtimer4ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccCtimer4ClkselMux {
        MrccCtimer4ClkselMux::from_bits(val)
    }
}
impl From<MrccCtimer4ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccCtimer4ClkselMux) -> u8 {
        MrccCtimer4ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccDac0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkdivHalt {
        MrccDac0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccDac0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkdivHalt) -> u8 {
        MrccDac0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccDac0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkdivReset {
        MrccDac0ClkdivReset::from_bits(val)
    }
}
impl From<MrccDac0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkdivReset) -> u8 {
        MrccDac0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccDac0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkdivUnstab {
        MrccDac0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccDac0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkdivUnstab) -> u8 {
        MrccDac0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDac0ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccDac0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDac0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDac0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccDac0ClkselMux {
        MrccDac0ClkselMux::from_bits(val)
    }
}
impl From<MrccDac0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccDac0ClkselMux) -> u8 {
        MrccDac0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccDbgTraceClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkdivHalt {
        MrccDbgTraceClkdivHalt::from_bits(val)
    }
}
impl From<MrccDbgTraceClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkdivHalt) -> u8 {
        MrccDbgTraceClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccDbgTraceClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkdivReset {
        MrccDbgTraceClkdivReset::from_bits(val)
    }
}
impl From<MrccDbgTraceClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkdivReset) -> u8 {
        MrccDbgTraceClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccDbgTraceClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkdivUnstab {
        MrccDbgTraceClkdivUnstab::from_bits(val)
    }
}
impl From<MrccDbgTraceClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkdivUnstab) -> u8 {
        MrccDbgTraceClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccDbgTraceClkselMux {
    #[doc = "CPU_CLK."]
    ClkrootCpu = 0x0,
    #[doc = "CLK_1M."]
    Clkroot1m = 0x01,
    #[doc = "CLK_16K."]
    Clkroot16k = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccDbgTraceClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccDbgTraceClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccDbgTraceClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccDbgTraceClkselMux {
        MrccDbgTraceClkselMux::from_bits(val)
    }
}
impl From<MrccDbgTraceClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccDbgTraceClkselMux) -> u8 {
        MrccDbgTraceClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccFlexcan0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkdivHalt {
        MrccFlexcan0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkdivHalt) -> u8 {
        MrccFlexcan0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccFlexcan0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkdivReset {
        MrccFlexcan0ClkdivReset::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkdivReset) -> u8 {
        MrccFlexcan0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccFlexcan0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkdivUnstab {
        MrccFlexcan0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkdivUnstab) -> u8 {
        MrccFlexcan0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexcan0ClkselMux {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccFlexcan0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexcan0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexcan0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexcan0ClkselMux {
        MrccFlexcan0ClkselMux::from_bits(val)
    }
}
impl From<MrccFlexcan0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexcan0ClkselMux) -> u8 {
        MrccFlexcan0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccFlexio0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkdivHalt {
        MrccFlexio0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccFlexio0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkdivHalt) -> u8 {
        MrccFlexio0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccFlexio0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkdivReset {
        MrccFlexio0ClkdivReset::from_bits(val)
    }
}
impl From<MrccFlexio0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkdivReset) -> u8 {
        MrccFlexio0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccFlexio0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkdivUnstab {
        MrccFlexio0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccFlexio0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkdivUnstab) -> u8 {
        MrccFlexio0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFlexio0ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    #[doc = "FRO_HF_GATED."]
    ClkrootFunc1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccFlexio0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFlexio0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFlexio0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccFlexio0ClkselMux {
        MrccFlexio0ClkselMux::from_bits(val)
    }
}
impl From<MrccFlexio0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccFlexio0ClkselMux) -> u8 {
        MrccFlexio0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccFroHfDivClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccFroHfDivClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccFroHfDivClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccFroHfDivClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccFroHfDivClkdivUnstab {
        MrccFroHfDivClkdivUnstab::from_bits(val)
    }
}
impl From<MrccFroHfDivClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccFroHfDivClkdivUnstab) -> u8 {
        MrccFroHfDivClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccI3c0FclkClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkdivHalt {
        MrccI3c0FclkClkdivHalt::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkdivHalt) -> u8 {
        MrccI3c0FclkClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccI3c0FclkClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkdivReset {
        MrccI3c0FclkClkdivReset::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkdivReset) -> u8 {
        MrccI3c0FclkClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccI3c0FclkClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkdivUnstab {
        MrccI3c0FclkClkdivUnstab::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkdivUnstab) -> u8 {
        MrccI3c0FclkClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccI3c0FclkClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccI3c0FclkClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccI3c0FclkClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccI3c0FclkClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccI3c0FclkClkselMux {
        MrccI3c0FclkClkselMux::from_bits(val)
    }
}
impl From<MrccI3c0FclkClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccI3c0FclkClkselMux) -> u8 {
        MrccI3c0FclkClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpi2c0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkdivHalt {
        MrccLpi2c0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkdivHalt) -> u8 {
        MrccLpi2c0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpi2c0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkdivReset {
        MrccLpi2c0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkdivReset) -> u8 {
        MrccLpi2c0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpi2c0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkdivUnstab {
        MrccLpi2c0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkdivUnstab) -> u8 {
        MrccLpi2c0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c0ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpi2c0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c0ClkselMux {
        MrccLpi2c0ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c0ClkselMux) -> u8 {
        MrccLpi2c0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpi2c1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkdivHalt {
        MrccLpi2c1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkdivHalt) -> u8 {
        MrccLpi2c1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpi2c1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkdivReset {
        MrccLpi2c1ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkdivReset) -> u8 {
        MrccLpi2c1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpi2c1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkdivUnstab {
        MrccLpi2c1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkdivUnstab) -> u8 {
        MrccLpi2c1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c1ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpi2c1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c1ClkselMux {
        MrccLpi2c1ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c1ClkselMux) -> u8 {
        MrccLpi2c1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpi2c2ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkdivHalt {
        MrccLpi2c2ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkdivHalt) -> u8 {
        MrccLpi2c2ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpi2c2ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkdivReset {
        MrccLpi2c2ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkdivReset) -> u8 {
        MrccLpi2c2ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpi2c2ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkdivUnstab {
        MrccLpi2c2ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkdivUnstab) -> u8 {
        MrccLpi2c2ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c2ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpi2c2ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c2ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c2ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c2ClkselMux {
        MrccLpi2c2ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c2ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c2ClkselMux) -> u8 {
        MrccLpi2c2ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpi2c3ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkdivHalt {
        MrccLpi2c3ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkdivHalt) -> u8 {
        MrccLpi2c3ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpi2c3ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkdivReset {
        MrccLpi2c3ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkdivReset) -> u8 {
        MrccLpi2c3ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpi2c3ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkdivUnstab {
        MrccLpi2c3ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkdivUnstab) -> u8 {
        MrccLpi2c3ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpi2c3ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpi2c3ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpi2c3ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpi2c3ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpi2c3ClkselMux {
        MrccLpi2c3ClkselMux::from_bits(val)
    }
}
impl From<MrccLpi2c3ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpi2c3ClkselMux) -> u8 {
        MrccLpi2c3ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpspi0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkdivHalt {
        MrccLpspi0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpspi0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkdivHalt) -> u8 {
        MrccLpspi0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpspi0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkdivReset {
        MrccLpspi0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpspi0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkdivReset) -> u8 {
        MrccLpspi0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpspi0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkdivUnstab {
        MrccLpspi0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpspi0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkdivUnstab) -> u8 {
        MrccLpspi0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi0ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpspi0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi0ClkselMux {
        MrccLpspi0ClkselMux::from_bits(val)
    }
}
impl From<MrccLpspi0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi0ClkselMux) -> u8 {
        MrccLpspi0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpspi1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkdivHalt {
        MrccLpspi1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpspi1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkdivHalt) -> u8 {
        MrccLpspi1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpspi1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkdivReset {
        MrccLpspi1ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpspi1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkdivReset) -> u8 {
        MrccLpspi1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpspi1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkdivUnstab {
        MrccLpspi1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpspi1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkdivUnstab) -> u8 {
        MrccLpspi1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpspi1ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpspi1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpspi1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpspi1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpspi1ClkselMux {
        MrccLpspi1ClkselMux::from_bits(val)
    }
}
impl From<MrccLpspi1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpspi1ClkselMux) -> u8 {
        MrccLpspi1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLptmr0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkdivHalt {
        MrccLptmr0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLptmr0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkdivHalt) -> u8 {
        MrccLptmr0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLptmr0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkdivReset {
        MrccLptmr0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLptmr0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkdivReset) -> u8 {
        MrccLptmr0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLptmr0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkdivUnstab {
        MrccLptmr0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLptmr0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkdivUnstab) -> u8 {
        MrccLptmr0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLptmr0ClkselMux {
    #[doc = "FRO_12M."]
    ClkrootFunc0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV."]
    ClkrootFunc2 = 0x02,
    #[doc = "CLK_IN."]
    ClkrootFunc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M."]
    ClkrootFunc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLptmr0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLptmr0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLptmr0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLptmr0ClkselMux {
        MrccLptmr0ClkselMux::from_bits(val)
    }
}
impl From<MrccLptmr0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLptmr0ClkselMux) -> u8 {
        MrccLptmr0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpuart0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkdivHalt {
        MrccLpuart0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkdivHalt) -> u8 {
        MrccLpuart0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpuart0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkdivReset {
        MrccLpuart0ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkdivReset) -> u8 {
        MrccLpuart0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpuart0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkdivUnstab {
        MrccLpuart0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkdivUnstab) -> u8 {
        MrccLpuart0ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart0ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpuart0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart0ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart0ClkselMux {
        MrccLpuart0ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart0ClkselMux) -> u8 {
        MrccLpuart0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpuart1ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkdivHalt {
        MrccLpuart1ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart1ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkdivHalt) -> u8 {
        MrccLpuart1ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpuart1ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkdivReset {
        MrccLpuart1ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart1ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkdivReset) -> u8 {
        MrccLpuart1ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpuart1ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkdivUnstab {
        MrccLpuart1ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart1ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkdivUnstab) -> u8 {
        MrccLpuart1ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart1ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpuart1ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart1ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart1ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart1ClkselMux {
        MrccLpuart1ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart1ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart1ClkselMux) -> u8 {
        MrccLpuart1ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpuart2ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkdivHalt {
        MrccLpuart2ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart2ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkdivHalt) -> u8 {
        MrccLpuart2ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpuart2ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkdivReset {
        MrccLpuart2ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart2ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkdivReset) -> u8 {
        MrccLpuart2ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpuart2ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkdivUnstab {
        MrccLpuart2ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart2ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkdivUnstab) -> u8 {
        MrccLpuart2ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart2ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpuart2ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart2ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart2ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart2ClkselMux {
        MrccLpuart2ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart2ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart2ClkselMux) -> u8 {
        MrccLpuart2ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpuart3ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkdivHalt {
        MrccLpuart3ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart3ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkdivHalt) -> u8 {
        MrccLpuart3ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpuart3ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkdivReset {
        MrccLpuart3ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart3ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkdivReset) -> u8 {
        MrccLpuart3ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpuart3ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkdivUnstab {
        MrccLpuart3ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart3ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkdivUnstab) -> u8 {
        MrccLpuart3ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart3ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpuart3ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart3ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart3ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart3ClkselMux {
        MrccLpuart3ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart3ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart3ClkselMux) -> u8 {
        MrccLpuart3ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccLpuart4ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkdivHalt {
        MrccLpuart4ClkdivHalt::from_bits(val)
    }
}
impl From<MrccLpuart4ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkdivHalt) -> u8 {
        MrccLpuart4ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccLpuart4ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkdivReset {
        MrccLpuart4ClkdivReset::from_bits(val)
    }
}
impl From<MrccLpuart4ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkdivReset) -> u8 {
        MrccLpuart4ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccLpuart4ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkdivUnstab {
        MrccLpuart4ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccLpuart4ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkdivUnstab) -> u8 {
        MrccLpuart4ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccLpuart4ClkselMux {
    #[doc = "FRO_12M."]
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
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MrccLpuart4ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccLpuart4ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccLpuart4ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccLpuart4ClkselMux {
        MrccLpuart4ClkselMux::from_bits(val)
    }
}
impl From<MrccLpuart4ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccLpuart4ClkselMux) -> u8 {
        MrccLpuart4ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccOstimer0ClkselMux {
    #[doc = "CLK_16K."]
    Clkroot16k = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "CLK_1M."]
    Clkroot1m = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccOstimer0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccOstimer0ClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccOstimer0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccOstimer0ClkselMux {
        MrccOstimer0ClkselMux::from_bits(val)
    }
}
impl From<MrccOstimer0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccOstimer0ClkselMux) -> u8 {
        MrccOstimer0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccSystickClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkdivHalt {
        MrccSystickClkdivHalt::from_bits(val)
    }
}
impl From<MrccSystickClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkdivHalt) -> u8 {
        MrccSystickClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccSystickClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkdivReset {
        MrccSystickClkdivReset::from_bits(val)
    }
}
impl From<MrccSystickClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkdivReset) -> u8 {
        MrccSystickClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccSystickClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkdivUnstab {
        MrccSystickClkdivUnstab::from_bits(val)
    }
}
impl From<MrccSystickClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkdivUnstab) -> u8 {
        MrccSystickClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccSystickClkselMux {
    #[doc = "CPU_CLK."]
    ClkrootCpu = 0x0,
    #[doc = "CLK_1M."]
    Clkroot1m = 0x01,
    #[doc = "CLK_16K."]
    Clkroot16k = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccSystickClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccSystickClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccSystickClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccSystickClkselMux {
        MrccSystickClkselMux::from_bits(val)
    }
}
impl From<MrccSystickClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccSystickClkselMux) -> u8 {
        MrccSystickClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccUsb0ClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "CLK_48M."]
    ScgScgFirc48mhzClk = 0x01,
    #[doc = "CLK_IN."]
    ClkrootSosc = 0x02,
    _RESERVED_3 = 0x03,
}
impl MrccUsb0ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccUsb0ClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccUsb0ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> MrccUsb0ClkselMux {
        MrccUsb0ClkselMux::from_bits(val)
    }
}
impl From<MrccUsb0ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: MrccUsb0ClkselMux) -> u8 {
        MrccUsb0ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccWwdt0ClkdivHalt {
    #[doc = "Divider clock is running."]
    On = 0x0,
    #[doc = "Divider clock is stopped."]
    Off = 0x01,
}
impl MrccWwdt0ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccWwdt0ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccWwdt0ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MrccWwdt0ClkdivHalt {
        MrccWwdt0ClkdivHalt::from_bits(val)
    }
}
impl From<MrccWwdt0ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MrccWwdt0ClkdivHalt) -> u8 {
        MrccWwdt0ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccWwdt0ClkdivReset {
    #[doc = "Divider isn't reset."]
    On = 0x0,
    #[doc = "Divider is reset."]
    Off = 0x01,
}
impl MrccWwdt0ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccWwdt0ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccWwdt0ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MrccWwdt0ClkdivReset {
        MrccWwdt0ClkdivReset::from_bits(val)
    }
}
impl From<MrccWwdt0ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MrccWwdt0ClkdivReset) -> u8 {
        MrccWwdt0ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrccWwdt0ClkdivUnstab {
    #[doc = "Divider clock is stable."]
    On = 0x0,
    #[doc = "Clock frequency isn't stable."]
    Off = 0x01,
}
impl MrccWwdt0ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrccWwdt0ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrccWwdt0ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MrccWwdt0ClkdivUnstab {
        MrccWwdt0ClkdivUnstab::from_bits(val)
    }
}
impl From<MrccWwdt0ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MrccWwdt0ClkdivUnstab) -> u8 {
        MrccWwdt0ClkdivUnstab::to_bits(val)
    }
}
