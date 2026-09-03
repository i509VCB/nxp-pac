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
    pub const fn mrcc_glb_rst0(self) -> crate::common::Reg<regs::MrccGlbRst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 0."]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_set(
        self,
    ) -> crate::common::Reg<regs::MrccGlbRst0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 0."]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_clr(
        self,
    ) -> crate::common::Reg<regs::MrccGlbRst0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Peripheral Reset Control 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1(self) -> crate::common::Reg<regs::MrccGlbRst1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_set(
        self,
    ) -> crate::common::Reg<regs::MrccGlbRst1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 1."]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_clr(
        self,
    ) -> crate::common::Reg<regs::MrccGlbRst1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "AHB Clock Control 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0(self) -> crate::common::Reg<regs::MrccGlbCc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "AHB Clock Control Set 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_set(
        self,
    ) -> crate::common::Reg<regs::MrccGlbCc0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 0."]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_clr(
        self,
    ) -> crate::common::Reg<regs::MrccGlbCc0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "AHB Clock Control 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1(self) -> crate::common::Reg<regs::MrccGlbCc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "AHB Clock Control Set 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_set(
        self,
    ) -> crate::common::Reg<regs::MrccGlbCc1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 1."]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_clr(
        self,
    ) -> crate::common::Reg<regs::MrccGlbCc1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 0."]
    #[inline(always)]
    pub const fn mrcc_glb_acc0(self) -> crate::common::Reg<regs::MrccGlbAcc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 1."]
    #[inline(always)]
    pub const fn mrcc_glb_acc1(self) -> crate::common::Reg<regs::MrccGlbAcc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "I3C0_FCLK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccI3c0FclkClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "I3C0_FCLK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccI3c0FclkClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "CTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "CTIMER0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "CTIMER1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer1Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "CTIMER1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer1Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "CTIMER2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer2Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "CTIMER2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer2Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "CTIMER3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer3Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "CTIMER3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer3Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "CTIMER4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer4Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "CTIMER4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCtimer4Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "WWDT0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_wwdt0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccWwdt0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "FLEXIO0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccFlexio0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "FLEXIO0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexio0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccFlexio0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "LPI2C0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "LPI2C0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "LPI2C1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c1Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "LPI2C1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c1Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "LPSPI0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpspi0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "LPSPI0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpspi0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "LPSPI1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpspi1Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "LPSPI1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpspi1Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "LPUART0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "LPUART0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "LPUART1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart1Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "LPUART1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart1Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "LPUART2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart2Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "LPUART2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart2Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "LPUART3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart3Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "LPUART3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart3Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "LPUART4 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart4Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "LPUART4 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpuart4Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "USB0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_usb0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccUsb0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "LPTMR0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLptmr0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "LPTMR0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLptmr0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "OSTIMER0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_ostimer0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccOstimer0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ADC0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_adc0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccAdc0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ADC0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_adc0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccAdc0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "ADC1 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_adc1_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccAdc1Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "ADC1 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_adc1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccAdc1Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "CMP0_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_func_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCmp0FuncClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "CMP0_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccCmp0RrClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "CMP0_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCmp0RrClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "CMP1_FUNC clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_func_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCmp1FuncClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "CMP1_RR clock selection control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccCmp1RrClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "CMP1_RR clock divider control."]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccCmp1RrClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "DAC0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccDac0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "DAC0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dac0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccDac0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "FLEXCAN0 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccFlexcan0Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "FLEXCAN0 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccFlexcan0Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "LPI2C2 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c2Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "LPI2C2 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c2Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "LPI2C3 clock selection control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c3Clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "LPI2C3 clock divider control."]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccLpi2c3Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "DBG_TRACE clock selection control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccDbgTraceClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "DBG_TRACE clock divider control."]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccDbgTraceClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "CLKOUT clock selection control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccClkoutClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "CLKOUT clock divider control."]
    #[inline(always)]
    pub const fn mrcc_clkout_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccClkoutClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "SYSTICK clock selection control."]
    #[inline(always)]
    pub const fn mrcc_systick_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccSystickClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "SYSTICK clock divider control."]
    #[inline(always)]
    pub const fn mrcc_systick_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccSystickClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "FRO_HF_DIV clock divider control."]
    #[inline(always)]
    pub const fn mrcc_fro_hf_div_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccFroHfDivClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
}
pub mod regs;
pub mod vals;
