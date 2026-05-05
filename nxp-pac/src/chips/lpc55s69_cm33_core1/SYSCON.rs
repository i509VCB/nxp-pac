#[doc = "SYSCON."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSCON {
    ptr: *mut u8,
}
unsafe impl Send for SYSCON {}
unsafe impl Sync for SYSCON {}
impl SYSCON {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Memory Remap control register."]
    #[inline(always)]
    pub const fn MEMORYREMAP(self) -> crate::common::Reg<regs::MEMORYREMAP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest."]
    #[inline(always)]
    pub const fn AHBMATPRIO(self) -> crate::common::Reg<regs::AHBMATPRIO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "System tick calibration for secure part of CPU0."]
    #[inline(always)]
    pub const fn CPU0STCKCAL(self) -> crate::common::Reg<regs::CPU0STCKCAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "System tick calibration for non-secure part of CPU0."]
    #[inline(always)]
    pub const fn CPU0NSTCKCAL(self) -> crate::common::Reg<regs::CPU0NSTCKCAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "System tick calibration for CPU1."]
    #[inline(always)]
    pub const fn CPU1STCKCAL(self) -> crate::common::Reg<regs::CPU1STCKCAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "NMI Source Select."]
    #[inline(always)]
    pub const fn NMISRC(self) -> crate::common::Reg<regs::NMISRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Peripheral reset control 0."]
    #[inline(always)]
    pub const fn PRESETCTRL0(self) -> crate::common::Reg<regs::PRESETCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn PRESETCTRLX0(self) -> crate::common::Reg<regs::PRESETCTRLX0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Peripheral reset control 1."]
    #[inline(always)]
    pub const fn PRESETCTRL1(self) -> crate::common::Reg<regs::PRESETCTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn PRESETCTRLX1(self) -> crate::common::Reg<regs::PRESETCTRLX1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Peripheral reset control 2."]
    #[inline(always)]
    pub const fn PRESETCTRL2(self) -> crate::common::Reg<regs::PRESETCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn PRESETCTRLX2(self) -> crate::common::Reg<regs::PRESETCTRLX2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Peripheral reset control set register."]
    #[inline(always)]
    pub const fn PRESETCTRLSET(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PRESETCTRLSET, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "Peripheral reset control clear register."]
    #[inline(always)]
    pub const fn PRESETCTRLCLR(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PRESETCTRLCLR, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "generate a software_reset."]
    #[inline(always)]
    pub const fn SWR_RESET(self) -> crate::common::Reg<regs::SWR_RESET, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "AHB Clock control 0."]
    #[inline(always)]
    pub const fn AHBCLKCTRL0(self) -> crate::common::Reg<regs::AHBCLKCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn AHBCLKCTRLX0(self) -> crate::common::Reg<regs::AHBCLKCTRLX0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AHB Clock control 1."]
    #[inline(always)]
    pub const fn AHBCLKCTRL1(self) -> crate::common::Reg<regs::AHBCLKCTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn AHBCLKCTRLX1(self) -> crate::common::Reg<regs::AHBCLKCTRLX1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "AHB Clock control 2."]
    #[inline(always)]
    pub const fn AHBCLKCTRL2(self) -> crate::common::Reg<regs::AHBCLKCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn AHBCLKCTRLX2(self) -> crate::common::Reg<regs::AHBCLKCTRLX2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn AHBCLKCTRLSET(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AHBCLKCTRLSET, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize + n * 4usize) as _)
        }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn AHBCLKCTRLCLR(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AHBCLKCTRLCLR, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "System Tick Timer for CPU0 source select."]
    #[inline(always)]
    pub const fn SYSTICKCLKSEL0(
        self,
    ) -> crate::common::Reg<regs::SYSTICKCLKSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn SYSTICKCLKSELX0(
        self,
    ) -> crate::common::Reg<regs::SYSTICKCLKSELX0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "System Tick Timer for CPU1 source select."]
    #[inline(always)]
    pub const fn SYSTICKCLKSEL1(
        self,
    ) -> crate::common::Reg<regs::SYSTICKCLKSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn SYSTICKCLKSELX1(
        self,
    ) -> crate::common::Reg<regs::SYSTICKCLKSELX1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Trace clock source select."]
    #[inline(always)]
    pub const fn TRACECLKSEL(self) -> crate::common::Reg<regs::TRACECLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "CTimer 0 clock source select."]
    #[inline(always)]
    pub const fn CTIMERCLKSEL0(self) -> crate::common::Reg<regs::CTIMERCLKSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn CTIMERCLKSELX0(
        self,
    ) -> crate::common::Reg<regs::CTIMERCLKSELX0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "CTimer 1 clock source select."]
    #[inline(always)]
    pub const fn CTIMERCLKSEL1(self) -> crate::common::Reg<regs::CTIMERCLKSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn CTIMERCLKSELX1(
        self,
    ) -> crate::common::Reg<regs::CTIMERCLKSELX1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "CTimer 2 clock source select."]
    #[inline(always)]
    pub const fn CTIMERCLKSEL2(self) -> crate::common::Reg<regs::CTIMERCLKSEL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn CTIMERCLKSELX2(
        self,
    ) -> crate::common::Reg<regs::CTIMERCLKSELX2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "CTimer 3 clock source select."]
    #[inline(always)]
    pub const fn CTIMERCLKSEL3(self) -> crate::common::Reg<regs::CTIMERCLKSEL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn CTIMERCLKSELX3(
        self,
    ) -> crate::common::Reg<regs::CTIMERCLKSELX3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "CTimer 4 clock source select."]
    #[inline(always)]
    pub const fn CTIMERCLKSEL4(self) -> crate::common::Reg<regs::CTIMERCLKSEL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn CTIMERCLKSELX4(
        self,
    ) -> crate::common::Reg<regs::CTIMERCLKSELX4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Main clock A source select."]
    #[inline(always)]
    pub const fn MAINCLKSELA(self) -> crate::common::Reg<regs::MAINCLKSELA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Main clock source select."]
    #[inline(always)]
    pub const fn MAINCLKSELB(self) -> crate::common::Reg<regs::MAINCLKSELB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "CLKOUT clock source select."]
    #[inline(always)]
    pub const fn CLKOUTSEL(self) -> crate::common::Reg<regs::CLKOUTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "PLL0 clock source select."]
    #[inline(always)]
    pub const fn PLL0CLKSEL(self) -> crate::common::Reg<regs::PLL0CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "PLL1 clock source select."]
    #[inline(always)]
    pub const fn PLL1CLKSEL(self) -> crate::common::Reg<regs::PLL1CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "ADC clock source select."]
    #[inline(always)]
    pub const fn ADCCLKSEL(self) -> crate::common::Reg<regs::ADCCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "FS USB clock source select."]
    #[inline(always)]
    pub const fn USB0CLKSEL(self) -> crate::common::Reg<regs::USB0CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL0(self) -> crate::common::Reg<regs::FCCLKSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX0(self) -> crate::common::Reg<regs::FCCLKSELX0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL1(self) -> crate::common::Reg<regs::FCCLKSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX1(self) -> crate::common::Reg<regs::FCCLKSELX1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL2(self) -> crate::common::Reg<regs::FCCLKSEL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX2(self) -> crate::common::Reg<regs::FCCLKSELX2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL3(self) -> crate::common::Reg<regs::FCCLKSEL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX3(self) -> crate::common::Reg<regs::FCCLKSELX3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL4(self) -> crate::common::Reg<regs::FCCLKSEL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX4(self) -> crate::common::Reg<regs::FCCLKSELX4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL5(self) -> crate::common::Reg<regs::FCCLKSEL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX5(self) -> crate::common::Reg<regs::FCCLKSELX5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL6(self) -> crate::common::Reg<regs::FCCLKSEL6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX6(self) -> crate::common::Reg<regs::FCCLKSELX6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn FCCLKSEL7(self) -> crate::common::Reg<regs::FCCLKSEL7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FCCLKSELX7(self) -> crate::common::Reg<regs::FCCLKSELX7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "HS LSPI clock source select."]
    #[inline(always)]
    pub const fn HSLSPICLKSEL(self) -> crate::common::Reg<regs::HSLSPICLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "MCLK clock source select."]
    #[inline(always)]
    pub const fn MCLKCLKSEL(self) -> crate::common::Reg<regs::MCLKCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "SCTimer/PWM clock source select."]
    #[inline(always)]
    pub const fn SCTCLKSEL(self) -> crate::common::Reg<regs::SCTCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "SDIO clock source select."]
    #[inline(always)]
    pub const fn SDIOCLKSEL(self) -> crate::common::Reg<regs::SDIOCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "System Tick Timer divider for CPU0."]
    #[inline(always)]
    pub const fn SYSTICKCLKDIV0(
        self,
    ) -> crate::common::Reg<regs::SYSTICKCLKDIV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "System Tick Timer divider for CPU1."]
    #[inline(always)]
    pub const fn SYSTICKCLKDIV1(
        self,
    ) -> crate::common::Reg<regs::SYSTICKCLKDIV1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "TRACE clock divider."]
    #[inline(always)]
    pub const fn TRACECLKDIV(self) -> crate::common::Reg<regs::TRACECLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 0."]
    #[inline(always)]
    pub const fn FLEXFRG0CTRL(self) -> crate::common::Reg<regs::FLEXFRG0CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL0(self) -> crate::common::Reg<regs::FLEXFRGXCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 1."]
    #[inline(always)]
    pub const fn FLEXFRG1CTRL(self) -> crate::common::Reg<regs::FLEXFRG1CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL1(self) -> crate::common::Reg<regs::FLEXFRGXCTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 2."]
    #[inline(always)]
    pub const fn FLEXFRG2CTRL(self) -> crate::common::Reg<regs::FLEXFRG2CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL2(self) -> crate::common::Reg<regs::FLEXFRGXCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 3."]
    #[inline(always)]
    pub const fn FLEXFRG3CTRL(self) -> crate::common::Reg<regs::FLEXFRG3CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL3(self) -> crate::common::Reg<regs::FLEXFRGXCTRL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 4."]
    #[inline(always)]
    pub const fn FLEXFRG4CTRL(self) -> crate::common::Reg<regs::FLEXFRG4CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL4(self) -> crate::common::Reg<regs::FLEXFRGXCTRL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 5."]
    #[inline(always)]
    pub const fn FLEXFRG5CTRL(self) -> crate::common::Reg<regs::FLEXFRG5CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL5(self) -> crate::common::Reg<regs::FLEXFRGXCTRL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 6."]
    #[inline(always)]
    pub const fn FLEXFRG6CTRL(self) -> crate::common::Reg<regs::FLEXFRG6CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL6(self) -> crate::common::Reg<regs::FLEXFRGXCTRL6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 7."]
    #[inline(always)]
    pub const fn FLEXFRG7CTRL(self) -> crate::common::Reg<regs::FLEXFRG7CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "Peripheral reset control register."]
    #[inline(always)]
    pub const fn FLEXFRGXCTRL7(self) -> crate::common::Reg<regs::FLEXFRGXCTRL7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "System clock divider."]
    #[inline(always)]
    pub const fn AHBCLKDIV(self) -> crate::common::Reg<regs::AHBCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "CLKOUT clock divider."]
    #[inline(always)]
    pub const fn CLKOUTDIV(self) -> crate::common::Reg<regs::CLKOUTDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "FRO_HF (96MHz) clock divider."]
    #[inline(always)]
    pub const fn FROHFDIV(self) -> crate::common::Reg<regs::FROHFDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "WDT clock divider."]
    #[inline(always)]
    pub const fn WDTCLKDIV(self) -> crate::common::Reg<regs::WDTCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "ADC clock divider."]
    #[inline(always)]
    pub const fn ADCCLKDIV(self) -> crate::common::Reg<regs::ADCCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "USB0 Clock divider."]
    #[inline(always)]
    pub const fn USB0CLKDIV(self) -> crate::common::Reg<regs::USB0CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "I2S MCLK clock divider."]
    #[inline(always)]
    pub const fn MCLKDIV(self) -> crate::common::Reg<regs::MCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "SCT/PWM clock divider."]
    #[inline(always)]
    pub const fn SCTCLKDIV(self) -> crate::common::Reg<regs::SCTCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "SDIO clock divider."]
    #[inline(always)]
    pub const fn SDIOCLKDIV(self) -> crate::common::Reg<regs::SDIOCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "PLL0 clock divider."]
    #[inline(always)]
    pub const fn PLL0CLKDIV(self) -> crate::common::Reg<regs::PLL0CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub const fn CLOCKGENUPDATELOCKOUT(
        self,
    ) -> crate::common::Reg<regs::CLOCKGENUPDATELOCKOUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "FMC configuration register."]
    #[inline(always)]
    pub const fn FMCCR(self) -> crate::common::Reg<regs::FMCCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "USB0 need clock control."]
    #[inline(always)]
    pub const fn USB0NEEDCLKCTRL(
        self,
    ) -> crate::common::Reg<regs::USB0NEEDCLKCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "USB0 need clock status."]
    #[inline(always)]
    pub const fn USB0NEEDCLKSTAT(
        self,
    ) -> crate::common::Reg<regs::USB0NEEDCLKSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "FMCflush control."]
    #[inline(always)]
    pub const fn FMCFLUSH(self) -> crate::common::Reg<regs::FMCFLUSH, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "MCLK control."]
    #[inline(always)]
    pub const fn MCLKIO(self) -> crate::common::Reg<regs::MCLKIO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "USB1 need clock control."]
    #[inline(always)]
    pub const fn USB1NEEDCLKCTRL(
        self,
    ) -> crate::common::Reg<regs::USB1NEEDCLKCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "USB1 need clock status."]
    #[inline(always)]
    pub const fn USB1NEEDCLKSTAT(
        self,
    ) -> crate::common::Reg<regs::USB1NEEDCLKSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "SDIO CCLKIN phase and delay control."]
    #[inline(always)]
    pub const fn SDIOCLKCTRL(self) -> crate::common::Reg<regs::SDIOCLKCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "PLL1 550m control."]
    #[inline(always)]
    pub const fn PLL1CTRL(self) -> crate::common::Reg<regs::PLL1CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "PLL1 550m status."]
    #[inline(always)]
    pub const fn PLL1STAT(self) -> crate::common::Reg<regs::PLL1STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "PLL1 550m N divider."]
    #[inline(always)]
    pub const fn PLL1NDEC(self) -> crate::common::Reg<regs::PLL1NDEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "PLL1 550m M divider."]
    #[inline(always)]
    pub const fn PLL1MDEC(self) -> crate::common::Reg<regs::PLL1MDEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "PLL1 550m P divider."]
    #[inline(always)]
    pub const fn PLL1PDEC(self) -> crate::common::Reg<regs::PLL1PDEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "PLL0 550m control."]
    #[inline(always)]
    pub const fn PLL0CTRL(self) -> crate::common::Reg<regs::PLL0CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "PLL0 550m status."]
    #[inline(always)]
    pub const fn PLL0STAT(self) -> crate::common::Reg<regs::PLL0STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "PLL0 550m N divider."]
    #[inline(always)]
    pub const fn PLL0NDEC(self) -> crate::common::Reg<regs::PLL0NDEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "PLL0 550m P divider."]
    #[inline(always)]
    pub const fn PLL0PDEC(self) -> crate::common::Reg<regs::PLL0PDEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "PLL0 Spread Spectrum Wrapper control register 0."]
    #[inline(always)]
    pub const fn PLL0SSCG0(self) -> crate::common::Reg<regs::PLL0SSCG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "PLL0 Spread Spectrum Wrapper control register 1."]
    #[inline(always)]
    pub const fn PLL0SSCG1(self) -> crate::common::Reg<regs::PLL0SSCG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "Functional retention control register."]
    #[inline(always)]
    pub const fn FUNCRETENTIONCTRL(
        self,
    ) -> crate::common::Reg<regs::FUNCRETENTIONCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "CPU Control for multiple processors."]
    #[inline(always)]
    pub const fn CPUCTRL(self) -> crate::common::Reg<regs::CPUCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Coprocessor Boot Address."]
    #[inline(always)]
    pub const fn CPBOOT(self) -> crate::common::Reg<regs::CPBOOT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "CPU Status."]
    #[inline(always)]
    pub const fn CPSTAT(self) -> crate::common::Reg<regs::CPSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures."]
    #[inline(always)]
    pub const fn CLOCK_CTRL(self) -> crate::common::Reg<regs::CLOCK_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a18usize) as _) }
    }
    #[doc = "Comparator Interrupt control."]
    #[inline(always)]
    pub const fn COMP_INT_CTRL(self) -> crate::common::Reg<regs::COMP_INT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "Comparator Interrupt status."]
    #[inline(always)]
    pub const fn COMP_INT_STATUS(
        self,
    ) -> crate::common::Reg<regs::COMP_INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Control automatic clock gating."]
    #[inline(always)]
    pub const fn AUTOCLKGATEOVERRIDE(
        self,
    ) -> crate::common::Reg<regs::AUTOCLKGATEOVERRIDE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub const fn GPIOPSYNC(self) -> crate::common::Reg<regs::GPIOPSYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e08usize) as _) }
    }
    #[doc = "Control write access to security registers."]
    #[inline(always)]
    pub const fn DEBUG_LOCK_EN(self) -> crate::common::Reg<regs::DEBUG_LOCK_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
    #[inline(always)]
    pub const fn DEBUG_FEATURES(
        self,
    ) -> crate::common::Reg<regs::DEBUG_FEATURES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
    #[inline(always)]
    pub const fn DEBUG_FEATURES_DP(
        self,
    ) -> crate::common::Reg<regs::DEBUG_FEATURES_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "block quiddikey/PUF all index."]
    #[inline(always)]
    pub const fn KEY_BLOCK(self) -> crate::common::Reg<regs::KEY_BLOCK, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "Debug authentication BEACON register."]
    #[inline(always)]
    pub const fn DEBUG_AUTH_BEACON(
        self,
    ) -> crate::common::Reg<regs::DEBUG_AUTH_BEACON, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "CPUs configuration register."]
    #[inline(always)]
    pub const fn CPUCFG(self) -> crate::common::Reg<regs::CPUCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Device ID."]
    #[inline(always)]
    pub const fn DEVICE_ID0(self) -> crate::common::Reg<regs::DEVICE_ID0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Chip revision ID and Number."]
    #[inline(always)]
    pub const fn DIEID(self) -> crate::common::Reg<regs::DIEID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
