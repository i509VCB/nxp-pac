#[doc = "SYSCON"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon {
    ptr: *mut u8,
}
unsafe impl Send for Syscon {}
unsafe impl Sync for Syscon {}
impl Syscon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Memory Remap control register"]
    #[inline(always)]
    pub const fn memoryremap(self) -> crate::common::Reg<regs::Memoryremap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
    #[inline(always)]
    pub const fn ahbmatprio(self) -> crate::common::Reg<regs::Ahbmatprio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "System tick calibration for secure part of CPU0"]
    #[inline(always)]
    pub const fn cpu0stckcal(self) -> crate::common::Reg<regs::Cpu0stckcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "System tick calibration for non-secure part of CPU0"]
    #[inline(always)]
    pub const fn cpu0nstckcal(self) -> crate::common::Reg<regs::Cpu0nstckcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "System tick calibration for CPU1"]
    #[inline(always)]
    pub const fn cpu1stckcal(self) -> crate::common::Reg<regs::Cpu1stckcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "NMI Source Select"]
    #[inline(always)]
    pub const fn nmisrc(self) -> crate::common::Reg<regs::Nmisrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Peripheral reset control 0"]
    #[inline(always)]
    pub const fn presetctrl0(self) -> crate::common::Reg<regs::Presetctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Peripheral reset control 1"]
    #[inline(always)]
    pub const fn presetctrl1(self) -> crate::common::Reg<regs::Presetctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Peripheral reset control 2"]
    #[inline(always)]
    pub const fn presetctrl2(self) -> crate::common::Reg<regs::Presetctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Peripheral reset control set register"]
    #[inline(always)]
    pub const fn presetctrlset(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Presetctrlset, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "Peripheral reset control clear register"]
    #[inline(always)]
    pub const fn presetctrlclr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Presetctrlclr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "generate a software_reset"]
    #[inline(always)]
    pub const fn swr_reset(self) -> crate::common::Reg<regs::SwrReset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "AHB Clock control 0"]
    #[inline(always)]
    pub const fn ahbclkctrl0(self) -> crate::common::Reg<regs::Ahbclkctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AHB Clock control 1"]
    #[inline(always)]
    pub const fn ahbclkctrl1(self) -> crate::common::Reg<regs::Ahbclkctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "AHB Clock control 2"]
    #[inline(always)]
    pub const fn ahbclkctrl2(self) -> crate::common::Reg<regs::Ahbclkctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrlset(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ahbclkctrlset, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize + n * 4usize) as _)
        }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrlclr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ahbclkctrlclr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "System Tick Timer for CPU0 source select"]
    #[inline(always)]
    pub const fn systickclksel0(
        self,
    ) -> crate::common::Reg<regs::Systickclksel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn systickclkselx0(
        self,
    ) -> crate::common::Reg<regs::Systickclkselx0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "System Tick Timer for CPU1 source select"]
    #[inline(always)]
    pub const fn systickclksel1(
        self,
    ) -> crate::common::Reg<regs::Systickclksel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn systickclkselx1(
        self,
    ) -> crate::common::Reg<regs::Systickclkselx1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Trace clock source select"]
    #[inline(always)]
    pub const fn traceclksel(self) -> crate::common::Reg<regs::Traceclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "CTimer 0 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel0(self) -> crate::common::Reg<regs::Ctimerclksel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx0(
        self,
    ) -> crate::common::Reg<regs::Ctimerclkselx0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "CTimer 1 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel1(self) -> crate::common::Reg<regs::Ctimerclksel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx1(
        self,
    ) -> crate::common::Reg<regs::Ctimerclkselx1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "CTimer 2 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel2(self) -> crate::common::Reg<regs::Ctimerclksel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx2(
        self,
    ) -> crate::common::Reg<regs::Ctimerclkselx2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "CTimer 3 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel3(self) -> crate::common::Reg<regs::Ctimerclksel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx3(
        self,
    ) -> crate::common::Reg<regs::Ctimerclkselx3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "CTimer 4 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel4(self) -> crate::common::Reg<regs::Ctimerclksel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx4(
        self,
    ) -> crate::common::Reg<regs::Ctimerclkselx4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Main clock A source select"]
    #[inline(always)]
    pub const fn mainclksela(self) -> crate::common::Reg<regs::Mainclksela, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Main clock source select"]
    #[inline(always)]
    pub const fn mainclkselb(self) -> crate::common::Reg<regs::Mainclkselb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "CLKOUT clock source select"]
    #[inline(always)]
    pub const fn clkoutsel(self) -> crate::common::Reg<regs::Clkoutsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "PLL0 clock source select"]
    #[inline(always)]
    pub const fn pll0clksel(self) -> crate::common::Reg<regs::Pll0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "PLL1 clock source select"]
    #[inline(always)]
    pub const fn pll1clksel(self) -> crate::common::Reg<regs::Pll1clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "ADC clock source select"]
    #[inline(always)]
    pub const fn adcclksel(self) -> crate::common::Reg<regs::Adcclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "FS USB clock source select"]
    #[inline(always)]
    pub const fn usb0clksel(self) -> crate::common::Reg<regs::Usb0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcckkselx(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Fcclkselx, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize + n * 4usize) as _)
        }
    }
    #[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel(self, n: usize) -> crate::common::Reg<regs::Fcclksel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize + n * 4usize) as _)
        }
    }
    #[doc = "HS LSPI clock source select"]
    #[inline(always)]
    pub const fn hslspiclksel(self) -> crate::common::Reg<regs::Hslspiclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "MCLK clock source select"]
    #[inline(always)]
    pub const fn mclkclksel(self) -> crate::common::Reg<regs::Mclkclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "SCTimer/PWM clock source select"]
    #[inline(always)]
    pub const fn sctclksel(self) -> crate::common::Reg<regs::Sctclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "SDIO clock source select"]
    #[inline(always)]
    pub const fn sdioclksel(self) -> crate::common::Reg<regs::Sdioclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "System Tick Timer divider for CPU0"]
    #[inline(always)]
    pub const fn systickclkdiv0(
        self,
    ) -> crate::common::Reg<regs::Systickclkdiv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "System Tick Timer divider for CPU1"]
    #[inline(always)]
    pub const fn systickclkdiv1(
        self,
    ) -> crate::common::Reg<regs::Systickclkdiv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "TRACE clock divider"]
    #[inline(always)]
    pub const fn traceclkdiv(self) -> crate::common::Reg<regs::Traceclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Fractional rate divider for flexcomm 0"]
    #[inline(always)]
    pub const fn flexfrgctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flexfrgctrl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize + n * 4usize) as _)
        }
    }
    #[doc = "Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flexfrgxctrl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize + n * 4usize) as _)
        }
    }
    #[doc = "System clock divider"]
    #[inline(always)]
    pub const fn ahbclkdiv(self) -> crate::common::Reg<regs::Ahbclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "CLKOUT clock divider"]
    #[inline(always)]
    pub const fn clkoutdiv(self) -> crate::common::Reg<regs::Clkoutdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "FRO_HF (96MHz) clock divider"]
    #[inline(always)]
    pub const fn frohfdiv(self) -> crate::common::Reg<regs::Frohfdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "WDT clock divider"]
    #[inline(always)]
    pub const fn wdtclkdiv(self) -> crate::common::Reg<regs::Wdtclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "ADC clock divider"]
    #[inline(always)]
    pub const fn adcclkdiv(self) -> crate::common::Reg<regs::Adcclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "USB0 Clock divider"]
    #[inline(always)]
    pub const fn usb0clkdiv(self) -> crate::common::Reg<regs::Usb0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "I2S MCLK clock divider"]
    #[inline(always)]
    pub const fn mclkdiv(self) -> crate::common::Reg<regs::Mclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "SCT/PWM clock divider"]
    #[inline(always)]
    pub const fn sctclkdiv(self) -> crate::common::Reg<regs::Sctclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "SDIO clock divider"]
    #[inline(always)]
    pub const fn sdioclkdiv(self) -> crate::common::Reg<regs::Sdioclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "PLL0 clock divider"]
    #[inline(always)]
    pub const fn pll0clkdiv(self) -> crate::common::Reg<regs::Pll0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    #[inline(always)]
    pub const fn clockgenupdatelockout(
        self,
    ) -> crate::common::Reg<regs::Clockgenupdatelockout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "FMC configuration register"]
    #[inline(always)]
    pub const fn fmccr(self) -> crate::common::Reg<regs::Fmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "USB0 need clock control"]
    #[inline(always)]
    pub const fn usb0needclkctrl(
        self,
    ) -> crate::common::Reg<regs::Usb0needclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "USB0 need clock status"]
    #[inline(always)]
    pub const fn usb0needclkstat(
        self,
    ) -> crate::common::Reg<regs::Usb0needclkstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "FMCflush control"]
    #[inline(always)]
    pub const fn fmcflush(self) -> crate::common::Reg<regs::Fmcflush, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "MCLK control"]
    #[inline(always)]
    pub const fn mclkio(self) -> crate::common::Reg<regs::Mclkio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "USB1 need clock control"]
    #[inline(always)]
    pub const fn usb1needclkctrl(
        self,
    ) -> crate::common::Reg<regs::Usb1needclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "USB1 need clock status"]
    #[inline(always)]
    pub const fn usb1needclkstat(
        self,
    ) -> crate::common::Reg<regs::Usb1needclkstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "SDIO CCLKIN phase and delay control"]
    #[inline(always)]
    pub const fn sdioclkctrl(self) -> crate::common::Reg<regs::Sdioclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "PLL1 550m control"]
    #[inline(always)]
    pub const fn pll1ctrl(self) -> crate::common::Reg<regs::Pll1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "PLL1 550m status"]
    #[inline(always)]
    pub const fn pll1stat(self) -> crate::common::Reg<regs::Pll1stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "PLL1 550m N divider"]
    #[inline(always)]
    pub const fn pll1ndec(self) -> crate::common::Reg<regs::Pll1ndec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "PLL1 550m M divider"]
    #[inline(always)]
    pub const fn pll1mdec(self) -> crate::common::Reg<regs::Pll1mdec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "PLL1 550m P divider"]
    #[inline(always)]
    pub const fn pll1pdec(self) -> crate::common::Reg<regs::Pll1pdec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "PLL0 550m control"]
    #[inline(always)]
    pub const fn pll0ctrl(self) -> crate::common::Reg<regs::Pll0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "PLL0 550m status"]
    #[inline(always)]
    pub const fn pll0stat(self) -> crate::common::Reg<regs::Pll0stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "PLL0 550m N divider"]
    #[inline(always)]
    pub const fn pll0ndec(self) -> crate::common::Reg<regs::Pll0ndec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "PLL0 550m P divider"]
    #[inline(always)]
    pub const fn pll0pdec(self) -> crate::common::Reg<regs::Pll0pdec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
    #[inline(always)]
    pub const fn pll0sscg0(self) -> crate::common::Reg<regs::Pll0sscg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
    #[inline(always)]
    pub const fn pll0sscg1(self) -> crate::common::Reg<regs::Pll0sscg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "Functional retention control register"]
    #[inline(always)]
    pub const fn funcretentionctrl(
        self,
    ) -> crate::common::Reg<regs::Funcretentionctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "CPU Control for multiple processors"]
    #[inline(always)]
    pub const fn cpuctrl(self) -> crate::common::Reg<regs::Cpuctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Coprocessor Boot Address"]
    #[inline(always)]
    pub const fn cpboot(self) -> crate::common::Reg<regs::Cpboot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "CPU Status"]
    #[inline(always)]
    pub const fn cpstat(self) -> crate::common::Reg<regs::Cpstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
    #[inline(always)]
    pub const fn clock_ctrl(self) -> crate::common::Reg<regs::ClockCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a18usize) as _) }
    }
    #[doc = "Comparator Interrupt control"]
    #[inline(always)]
    pub const fn comp_int_ctrl(self) -> crate::common::Reg<regs::CompIntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "Comparator Interrupt status"]
    #[inline(always)]
    pub const fn comp_int_status(
        self,
    ) -> crate::common::Reg<regs::CompIntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Control automatic clock gating"]
    #[inline(always)]
    pub const fn autoclkgateoverride(
        self,
    ) -> crate::common::Reg<regs::Autoclkgateoverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
    #[inline(always)]
    pub const fn gpiopsync(self) -> crate::common::Reg<regs::Gpiopsync, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e08usize) as _) }
    }
    #[doc = "Control write access to security registers."]
    #[inline(always)]
    pub const fn debug_lock_en(self) -> crate::common::Reg<regs::DebugLockEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
    #[inline(always)]
    pub const fn debug_features(
        self,
    ) -> crate::common::Reg<regs::DebugFeatures, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
    #[inline(always)]
    pub const fn debug_features_dp(
        self,
    ) -> crate::common::Reg<regs::DebugFeaturesDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "block quiddikey/PUF all index."]
    #[inline(always)]
    pub const fn key_block(self) -> crate::common::Reg<regs::KeyBlock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "Debug authentication BEACON register"]
    #[inline(always)]
    pub const fn debug_auth_beacon(
        self,
    ) -> crate::common::Reg<regs::DebugAuthBeacon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "CPUs configuration register"]
    #[inline(always)]
    pub const fn cpucfg(self) -> crate::common::Reg<regs::Cpucfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Device ID"]
    #[inline(always)]
    pub const fn device_id0(self) -> crate::common::Reg<regs::DeviceId0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Chip revision ID and Number"]
    #[inline(always)]
    pub const fn dieid(self) -> crate::common::Reg<regs::Dieid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
