#[doc = "SYSCON"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon0 {
    ptr: *mut u8,
}
unsafe impl Send for Syscon0 {}
unsafe impl Sync for Syscon0 {}
impl Syscon0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AHB Matrix Priority Control"]
    #[inline(always)]
    pub const fn ahbmatprio(self) -> crate::common::Reg<regs::Ahbmatprio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Secure CPU0 System Tick Calibration"]
    #[inline(always)]
    pub const fn cpu0stckcal(self) -> crate::common::Reg<regs::Cpu0stckcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration"]
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
    #[doc = "Peripheral Reset Control 0"]
    #[inline(always)]
    pub const fn presetctrl0(self) -> crate::common::Reg<regs::Presetctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Peripheral Reset Control 1"]
    #[inline(always)]
    pub const fn presetctrl1(self) -> crate::common::Reg<regs::Presetctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Peripheral Reset Control 2"]
    #[inline(always)]
    pub const fn presetctrl2(self) -> crate::common::Reg<regs::Presetctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Peripheral Reset Control 3"]
    #[inline(always)]
    pub const fn presetctrl3(self) -> crate::common::Reg<regs::Presetctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Peripheral Reset Control Set"]
    #[inline(always)]
    pub const fn presetctrlset(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Presetctrlset, crate::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "Peripheral Reset Control Clear"]
    #[inline(always)]
    pub const fn presetctrlclr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Presetctrlclr, crate::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Clock Control 0"]
    #[inline(always)]
    pub const fn ahbclkctrl0(self) -> crate::common::Reg<regs::Ahbclkctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AHB Clock Control 1"]
    #[inline(always)]
    pub const fn ahbclkctrl1(self) -> crate::common::Reg<regs::Ahbclkctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "AHB Clock Control 2"]
    #[inline(always)]
    pub const fn ahbclkctrl2(self) -> crate::common::Reg<regs::Ahbclkctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "AHB Clock Control 3"]
    #[inline(always)]
    pub const fn ahbclkctrl3(self) -> crate::common::Reg<regs::Ahbclkctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "AHB Clock Control Set"]
    #[inline(always)]
    pub const fn ahbclkctrlset(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ahbclkctrlset, crate::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Clock Control Clear"]
    #[inline(always)]
    pub const fn ahbclkctrlclr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ahbclkctrlclr, crate::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "CPU0 System Tick Timer Source Select"]
    #[inline(always)]
    pub const fn systickclksel0(
        self,
    ) -> crate::common::Reg<regs::Systickclksel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "CPU1 System Tick Timer Source Select"]
    #[inline(always)]
    pub const fn systickclksel1(
        self,
    ) -> crate::common::Reg<regs::Systickclksel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Trace Clock Source Select"]
    #[inline(always)]
    pub const fn traceclksel(self) -> crate::common::Reg<regs::Traceclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "CTIMER Clock Source Select"]
    #[inline(always)]
    pub const fn ctimerclksel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimerclksel, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize + n * 4usize) as _)
        }
    }
    #[doc = "CLKOUT Clock Source Select"]
    #[inline(always)]
    pub const fn clkoutsel(self) -> crate::common::Reg<regs::Clkoutsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "ADC0 Clock Source Select"]
    #[inline(always)]
    pub const fn adc0clksel(self) -> crate::common::Reg<regs::Adc0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "USB-FS Clock Source Select"]
    #[inline(always)]
    pub const fn usb0clksel(self) -> crate::common::Reg<regs::Usb0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "LP_FLEXCOMM Clock Source Select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel(self, n: usize) -> crate::common::Reg<regs::Fcclksel, crate::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize + n * 4usize) as _)
        }
    }
    #[doc = "SCTimer/PWM Clock Source Select"]
    #[inline(always)]
    pub const fn sctclksel(self) -> crate::common::Reg<regs::Sctclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "CPU0 System Tick Timer Divider"]
    #[inline(always)]
    pub const fn systickclkdiv0(
        self,
    ) -> crate::common::Reg<regs::Systickclkdiv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "CPU1 System Tick Timer Divider"]
    #[inline(always)]
    pub const fn systickclkdiv1(
        self,
    ) -> crate::common::Reg<regs::Systickclkdiv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "TRACE Clock Divider"]
    #[inline(always)]
    pub const fn traceclkdiv(self) -> crate::common::Reg<regs::Traceclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "TSI Function Clock Source Select"]
    #[inline(always)]
    pub const fn tsiclksel(self) -> crate::common::Reg<regs::Tsiclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "SINC FILTER Function Clock Source Select"]
    #[inline(always)]
    pub const fn sincfiltclksel(
        self,
    ) -> crate::common::Reg<regs::Sincfiltclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "SLOW_CLK Clock Divider"]
    #[inline(always)]
    pub const fn slowclkdiv(self) -> crate::common::Reg<regs::Slowclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "TSI Function Clock Divider"]
    #[inline(always)]
    pub const fn tsiclkdiv(self) -> crate::common::Reg<regs::Tsiclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "System Clock Divider"]
    #[inline(always)]
    pub const fn ahbclkdiv(self) -> crate::common::Reg<regs::Ahbclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "CLKOUT Clock Divider"]
    #[inline(always)]
    pub const fn clkoutdiv(self) -> crate::common::Reg<regs::Clkoutdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "FRO_HF_DIV Clock Divider"]
    #[inline(always)]
    pub const fn frohfdiv(self) -> crate::common::Reg<regs::Frohfdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "WDT0 Clock Divider"]
    #[inline(always)]
    pub const fn wdt0clkdiv(self) -> crate::common::Reg<regs::Wdt0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "ADC0 Clock Divider"]
    #[inline(always)]
    pub const fn adc0clkdiv(self) -> crate::common::Reg<regs::Adc0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "USB-FS Clock Divider"]
    #[inline(always)]
    pub const fn usb0clkdiv(self) -> crate::common::Reg<regs::Usb0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "SCT/PWM Clock Divider"]
    #[inline(always)]
    pub const fn sctclkdiv(self) -> crate::common::Reg<regs::Sctclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "PLL Clock Divider"]
    #[inline(always)]
    pub const fn pllclkdiv(self) -> crate::common::Reg<regs::Pllclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "CTimer Clock Divider"]
    #[inline(always)]
    pub const fn ctimerclkdiv(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimerclkdiv, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize + n * 4usize) as _)
        }
    }
    #[doc = "PLL1 Clock 0 Divider"]
    #[inline(always)]
    pub const fn pll1clk0div(self) -> crate::common::Reg<regs::Pll1clk0div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "PLL1 Clock 1 Divider"]
    #[inline(always)]
    pub const fn pll1clk1div(self) -> crate::common::Reg<regs::Pll1clk1div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "UTICK Clock Divider"]
    #[inline(always)]
    pub const fn utickclkdiv(self) -> crate::common::Reg<regs::Utickclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "CLKOUT FRG Control"]
    #[inline(always)]
    pub const fn clkout_frgctrl(
        self,
    ) -> crate::common::Reg<regs::ClkoutFrgctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Clock Configuration Unlock"]
    #[inline(always)]
    pub const fn clkunlock(self) -> crate::common::Reg<regs::Clkunlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "NVM Control"]
    #[inline(always)]
    pub const fn nvm_ctrl(self) -> crate::common::Reg<regs::NvmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "ROM Wait State"]
    #[inline(always)]
    pub const fn romcr(self) -> crate::common::Reg<regs::Romcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "SmartDMA Interrupt Hijack"]
    #[inline(always)]
    pub const fn smart_dmaint(self) -> crate::common::Reg<regs::SmartDmaint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "ADC1 Clock Source Select"]
    #[inline(always)]
    pub const fn adc1clksel(self) -> crate::common::Reg<regs::Adc1clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "ADC1 Clock Divider"]
    #[inline(always)]
    pub const fn adc1clkdiv(self) -> crate::common::Reg<regs::Adc1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "Control PKC RAM Interleave Access"]
    #[inline(always)]
    pub const fn ram_interleave(
        self,
    ) -> crate::common::Reg<regs::RamInterleave, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "DAC0 Functional Clock Selection"]
    #[inline(always)]
    pub const fn dac0clksel(self) -> crate::common::Reg<regs::Dac0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "DAC0 functional clock divider"]
    #[inline(always)]
    pub const fn dac0clkdiv(self) -> crate::common::Reg<regs::Dac0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize) as _) }
    }
    #[doc = "DAC1 Functional Clock Selection"]
    #[inline(always)]
    pub const fn dac1clksel(self) -> crate::common::Reg<regs::Dac1clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0498usize) as _) }
    }
    #[doc = "DAC1 functional clock divider"]
    #[inline(always)]
    pub const fn dac1clkdiv(self) -> crate::common::Reg<regs::Dac1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049cusize) as _) }
    }
    #[doc = "DAC2 Functional Clock Selection"]
    #[inline(always)]
    pub const fn dac2clksel(self) -> crate::common::Reg<regs::Dac2clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "DAC2 functional clock divider"]
    #[inline(always)]
    pub const fn dac2clkdiv(self) -> crate::common::Reg<regs::Dac2clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a4usize) as _) }
    }
    #[doc = "FlexSPI Clock Selection"]
    #[inline(always)]
    pub const fn flex_spiclksel(
        self,
    ) -> crate::common::Reg<regs::FlexSpiclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a8usize) as _) }
    }
    #[doc = "FlexSPI Clock Divider"]
    #[inline(always)]
    pub const fn flex_spiclkdiv(
        self,
    ) -> crate::common::Reg<regs::FlexSpiclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04acusize) as _) }
    }
    #[doc = "PLL Clock Divider Clock Selection"]
    #[inline(always)]
    pub const fn pllclkdivsel(self) -> crate::common::Reg<regs::Pllclkdivsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "I3C0 Functional Clock Selection"]
    #[inline(always)]
    pub const fn i3c0fclksel(self) -> crate::common::Reg<regs::I3c0fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "I3C0 FCLK_STC Clock Selection"]
    #[inline(always)]
    pub const fn i3c0fclkstcsel(
        self,
    ) -> crate::common::Reg<regs::I3c0fclkstcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "I3C0 FCLK_STC Clock Divider"]
    #[inline(always)]
    pub const fn i3c0fclkstcdiv(
        self,
    ) -> crate::common::Reg<regs::I3c0fclkstcdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "I3C0 FCLK Slow Clock Divider"]
    #[inline(always)]
    pub const fn i3c0fclksdiv(self) -> crate::common::Reg<regs::I3c0fclksdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "I3C0 Functional Clock FCLK Divider"]
    #[inline(always)]
    pub const fn i3c0fclkdiv(self) -> crate::common::Reg<regs::I3c0fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "I3C0 FCLK Slow Selection"]
    #[inline(always)]
    pub const fn i3c0fclkssel(self) -> crate::common::Reg<regs::I3c0fclkssel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "MICFIL Clock Selection"]
    #[inline(always)]
    pub const fn micfilfclksel(self) -> crate::common::Reg<regs::Micfilfclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "MICFIL Clock Division"]
    #[inline(always)]
    pub const fn micfilfclkdiv(self) -> crate::common::Reg<regs::Micfilfclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "uSDHC Clock Selection"]
    #[inline(always)]
    pub const fn u_sdhcclksel(self) -> crate::common::Reg<regs::USdhcclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "uSDHC Function Clock Divider"]
    #[inline(always)]
    pub const fn u_sdhcclkdiv(self) -> crate::common::Reg<regs::USdhcclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "FLEXIO Clock Selection"]
    #[inline(always)]
    pub const fn flexioclksel(self) -> crate::common::Reg<regs::Flexioclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "FLEXIO Function Clock Divider"]
    #[inline(always)]
    pub const fn flexioclkdiv(self) -> crate::common::Reg<regs::Flexioclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "FLEXCAN0 Clock Selection"]
    #[inline(always)]
    pub const fn flexcan0clksel(
        self,
    ) -> crate::common::Reg<regs::Flexcan0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "FLEXCAN0 Function Clock Divider"]
    #[inline(always)]
    pub const fn flexcan0clkdiv(
        self,
    ) -> crate::common::Reg<regs::Flexcan0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "FLEXCAN1 Clock Selection"]
    #[inline(always)]
    pub const fn flexcan1clksel(
        self,
    ) -> crate::common::Reg<regs::Flexcan1clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "FLEXCAN1 Function Clock Divider"]
    #[inline(always)]
    pub const fn flexcan1clkdiv(
        self,
    ) -> crate::common::Reg<regs::Flexcan1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "Ethernet RMII Clock Selection"]
    #[inline(always)]
    pub const fn enetrmiiclksel(
        self,
    ) -> crate::common::Reg<regs::Enetrmiiclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "Ethernet RMII Function Clock Divider"]
    #[inline(always)]
    pub const fn enetrmiiclkdiv(
        self,
    ) -> crate::common::Reg<regs::Enetrmiiclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "Ethernet PTP REF Clock Selection"]
    #[inline(always)]
    pub const fn enetptprefclksel(
        self,
    ) -> crate::common::Reg<regs::Enetptprefclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "Ethernet PTP REF Function Clock Divider"]
    #[inline(always)]
    pub const fn enetptprefclkdiv(
        self,
    ) -> crate::common::Reg<regs::Enetptprefclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "Ethernet PHY Interface Select"]
    #[inline(always)]
    pub const fn enet_phy_intf_sel(
        self,
    ) -> crate::common::Reg<regs::EnetPhyIntfSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "Sideband Flow Control"]
    #[inline(always)]
    pub const fn enet_sbd_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::EnetSbdFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "EWM0 Clock Selection"]
    #[inline(always)]
    pub const fn ewm0clksel(self) -> crate::common::Reg<regs::Ewm0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "WDT1 Clock Selection"]
    #[inline(always)]
    pub const fn wdt1clksel(self) -> crate::common::Reg<regs::Wdt1clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "WDT1 Function Clock Divider"]
    #[inline(always)]
    pub const fn wdt1clkdiv(self) -> crate::common::Reg<regs::Wdt1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "OSTIMER Clock Selection"]
    #[inline(always)]
    pub const fn ostimerclksel(self) -> crate::common::Reg<regs::Ostimerclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "CMP0 Function Clock Selection"]
    #[inline(always)]
    pub const fn cmp0fclksel(self) -> crate::common::Reg<regs::Cmp0fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "CMP0 Function Clock Divider"]
    #[inline(always)]
    pub const fn cmp0fclkdiv(self) -> crate::common::Reg<regs::Cmp0fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "CMP0 Round Robin Clock Selection"]
    #[inline(always)]
    pub const fn cmp0rrclksel(self) -> crate::common::Reg<regs::Cmp0rrclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "CMP0 Round Robin Clock Divider"]
    #[inline(always)]
    pub const fn cmp0rrclkdiv(self) -> crate::common::Reg<regs::Cmp0rrclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "CMP1 Function Clock Selection"]
    #[inline(always)]
    pub const fn cmp1fclksel(self) -> crate::common::Reg<regs::Cmp1fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "CMP1 Function Clock Divider"]
    #[inline(always)]
    pub const fn cmp1fclkdiv(self) -> crate::common::Reg<regs::Cmp1fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "CMP1 Round Robin Clock Source Select"]
    #[inline(always)]
    pub const fn cmp1rrclksel(self) -> crate::common::Reg<regs::Cmp1rrclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "CMP1 Round Robin Clock Division"]
    #[inline(always)]
    pub const fn cmp1rrclkdiv(self) -> crate::common::Reg<regs::Cmp1rrclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "CMP2 Function Clock Source Select"]
    #[inline(always)]
    pub const fn cmp2fclksel(self) -> crate::common::Reg<regs::Cmp2fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "CMP2 Function Clock Division"]
    #[inline(always)]
    pub const fn cmp2fclkdiv(self) -> crate::common::Reg<regs::Cmp2fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "CMP2 Round Robin Clock Source Select"]
    #[inline(always)]
    pub const fn cmp2rrclksel(self) -> crate::common::Reg<regs::Cmp2rrclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "CMP2 Round Robin Clock Division"]
    #[inline(always)]
    pub const fn cmp2rrclkdiv(self) -> crate::common::Reg<regs::Cmp2rrclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "CPU Control for Multiple Processors"]
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
    pub const fn cpustat(self) -> crate::common::Reg<regs::Cpustat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "LPCAC Control"]
    #[inline(always)]
    pub const fn lpcac_ctrl(self) -> crate::common::Reg<regs::LpcacCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "LP_FLEXCOMM Clock Divider"]
    #[inline(always)]
    pub const fn flexcommclkdiv(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flexcommclkdiv, crate::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize + n * 4usize) as _)
        }
    }
    #[doc = "UTICK Function Clock Source Select"]
    #[inline(always)]
    pub const fn utickclksel(self) -> crate::common::Reg<regs::Utickclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0878usize) as _) }
    }
    #[doc = "SAI0 Function Clock Source Select"]
    #[inline(always)]
    pub const fn sai0clksel(self) -> crate::common::Reg<regs::Sai0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "SAI1 Function Clock Source Select"]
    #[inline(always)]
    pub const fn sai1clksel(self) -> crate::common::Reg<regs::Sai1clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0884usize) as _) }
    }
    #[doc = "SAI0 Function Clock Division"]
    #[inline(always)]
    pub const fn sai0clkdiv(self) -> crate::common::Reg<regs::Sai0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0888usize) as _) }
    }
    #[doc = "SAI1 Function Clock Division"]
    #[inline(always)]
    pub const fn sai1clkdiv(self) -> crate::common::Reg<regs::Sai1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x088cusize) as _) }
    }
    #[doc = "EMVSIM0 Clock Source Select"]
    #[inline(always)]
    pub const fn emvsim0clksel(self) -> crate::common::Reg<regs::Emvsim0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0890usize) as _) }
    }
    #[doc = "EMVSIM1 Clock Source Select"]
    #[inline(always)]
    pub const fn emvsim1clksel(self) -> crate::common::Reg<regs::Emvsim1clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0894usize) as _) }
    }
    #[doc = "EMVSIM0 Function Clock Division"]
    #[inline(always)]
    pub const fn emvsim0clkdiv(self) -> crate::common::Reg<regs::Emvsim0clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0898usize) as _) }
    }
    #[doc = "EMVSIM1 Function Clock Division"]
    #[inline(always)]
    pub const fn emvsim1clkdiv(self) -> crate::common::Reg<regs::Emvsim1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x089cusize) as _) }
    }
    #[doc = "Key Retain Control"]
    #[inline(always)]
    pub const fn key_retain_ctrl(
        self,
    ) -> crate::common::Reg<regs::KeyRetainCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0950usize) as _) }
    }
    #[doc = "FRO 48MHz Reference Clock Control"]
    #[inline(always)]
    pub const fn ref_clk_ctrl(self) -> crate::common::Reg<regs::RefClkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0960usize) as _) }
    }
    #[doc = "FRO 48MHz Reference Clock Control Set"]
    #[inline(always)]
    pub const fn ref_clk_ctrl_set(
        self,
    ) -> crate::common::Reg<regs::RefClkCtrlSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0964usize) as _) }
    }
    #[doc = "FRO 48MHz Reference Clock Control Clear"]
    #[inline(always)]
    pub const fn ref_clk_ctrl_clr(
        self,
    ) -> crate::common::Reg<regs::RefClkCtrlClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0968usize) as _) }
    }
    #[doc = "GDET Control Register"]
    #[inline(always)]
    pub const fn gdet_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::GdetCtrl, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x096cusize + n * 4usize) as _)
        }
    }
    #[doc = "ELS Asset Protection Register"]
    #[inline(always)]
    pub const fn els_asset_prot(self) -> crate::common::Reg<regs::ElsAssetProt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0974usize) as _) }
    }
    #[doc = "ELS Lock Control"]
    #[inline(always)]
    pub const fn els_lock_ctrl(self) -> crate::common::Reg<regs::ElsLockCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0978usize) as _) }
    }
    #[doc = "ELS Lock Control DP"]
    #[inline(always)]
    pub const fn els_lock_ctrl_dp(
        self,
    ) -> crate::common::Reg<regs::ElsLockCtrlDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x097cusize) as _) }
    }
    #[doc = "Life Cycle State Register"]
    #[inline(always)]
    pub const fn els_otp_lc_state(
        self,
    ) -> crate::common::Reg<regs::ElsOtpLcState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[doc = "Life Cycle State Register (Duplicate)"]
    #[inline(always)]
    pub const fn els_otp_lc_state_dp(
        self,
    ) -> crate::common::Reg<regs::ElsOtpLcStateDp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0984usize) as _) }
    }
    #[doc = "ELS Temporal State"]
    #[inline(always)]
    pub const fn els_temporal_state(
        self,
    ) -> crate::common::Reg<regs::ElsTemporalState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0988usize) as _) }
    }
    #[doc = "Key Derivation Function Mask"]
    #[inline(always)]
    pub const fn els_kdf_mask(self) -> crate::common::Reg<regs::ElsKdfMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x098cusize) as _) }
    }
    #[doc = "ELS AS Configuration"]
    #[inline(always)]
    pub const fn els_as_cfg0(self) -> crate::common::Reg<regs::ElsAsCfg0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d0usize) as _) }
    }
    #[doc = "ELS AS Configuration1"]
    #[inline(always)]
    pub const fn els_as_cfg1(self) -> crate::common::Reg<regs::ElsAsCfg1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d4usize) as _) }
    }
    #[doc = "ELS AS Configuration2"]
    #[inline(always)]
    pub const fn els_as_cfg2(self) -> crate::common::Reg<regs::ElsAsCfg2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d8usize) as _) }
    }
    #[doc = "ELS AS Configuration3"]
    #[inline(always)]
    pub const fn els_as_cfg3(self) -> crate::common::Reg<regs::ElsAsCfg3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09dcusize) as _) }
    }
    #[doc = "ELS AS State Register"]
    #[inline(always)]
    pub const fn els_as_st0(self) -> crate::common::Reg<regs::ElsAsSt0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
    #[doc = "ELS AS State1"]
    #[inline(always)]
    pub const fn els_as_st1(self) -> crate::common::Reg<regs::ElsAsSt1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e4usize) as _) }
    }
    #[doc = "Boot state captured during boot: Main ROM log"]
    #[inline(always)]
    pub const fn els_as_boot_log0(
        self,
    ) -> crate::common::Reg<regs::ElsAsBootLog0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e8usize) as _) }
    }
    #[doc = "Boot state captured during boot: Library log"]
    #[inline(always)]
    pub const fn els_as_boot_log1(
        self,
    ) -> crate::common::Reg<regs::ElsAsBootLog1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09ecusize) as _) }
    }
    #[doc = "Boot state captured during boot: Hardware status signals log"]
    #[inline(always)]
    pub const fn els_as_boot_log2(
        self,
    ) -> crate::common::Reg<regs::ElsAsBootLog2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f0usize) as _) }
    }
    #[doc = "Boot state captured during boot: Security log"]
    #[inline(always)]
    pub const fn els_as_boot_log3(
        self,
    ) -> crate::common::Reg<regs::ElsAsBootLog3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f4usize) as _) }
    }
    #[doc = "ELS AS Flag0"]
    #[inline(always)]
    pub const fn els_as_flag0(self) -> crate::common::Reg<regs::ElsAsFlag0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f8usize) as _) }
    }
    #[doc = "ELS AS Flag1"]
    #[inline(always)]
    pub const fn els_as_flag1(self) -> crate::common::Reg<regs::ElsAsFlag1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09fcusize) as _) }
    }
    #[doc = "Clock Control"]
    #[inline(always)]
    pub const fn clock_ctrl(self) -> crate::common::Reg<regs::ClockCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a18usize) as _) }
    }
    #[doc = "I3C1 Functional Clock Selection"]
    #[inline(always)]
    pub const fn i3c1fclksel(self) -> crate::common::Reg<regs::I3c1fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "Selects the I3C1 Time Control clock"]
    #[inline(always)]
    pub const fn i3c1fclkstcsel(
        self,
    ) -> crate::common::Reg<regs::I3c1fclkstcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "I3C1 FCLK_STC Clock Divider"]
    #[inline(always)]
    pub const fn i3c1fclkstcdiv(
        self,
    ) -> crate::common::Reg<regs::I3c1fclkstcdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b38usize) as _) }
    }
    #[doc = "I3C1 FCLK Slow clock Divider"]
    #[inline(always)]
    pub const fn i3c1fclksdiv(self) -> crate::common::Reg<regs::I3c1fclksdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b3cusize) as _) }
    }
    #[doc = "I3C1 Functional Clock FCLK Divider"]
    #[inline(always)]
    pub const fn i3c1fclkdiv(self) -> crate::common::Reg<regs::I3c1fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[doc = "I3C1 FCLK Slow Selection"]
    #[inline(always)]
    pub const fn i3c1fclkssel(self) -> crate::common::Reg<regs::I3c1fclkssel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b44usize) as _) }
    }
    #[doc = "ETB Counter Status Register"]
    #[inline(always)]
    pub const fn etb_status(self) -> crate::common::Reg<regs::EtbStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b50usize) as _) }
    }
    #[doc = "ETB Counter Control Register"]
    #[inline(always)]
    pub const fn etb_counter_ctrl(
        self,
    ) -> crate::common::Reg<regs::EtbCounterCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b54usize) as _) }
    }
    #[doc = "ETB Counter Reload Register"]
    #[inline(always)]
    pub const fn etb_counter_reload(
        self,
    ) -> crate::common::Reg<regs::EtbCounterReload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[doc = "ETB Counter Value Register"]
    #[inline(always)]
    pub const fn etb_counter_value(
        self,
    ) -> crate::common::Reg<regs::EtbCounterValue, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray code_gray\\[31:0\\]"]
    #[inline(always)]
    pub const fn gray_code_lsb(self) -> crate::common::Reg<regs::GrayCodeLsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b60usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray code_gray\\[41:32\\]"]
    #[inline(always)]
    pub const fn gray_code_msb(self) -> crate::common::Reg<regs::GrayCodeMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b64usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[31:0\\]"]
    #[inline(always)]
    pub const fn binary_code_lsb(
        self,
    ) -> crate::common::Reg<regs::BinaryCodeLsb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
    #[inline(always)]
    pub const fn binary_code_msb(
        self,
    ) -> crate::common::Reg<regs::BinaryCodeMsb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "Control Automatic Clock Gating"]
    #[inline(always)]
    pub const fn autoclkgateoverride(
        self,
    ) -> crate::common::Reg<regs::Autoclkgateoverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating C"]
    #[inline(always)]
    pub const fn autoclkgateoverridec(
        self,
    ) -> crate::common::Reg<regs::Autoclkgateoverridec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e2cusize) as _) }
    }
    #[doc = "PWM0 Submodule Control"]
    #[inline(always)]
    pub const fn pwm0subctl(self) -> crate::common::Reg<regs::Pwm0subctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e38usize) as _) }
    }
    #[doc = "PWM1 Submodule Control"]
    #[inline(always)]
    pub const fn pwm1subctl(self) -> crate::common::Reg<regs::Pwm1subctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e3cusize) as _) }
    }
    #[doc = "CTIMER Global Start Enable"]
    #[inline(always)]
    pub const fn ctimerglobalstarten(
        self,
    ) -> crate::common::Reg<regs::Ctimerglobalstarten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e40usize) as _) }
    }
    #[doc = "RAM ECC Enable Control"]
    #[inline(always)]
    pub const fn ecc_enable_ctrl(
        self,
    ) -> crate::common::Reg<regs::EccEnableCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e44usize) as _) }
    }
    #[doc = "Control Write Access to Security"]
    #[inline(always)]
    pub const fn debug_lock_en(self) -> crate::common::Reg<regs::DebugLockEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Cortex Debug Features Control"]
    #[inline(always)]
    pub const fn debug_features(
        self,
    ) -> crate::common::Reg<regs::DebugFeatures, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Cortex Debug Features Control (Duplicate)"]
    #[inline(always)]
    pub const fn debug_features_dp(
        self,
    ) -> crate::common::Reg<regs::DebugFeaturesDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "CPU0 Software Debug Access"]
    #[inline(always)]
    pub const fn swd_access_cpu0(
        self,
    ) -> crate::common::Reg<regs::SwdAccessCpu0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
    #[doc = "CPU1 Software Debug Access"]
    #[inline(always)]
    pub const fn swd_access_cpu1(
        self,
    ) -> crate::common::Reg<regs::SwdAccessCpu1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb8usize) as _) }
    }
    #[doc = "Debug Authentication BEACON"]
    #[inline(always)]
    pub const fn debug_auth_beacon(
        self,
    ) -> crate::common::Reg<regs::DebugAuthBeacon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "DSP Software Debug Access"]
    #[inline(always)]
    pub const fn swd_access_dsp(self) -> crate::common::Reg<regs::SwdAccessDsp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "JTAG Chip ID"]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::common::Reg<regs::JtagId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Device Type"]
    #[inline(always)]
    pub const fn device_type(self) -> crate::common::Reg<regs::DeviceType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Device ID"]
    #[inline(always)]
    pub const fn device_id0(self) -> crate::common::Reg<regs::DeviceId0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Chip Revision ID and Number"]
    #[inline(always)]
    pub const fn dieid(self) -> crate::common::Reg<regs::Dieid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
