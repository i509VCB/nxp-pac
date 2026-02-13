#[doc = "MRCC"]
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
    #[doc = "Peripheral Reset Control 0"]
    #[inline(always)]
    pub const fn mrcc_glb_rst0(self) -> crate::common::Reg<regs::MrccGlbRst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 0"]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_set(self) -> crate::common::Reg<regs::GlbRstSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 0"]
    #[inline(always)]
    pub const fn mrcc_glb_rst0_clr(self) -> crate::common::Reg<regs::GlbRstClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Peripheral Reset Control 1"]
    #[inline(always)]
    pub const fn mrcc_glb_rst1(self) -> crate::common::Reg<regs::MrccGlbRst1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 1"]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_set(self) -> crate::common::Reg<regs::GlbRstSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 1"]
    #[inline(always)]
    pub const fn mrcc_glb_rst1_clr(self) -> crate::common::Reg<regs::GlbRstClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Peripheral Reset Control 2"]
    #[inline(always)]
    pub const fn mrcc_glb_rst2(self) -> crate::common::Reg<regs::MrccGlbRst2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Peripheral Reset Control Set 2"]
    #[inline(always)]
    pub const fn mrcc_glb_rst2_set(self) -> crate::common::Reg<regs::GlbRstSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Peripheral Reset Control Clear 2"]
    #[inline(always)]
    pub const fn mrcc_glb_rst2_clr(self) -> crate::common::Reg<regs::GlbRstClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "AHB Clock Control 0"]
    #[inline(always)]
    pub const fn mrcc_glb_cc0(self) -> crate::common::Reg<regs::MrccGlbCc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "AHB Clock Control Set 0"]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_set(self) -> crate::common::Reg<regs::GlbCcSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 0"]
    #[inline(always)]
    pub const fn mrcc_glb_cc0_clr(self) -> crate::common::Reg<regs::GlbCcClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "AHB Clock Control 1"]
    #[inline(always)]
    pub const fn mrcc_glb_cc1(self) -> crate::common::Reg<regs::MrccGlbCc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "AHB Clock Control Set 1"]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_set(self) -> crate::common::Reg<regs::GlbCcSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 1"]
    #[inline(always)]
    pub const fn mrcc_glb_cc1_clr(self) -> crate::common::Reg<regs::GlbCcClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "AHB Clock Control 2"]
    #[inline(always)]
    pub const fn mrcc_glb_cc2(self) -> crate::common::Reg<regs::MrccGlbCc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "AHB Clock Control Set 2"]
    #[inline(always)]
    pub const fn mrcc_glb_cc2_set(self) -> crate::common::Reg<regs::GlbCcSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "AHB Clock Control Clear 2"]
    #[inline(always)]
    pub const fn mrcc_glb_cc2_clr(self) -> crate::common::Reg<regs::GlbCcClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 0"]
    #[inline(always)]
    pub const fn mrcc_glb_acc0(self) -> crate::common::Reg<regs::MrccGlbAcc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 1"]
    #[inline(always)]
    pub const fn mrcc_glb_acc1(self) -> crate::common::Reg<regs::MrccGlbAcc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating 2"]
    #[inline(always)]
    pub const fn mrcc_glb_acc2(self) -> crate::common::Reg<regs::MrccGlbAcc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "I3C0_FCLK clock selection control"]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clksel(
        self,
    ) -> crate::common::Reg<regs::I3cFclkClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "I3C0_FCLK clock divider control"]
    #[inline(always)]
    pub const fn mrcc_i3c0_fclk_clkdiv(
        self,
    ) -> crate::common::Reg<regs::I3cFclkClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "CTIMER0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clksel(
        self,
    ) -> crate::common::Reg<regs::CtimerClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "CTIMER0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_ctimer0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CtimerClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "CTIMER1 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clksel(
        self,
    ) -> crate::common::Reg<regs::CtimerClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "CTIMER1 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_ctimer1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CtimerClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "CTIMER2 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clksel(
        self,
    ) -> crate::common::Reg<regs::CtimerClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "CTIMER2 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_ctimer2_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CtimerClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "CTIMER3 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clksel(
        self,
    ) -> crate::common::Reg<regs::CtimerClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "CTIMER3 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_ctimer3_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CtimerClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "CTIMER4 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clksel(
        self,
    ) -> crate::common::Reg<regs::CtimerClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "CTIMER4 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_ctimer4_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CtimerClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "WWDT0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_wwdt0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::WwdtClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "FLEXIO0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_flexio0_clksel(
        self,
    ) -> crate::common::Reg<regs::FlexioClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "FLEXIO0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_flexio0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::FlexioClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "LPI2C0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clksel(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "LPI2C0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "LPI2C1 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clksel(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "LPI2C1 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "LPSPI0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clksel(
        self,
    ) -> crate::common::Reg<regs::LpspiClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "LPSPI0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpspi0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpspiClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "LPSPI1 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clksel(
        self,
    ) -> crate::common::Reg<regs::LpspiClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "LPSPI1 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpspi1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpspiClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "LPUART0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clksel(
        self,
    ) -> crate::common::Reg<regs::LpuartClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "LPUART0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpuart0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpuartClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "LPUART1 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clksel(
        self,
    ) -> crate::common::Reg<regs::LpuartClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "LPUART1 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpuart1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpuartClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "LPUART2 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clksel(
        self,
    ) -> crate::common::Reg<regs::LpuartClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "LPUART2 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpuart2_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpuartClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "LPUART3 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clksel(
        self,
    ) -> crate::common::Reg<regs::LpuartClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "LPUART3 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpuart3_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpuartClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "LPUART4 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clksel(
        self,
    ) -> crate::common::Reg<regs::LpuartClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "LPUART4 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpuart4_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpuartClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "USB0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_usb0_clksel(self) -> crate::common::Reg<regs::UsbClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "USB0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_usb0_clkdiv(self) -> crate::common::Reg<regs::UsbClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "LPTMR0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clksel(
        self,
    ) -> crate::common::Reg<regs::LptmrClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "LPTMR0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lptmr0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LptmrClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "OSTIMER0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_ostimer0_clksel(
        self,
    ) -> crate::common::Reg<regs::OstimerClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ADCx clock selection control"]
    #[inline(always)]
    pub const fn mrcc_adc_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccAdcClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ADCx clock divider control"]
    #[inline(always)]
    pub const fn mrcc_adc_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccAdcClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "CMP0_FUNC clock divider control"]
    #[inline(always)]
    pub const fn mrcc_cmp0_func_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CmpFuncClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "CMP0_RR clock selection control"]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clksel(
        self,
    ) -> crate::common::Reg<regs::CmpRrClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "CMP0_RR clock divider control"]
    #[inline(always)]
    pub const fn mrcc_cmp0_rr_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CmpRrClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "CMP1_FUNC clock divider control"]
    #[inline(always)]
    pub const fn mrcc_cmp1_func_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CmpFuncClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "CMP1_RR clock selection control"]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clksel(
        self,
    ) -> crate::common::Reg<regs::CmpRrClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "CMP1_RR clock divider control"]
    #[inline(always)]
    pub const fn mrcc_cmp1_rr_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CmpRrClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "CMP2_FUNC clock divider control"]
    #[inline(always)]
    pub const fn mrcc_cmp2_func_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CmpFuncClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "CMP2_RR clock selection control"]
    #[inline(always)]
    pub const fn mrcc_cmp2_rr_clksel(
        self,
    ) -> crate::common::Reg<regs::CmpRrClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "CMP2_RR clock divider control"]
    #[inline(always)]
    pub const fn mrcc_cmp2_rr_clkdiv(
        self,
    ) -> crate::common::Reg<regs::CmpRrClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "DAC0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_dac0_clksel(self) -> crate::common::Reg<regs::DacClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "DAC0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_dac0_clkdiv(self) -> crate::common::Reg<regs::DacClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "FLEXCAN0 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clksel(
        self,
    ) -> crate::common::Reg<regs::FlexcanClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "FLEXCAN0 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_flexcan0_clkdiv(
        self,
    ) -> crate::common::Reg<regs::FlexcanClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "FLEXCAN1 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_flexcan1_clksel(
        self,
    ) -> crate::common::Reg<regs::FlexcanClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "FLEXCAN1 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_flexcan1_clkdiv(
        self,
    ) -> crate::common::Reg<regs::FlexcanClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "LPI2C2 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clksel(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "LPI2C2 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c2_clkdiv(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "LPI2C3 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clksel(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "LPI2C3 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpi2c3_clkdiv(
        self,
    ) -> crate::common::Reg<regs::Lpi2cClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "LPUART5 clock selection control"]
    #[inline(always)]
    pub const fn mrcc_lpuart5_clksel(
        self,
    ) -> crate::common::Reg<regs::LpuartClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "LPUART5 clock divider control"]
    #[inline(always)]
    pub const fn mrcc_lpuart5_clkdiv(
        self,
    ) -> crate::common::Reg<regs::LpuartClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "DBG_TRACE clock selection control"]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccDbgTraceClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "DBG_TRACE clock divider control"]
    #[inline(always)]
    pub const fn mrcc_dbg_trace_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccDbgTraceClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "CLKOUT clock selection control"]
    #[inline(always)]
    pub const fn mrcc_clkout_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccClkoutClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "CLKOUT clock divider control"]
    #[inline(always)]
    pub const fn mrcc_clkout_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccClkoutClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "SYSTICK clock selection control"]
    #[inline(always)]
    pub const fn mrcc_systick_clksel(
        self,
    ) -> crate::common::Reg<regs::MrccSystickClksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "SYSTICK clock divider control"]
    #[inline(always)]
    pub const fn mrcc_systick_clkdiv(
        self,
    ) -> crate::common::Reg<regs::MrccSystickClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
